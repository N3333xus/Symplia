use compiler::{Lexer, Parser, SemanticAnalyzer};
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
    let _tokens = match lexer.tokenize() {
        Ok(tokens) => {
            println!("Tokens reconhecidos: {}", tokens.len());
            tokens
        }
        Err(e) => {
            eprintln!("❌ ERRO LÉXICO: {}", e);
            process::exit(1);
        }
    };
    
    println!("\n=== ANALISE SINTÁTICA ===");
    let program = match Parser::parse_from_source(&source_code) {
        Ok(program) => {
            println!("Análise sintática concluída com sucesso!");
            program
            
        }
        Err(errors) => {
            eprintln!("❌ Foram encontrados {} erros sintáticos:", errors.len());
            for (i, error) in errors.iter().enumerate() {
                println!("  {}. {}", i + 1, error);
            }
            process::exit(1);
        }
    };

    println!("\n=== ANALISE SEMÂNTICA ===");
    let mut semantic_analyzer = SemanticAnalyzer::new();
    let semantic_result = semantic_analyzer.analyze(program);
    
    if !semantic_result.errors.is_empty() {
        eprintln!("❌ Foram encontrados {} erros semânticos:", semantic_result.errors.len());
        for (i, error) in semantic_result.errors.iter().enumerate() {
            println!("  {}. Linha {}: {}", i + 1, error.line, error.message);
        }
        process::exit(1);
    }
    
    println!("Análise semântica concluída com sucesso!");
    
    println!("\n=== ESTATÍSTICAS FINAIS ===");
    println!("Funções definidas: {}", semantic_result.annotated_ast.functions.len());
    println!("Tabela de símbolos: {} entradas", "TODO"); // pode adicionar um método para contar
    
    println!("\n=== AST ===");
    println!("{:#?}", semantic_result.annotated_ast);
}

pub fn analyze_snippet(source: &str) -> Result<compiler::SemanticAnalysisResult, Vec<compiler::ParserError>> {
    let program = Parser::parse_from_source(source)?;
    let mut analyzer = SemanticAnalyzer::new();
    Ok(analyzer.analyze(program))
}