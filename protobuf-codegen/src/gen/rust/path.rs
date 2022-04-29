use std::fmt;

use crate::gen::rust::component::RustPathComponent;
use crate::gen::rust::ident::RustIdent;
use crate::gen::rust::ident_with_path::RustIdentWithPath;
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

    pub fn first(&self) -> Option<RustPathComponent> {
        assert!(!self.absolute);
        self.path.first()
    }

    pub fn remove_first(&mut self) -> Option<RustPathComponent> {
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

    pub(crate) fn append_component(mut self, component: RustPathComponent) -> RustPath {
        self.path.path.push(component);
        self
    }

    pub fn append_ident(self, ident: RustIdent) -> RustPath {
        self.append_component(RustPathComponent::Ident(ident))
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
