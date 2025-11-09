pub mod afds;
pub mod error;
pub mod lexer;
pub mod token;

// Re-export para facilitar o acesso
pub use error::LexerError;
pub use lexer::Lexer;
pub use token::{Token, TokenType};