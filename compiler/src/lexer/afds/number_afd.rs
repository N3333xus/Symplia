use crate::lexer::token::{Token, TokenType};
use crate::lexer::error::LexerError;

pub fn try_consume_number(
    source: &[char],
    current_pos: &mut usize,
    current_line: &mut usize,
    current_column: &mut usize,
) -> Result<Option<Token>, LexerError> {
    if !source[*current_pos].is_ascii_digit() {
        return Ok(None);
    }

    let start_pos = *current_pos;
    let start_line = *current_line;
    let start_column = *current_column;
    let mut has_decimal_point = false;

    // consome a parte inteira
    while *current_pos < source.len() {
        let char = source[*current_pos];
        if char.is_ascii_digit() {
            advance(source, current_pos, current_line, current_column);
        } else if char == '.' && !has_decimal_point {
            has_decimal_point = true;
            advance(source, current_pos, current_line, current_column);
            
            // aqui verifica se há dígitos apos o ponto
            if *current_pos >= source.len() || !source[*current_pos].is_ascii_digit() {
                return Err(LexerError::invalid_number(
                    &source[start_pos..*current_pos].iter().collect::<String>(),
                    start_line,
                    start_column,
                ));
            }
        } else {
            break;
        }
    }

    let lexema: String = source[start_pos..*current_pos].iter().collect();

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