#![doc(hidden)]

use std::fmt;
use std::iter;

use crate::protobuf_abs_path::ProtobufAbsPath;
use crate::protobuf_ident::ProtobufIdent;
use crate::ProtobufIdentRef;

impl From<String> for ProtobufRelPath {
    fn from(s: String) -> ProtobufRelPath {
        ProtobufRelPath::new(s)
    }
}

impl From<Vec<ProtobufIdent>> for ProtobufRelPath {
    fn from(s: Vec<ProtobufIdent>) -> ProtobufRelPath {
        ProtobufRelPath::from_components(s.into_iter())
    }
}

impl From<&'_ str> for ProtobufRelPath {
    fn from(s: &str) -> ProtobufRelPath {
        ProtobufRelPath::from(s.to_owned())
    }
}

impl ProtobufRelPath {
    pub fn empty() -> ProtobufRelPath {
        ProtobufRelPath {
            path: String::new(),
        }
    }

    pub fn new<S: Into<String>>(path: S) -> ProtobufRelPath {
        let path = path.into();
        assert!(!path.starts_with("."));
        ProtobufRelPath { path }
    }

    pub fn from_components<I: IntoIterator<Item = ProtobufIdent>>(i: I) -> ProtobufRelPath {
        let v: Vec<String> = i.into_iter().map(|c| c.get().to_owned()).collect();
        ProtobufRelPath::from(v.join("."))
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn to_absolute(&self) -> ProtobufAbsPath {
        self.clone().into_absolute()
    }

    pub fn into_absolute(self) -> ProtobufAbsPath {
        if self.is_empty() {
            ProtobufAbsPath::root()
        } else {
            ProtobufAbsPath::from(format!(".{}", self))
        }
    }

    fn parent(&self) -> Option<ProtobufRelPath> {
        if self.path.is_empty() {
            None
        } else {
            match self.path.rfind('.') {
                Some(i) => Some(ProtobufRelPath {
                    path: self.path[..i].to_owned(),
                }),
                None => Some(ProtobufRelPath::empty()),
            }
        }
    }

    pub fn self_and_parents(&self) -> Vec<ProtobufRelPath> {
        let mut tmp = self.clone();

        let mut r = Vec::new();

        r.push(self.clone());

        while let Some(parent) = tmp.parent() {
            r.push(parent.clone());
            tmp = parent;
        }

        r
    }

    pub fn append(&self, simple: &ProtobufRelPath) -> ProtobufRelPath {
        if self.is_empty() {
            simple.clone()
        } else if simple.is_empty() {
            self.clone()
        } else {
            ProtobufRelPath {
                path: format!("{}.{}", self.path, simple.path),
            }
        }
    }

    pub fn append_ident(&self, simple: &ProtobufIdentRef) -> ProtobufRelPath {
        self.append(&ProtobufRelPath::from(simple.to_owned()))
    }

    pub fn split_first_rem(&self) -> Option<(&ProtobufIdentRef, ProtobufRelPath)> {
        if self.is_empty() {
            None
        } else {
            match self.path.find('.') {
                Some(i) => Some((
                    ProtobufIdentRef::new(&self.path[..i]),
                    ProtobufRelPath {
                        path: self.path[i + 1..].to_owned(),
                    },
                )),
                None => Some((ProtobufIdentRef::new(&self.path), ProtobufRelPath::empty())),
            }
        }
    }

    pub fn components(&self) -> impl Iterator<Item = &ProtobufIdentRef> {
        iter::once(&self.path)
            .filter(|s| !s.is_empty())
            .flat_map(|p| p.split('.').map(|s| ProtobufIdentRef::new(s)))
    }
}

#[doc(hidden)]
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ProtobufRelPath {
    pub(crate) path: String,
}

impl From<ProtobufIdent> for ProtobufRelPath {
    fn from(s: ProtobufIdent) -> ProtobufRelPath {
        ProtobufRelPath::from(s.get())
    }
}

impl fmt::Display for ProtobufRelPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parent() {
        assert_eq!(None, ProtobufRelPath::empty().parent());
        assert_eq!(
            Some(ProtobufRelPath::empty()),
            ProtobufRelPath::new("aaa".to_owned()).parent()
        );
        assert_eq!(
            Some(ProtobufRelPath::new("abc".to_owned())),
            ProtobufRelPath::new("abc.def".to_owned()).parent()
        );
        assert_eq!(
            Some(ProtobufRelPath::new("abc.def".to_owned())),
            ProtobufRelPath::new("abc.def.gh".to_owned()).parent()
        );
    }

    #[test]
    fn self_and_parents() {
        assert_eq!(
            vec![
                ProtobufRelPath::new("ab.cde.fghi".to_owned()),
                ProtobufRelPath::new("ab.cde".to_owned()),
                ProtobufRelPath::new("ab".to_owned()),
                ProtobufRelPath::empty(),
            ],
            ProtobufRelPath::new("ab.cde.fghi".to_owned()).self_and_parents()
        );
    }

    #[test]
    fn components() {
        assert_eq!(
            Vec::<&ProtobufIdentRef>::new(),
            ProtobufRelPath::empty().components().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![ProtobufIdentRef::new("ab")],
            ProtobufRelPath::new("ab").components().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![ProtobufIdentRef::new("ab"), ProtobufIdentRef::new("cd")],
            ProtobufRelPath::new("ab.cd")
                .components()
                .collect::<Vec<_>>()
        );
    }
}
