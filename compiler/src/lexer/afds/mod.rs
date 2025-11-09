pub mod comment_afd;
pub mod string_afd;
pub mod number_afd;
pub mod operator_afd;
pub mod delimiter_afd;
pub mod keyword_afd;

// nota interessante: fazendo re-import facilita o acesso
pub use comment_afd::try_consume_comment;
pub use string_afd::try_consume_string;
pub use number_afd::try_consume_number;
pub use operator_afd::try_consume_operator;
pub use delimiter_afd::try_consume_delimiter;
pub use keyword_afd::{try_consume_keyword, try_consume_identifier, classify_keyword};