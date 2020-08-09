use crate::ProtobufAbsolutePath;
use crate::ProtobufIdent;
use std::fmt;

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
        ProtobufRelativePath::new(String::new())
    }

    pub fn new(path: String) -> ProtobufRelativePath {
        assert!(!path.starts_with("."));

        ProtobufRelativePath { path }
    }

    pub fn from_components<I: IntoIterator<Item = ProtobufIdent>>(i: I) -> ProtobufRelativePath {
        let v: Vec<String> = i.into_iter().map(|c| c.get().to_owned()).collect();
        ProtobufRelativePath::from(v.join("."))
    }

    pub fn get(&self) -> &str {
        &self.path
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn into_absolute(self) -> ProtobufAbsolutePath {
        if self.is_empty() {
            ProtobufAbsolutePath::root()
        } else {
            ProtobufAbsolutePath::from(format!(".{}", self))
        }
    }

    fn _last_part(&self) -> Option<&str> {
        match self.path.rfind('.') {
            Some(pos) => Some(&self.path[pos + 1..]),
            None => {
                if self.path.is_empty() {
                    None
                } else {
                    Some(&self.path)
                }
            }
        }
    }

    fn parent(&self) -> Option<ProtobufRelativePath> {
        match self.path.rfind('.') {
            Some(pos) => Some(ProtobufRelativePath::new(self.path[..pos].to_owned())),
            None => {
                if self.path.is_empty() {
                    None
                } else {
                    Some(ProtobufRelativePath::empty())
                }
            }
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
        if self.path.is_empty() {
            ProtobufRelativePath::from(simple.get())
        } else {
            ProtobufRelativePath::new(format!("{}.{}", self.path, simple))
        }
    }

    pub fn append_ident(&self, simple: &ProtobufIdent) -> ProtobufRelativePath {
        self.append(&ProtobufRelativePath::from(simple.clone()))
    }

    pub fn split_first_rem(&self) -> Option<(ProtobufIdent, ProtobufRelativePath)> {
        if self.is_empty() {
            None
        } else {
            Some(match self.path.find('.') {
                Some(dot) => (
                    ProtobufIdent::from(&self.path[..dot]),
                    ProtobufRelativePath::new(self.path[dot + 1..].to_owned()),
                ),
                None => (
                    ProtobufIdent::from(self.path.clone()),
                    ProtobufRelativePath::empty(),
                ),
            })
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ProtobufRelativePath {
    pub path: String,
}

impl From<ProtobufIdent> for ProtobufRelativePath {
    fn from(s: ProtobufIdent) -> ProtobufRelativePath {
        ProtobufRelativePath::from(s.get())
    }
}

impl fmt::Display for ProtobufRelativePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.path, f)
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
    fn last_part() {
        assert_eq!(None, ProtobufRelativePath::empty()._last_part());
        assert_eq!(
            Some("aaa"),
            ProtobufRelativePath::new("aaa".to_owned())._last_part()
        );
        assert_eq!(
            Some("def"),
            ProtobufRelativePath::new("abc.def".to_owned())._last_part()
        );
        assert_eq!(
            Some("gh"),
            ProtobufRelativePath::new("abc.def.gh".to_owned())._last_part()
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
