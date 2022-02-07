//! Pure rust `.proto` file parser.

pub(crate) mod convert;
pub(crate) mod model;
pub(crate) mod parse_and_typecheck;
pub(crate) mod parse_dependencies;
mod parser;

pub use parse_and_typecheck::*;
pub use parse_dependencies::*;
