use symplia_compiler::lexer::Lexer;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Uso: {} <arquivo.sym>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let source_code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Erro ao ler arquivo {}: {}", filename, err);
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source_code);
    
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("{:<20} {:<30} {:<10} {:<10}", "TOKEN", "TIPO", "LINHA", "COLUNA");
            println!("{:-<70}", "");

            for token in tokens {
                let tipo_desc = match token.token_type {
                    symplia_compiler::lexer::token::TokenType::Identificador(_) => "IDENTIFICADOR".to_string(),
                    symplia_compiler::lexer::token::TokenType::InteiroLiteral(_) => "INTEIRO_LITERAL".to_string(),
                    symplia_compiler::lexer::token::TokenType::DecimalLiteral(_) => "DECIMAL_LITERAL".to_string(),
                    symplia_compiler::lexer::token::TokenType::StringLiteral(_) => "STRING_LITERAL".to_string(),
                    symplia_compiler::lexer::token::TokenType::EOF => "FIM_DE_ARQUIVO".to_string(),
                    _ => format!("{:?}", token.token_type).to_uppercase(),
                };
                
                println!("{:<20} {:<30} {:<10} {:<10}", token.lexema, tipo_desc, token.linha, token.coluna);
            }
        }
        Err(err) => {
            // Obter as linhas do código fonte
            let lines: Vec<&str> = source_code.split('\n').collect();
            let linha_erro = err.linha;
            let coluna_erro = err.coluna;

            eprintln!("{}", err);

            if linha_erro <= lines.len() {
                let linha_texto = lines[linha_erro - 1];
                eprintln!("{}", linha_texto);
                
                let espacos = " ".repeat(coluna_erro - 1);
                eprintln!("{}^", espacos);
            }

            process::exit(1);
        }
    }
}