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
                let tipo_desc = match &token.token_type {
                    symplia_compiler::lexer::token::TokenType::Identificador(s) => format!("IDENTIFICADOR({})", s),
                    symplia_compiler::lexer::token::TokenType::InteiroLiteral(n) => format!("INTEIRO_LITERAL({})", n),
                    symplia_compiler::lexer::token::TokenType::DecimalLiteral(n) => format!("DECIMAL_LITERAL({})", n),
                    symplia_compiler::lexer::token::TokenType::StringLiteral(s) => format!("STRING_LITERAL(\"{}\")", s),
                    symplia_compiler::lexer::token::TokenType::EOF => "FIM_DE_ARQUIVO".to_string(),
                    _ => format!("{:?}", token.token_type).to_uppercase(),
                };
                
                println!("{:<20} {:<30} {:<10} {:<10}", 
                    token.lexema, 
                    tipo_desc,
                    token.linha,
                    token.coluna);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}