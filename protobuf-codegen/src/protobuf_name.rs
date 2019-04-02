use std::fmt;

/// Identifier in `.proto` file
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ProtobufIdent(String);

impl ProtobufIdent {
    #[allow(dead_code)]
    pub fn new(s: &str) -> ProtobufIdent {
        assert!(!s.is_empty());
        assert!(!s.contains("/"));
        assert!(!s.contains("."));
        assert!(!s.contains(":"));
        ProtobufIdent(s.to_owned())
    }

    pub fn get(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProtobufIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
    }
}
