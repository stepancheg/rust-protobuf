#![doc(hidden)]

use std::fmt;

/// Identifier in `.proto` file
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
#[doc(hidden)]
pub struct ProtobufIdent(String);

impl From<&'_ str> for ProtobufIdent {
    fn from(s: &str) -> Self {
        ProtobufIdent::new(s)
    }
}

impl From<String> for ProtobufIdent {
    fn from(s: String) -> Self {
        ProtobufIdent::new(&s)
    }
}

impl fmt::Display for ProtobufIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
    }
}

impl ProtobufIdent {
    #[allow(dead_code)]
    pub fn new(s: &str) -> ProtobufIdent {
        assert!(!s.is_empty());
        assert!(!s.contains("/"));
        assert!(!s.contains("."));
        assert!(!s.contains(":"));
        assert!(!s.contains("("));
        assert!(!s.contains(")"));
        ProtobufIdent(s.to_owned())
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}
