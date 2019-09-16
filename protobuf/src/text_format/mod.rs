//! Protobuf "text format" implementation.
//!
//! Text format message look like this:
//!
//! ```text,ignore
//! size: 17
//! color: "red"
//! children {
//!     size: 18
//!     color: "blue"
//! }
//! children {
//!     size: 19
//!     color: "green"
//! }
//! ```
//!
//! This format is not specified, but it is implemented by all official
//! protobuf implementations, including `protoc` command which can decode
//! and encode messages using text format.

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
pub use self::parse::ParseError;
