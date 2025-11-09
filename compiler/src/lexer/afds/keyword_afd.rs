use crate::lexer::token::{Token, TokenType};
use crate::lexer::error::LexerError;

pub fn try_consume_keyword(
    source: &[char],
    current_pos: &mut usize,
    current_line: &mut usize,
    current_column: &mut usize,
) -> Result<Option<Token>, LexerError> {
    let start_pos = *current_pos;
    let start_line = *current_line;
    let start_column = *current_column;

    // Verificar se começa com letra
    if !source[*current_pos].is_alphabetic() {
        return Ok(None);
    }

    // Consumir identificador
    while *current_pos < source.len() {
        let char = source[*current_pos];
        if char.is_alphanumeric() || char == '_' {
            advance(source, current_pos, current_line, current_column);
        } else {
            break;
        }
    }

    let lexema: String = source[start_pos..*current_pos].iter().collect();
    
    // Verificar se é palavra-chave
    if let Some(token_type) = classify_keyword(&lexema) {
        Ok(Some(Token::new(token_type, lexema, start_line, start_column)))
    } else {
        // Não é palavra-chave, então retroceder e deixar para o identificador
        *current_pos = start_pos;
        *current_line = start_line;
        *current_column = start_column;
        Ok(None)
    }
}

pub fn try_consume_identifier(
    source: &[char],
    current_pos: &mut usize,
    current_line: &mut usize,
    current_column: &mut usize,
) -> Result<Option<Token>, LexerError> {
    let start_pos = *current_pos;
    let start_line = *current_line;
    let start_column = *current_column;

    // Verificar se começa com letra ou underscore
    if !(source[*current_pos].is_alphabetic() || source[*current_pos] == '_') {
        return Ok(None);
    }

    advance(source, current_pos, current_line, current_column);

    // Consumir resto do identificador
    while *current_pos < source.len() {
        let char = source[*current_pos];
        if char.is_alphanumeric() || char == '_' {
            advance(source, current_pos, current_line, current_column);
        } else {
            break;
        }
    }

    let lexema: String = source[start_pos..*current_pos].iter().collect();
    Ok(Some(Token::new(TokenType::Identificador(lexema.clone()), lexema, start_line, start_column)))
}

pub fn classify_keyword(lexema: &str) -> Option<TokenType> {
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