#![doc(hidden)]

use std::fmt;
use std::iter;
use std::mem;
use std::ops::Deref;

use crate::protobuf_abs_path::ProtobufAbsPath;
use crate::protobuf_ident::ProtobufIdent;
use crate::ProtobufIdentRef;

impl From<String> for ProtobufRelPath {
    fn from(s: String) -> ProtobufRelPath {
        ProtobufRelPath::new(s)
    }
}

impl From<&'_ str> for ProtobufRelPath {
    fn from(s: &str) -> ProtobufRelPath {
        ProtobufRelPath::from(s.to_owned())
    }
}

impl ProtobufRelPathRef {
    pub fn as_str(&self) -> &str {
        &self
    }

    pub fn empty() -> &'static ProtobufRelPathRef {
        Self::new("")
    }

    pub fn new(path: &str) -> &ProtobufRelPathRef {
        assert!(!path.starts_with('.'));
        // SAFETY: repr(transparent)
        unsafe { mem::transmute(path) }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn split_first_rem(&self) -> Option<(&ProtobufIdentRef, &ProtobufRelPathRef)> {
        if self.is_empty() {
            None
        } else {
            match self.0.find('.') {
                Some(i) => Some((
                    ProtobufIdentRef::new(&self.0[..i]),
                    ProtobufRelPathRef::new(&self.0[i + 1..]),
                )),
                None => Some((ProtobufIdentRef::new(&self.0), ProtobufRelPathRef::empty())),
            }
        }
    }

    pub fn components(&self) -> impl Iterator<Item = &ProtobufIdentRef> {
        iter::once(&self.0)
            .filter(|s| !s.is_empty())
            .flat_map(|p| p.split('.').map(|s| ProtobufIdentRef::new(s)))
    }

    fn parent(&self) -> Option<&ProtobufRelPathRef> {
        if self.0.is_empty() {
            None
        } else {
            match self.0.rfind('.') {
                Some(i) => Some(ProtobufRelPathRef::new(&self.0[..i])),
                None => Some(ProtobufRelPathRef::empty()),
            }
        }
    }

    pub fn self_and_parents(&self) -> Vec<&ProtobufRelPathRef> {
        let mut tmp = self;

        let mut r = Vec::new();

        r.push(self);

        while let Some(parent) = tmp.parent() {
            r.push(parent);
            tmp = parent;
        }

        r
    }

    pub fn append(&self, simple: &ProtobufRelPathRef) -> ProtobufRelPath {
        if self.is_empty() {
            simple.to_owned()
        } else if simple.is_empty() {
            self.to_owned()
        } else {
            ProtobufRelPath {
                path: format!("{}.{}", &self.0, &simple.0),
            }
        }
    }

    pub fn append_ident(&self, simple: &ProtobufIdentRef) -> ProtobufRelPath {
        self.append(&ProtobufRelPath::from(simple.to_owned()))
    }

    pub fn to_absolute(&self) -> ProtobufAbsPath {
        self.to_owned().into_absolute()
    }

    pub fn to_owned(&self) -> ProtobufRelPath {
        ProtobufRelPath {
            path: self.0.to_owned(),
        }
    }
}

impl ProtobufRelPath {
    pub fn as_ref(&self) -> &ProtobufRelPathRef {
        &self
    }

    pub fn empty() -> ProtobufRelPath {
        ProtobufRelPath {
            path: String::new(),
        }
    }

    pub fn new<S: Into<String>>(path: S) -> ProtobufRelPath {
        let path = path.into();
        // Validate
        ProtobufRelPathRef::new(&path);
        ProtobufRelPath { path }
    }

    pub fn from_components<'a, I: IntoIterator<Item = &'a ProtobufIdentRef>>(
        i: I,
    ) -> ProtobufRelPath {
        let v: Vec<&str> = i.into_iter().map(|c| c.as_str()).collect();
        ProtobufRelPath::from(v.join("."))
    }

    pub fn into_absolute(self) -> ProtobufAbsPath {
        if self.is_empty() {
            ProtobufAbsPath::root()
        } else {
            ProtobufAbsPath::from(format!(".{}", self))
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ProtobufRelPath {
    pub(crate) path: String,
}

#[doc(hidden)]
#[derive(Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct ProtobufRelPathRef(str);

impl Deref for ProtobufRelPathRef {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl Deref for ProtobufRelPath {
    type Target = ProtobufRelPathRef;

    fn deref(&self) -> &ProtobufRelPathRef {
        ProtobufRelPathRef::new(&self.path)
    }
}

impl From<ProtobufIdent> for ProtobufRelPath {
    fn from(s: ProtobufIdent) -> ProtobufRelPath {
        ProtobufRelPath { path: s.into() }
    }
}

impl fmt::Display for ProtobufRelPathRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
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
        assert_eq!(None, ProtobufRelPathRef::empty().parent());
        assert_eq!(
            Some(ProtobufRelPathRef::empty()),
            ProtobufRelPath::new("aaa".to_owned()).parent()
        );
        assert_eq!(
            Some(ProtobufRelPathRef::new("abc")),
            ProtobufRelPath::new("abc.def".to_owned()).parent()
        );
        assert_eq!(
            Some(ProtobufRelPathRef::new("abc.def")),
            ProtobufRelPath::new("abc.def.gh".to_owned()).parent()
        );
    }

    #[test]
    fn self_and_parents() {
        assert_eq!(
            vec![
                ProtobufRelPathRef::new("ab.cde.fghi"),
                ProtobufRelPathRef::new("ab.cde"),
                ProtobufRelPathRef::new("ab"),
                ProtobufRelPathRef::empty(),
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
