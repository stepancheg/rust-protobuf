#![doc(hidden)]

use std::fmt;
use std::mem;
use std::ops::Deref;

use crate::protobuf_ident::ProtobufIdent;
use crate::protobuf_rel_path::ProtobufRelPath;
use crate::ProtobufIdentRef;
use crate::ProtobufRelPathRef;

/// Protobuf absolute name (e. g. `.foo.Bar`).
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[doc(hidden)]
pub struct ProtobufAbsPath {
    pub path: String,
}

#[doc(hidden)]
#[derive(Eq, PartialEq, Debug, Hash)]
#[repr(C)]
pub struct ProtobufAbsPathRef(str);

impl Default for ProtobufAbsPath {
    fn default() -> ProtobufAbsPath {
        ProtobufAbsPath::root()
    }
}

impl Deref for ProtobufAbsPathRef {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl Deref for ProtobufAbsPath {
    type Target = ProtobufAbsPathRef;

    fn deref(&self) -> &ProtobufAbsPathRef {
        ProtobufAbsPathRef::new(&self.path)
    }
}

impl ProtobufAbsPathRef {
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn root() -> &'static ProtobufAbsPathRef {
        Self::new("")
    }

    pub fn new(path: &str) -> &ProtobufAbsPathRef {
        assert!(ProtobufAbsPath::is_abs(path));
        // SAFETY: repr(transparent)
        unsafe { mem::transmute(path) }
    }

    pub fn to_owned(&self) -> ProtobufAbsPath {
        ProtobufAbsPath {
            path: self.0.to_owned(),
        }
    }

    pub fn parent(&self) -> Option<&ProtobufAbsPathRef> {
        match self.0.rfind('.') {
            Some(pos) => Some(ProtobufAbsPathRef::new(&self.0[..pos])),
            None => {
                if self.0.is_empty() {
                    None
                } else {
                    Some(ProtobufAbsPathRef::root())
                }
            }
        }
    }

    pub fn self_and_parents(&self) -> Vec<&ProtobufAbsPathRef> {
        let mut tmp = self;

        let mut r: Vec<&ProtobufAbsPathRef> = Vec::new();

        r.push(&self);

        while let Some(parent) = tmp.parent() {
            r.push(parent);
            tmp = parent;
        }

        r
    }
}

impl ProtobufAbsPath {
    pub fn root() -> ProtobufAbsPath {
        ProtobufAbsPathRef::root().to_owned()
    }

    /// If given name is an fully quialified protobuf name.
    pub fn is_abs(path: &str) -> bool {
        path.is_empty() || (path.starts_with(".") && path != ".")
    }

    pub fn try_new(path: &str) -> Option<ProtobufAbsPath> {
        if ProtobufAbsPath::is_abs(path) {
            Some(ProtobufAbsPath::new(path))
        } else {
            None
        }
    }

    pub fn new<S: Into<String>>(path: S) -> ProtobufAbsPath {
        let path = path.into();
        assert!(
            ProtobufAbsPath::is_abs(&path),
            "path is not absolute: `{}`",
            path
        );
        assert!(!path.ends_with("."), "{}", path);
        ProtobufAbsPath { path }
    }

    pub fn concat(a: &ProtobufAbsPathRef, b: &ProtobufRelPathRef) -> ProtobufAbsPath {
        let mut a = a.to_owned();
        a.push_relative(b);
        a
    }

    pub fn from_path_without_dot(path: &str) -> ProtobufAbsPath {
        assert!(!path.is_empty());
        assert!(!path.starts_with("."));
        assert!(!path.ends_with("."));
        ProtobufAbsPath::new(format!(".{}", path))
    }

    pub fn from_path_maybe_dot(path: &str) -> ProtobufAbsPath {
        if path.starts_with(".") {
            ProtobufAbsPath::new(path.to_owned())
        } else {
            ProtobufAbsPath::from_path_without_dot(path)
        }
    }

    pub fn push_simple(&mut self, simple: &ProtobufIdentRef) {
        self.path.push('.');
        self.path.push_str(&simple);
    }

    pub fn push_relative(&mut self, relative: &ProtobufRelPathRef) {
        if !relative.is_empty() {
            self.path.push_str(&format!(".{}", relative));
        }
    }

    pub fn remove_prefix(&self, prefix: &ProtobufAbsPathRef) -> Option<&ProtobufRelPathRef> {
        if self.path.starts_with(&prefix.0) {
            let rem = &self.path[prefix.0.len()..];
            if rem.is_empty() {
                return Some(ProtobufRelPathRef::empty());
            }
            if rem.starts_with('.') {
                return Some(ProtobufRelPathRef::new(&rem[1..]));
            }
        }
        None
    }

    pub fn remove_suffix(&self, suffix: &ProtobufRelPathRef) -> Option<&ProtobufAbsPathRef> {
        if suffix.is_empty() {
            return Some(ProtobufAbsPathRef::new(&self.path));
        }

        if self.path.ends_with(suffix.as_str()) {
            let rem = &self.path[..self.path.len() - suffix.as_str().len()];
            if rem.is_empty() {
                return Some(ProtobufAbsPathRef::root());
            }
            if rem.ends_with('.') {
                return Some(ProtobufAbsPathRef::new(&rem[..rem.len() - 1]));
            }
        }
        None
    }

    /// Pop the last name component
    pub fn pop(&mut self) -> Option<ProtobufIdent> {
        match self.path.rfind('.') {
            Some(dot) => {
                let ident = ProtobufIdent::new(&self.path[dot + 1..]);
                self.path.truncate(dot);
                Some(ident)
            }
            None => None,
        }
    }

    pub fn to_root_rel(&self) -> ProtobufRelPath {
        if self == &Self::root() {
            ProtobufRelPath::empty()
        } else {
            ProtobufRelPath::new(&self.path[1..])
        }
    }

    pub fn starts_with(&self, that: &ProtobufAbsPath) -> bool {
        self.remove_prefix(that).is_some()
    }

    pub fn ends_with(&self, that: &ProtobufRelPath) -> bool {
        self.remove_suffix(that).is_some()
    }
}

impl From<&'_ str> for ProtobufAbsPath {
    fn from(s: &str) -> Self {
        ProtobufAbsPath::new(s.to_owned())
    }
}

impl From<String> for ProtobufAbsPath {
    fn from(s: String) -> Self {
        ProtobufAbsPath::new(s)
    }
}

impl fmt::Display for ProtobufAbsPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.path, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn absolute_path_push_simple() {
        let mut foo = ProtobufAbsPath::new(".foo".to_owned());
        foo.push_simple(ProtobufIdentRef::new("bar"));
        assert_eq!(ProtobufAbsPath::new(".foo.bar".to_owned()), foo);

        let mut foo = ProtobufAbsPath::root();
        foo.push_simple(ProtobufIdentRef::new("bar"));
        assert_eq!(ProtobufAbsPath::new(".bar".to_owned()), foo);
    }

    #[test]
    fn absolute_path_remove_prefix() {
        assert_eq!(
            Some(ProtobufRelPathRef::empty()),
            ProtobufAbsPath::new(".foo".to_owned())
                .remove_prefix(&ProtobufAbsPath::new(".foo".to_owned()))
        );
        assert_eq!(
            Some(ProtobufRelPathRef::new("bar")),
            ProtobufAbsPath::new(".foo.bar".to_owned())
                .remove_prefix(&ProtobufAbsPath::new(".foo".to_owned()))
        );
        assert_eq!(
            Some(ProtobufRelPathRef::new("baz.qux")),
            ProtobufAbsPath::new(".foo.bar.baz.qux".to_owned())
                .remove_prefix(&ProtobufAbsPath::new(".foo.bar".to_owned()))
        );
        assert_eq!(
            None,
            ProtobufAbsPath::new(".foo.barbaz".to_owned())
                .remove_prefix(ProtobufAbsPathRef::new(".foo.bar"))
        );
    }

    #[test]
    fn self_and_parents() {
        assert_eq!(
            vec![
                ProtobufAbsPathRef::new(".ab.cde.fghi"),
                ProtobufAbsPathRef::new(".ab.cde"),
                ProtobufAbsPathRef::new(".ab"),
                ProtobufAbsPathRef::root(),
            ],
            ProtobufAbsPath::new(".ab.cde.fghi".to_owned()).self_and_parents()
        );
    }

    #[test]
    fn ends_with() {
        assert!(ProtobufAbsPath::new(".foo.bar").ends_with(&ProtobufRelPath::new("")));
        assert!(ProtobufAbsPath::new(".foo.bar").ends_with(&ProtobufRelPath::new("bar")));
        assert!(ProtobufAbsPath::new(".foo.bar").ends_with(&ProtobufRelPath::new("foo.bar")));
        assert!(!ProtobufAbsPath::new(".foo.bar").ends_with(&ProtobufRelPath::new("foo.bar.baz")));
    }
}
