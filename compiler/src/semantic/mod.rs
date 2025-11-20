pub mod symbol_table;
pub mod type_checker;
pub mod semantic;

pub use semantic::{SemanticAnalyzer, SemanticAnalysisResult, SemanticError};
pub use symbol_table::SymbolTable;