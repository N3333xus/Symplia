use crate::lexer::token::{Token, TokenType};
use crate::lexer::error::LexerError;

pub fn try_consume_operator(
    source: &[char],
    current_pos: &mut usize,
    current_line: &mut usize,
    current_column: &mut usize,
) -> Result<Option<Token>, LexerError> {
    let char_atual = source[*current_pos];
    
    // operadores de 2 caracteres (primeiro mais longo)
    if *current_pos + 1 < source.len() {
        let dois_chars = format!("{}{}", char_atual, source[*current_pos + 1]);
        
        if let Some(token_type) = match_operator_duplo(&dois_chars) {
            let token = Token::new(
                token_type,
                dois_chars.clone(),
                *current_line,
                *current_column,
            );
            advance(source, current_pos, current_line, current_column);
            advance(source, current_pos, current_line, current_column);
            return Ok(Some(token));
        }
    }
    
    //p operadores de 1 caractere
    if let Some(token_type) = match_operator_simples(char_atual) {
        let token = Token::new(
            token_type,
            char_atual.to_string(),
            *current_line,
            *current_column,
        );
        advance(source, current_pos, current_line, current_column);
        return Ok(Some(token));
    }
    
    Ok(None)
}

fn match_operator_duplo(op: &str) -> Option<TokenType> {
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

fn match_operator_simples(op: char) -> Option<TokenType> {
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