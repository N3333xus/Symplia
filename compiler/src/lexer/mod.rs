pub mod token;
pub mod afd;
pub mod error;

pub use token::{Token, TokenType};
pub use error::LexerError;
pub use afd::Lexer;