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
    
    // Ler o arquivo de entrada
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
    
    // Fase de análise léxica
    println!("=== FASE LÉXICA ===");
    let mut lexer = Lexer::new(&source_code);
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("Tokens reconhecidos: {}", tokens.len());
            
            // Mostrar primeiros 20 tokens para debug
            for (i, token) in tokens.iter().take(20).enumerate() {
                println!("  {}: {}", i, token);
            }
            if tokens.len() > 20 {
                println!("  ... ({} tokens omitidos)", tokens.len() - 20);
            }
        }
        Err(e) => {
            eprintln!("Erro léxico: {}", e);
            process::exit(1);
        }
    }
    
    // Fase de análise sintática
    println!("\n=== FASE SINTÁTICA ===");
    match Parser::parse_from_source(&source_code) {
        Ok(program) => {
            println!("✅ Análise sintática concluída com sucesso!");
            println!("\n=== ESTRUTURA DO PROGRAMA ===");
            println!("{:?}", program);
            
            // Estatísticas
            let total_functions = program.functions.len();
            let total_statements = program.statements.len();
            println!("\n=== ESTATÍSTICAS ===");
            println!("Funções: {}", total_functions);
            println!("Comandos globais: {}", total_statements);
        }
        Err(errors) => {
            eprintln!("❌ Foram encontrados {} erros sintáticos:", errors.len());
            for error in errors {
                eprintln!("  {}", error);
            }
            process::exit(1);
        }
    }
}

// Função auxiliar para análise interativa (útil para testes)
pub fn analyze_snippet(source: &str) -> Result<compiler::Program, Vec<compiler::ParserError>> {
    Parser::parse_from_source(source)
}