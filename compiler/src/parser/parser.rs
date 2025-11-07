// src/parser/parser.rs
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::ast::*;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub linha: usize,
    pub coluna: usize,
    pub expected: Vec<TokenType>,
    pub found: TokenType,
}

impl ParserError {
    pub fn new(message: String, linha: usize, coluna: usize, expected: Vec<TokenType>, found: TokenType) -> Self {
        Self {
            message,
            linha,
            coluna,
            expected,
            found,
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "ERRO SINTÁTICO: Linha {}, Coluna {} - {}\nEsperado: {:?}\nEncontrado: {:?}",
            self.linha, self.coluna, self.message, self.expected, self.found
        )
    }
}

impl std::error::Error for ParserError {}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    errors: Vec<ParserError>,
    lookahead_buffer: VecDeque<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Self, ParserError> {
        let current_token = lexer.next_token_for_parser()
            .map_err(|e| ParserError::new(
                e.to_string(), 
                0, 0, 
                vec![], 
                TokenType::EOF
            ))?;
        
        Ok(Self {
            lexer,
            current_token,
            errors: Vec::new(),
            lookahead_buffer: VecDeque::new(),
        })
    }

    // === MÉTODOS AUXILIARES ===

    fn advance(&mut self) -> Result<(), ParserError> {
        if let Some(token) = self.lookahead_buffer.pop_front() {
            self.current_token = token;
        } else {
            self.current_token = self.lexer.next_token_for_parser()
                .map_err(|e| ParserError::new(
                    e.to_string(),
                    self.current_token.linha,
                    self.current_token.coluna,
                    vec![],
                    self.current_token.token_type.clone()
                ))?;
        }
        Ok(())
    }

    fn consume(&mut self, expected: TokenType) -> Result<(), ParserError> {
        if self.current_token.token_type == expected {
            self.advance()?;
            Ok(())
        } else {
            let error = ParserError::new(
                format!("Token inesperado"),
                self.current_token.linha,
                self.current_token.coluna,
                vec![expected.clone()],
                self.current_token.token_type.clone()
            );
            self.errors.push(error.clone());
            Err(error)
        }
    }

    fn check(&self, expected: &TokenType) -> bool {
        &self.current_token.token_type == expected
    }

    fn check_any(&self, expected: &[TokenType]) -> bool {
        expected.iter().any(|t| &self.current_token.token_type == t)
    }

    fn expect(&mut self, expected: &TokenType) -> bool {
        if self.check(expected) {
            true
        } else {
            let error = ParserError::new(
                format!("Token inesperado"),
                self.current_token.linha,
                self.current_token.coluna,
                vec![expected.clone()],
                self.current_token.token_type.clone()
            );
            self.errors.push(error);
            false
        }
    }

    fn peek(&mut self, k: usize) -> Result<TokenType, ParserError> {
        while self.lookahead_buffer.len() <= k {
            let token = self.lexer.next_token_for_parser()
                .map_err(|e| ParserError::new(
                    e.to_string(),
                    self.current_token.linha,
                    self.current_token.coluna,
                    vec![],
                    self.current_token.token_type.clone()
                ))?;
            self.lookahead_buffer.push_back(token);
        }
        
        Ok(self.lookahead_buffer[k].token_type.clone())
    }

    fn sync_recovery(&mut self, sync_tokens: &[TokenType]) {
        let mut recovery_count = 0;
        let max_recovery_attempts = 50;
        
        while !self.check_any(sync_tokens) 
            && !self.check(&TokenType::EOF) 
            && recovery_count < max_recovery_attempts {
            
            if let Err(_) = self.advance() {
                break;
            }
            recovery_count += 1;
        }
        
        if recovery_count >= max_recovery_attempts {
            while !self.check(&TokenType::EOF) {
                if let Err(_) = self.advance() {
                    break;
                }
            }
        }
    }

    // === MÉTODOS DE PARSING PRINCIPAIS 

    pub fn parse_program(&mut self) -> Result<Program, Vec<ParserError>> { // correção de outro motivo de memory leaksss
        let mut functions = Vec::new();
        let mut statements = Vec::new();

        while !self.check(&TokenType::EOF) {
            if self.check(&TokenType::Funcao) {
                match self.parse_function_decl() {
                    Ok(func) => functions.push(func),
                    Err(e) => {
                        self.errors.push(e);
                        if let Err(_) = self.advance() {
                            break; 
                        }
                        self.sync_recovery(&[TokenType::Funcao, TokenType::EOF]);
                    }
                }
            } else {
                match self.parse_statement() {
                    Ok(stmt) => statements.push(stmt),
                    Err(e) => {
                        self.errors.push(e);
                        // Avançar pelo menos um token para evitar loop infinito
                        if let Err(_) = self.advance() {
                            break; // Se não conseguirmos avançar, sair do loop
                        }
                        self.sync_recovery(&[
                            TokenType::Funcao, TokenType::Inteiro, TokenType::Decimal, 
                            TokenType::Texto, TokenType::Logico, TokenType::EOF
                        ]);
                    }
                }
            }
        }

        if !self.errors.is_empty() {
            Err(std::mem::take(&mut self.errors))
        } else {
            Ok(Program { functions, statements })
        }
    }

    fn parse_function_decl(&mut self) -> Result<FunctionDecl, ParserError> {
        self.consume(TokenType::Funcao)?;

        let return_type = if self.check_any(&[TokenType::Inteiro, TokenType::Decimal, 
                                            TokenType::Texto, TokenType::Logico]) {
            Some(self.parse_type()?)
        } else {
            None
        };

        // Nome da função
        let name = if let TokenType::Identificador(name) = &self.current_token.token_type {
            let name = name.clone();
            self.advance()?;
            name
        } else {
            return Err(ParserError::new(
                "Esperado identificador para nome da função".to_string(),
                self.current_token.linha,
                self.current_token.coluna,
                vec![TokenType::Identificador("".to_string())],
                self.current_token.token_type.clone()
            ));
        };

        self.consume(TokenType::ParenteseEsquerdo)?;
        
        let parameters = self.parse_parameters()?;
        
        self.consume(TokenType::ParenteseDireito)?;
        
        let body = self.parse_block()?;

        Ok(FunctionDecl {
            return_type,
            name,
            parameters,
            body,
        })
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, ParserError> {
        let mut parameters = Vec::new();

        if self.check_any(&[TokenType::Inteiro, TokenType::Decimal, 
                           TokenType::Texto, TokenType::Logico]) {
            parameters.push(self.parse_parameter()?);

            while self.check(&TokenType::Virgula) {
                self.advance()?; // Consome a vírgula
                parameters.push(self.parse_parameter()?);
            }
        }

        Ok(parameters)
    }

    fn parse_parameter(&mut self) -> Result<Parameter, ParserError> {
        let param_type = self.parse_type()?;
        
        let name = if let TokenType::Identificador(name) = &self.current_token.token_type {
            let name = name.clone();
            self.advance()?;
            name
        } else {
            return Err(ParserError::new(
                "Esperado identificador para parâmetro".to_string(),
                self.current_token.linha,
                self.current_token.coluna,
                vec![TokenType::Identificador("".to_string())],
                self.current_token.token_type.clone()
            ));
        };

        Ok(Parameter { param_type, name })
    }

    // bloco ::= "{" (comando | declaracao_variavel)* "}"
    fn parse_block(&mut self) -> Result<Block, ParserError> {
        self.consume(TokenType::ChaveEsquerda)?;

        let mut statements = Vec::new();

        while !self.check(&TokenType::ChaveDireita) && !self.check(&TokenType::EOF) {
            match self.parse_statement_or_declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.errors.push(e);
                    self.sync_recovery(&[
                        TokenType::ChaveDireita, TokenType::Inteiro, TokenType::Decimal, 
                        TokenType::Texto, TokenType::Logico, TokenType::Se, TokenType::Enquanto, 
                        TokenType::Para, TokenType::Retorne, TokenType::Escreva, TokenType::Leia
                    ]);
                }
            }
        }

        self.consume(TokenType::ChaveDireita)?;

        Ok(Block { statements })
    }

    fn parse_statement_or_declaration(&mut self) -> Result<Statement, ParserError> {
        if self.check_any(&[TokenType::Inteiro, TokenType::Decimal, 
                           TokenType::Texto, TokenType::Logico]) {
            Ok(Statement::VariableDecl(self.parse_variable_decl()?))
        } else {
            self.parse_statement()
        }
    }

    // comando ::= expressao_comando | estrutura_controle | comando_retorno | comando_escreva | comando_leia
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token.token_type {
            TokenType::Se => Ok(Statement::IfStmt(self.parse_if_stmt()?)),
            TokenType::Enquanto => Ok(Statement::WhileStmt(self.parse_while_stmt()?)),
            TokenType::Para => Ok(Statement::ForStmt(self.parse_for_stmt()?)),
            TokenType::Retorne => Ok(Statement::ReturnStmt(self.parse_return_stmt()?)),
            TokenType::Escreva => Ok(Statement::WriteStmt(self.parse_write_stmt()?)),
            TokenType::Leia => Ok(Statement::ReadStmt(self.parse_read_stmt()?)),
            _ => Ok(Statement::ExprStmt(self.parse_expr_stmt()?)),
        }
    }

    // declaracao_variavel ::= tipo identificador ("=" expressao)? ";"
    fn parse_variable_decl(&mut self) -> Result<VariableDecl, ParserError> {
        let var_type = self.parse_type()?;
        
        let name = if let TokenType::Identificador(name) = &self.current_token.token_type {
            let name = name.clone();
            self.advance()?;
            name
        } else {
            return Err(ParserError::new(
                "Esperado identificador para variável".to_string(),
                self.current_token.linha,
                self.current_token.coluna,
                vec![TokenType::Identificador("".to_string())],
                self.current_token.token_type.clone()
            ));
        };

        let initializer = if self.check(&TokenType::Atribuicao) {
            self.advance()?; // Consome "="
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(TokenType::PontoEVirgula)?;

        Ok(VariableDecl {
            var_type,
            name,
            initializer,
        })
    }

    // expressao_comando ::= expressao ";"
    fn parse_expr_stmt(&mut self) -> Result<ExprStmt, ParserError> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::PontoEVirgula)?;
        Ok(ExprStmt { expr })
    }

    // condicional ::= "se" expressao "entao" bloco ("senao" bloco)? "fimse"
    fn parse_if_stmt(&mut self) -> Result<IfStmt, ParserError> {
        self.consume(TokenType::Se)?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::Entao)?;
        let then_branch = self.parse_block()?;

        let else_branch = if self.check(&TokenType::Senao) {
            self.advance()?;
            Some(self.parse_block()?)
        } else {
            None
        };

        self.consume(TokenType::FimSe)?;

        Ok(IfStmt {
            condition,
            then_branch,
            else_branch,
        })
    }

    // loop_enquanto ::= "enquanto" expressao "faca" bloco "fimenquanto"
    fn parse_while_stmt(&mut self) -> Result<WhileStmt, ParserError> {
        self.consume(TokenType::Enquanto)?;
        let condition = self.parse_expression()?;
        self.consume(TokenType::Faca)?;
        let body = self.parse_block()?;
        self.consume(TokenType::FimEnquanto)?;

        Ok(WhileStmt { condition, body })
    }

    // loop_para ::= "para" identificador "de" expressao "ate" expressao "faca" bloco "fimpara"
    fn parse_for_stmt(&mut self) -> Result<ForStmt, ParserError> {
        self.consume(TokenType::Para)?;

        let variable = if let TokenType::Identificador(name) = &self.current_token.token_type {
            let name = name.clone();
            self.advance()?;
            name
        } else {
            return Err(ParserError::new(
                "Esperado identificador para variável do loop".to_string(),
                self.current_token.linha,
                self.current_token.coluna,
                vec![TokenType::Identificador("".to_string())],
                self.current_token.token_type.clone()
            ));
        };

        self.consume(TokenType::De)?;
        let start = self.parse_expression()?;
        self.consume(TokenType::Ate)?;
        let end = self.parse_expression()?;
        self.consume(TokenType::Faca)?;
        let body = self.parse_block()?;
        self.consume(TokenType::FimPara)?;

        Ok(ForStmt {
            variable,
            start,
            end,
            body,
        })
    }

    // comando_retorno ::= "retorne" expressao? ";"
    fn parse_return_stmt(&mut self) -> Result<ReturnStmt, ParserError> {
        self.consume(TokenType::Retorne)?;

        let value = if !self.check(&TokenType::PontoEVirgula) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(TokenType::PontoEVirgula)?;

        Ok(ReturnStmt { value })
    }

    // comando_escreva ::= "escreva" "(" argumentos ")" ";"
    fn parse_write_stmt(&mut self) -> Result<WriteStmt, ParserError> {
        self.consume(TokenType::Escreva)?;
        self.consume(TokenType::ParenteseEsquerdo)?;
        let arguments = self.parse_arguments()?;
        self.consume(TokenType::ParenteseDireito)?;
        self.consume(TokenType::PontoEVirgula)?;

        Ok(WriteStmt { arguments })
    }

    // comando_leia ::= "leia" "(" expressao ")" ";"
    fn parse_read_stmt(&mut self) -> Result<ReadStmt, ParserError> {
        self.consume(TokenType::Leia)?;
        self.consume(TokenType::ParenteseEsquerdo)?;
        let target = self.parse_expression()?;
        self.consume(TokenType::ParenteseDireito)?;
        self.consume(TokenType::PontoEVirgula)?;

        Ok(ReadStmt { target })
    }

    // === EXPRESSÕES (com precedência) ===

    // expressao ::= expressao_logica
    fn parse_expression(&mut self) -> Result<Expr, ParserError> {
        self.parse_logical_or()
    }

    // expressao_logica ::= expressao_logica_aux ( ("&&" | "||") expressao_logica_aux )*
    fn parse_logical_or(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_logical_and()?;

        while self.check(&TokenType::OuLogico) {
            let op = BinaryOperator::Or;
            self.advance()?;
            let right = self.parse_logical_and()?;
            left = Expr::BinaryOp(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_equality()?;

        while self.check(&TokenType::ELogico) {
            let op = BinaryOperator::And;
            self.advance()?;
            let right = self.parse_equality()?;
            left = Expr::BinaryOp(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    // expressao_relacional ::= expressao_aritmetica (operador_relacional expressao_aritmetica)?
    fn parse_equality(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_relational()?;

        while self.check_any(&[TokenType::Igual, TokenType::Diferente]) {
            let op = match self.current_token.token_type {
                TokenType::Igual => BinaryOperator::Equal,
                TokenType::Diferente => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            self.advance()?;
            let right = self.parse_relational()?;
            left = Expr::BinaryOp(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_relational(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_additive()?;

        while self.check_any(&[TokenType::Menor, TokenType::MenorIgual, 
                              TokenType::Maior, TokenType::MaiorIgual]) {
            let op = match self.current_token.token_type {
                TokenType::Menor => BinaryOperator::Less,
                TokenType::MenorIgual => BinaryOperator::LessEqual,
                TokenType::Maior => BinaryOperator::Greater,
                TokenType::MaiorIgual => BinaryOperator::GreaterEqual,
                _ => unreachable!(),
            };
            self.advance()?;
            let right = self.parse_additive()?;
            left = Expr::BinaryOp(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    // expressao_aritmetica ::= expressao_aritmetica ("+" | "-") termo | termo
    fn parse_additive(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_multiplicative()?;

        while self.check_any(&[TokenType::Mais, TokenType::Menos]) {
            let op = match self.current_token.token_type {
                TokenType::Mais => BinaryOperator::Add,
                TokenType::Menos => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            self.advance()?;
            let right = self.parse_multiplicative()?;
            left = Expr::BinaryOp(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_unary()?;

        while self.check_any(&[TokenType::Multiplicacao, TokenType::Divisao, TokenType::Modulo]) {
            let op = match self.current_token.token_type {
                TokenType::Multiplicacao => BinaryOperator::Multiply,
                TokenType::Divisao => BinaryOperator::Divide,
                TokenType::Modulo => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            self.advance()?;
            let right = self.parse_unary()?;
            left = Expr::BinaryOp(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    // fator ::= operador_unario fator | base_fator
    fn parse_unary(&mut self) -> Result<Expr, ParserError> {
        if self.check_any(&[TokenType::Menos, TokenType::Mais, TokenType::NaoLogico]) {
            let op = match self.current_token.token_type {
                TokenType::Menos => UnaryOperator::Negate,
                TokenType::Mais => UnaryOperator::Plus,
                TokenType::NaoLogico => UnaryOperator::Not,
                _ => unreachable!(),
            };
            self.advance()?;
            let expr = self.parse_unary()?;
            Ok(Expr::UnaryOp(op, Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    // base_fator ::= literal | identificador | chamada_funcao | "(" expressao ")"
    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        match &self.current_token.token_type {
            TokenType::InteiroLiteral(n) => {
                let value = *n;
                self.advance()?;
                Ok(Expr::Literal(Literal::Inteiro(value)))
            }
            TokenType::DecimalLiteral(n) => {
                let value = *n;
                self.advance()?;
                Ok(Expr::Literal(Literal::Decimal(value)))
            }
            TokenType::StringLiteral(s) => {
                let value = s.clone();
                self.advance()?;
                Ok(Expr::Literal(Literal::Texto(value)))
            }
            TokenType::Verdadeiro => {
                self.advance()?;
                Ok(Expr::Literal(Literal::Logico(true)))
            }
            TokenType::Falso => {
                self.advance()?;
                Ok(Expr::Literal(Literal::Logico(false)))
            }
            TokenType::Identificador(name) => {
                let name = name.clone();
                self.advance()?;

                // Verifica se é chamada de função
                if self.check(&TokenType::ParenteseEsquerdo) {
                    self.consume(TokenType::ParenteseEsquerdo)?;
                    let arguments = self.parse_arguments()?;
                    self.consume(TokenType::ParenteseDireito)?;
                    Ok(Expr::Call(CallExpr { function: name, arguments }))
                } else {
                    Ok(Expr::Variable(name))
                }
            }
            TokenType::ParenteseEsquerdo => {
                self.advance()?;
                let expr = self.parse_expression()?;
                self.consume(TokenType::ParenteseDireito)?;
                Ok(expr)
            }
            _ => Err(ParserError::new(
                "Esperado expressão".to_string(),
                self.current_token.linha,
                self.current_token.coluna,
                vec![
                    TokenType::InteiroLiteral(0),
                    TokenType::DecimalLiteral(0.0),
                    TokenType::StringLiteral("".to_string()),
                    TokenType::Verdadeiro,
                    TokenType::Falso,
                    TokenType::Identificador("".to_string()),
                    TokenType::ParenteseEsquerdo,
                ],
                self.current_token.token_type.clone()
            )),
        }
    }

    // argumentos ::= expressao ("," expressao)*
    fn parse_arguments(&mut self) -> Result<Vec<Expr>, ParserError> {
        let mut arguments = Vec::new();

        if !self.check(&TokenType::ParenteseDireito) {
            arguments.push(self.parse_expression()?);

            while self.check(&TokenType::Virgula) {
                self.advance()?;
                arguments.push(self.parse_expression()?);
            }
        }

        Ok(arguments)
    }

    // tipo ::= "inteiro" | "decimal" | "texto" | "logico"
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        let token_type = self.current_token.token_type.clone();
        match token_type {
            TokenType::Inteiro => {
                self.advance()?;
                Ok(Type::Inteiro)
            }
            TokenType::Decimal => {
                self.advance()?;
                Ok(Type::Decimal)
            }
            TokenType::Texto => {
                self.advance()?;
                Ok(Type::Texto)
            }
            TokenType::Logico => {
                self.advance()?;
                Ok(Type::Logico)
            }
            _ => Err(ParserError::new(
                "Esperado tipo de dado".to_string(),
                self.current_token.linha,
                self.current_token.coluna,
                vec![TokenType::Inteiro, TokenType::Decimal, TokenType::Texto, TokenType::Logico],
                self.current_token.token_type.clone()
            )),
        }
    }

    pub fn get_errors(&self) -> &Vec<ParserError> {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    // Método utilitário para parsing direto de fonte
    pub fn parse_from_source(source: &str) -> Result<Program, Vec<ParserError>> {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer)
            .map_err(|e| vec![e])?;
        parser.parse_program()
    }
}

// Implementações de Debug para facilitar testes
impl std::fmt::Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parser{{current_token: {:?}, errors: {}}}", 
               self.current_token, self.errors.len())
    }
}