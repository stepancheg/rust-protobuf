use std::fmt;

use crate::gen::rust::ident::RustIdent;
use crate::gen::rust::rel_path::RustRelativePath;

#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub(crate) struct RustPath {
    pub(crate) absolute: bool,
    pub(crate) path: RustRelativePath,
}

impl RustPath {
    pub fn super_path() -> RustPath {
        RustPath::from("super")
    }

    pub fn is_absolute(&self) -> bool {
        self.absolute
    }

    pub fn with_ident(self, ident: RustIdent) -> RustIdentWithPath {
        RustIdentWithPath { path: self, ident }
    }

    pub fn first(&self) -> Option<RustIdent> {
        assert!(!self.absolute);
        self.path.first()
    }

    pub fn remove_first(&mut self) -> Option<RustIdent> {
        assert!(!self.absolute);
        self.path.remove_first()
    }

    pub fn prepend_ident(&mut self, ident: RustIdent) {
        assert!(!self.absolute);
        self.path.prepend_ident(ident);
    }

    pub fn append(self, path: RustPath) -> RustPath {
        if path.absolute {
            path
        } else {
            RustPath {
                absolute: self.absolute,
                path: self.path.append(path.path),
            }
        }
    }

    pub fn append_ident(mut self, ident: RustIdent) -> RustPath {
        self.path.path.push(ident);
        self
    }

    pub fn append_with_ident(self, path: RustIdentWithPath) -> RustIdentWithPath {
        self.append(path.path).with_ident(path.ident)
    }

    pub fn into_relative_or_panic(self) -> RustRelativePath {
        assert!(!self.absolute);
        self.path
    }
}

impl From<&'_ str> for RustPath {
    fn from(s: &str) -> Self {
        let (s, absolute) = if s.starts_with("::") {
            (&s[2..], true)
        } else {
            (s, false)
        };
        RustPath {
            absolute,
            path: RustRelativePath::from(s),
        }
    }
}

impl From<String> for RustPath {
    fn from(s: String) -> Self {
        RustPath::from(&s[..])
    }
}

impl fmt::Display for RustPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.absolute {
            write!(f, "::")?;
        }
        write!(f, "{}", self.path)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct RustIdentWithPath {
    pub path: RustPath,
    pub ident: RustIdent,
}

impl RustIdentWithPath {
    pub fn new(s: String) -> RustIdentWithPath {
        let mut path = RustPath::from(s);
        let ident = path.path.path.pop().unwrap();
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
