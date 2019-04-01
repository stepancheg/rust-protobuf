use std::fmt;

/// Valid Rust identifier
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RustIdent(String);

impl RustIdent {
    pub fn new(s: &str) -> RustIdent {
        assert!(!s.is_empty());
        assert!(!s.contains("/"), "{}", s);
        assert!(!s.contains("."), "{}", s);
        assert!(!s.contains(":"), "{}", s);
        RustIdent(s.to_owned())
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn to_path(&self) -> RustIdentWithPath {
        RustIdentWithPath::new(&self.0)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RustIdentWithPath(String);

impl RustIdentWithPath {
    pub fn new(s: &str) -> RustIdentWithPath {
        assert!(!s.is_empty());
        assert!(!s.contains("."), "{}", s);
        RustIdentWithPath(s.to_owned())
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn assert_no_path(self) -> RustIdent {
        RustIdent::new(&self.0)
    }

    pub fn child(&self, child: &RustIdent) -> RustIdentWithPath {
        RustIdentWithPath::new(&format!("{}::{}", self, child))
    }
}

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

impl fmt::Display for RustIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
    }
}

impl fmt::Display for RustIdentWithPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
    }
}

impl fmt::Display for ProtobufIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
    }
}
