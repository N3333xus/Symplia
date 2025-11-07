pub mod lexer;
pub mod parser;

// Re-export das funcionalidades principais
pub use lexer::{Lexer, Token, TokenType, LexerError};
pub use parser::{Parser, ParserError, Program, Expr, Statement, Literal, Type};