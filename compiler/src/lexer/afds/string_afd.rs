use crate::lexer::token::{Token, TokenType};
use crate::lexer::error::LexerError;

pub fn try_consume_string(
    source: &[char],
    current_pos: &mut usize,
    current_line: &mut usize,
    current_column: &mut usize,
) -> Result<Option<Token>, LexerError> {
    if source[*current_pos] != '"' {
        return Ok(None);
    }

    let start_line = *current_line;
    let start_column = *current_column;
    let mut string_content = String::new();
    let mut escape = false;

    // pula inicio de aspas
    advance(source, current_pos, current_line, current_column);

    while *current_pos < source.len() {
        let char = source[*current_pos];

        if escape {
            match char {
                'n' => string_content.push('\n'),
                't' => string_content.push('\t'),
                'r' => string_content.push('\r'),
                '"' => string_content.push('"'),
                '\\' => string_content.push('\\'),
                _ => return Err(LexerError::invalid_escape(char, *current_line, *current_column)),
            }
            escape = false;
            advance(source, current_pos, current_line, current_column);
        } else if char == '\\' {
            escape = true;
            advance(source, current_pos, current_line, current_column);
        } else if char == '"' {
            // chega no fim da string
            advance(source, current_pos, current_line, current_column);
            let lexema = format!("\"{}\"", string_content);
            return Ok(Some(Token::new(
                TokenType::StringLiteral(string_content),
                lexema,
                start_line,
                start_column,
            )));
        } else {
            string_content.push(char);
            advance(source, current_pos, current_line, current_column);
        }
    }

    Err(LexerError::unterminated_string(start_line, start_column))
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