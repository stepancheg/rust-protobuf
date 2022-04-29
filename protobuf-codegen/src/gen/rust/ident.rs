use std::fmt;

use crate::gen::rust::rel_path::RustRelativePath;
use crate::gen::rust_name::RustIdentWithPath;

/// Valid Rust identifier
#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct RustIdent(String);

impl RustIdent {
    pub fn new(s: &str) -> RustIdent {
        assert!(!s.is_empty());
        assert!(!s.contains("/"), "{}", s);
        assert!(!s.contains("."), "{}", s);
        assert!(!s.contains(":"), "{}", s);
        RustIdent(s.to_owned())
    }

    pub fn super_ident() -> RustIdent {
        RustIdent::new("super")
    }

    pub fn get(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn to_path(&self) -> RustIdentWithPath {
        RustIdentWithPath::from(&self.0)
    }

    pub(crate) fn into_rel_path(self) -> RustRelativePath {
        RustRelativePath::from_components([self])
    }
}

impl fmt::Display for RustIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
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
