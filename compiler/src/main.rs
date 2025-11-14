use compiler::{Lexer, Parser};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Uso: {} <arquivo.sym>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    
    let source_code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Erro ao ler arquivo {}: {}", filename, e);
            process::exit(1);
        }
    };
    
    println!("=== ANALISADOR SINTÁTICO SYMPLIA ===\n");
    println!("Arquivo: {}", filename);
    println!("Tamanho do código: {} caracteres\n", source_code.len());

    println!("=== ANALISE LÉXICA ===");
    let mut lexer = Lexer::new(&source_code);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("TOKENS RECONHECIDOS: {}", tokens.len());
            
            // Mostrar TODOS os tokens
            println!("\n--- TODOS OS TOKENS RECONHECIDOS ({} tokens) ---", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                println!("  {:3}: {}", i, token);
            }
        }
        Err(e) => {
            eprintln!("ERRO LÉXICO: {}", e);
            process::exit(1);
        }
    }
    
    println!("\n=== ANALISE SINTÁTICA ===");
    match Parser::parse_from_source(&source_code) {
        Ok(program) => {
            println!("✅ Análise sintática concluída com sucesso!");
            
            println!("\n=== ESTATÍSTICAS ===");
            let total_functions = program.functions.len();
            let total_statements: usize = program.statements.len() + 
                program.functions.iter()
                    .map(|f| f.body.statements.len())
                    .sum::<usize>();
            
            println!("Funções definidas: {}", total_functions);
            println!("Comandos globais: {}", program.statements.len());
            println!("Total de comandos: {}", total_statements);
            
            println!("\n=== ÁRVORE SINTÁTICA ABSTRATA (AST) ===");
            println!("{:#?}", program);
            
        }
        Err(errors) => {
            eprintln!("❌ Foram encontrados {} erros sintáticos:", errors.len());
            for (i, error) in errors.iter().enumerate() {
                println!("  {}. {}", i + 1, error);
            }
            process::exit(1);
        }

        
    }
}

// Função auxiliar para análise interativa em testes
pub fn analyze_snippet(source: &str) -> Result<compiler::Program, Vec<compiler::ParserError>> {
    Parser::parse_from_source(source)
}

