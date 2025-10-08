use super::token::{Token, TokenType};
use super::error::LexerError;

pub struct Lexer {
    source: Vec<char>,
    current_pos: usize,
    current_line: usize,
    current_column: usize,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        Self {
            source: source_code.chars().collect(),
            current_pos: 0,
            current_line: 1,
            current_column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while self.current_pos < self.source.len() {
            // Pular espaços em branco e comentários
            if self.skip_whitespace() || self.skip_comments()? {
                continue;
            }

            if self.current_pos >= self.source.len() {
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        tokens.push(Token::eof(self.current_line, self.current_column));
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        let char = self.current_char();

        match char {
            // Stringss
            '"' => self.consume_string(),

            // Números
            '0'..='9' => self.consume_number(),

            // Operadores e delimitadores
            '+' => self.single_char_token(TokenType::Mais),
            '-' => self.single_char_token(TokenType::Menos),
            '*' => self.single_char_token(TokenType::Multiplicacao),
            '/' => self.single_char_token(TokenType::Divisao),
            '%' => self.single_char_token(TokenType::Modulo),
            '(' => self.single_char_token(TokenType::ParenteseEsquerdo),
            ')' => self.single_char_token(TokenType::ParenteseDireito),
            '{' => self.single_char_token(TokenType::ChaveEsquerda),
            '}' => self.single_char_token(TokenType::ChaveDireita),
            '[' => self.single_char_token(TokenType::ColcheteEsquerdo),
            ']' => self.single_char_token(TokenType::ColcheteDireito),
            ',' => self.single_char_token(TokenType::Virgula),
            ';' => self.single_char_token(TokenType::PontoEVirgula),
            ':' => self.single_char_token(TokenType::DoisPontos),
            '.' => self.single_char_token(TokenType::Ponto),

            // Operadores compostos
            '=' => self.consume_operator('=', TokenType::Igual, TokenType::Atribuicao),
            '!' => self.consume_operator('=', TokenType::Diferente, TokenType::NaoLogico),
            '<' => self.consume_operator('=', TokenType::MenorIgual, TokenType::Menor),
            '>' => self.consume_operator('=', TokenType::MaiorIgual, TokenType::Maior),
            '&' => self.consume_double_char('&', TokenType::ELogico),
            '|' => self.consume_double_char('|', TokenType::OuLogico),

            // Identificadores e palavras-chave
            'a'..='z' | 'A'..='Z' | '_' => self.consume_identifier_or_keyword(),

            // Caractere inválido
            _ => Err(LexerError::invalid_char(char, self.current_line, self.current_column)),
        }
    }

    // AFD para identificadores/palavras-chave
    fn consume_identifier_or_keyword(&mut self) -> Result<Token, LexerError> {
        let start_pos = self.current_pos;
        let start_line = self.current_line;
        let start_column = self.current_column;

        // Estado q0 -> q1: primeiro caractere (já verificado)
        self.advance();

        // Estado q1: zero ou mais letras, dígitos ou _
        while self.current_pos < self.source.len() {
            let char = self.current_char();
            if char.is_alphanumeric() || char == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let lexema: String = self.source[start_pos..self.current_pos].iter().collect();
        
        let token_type = self.classify_keyword(&lexema)
            .unwrap_or_else(|| TokenType::Identificador(lexema.clone()));

        Ok(Token::new(token_type, start_line, start_column, lexema))
    }

    // AFD paraa números (inteiros ou decimais)
    fn consume_number(&mut self) -> Result<Token, LexerError> {
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

        Ok(Token::new(token_type, start_line, start_column, lexema))
    }

    // AFD para strings
    fn consume_string(&mut self) -> Result<Token, LexerError> {
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
                return Ok(Token::new(
                    TokenType::StringLiteral(string_content),
                    start_line,
                    start_column,
                    lexema,
                ));
            } else {
                string_content.push(char);
                self.advance();
            }
        }

        Err(LexerError::unterminated_string(start_line, start_column))
    }

    // Funções auxiliares
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

    fn skip_whitespace(&mut self) -> bool {
        let start_pos = self.current_pos;
        while self.current_pos < self.source.len() && self.current_char().is_whitespace() {
            self.advance();
        }
        self.current_pos > start_pos
    }

    fn skip_comments(&mut self) -> Result<bool, LexerError> {
        if self.current_char() == '/' && self.peek_next() == Some('/') {
            // Comentário de linha
            while self.current_pos < self.source.len() && self.current_char() != '\n' {
                self.advance();
            }
            Ok(true)
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
                    return Ok(true);
                }
                self.advance();
            }
            Err(LexerError::unclosed_comment(start_line, start_column))
        } else {
            Ok(false)
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current_pos + 1 < self.source.len() {
            Some(self.source[self.current_pos + 1])
        } else {
            None
        }
    }

    fn single_char_token(&mut self, token_type: TokenType) -> Result<Token, LexerError> {
        let linha = self.current_line;
        let coluna = self.current_column;
        let lexema = self.current_char().to_string();
        self.advance();
        Ok(Token::new(token_type, linha, coluna, lexema))
    }

    fn consume_operator(
        &mut self,
        expected: char,
        double_type: TokenType,
        single_type: TokenType,
    ) -> Result<Token, LexerError> {
        let start_line = self.current_line;
        let start_column = self.current_column;
        let first_char = self.current_char();
        self.advance();

        if self.current_pos < self.source.len() && self.current_char() == expected {
            self.advance();
            let lexema = format!("{}{}", first_char, expected);
            Ok(Token::new(double_type, start_line, start_column, lexema))
        } else {
            let lexema = first_char.to_string();
            Ok(Token::new(single_type, start_line, start_column, lexema))
        }
    }

    fn consume_double_char(&mut self, expected: char, token_type: TokenType) -> Result<Token, LexerError> {
        let start_line = self.current_line;
        let start_column = self.current_column;
        let first_char = self.current_char();
        self.advance();

        if self.current_pos < self.source.len() && self.current_char() == expected {
            self.advance();
            let lexema = format!("{}{}", first_char, expected);
            Ok(Token::new(token_type, start_line, start_column, lexema))
        } else {
            Err(LexerError::invalid_char(expected, start_line, start_column))
        }
    }

    fn classify_keyword(&self, lexema: &str) -> Option<TokenType> {
        match lexema {
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
}