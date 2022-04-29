use std::fmt;
use std::iter;

use crate::gen::rust::component::RustPathComponent;
use crate::gen::rust::ident::RustIdent;
use crate::gen::rust::path::RustPath;

#[derive(Default, Eq, PartialEq, Debug, Clone)]
pub(crate) struct RustRelativePath {
    pub(crate) path: Vec<RustPathComponent>,
}

impl RustRelativePath {
    pub fn into_path(self) -> RustPath {
        RustPath {
            absolute: false,
            path: self,
        }
    }

    pub fn _empty() -> RustRelativePath {
        RustRelativePath { path: Vec::new() }
    }

    pub fn from_components<I: IntoIterator<Item = RustPathComponent>>(i: I) -> RustRelativePath {
        RustRelativePath {
            path: i.into_iter().collect(),
        }
    }

    pub fn from_idents<I: IntoIterator<Item = RustIdent>>(i: I) -> RustRelativePath {
        Self::from_components(i.into_iter().map(RustPathComponent::Ident))
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn first(&self) -> Option<RustPathComponent> {
        self.path.iter().cloned().next()
    }

    pub fn remove_first(&mut self) -> Option<RustPathComponent> {
        if self.path.is_empty() {
            None
        } else {
            Some(self.path.remove(0))
        }
    }

    pub fn prepend_ident(&mut self, ident: RustIdent) {
        self.path.insert(0, RustPathComponent::Ident(ident));
    }

    pub fn append(mut self, path: RustRelativePath) -> RustRelativePath {
        for c in path.path {
            self.path.push(c);
        }
        self
    }

    pub fn push_ident(&mut self, ident: RustIdent) {
        self.path.push(RustPathComponent::Ident(ident));
    }

    pub fn append_ident(mut self, ident: RustIdent) -> RustRelativePath {
        self.push_ident(ident);
        self
    }

    pub fn to_reverse(&self) -> RustRelativePath {
        RustRelativePath::from_components(
            iter::repeat(RustPathComponent::SUPER).take(self.path.len()),
        )
    }
}

impl fmt::Display for RustRelativePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, c) in self.path.iter().enumerate() {
            if i != 0 {
                write!(f, "::")?;
            }
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl From<&'_ str> for RustRelativePath {
    fn from(s: &str) -> Self {
        assert!(!s.starts_with("::"), "path is absolute: {:?}", s);
        RustRelativePath {
            path: s.split("::").map(RustPathComponent::parse).collect(),
        }
    }
}
