use std::collections::VecDeque;
use super::token::{Token, TokenType};
use super::error::LexerError;

pub struct Lexer {
    source: Vec<char>,
    current_pos: usize,
    current_line: usize,
    current_column: usize,
    tokens: Vec<Token>,
    lookahead_buffer: VecDeque<Token>,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        Self {
            source: source_code.chars().collect(),
            current_pos: 0,
            current_line: 1,
            current_column: 1,
            tokens: Vec::new(),
            lookahead_buffer: VecDeque::new(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        self.reset();

        while let Some(token) = self.next_token_internal()? {
            tokens.push(token);
        }

        tokens.push(Token::eof(self.current_line, self.current_column));
        Ok(tokens)
    }

    pub fn next_token_for_parser(&mut self) -> Result<Token, LexerError> { // correção de possssivel memory leak
        if let Some(token) = self.lookahead_buffer.pop_front() {
            Ok(token)
        } else {
            match self.next_token_internal()? {
                Some(token) => Ok(token),
                None => Ok(Token::eof(self.current_line, self.current_column))
            }
        }
    }

    pub fn peek(&mut self, k: usize) -> Result<Option<&Token>, LexerError> {
        while self.lookahead_buffer.len() <= k {
            match self.next_token_internal()? {
                Some(token) => self.lookahead_buffer.push_back(token),
                None => break,
            }
        }
        
        Ok(self.lookahead_buffer.get(k))
    }

    pub fn consume(&mut self) -> Result<Option<Token>, LexerError> {
        if let Some(token) = self.lookahead_buffer.pop_front() {
            Ok(Some(token))
        } else {
            self.next_token_internal()
        }
    }

    pub fn expect(&mut self, expected_type: TokenType) -> Result<bool, LexerError> {
        if let Some(next_token) = self.peek(0)? {
            Ok(next_token.token_type == expected_type)
        } else {
            Ok(false)
        }
    }

    pub fn reset(&mut self) {
        self.current_pos = 0;
        self.current_line = 1;
        self.current_column = 1;
        self.tokens.clear();
        self.lookahead_buffer.clear();
    }

    pub fn get_all_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        self.tokenize()
    }

    // --- IMPLEMENTAÇÃO INTERNA DOS AFDs ---

    fn next_token_internal(&mut self) -> Result<Option<Token>, LexerError> {

        self.skip_whitespace();
        
        if self.current_pos >= self.source.len() {
            return Ok(None);
        }
        
        let start_line = self.current_line;
        let start_column = self.current_column;
        
        // aplicacao do princípio do match mais longo
        if let Some(_token) = self.try_consume_comment()? {
            // Comentários são ignorados, chama recursivamente
            return self.next_token_internal();
        } else if let Some(token) = self.try_consume_string()? {
            Ok(Some(token))
        } else if let Some(token) = self.try_consume_number()? {
            Ok(Some(token))
        } else if let Some(token) = self.try_consume_operator()? {
            Ok(Some(token))
        } else if let Some(token) = self.try_consume_delimiter()? {
            Ok(Some(token))
        } else if let Some(token) = self.try_consume_keyword()? {
            Ok(Some(token))
        } else if let Some(token) = self.try_consume_identifier()? {
            Ok(Some(token))
        } else {
            // caractere invalido = recuperacao automatica
            let char_invalido = self.current_char();
            let erro = LexerError::with_recovery_suggestion(
                format!("Caractere inválido: '{}'", char_invalido),
                start_line,
                start_column,
                "Tente usar apenas caracteres válidos da linguagem Symplia".to_string(),
            );

            self.advance(); // AGORA funciona porque estamos dentro do lexer
            Err(erro)
        }
    }

    // AFD para Comentários
    fn try_consume_comment(&mut self) -> Result<Option<Token>, LexerError> {
        if self.current_char() == '/' && self.peek_next() == Some('/') {
            // Comentário de linha
            while self.current_pos < self.source.len() && self.current_char() != '\n' {
                self.advance();
            }
            Ok(Some(Token::eof(self.current_line, self.current_column)))
        } else if self.current_char() == '/' && self.peek_next() == Some('*') {
            // Comentário de bloco
            let start_line = self.current_line;
            let start_column = self.current_column;
            
            self.advance(); // /
            self.advance(); // *
            
            while self.current_pos + 1 < self.source.len() {
                if self.current_char() == '*' && self.peek_next() == Some('/') {
                    self.advance(); // *
                    self.advance(); // /
                    return Ok(Some(Token::eof(self.current_line, self.current_column)));
                }
                self.advance();
            }
            Err(LexerError::unclosed_comment(start_line, start_column))
        } else {
            Ok(None)
        }
    }

    // AFD para Strings
    fn try_consume_string(&mut self) -> Result<Option<Token>, LexerError> {
        if self.current_char() != '"' {
            return Ok(None);
        }

        let start_line = self.current_line;
        let start_column = self.current_column;
        let mut string_content = String::new();
        let mut escape = false;

        // Pular aspas inicial
        self.advance();

        while self.current_pos < self.source.len() {
            let char = self.current_char();

            if escape {
                match char {
                    'n' => string_content.push('\n'),
                    't' => string_content.push('\t'),
                    'r' => string_content.push('\r'),
                    '"' => string_content.push('"'),
                    '\\' => string_content.push('\\'),
                    _ => return Err(LexerError::invalid_escape(char, self.current_line, self.current_column)),
                }
                escape = false;
                self.advance();
            } else if char == '\\' {
                escape = true;
                self.advance();
            } else if char == '"' {
                // Fim da string
                self.advance();
                let lexema = format!("\"{}\"", string_content);
                return Ok(Some(Token::new(
                    TokenType::StringLiteral(string_content),
                    lexema,
                    start_line,
                    start_column,
                )));
            } else {
                string_content.push(char);
                self.advance();
            }
        }

        Err(LexerError::unterminated_string(start_line, start_column))
    }

    // AFD para Números (Inteiros e Decimais)
    fn try_consume_number(&mut self) -> Result<Option<Token>, LexerError> {
        if !self.current_char().is_ascii_digit() {
            return Ok(None);
        }

        let start_pos = self.current_pos;
        let start_line = self.current_line;
        let start_column = self.current_column;
        let mut has_decimal_point = false;

        // Consumir parte inteira
        while self.current_pos < self.source.len() {
            let char = self.current_char();
            if char.is_ascii_digit() {
                self.advance();
            } else if char == '.' && !has_decimal_point {
                has_decimal_point = true;
                self.advance();
                
                // Verificar se há dígitos após o ponto
                if self.current_pos >= self.source.len() || !self.current_char().is_ascii_digit() {
                    return Err(LexerError::invalid_number(
                        &self.source[start_pos..self.current_pos].iter().collect::<String>(),
                        start_line,
                        start_column,
                    ));
                }
            } else {
                break;
            }
        }

        let lexema: String = self.source[start_pos..self.current_pos].iter().collect();

        let token_type = if has_decimal_point {
            TokenType::DecimalLiteral(lexema.parse().map_err(|_| {
                LexerError::invalid_number(&lexema, start_line, start_column)
            })?)
        } else {
            TokenType::InteiroLiteral(lexema.parse().map_err(|_| {
                LexerError::invalid_number(&lexema, start_line, start_column)
            })?)
        };

        Ok(Some(Token::new(token_type, lexema, start_line, start_column)))
    }

    // AFD para Operadores (Maximal Munch)
    fn try_consume_operator(&mut self) -> Result<Option<Token>, LexerError> {
        let char_atual = self.current_char();
        
        // Operadores de 2 caracteres (mais longos primeiro)
        if self.current_pos + 1 < self.source.len() {
            let dois_chars = format!("{}{}", char_atual, self.source[self.current_pos + 1]);
            
            if let Some(token_type) = self.match_operator_duplo(&dois_chars) {
                let token = Token::new(
                    token_type,
                    dois_chars.clone(),
                    self.current_line,
                    self.current_column,
                );
                self.advance();
                self.advance();
                return Ok(Some(token));
            }
        }
        
        // Operadores de 1 caractere
        if let Some(token_type) = self.match_operator_simples(char_atual) {
            let token = Token::new(
                token_type,
                char_atual.to_string(),
                self.current_line,
                self.current_column,
            );
            self.advance();
            return Ok(Some(token));
        }
        
        Ok(None)
    }

    fn match_operator_duplo(&self, op: &str) -> Option<TokenType> {
        match op {
            "==" => Some(TokenType::Igual),
            "!=" => Some(TokenType::Diferente),
            "<=" => Some(TokenType::MenorIgual),
            ">=" => Some(TokenType::MaiorIgual),
            "&&" => Some(TokenType::ELogico),
            "||" => Some(TokenType::OuLogico),
            _ => None,
        }
    }

    fn match_operator_simples(&self, op: char) -> Option<TokenType> {
        match op {
            '+' => Some(TokenType::Mais),
            '-' => Some(TokenType::Menos),
            '*' => Some(TokenType::Multiplicacao),
            '/' => Some(TokenType::Divisao),
            '%' => Some(TokenType::Modulo),
            '=' => Some(TokenType::Atribuicao),
            '!' => Some(TokenType::NaoLogico),
            '<' => Some(TokenType::Menor),
            '>' => Some(TokenType::Maior),
            _ => None,
        }
    }

    // AFD para Delimitadores
    fn try_consume_delimiter(&mut self) -> Result<Option<Token>, LexerError> {
        let char_atual = self.current_char();
        let token_type = match char_atual {
            '(' => Some(TokenType::ParenteseEsquerdo),
            ')' => Some(TokenType::ParenteseDireito),
            '{' => Some(TokenType::ChaveEsquerda),
            '}' => Some(TokenType::ChaveDireita),
            '[' => Some(TokenType::ColcheteEsquerdo),
            ']' => Some(TokenType::ColcheteDireito),
            ',' => Some(TokenType::Virgula),
            ';' => Some(TokenType::PontoEVirgula),
            ':' => Some(TokenType::DoisPontos),
            '.' => Some(TokenType::Ponto),
            _ => None,
        };

        if let Some(token_type) = token_type {
            let token = Token::new(
                token_type,
                char_atual.to_string(),
                self.current_line,
                self.current_column,
            );
            self.advance();
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }

    // AFD para Palavras-Chave (antes de identificadores)
    fn try_consume_keyword(&mut self) -> Result<Option<Token>, LexerError> {
        let start_pos = self.current_pos;
        let start_line = self.current_line;
        let start_column = self.current_column;

        // Verificar se começa com letra
        if !self.current_char().is_alphabetic() {
            return Ok(None);
        }

        // Consumir identificador
        while self.current_pos < self.source.len() {
            let char = self.current_char();
            if char.is_alphanumeric() || char == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexema: String = self.source[start_pos..self.current_pos].iter().collect();
        
        // Verificar se é palavra-chave
        if let Some(token_type) = self.classify_keyword(&lexema) {
            Ok(Some(Token::new(token_type, lexema, start_line, start_column)))
        } else {
            // Não é palavra-chave, então retroceder e deixar para o identificador
            self.current_pos = start_pos;
            self.current_line = start_line;
            self.current_column = start_column;
            Ok(None)
        }
    }

    // AFD para Identificadores
    fn try_consume_identifier(&mut self) -> Result<Option<Token>, LexerError> {
        let start_pos = self.current_pos;
        let start_line = self.current_line;
        let start_column = self.current_column;

        // Verificar se começa com letra ou underscore
        if !(self.current_char().is_alphabetic() || self.current_char() == '_') {
            return Ok(None);
        }

        self.advance();

        // Consumir resto do identificador
        while self.current_pos < self.source.len() {
            let char = self.current_char();
            if char.is_alphanumeric() || char == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexema: String = self.source[start_pos..self.current_pos].iter().collect();
        Ok(Some(Token::new(TokenType::Identificador(lexema.clone()), lexema, start_line, start_column)))
    }

    fn classify_keyword(&self, lexema: &str) -> Option<TokenType> {
        match lexema {
            // Palavras-chave SEM acentos (como definido no seu TokenType)
            "se" => Some(TokenType::Se),
            "entao" => Some(TokenType::Entao),
            "senao" => Some(TokenType::Senao),
            "fimse" => Some(TokenType::FimSe),
            "enquanto" => Some(TokenType::Enquanto),
            "faca" => Some(TokenType::Faca),
            "fimenquanto" => Some(TokenType::FimEnquanto),
            "para" => Some(TokenType::Para),
            "de" => Some(TokenType::De),
            "ate" => Some(TokenType::Ate),
            "fimpara" => Some(TokenType::FimPara),
            "funcao" => Some(TokenType::Funcao),
            "retorne" => Some(TokenType::Retorne),
            "fimfuncao" => Some(TokenType::FimFuncao),
            "inteiro" => Some(TokenType::Inteiro),
            "decimal" => Some(TokenType::Decimal),
            "texto" => Some(TokenType::Texto),
            "logico" => Some(TokenType::Logico),
            "verdadeiro" => Some(TokenType::Verdadeiro),
            "falso" => Some(TokenType::Falso),
            "escreva" => Some(TokenType::Escreva),
            "leia" => Some(TokenType::Leia),
            "principal" => Some(TokenType::Principal),
            _ => None,
        }
    }

    // --- FUNÇÕES AUXILIARES ---

    fn current_char(&self) -> char {
        self.source[self.current_pos]
    }

    fn advance(&mut self) {
        if self.current_pos < self.source.len() {
            if self.current_char() == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            } else {
                self.current_column += 1;
            }
            self.current_pos += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_pos < self.source.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current_pos + 1 < self.source.len() {
            Some(self.source[self.current_pos + 1])
        } else {
            None
        }
    }
}