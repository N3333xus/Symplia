pub mod token;
pub mod error;
pub mod lexer;

pub use token::{Token, TokenType};
pub use error::{LexerError, ErrorSeverity};
pub use lexer::Lexer;