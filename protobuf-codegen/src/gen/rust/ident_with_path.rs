use std::fmt;

use crate::gen::rust::component::RustPathComponent;
use crate::gen::rust::ident::RustIdent;
use crate::gen::rust::path::RustPath;

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct RustIdentWithPath {
    pub path: RustPath,
    pub ident: RustIdent,
}

impl RustIdentWithPath {
    pub fn new(s: String) -> RustIdentWithPath {
        let mut path = RustPath::from(s);
        let ident = match path.path.path.pop() {
            None => panic!("empty path"),
            Some(RustPathComponent::Ident(ident)) => ident,
            Some(RustPathComponent::Keyword(kw)) => {
                panic!("last path component is a keyword: {}", kw)
            }
        };
        RustIdentWithPath { path, ident }
    }

    pub fn prepend_ident(&mut self, ident: RustIdent) {
        self.path.prepend_ident(ident)
    }

    pub fn to_path(&self) -> RustPath {
        self.path.clone().append_ident(self.ident.clone())
    }
}

impl<S: Into<String>> From<S> for RustIdentWithPath {
    fn from(s: S) -> Self {
        RustIdentWithPath::new(s.into())
    }
}

impl fmt::Display for RustIdentWithPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.to_path(), f)
    }
}
