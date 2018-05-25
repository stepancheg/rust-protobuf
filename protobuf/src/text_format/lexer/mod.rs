//! Implementation of lexer for both protobuf parser and for text format parser.

mod lexer_impl;
mod loc;
mod str_lit;
mod token;
pub mod float;
mod num_lit;

pub use self::lexer_impl::Lexer;
pub use self::lexer_impl::LexerError;
pub use self::lexer_impl::LexerCommentStyle;
pub use self::loc::Loc;
pub use self::token::Token;
pub use self::token::TokenWithLocation;
pub use self::num_lit::NumLit;
pub use self::str_lit::StrLit;
pub use self::str_lit::StrLitDecodeError;
