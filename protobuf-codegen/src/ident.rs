use std::fmt;

/// Valid Rust identifier
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RustIdent(pub String);

/// Identifier in `.proto` file
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ProtobufIdent(pub String);

impl fmt::Display for RustIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Display for ProtobufIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
