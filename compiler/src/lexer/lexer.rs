use std::collections::VecDeque;
use super::token::{Token, TokenType};
use super::error::LexerError;
use super::afds;

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

    pub fn next_token_for_parser(&mut self) -> Result<Token, LexerError> {
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

    // --- IMPLEMENTAÇÃO INTERNA USANDO AFDs ---

    fn next_token_internal(&mut self) -> Result<Option<Token>, LexerError> {
        self.skip_whitespace();
        
        if self.current_pos >= self.source.len() {
            return Ok(None);
        }
        
        let start_line = self.current_line;
        let start_column = self.current_column;
        
        // aqui aplicamos princípio do match mais longo usando AFDs
        if let Some(_token) = afds::try_consume_comment(
            &self.source, 
            &mut self.current_pos, 
            &mut self.current_line, 
            &mut self.current_column
        )? {
            // comentario são ignorados, chama recursivamente
            return self.next_token_internal();
        } else if let Some(token) = afds::try_consume_string(
            &self.source, 
            &mut self.current_pos, 
            &mut self.current_line, 
            &mut self.current_column
        )? {
            Ok(Some(token))
        } else if let Some(token) = afds::try_consume_number(
            &self.source, 
            &mut self.current_pos, 
            &mut self.current_line, 
            &mut self.current_column
        )? {
            Ok(Some(token))
        } else if let Some(token) = afds::try_consume_operator(
            &self.source, 
            &mut self.current_pos, 
            &mut self.current_line, 
            &mut self.current_column
        )? {
            Ok(Some(token))
        } else if let Some(token) = afds::try_consume_delimiter(
            &self.source, 
            &mut self.current_pos, 
            &mut self.current_line, 
            &mut self.current_column
        )? {
            Ok(Some(token))
        } else if let Some(token) = afds::try_consume_keyword(
            &self.source, 
            &mut self.current_pos, 
            &mut self.current_line, 
            &mut self.current_column
        )? {
            Ok(Some(token))
        } else if let Some(token) = afds::try_consume_identifier(
            &self.source, 
            &mut self.current_pos, 
            &mut self.current_line, 
            &mut self.current_column
        )? {
            Ok(Some(token))
        } else {
            // caractere inválido
            let char_invalido = self.current_char();
            let erro = LexerError::with_recovery_suggestion(
                format!("Caractere inválido: '{}'", char_invalido),
                start_line,
                start_column,
                "Tente usar apenas caracteres válidos da linguagem Symplia".to_string(),
            );

            self.advance();
            Err(erro)
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
}