use crate::lexer::token::{Token /*TokenType*/};
use crate::lexer::error::LexerError;

pub fn try_consume_comment(
    source: &[char],
    current_pos: &mut usize,
    current_line: &mut usize,
    current_column: &mut usize,
) -> Result<Option<Token>, LexerError> {
    if source[*current_pos] == '/' && peek_next(source, *current_pos) == Some('/') {
        // p/ comentario de linha
        while *current_pos < source.len() && source[*current_pos] != '\n' {
            advance(source, current_pos, current_line, current_column);
        }
        Ok(Some(Token::eof(*current_line, *current_column)))
    } else if source[*current_pos] == '/' && peek_next(source, *current_pos) == Some('*') {
        // p/ comentário de bloco
        let start_line = *current_line;
        let start_column = *current_column;
        
        advance(source, current_pos, current_line, current_column); // /
        advance(source, current_pos, current_line, current_column); // *
        
        while *current_pos + 1 < source.len() {
            if source[*current_pos] == '*' && peek_next(source, *current_pos) == Some('/') {
                advance(source, current_pos, current_line, current_column); // *
                advance(source, current_pos, current_line, current_column); // /
                return Ok(Some(Token::eof(*current_line, *current_column)));
            }
            advance(source, current_pos, current_line, current_column);
        }
        Err(LexerError::unclosed_comment(start_line, start_column))
    } else {
        Ok(None)
    }
}

fn peek_next(source: &[char], current_pos: usize) -> Option<char> {
    if current_pos + 1 < source.len() {
        Some(source[current_pos + 1])
    } else {
        None
    }
}

fn advance(
    source: &[char],
    current_pos: &mut usize,
    current_line: &mut usize,
    current_column: &mut usize,
) {
    if *current_pos < source.len() {
        if source[*current_pos] == '\n' {
            *current_line += 1;
            *current_column = 1;
        } else {
            *current_column += 1;
        }
        *current_pos += 1;
    }
}