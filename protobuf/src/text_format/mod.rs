mod parse;
mod print;

// Used by text format parser and by pure-rust codegen parsed
// this it is public but hidden module.
// https://github.com/rust-lang/rust/issues/44663
#[doc(hidden)]
pub mod lexer;

pub use self::print::fmt;
pub use self::print::print_to;
pub use self::print::print_to_string;
#[doc(hidden)]
pub use self::print::quote_escape_bytes;

pub use self::parse::merge_from_str;
pub use self::parse::parse_from_str;
