use std::fmt;

use crate::ProtobufAbsolutePath;
use crate::ProtobufIdent;

impl From<String> for ProtobufRelativePath {
    fn from(s: String) -> ProtobufRelativePath {
        ProtobufRelativePath::new(s)
    }
}

impl From<Vec<ProtobufIdent>> for ProtobufRelativePath {
    fn from(s: Vec<ProtobufIdent>) -> ProtobufRelativePath {
        ProtobufRelativePath::from_components(s.into_iter())
    }
}

impl From<&'_ str> for ProtobufRelativePath {
    fn from(s: &str) -> ProtobufRelativePath {
        ProtobufRelativePath::from(s.to_owned())
    }
}

impl ProtobufRelativePath {
    pub fn empty() -> ProtobufRelativePath {
        ProtobufRelativePath { path: Vec::new() }
    }

    pub fn new<S: Into<String>>(path: S) -> ProtobufRelativePath {
        let path = path.into();
        assert!(!path.starts_with("."));
        if path.is_empty() {
            ProtobufRelativePath::empty()
        } else {
            let path = path.split('.').map(ProtobufIdent::new).collect();
            ProtobufRelativePath { path }
        }
    }

    pub fn from_components<I: IntoIterator<Item = ProtobufIdent>>(i: I) -> ProtobufRelativePath {
        let v: Vec<String> = i.into_iter().map(|c| c.get().to_owned()).collect();
        ProtobufRelativePath::from(v.join("."))
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn to_absolute(&self) -> ProtobufAbsolutePath {
        self.clone().into_absolute()
    }

    pub fn into_absolute(self) -> ProtobufAbsolutePath {
        if self.is_empty() {
            ProtobufAbsolutePath::root()
        } else {
            ProtobufAbsolutePath::from(format!(".{}", self))
        }
    }

    fn parent(&self) -> Option<ProtobufRelativePath> {
        if self.path.is_empty() {
            None
        } else {
            Some(ProtobufRelativePath {
                path: self.path[..self.path.len() - 1].to_vec(),
            })
        }
    }

    pub fn self_and_parents(&self) -> Vec<ProtobufRelativePath> {
        let mut tmp = self.clone();

        let mut r = Vec::new();

        r.push(self.clone());

        while let Some(parent) = tmp.parent() {
            r.push(parent.clone());
            tmp = parent;
        }

        r
    }

    pub fn append(&self, simple: &ProtobufRelativePath) -> ProtobufRelativePath {
        let mut path = self.clone();
        path.path.extend(simple.path.iter().cloned());
        path
    }

    pub fn append_ident(&self, simple: &ProtobufIdent) -> ProtobufRelativePath {
        self.append(&ProtobufRelativePath::from(simple.clone()))
    }

    pub fn split_first_rem(&self) -> Option<(ProtobufIdent, ProtobufRelativePath)> {
        if self.is_empty() {
            None
        } else {
            Some((
                self.path[0].clone(),
                ProtobufRelativePath {
                    path: self.path[1..].to_vec(),
                },
            ))
        }
    }

    pub fn components(&self) -> impl Iterator<Item = &ProtobufIdent> {
        self.path.iter()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ProtobufRelativePath {
    path: Vec<ProtobufIdent>,
}

impl From<ProtobufIdent> for ProtobufRelativePath {
    fn from(s: ProtobufIdent) -> ProtobufRelativePath {
        ProtobufRelativePath::from(s.get())
    }
}

impl fmt::Display for ProtobufRelativePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, c) in self.path.iter().enumerate() {
            if i != 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parent() {
        assert_eq!(None, ProtobufRelativePath::empty().parent());
        assert_eq!(
            Some(ProtobufRelativePath::empty()),
            ProtobufRelativePath::new("aaa".to_owned()).parent()
        );
        assert_eq!(
            Some(ProtobufRelativePath::new("abc".to_owned())),
            ProtobufRelativePath::new("abc.def".to_owned()).parent()
        );
        assert_eq!(
            Some(ProtobufRelativePath::new("abc.def".to_owned())),
            ProtobufRelativePath::new("abc.def.gh".to_owned()).parent()
        );
    }

    #[test]
    fn self_and_parents() {
        assert_eq!(
            vec![
                ProtobufRelativePath::new("ab.cde.fghi".to_owned()),
                ProtobufRelativePath::new("ab.cde".to_owned()),
                ProtobufRelativePath::new("ab".to_owned()),
                ProtobufRelativePath::empty(),
            ],
            ProtobufRelativePath::new("ab.cde.fghi".to_owned()).self_and_parents()
        );
    }
}
