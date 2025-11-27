use compiler::{Lexer, Parser, SemanticAnalyzer};
use compiler::serialization::save_semantic_result_to_json;
use std::env;
use std::fs;
use std::path::Path;
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
    
    println!("=== COMPILADOR SYMPLIA ===\n");
    println!("Arquivo: {}", filename);
    println!("Tamanho do código: {} caracteres\n", source_code.len());

    println!("=== ANÁLISE LÉXICA ===");
    let mut lexer = Lexer::new(&source_code);
    let _tokens = match lexer.tokenize() {
        Ok(tokens) => {
            println!("✅ Tokens reconhecidos: {}", tokens.len());
            if cfg!(debug_assertions) {
                // Mostrar tokens apenas em modo debug
                for (i, token) in tokens.iter().enumerate() {
                    println!("  {}: {}", i, token);
                }
            }
            tokens
        }
        Err(e) => {
            eprintln!("❌ ERRO LÉXICO: {}", e);
            process::exit(1);
        }
    };
    
    println!("\n=== ANÁLISE SINTÁTICA ===");
    let program = match Parser::parse_from_source(&source_code) {
        Ok(program) => {
            println!("✅ Análise sintática concluída com sucesso!");
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

    println!("\n=== ANÁLISE SEMÂNTICA ===");
    let mut semantic_analyzer = SemanticAnalyzer::new();
    let semantic_result = semantic_analyzer.analyze(program);
    
    if !semantic_result.errors.is_empty() {
        eprintln!("❌ Foram encontrados {} erros semânticos:", semantic_result.errors.len());
        for (i, error) in semantic_result.errors.iter().enumerate() {
            println!("  {}. Linha {}: {}", i + 1, error.line, error.message);
        }
        process::exit(1);
    }
    
    println!("✅ Análise semântica concluída com sucesso!");

    println!("\n=== SERIALIZAÇÃO DA AST ===");
    
    let file_stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("programa");
    
    let json_filename = format!("../build/{}.ast.json", file_stem);
    
    match save_semantic_result_to_json(&semantic_result, &json_filename) {
        Ok(()) => {
            println!("✅ AST serializada salva em: {}", json_filename);
            println!("\n🎉 Compilação concluída com sucesso!");
            println!("📁 Arquivo JSON gerado: {}", json_filename);
        }
        Err(e) => {
            eprintln!("❌ Erro ao serializar AST: {}", e);
            process::exit(1);
        }
    }
}


//testes unitários para o main
#[cfg(test)]
mod tests {

    #[test]
    fn test_args_validation() {
        // Teste simulado de validação de argumentos
        let args = vec!["compiler".to_string(), "test.sym".to_string()];
        assert_eq!(args.len(), 2);
        assert!(args[1].ends_with(".sym"));
    }

    #[test]
    fn test_json_filename_generation() {
        use std::path::Path;
        
        let filename = "programas/exemplo.sym";
        let file_stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("programa");
        
        let json_filename = format!("../build/{}.ast.json", file_stem);
        assert_eq!(json_filename, "../build/exemplo.ast.json");
        
        let filename_without_path = "teste.sym";
        let file_stem2 = Path::new(filename_without_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("programa");
        
        let json_filename2 = format!("../build/{}.ast.json", file_stem2);
        assert_eq!(json_filename2, "../build/teste.ast.json");
    }
}
