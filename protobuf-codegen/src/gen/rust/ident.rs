use std::fmt;

use crate::gen::rust::ident_with_path::RustIdentWithPath;
use crate::gen::rust::keywords::is_rust_keyword;
use crate::gen::rust::rel_path::RustRelativePath;

/// Valid Rust identifier
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub(crate) struct RustIdent(String);

impl RustIdent {
    pub fn new(s: &str) -> RustIdent {
        assert!(!s.is_empty());
        assert!(!s.contains("/"), "{}", s);
        assert!(!s.contains("."), "{}", s);
        assert!(!s.contains(":"), "{}", s);
        assert!(!s.contains(" "), "{}", s);
        assert!(!s.contains("#"), "{}", s);
        RustIdent(s.to_owned())
    }

    pub(crate) fn get(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn to_path(&self) -> RustIdentWithPath {
        RustIdentWithPath::from(&self.0)
    }

    pub(crate) fn into_rel_path(self) -> RustRelativePath {
        RustRelativePath::from_idents([self])
    }
}

impl fmt::Display for RustIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Rust-protobuf uses `_` suffix to escape identifiers instead of raw identifiers
        // because some identifiers cannot be escaped as raw identifiers,
        // e.g. `r#self` is not a valid raw identifier.
        if is_rust_keyword(&self.0) {
            write!(f, "{}_", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl From<&'_ str> for RustIdent {
    fn from(s: &str) -> Self {
        RustIdent::new(s)
    }
}

impl From<String> for RustIdent {
    fn from(s: String) -> Self {
        RustIdent::new(&s)
    }
}
