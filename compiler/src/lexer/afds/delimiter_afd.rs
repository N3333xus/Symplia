use crate::lexer::token::{Token, TokenType};
use crate::lexer::error::LexerError;

pub fn try_consume_delimiter(
    source: &[char],
    current_pos: &mut usize,
    current_line: &mut usize,
    current_column: &mut usize,
) -> Result<Option<Token>, LexerError> {
    let char_atual = source[*current_pos];
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
            *current_line,
            *current_column,
        );
        advance(source, current_pos, current_line, current_column);
        Ok(Some(token))
    } else {
        Ok(None)
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