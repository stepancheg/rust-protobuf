#![doc(hidden)]

use std::fmt;
use std::ops::Deref;

use crate::protobuf_ident::ProtobufIdent;
use crate::protobuf_rel_path::ProtobufRelPath;
use crate::ProtobufIdentRef;

/// Protobuf absolute name (e. g. `.foo.Bar`).
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[doc(hidden)]
pub struct ProtobufAbsPath {
    pub path: String,
}

impl Default for ProtobufAbsPath {
    fn default() -> ProtobufAbsPath {
        ProtobufAbsPath::root()
    }
}

impl Deref for ProtobufAbsPath {
    type Target = str;

    fn deref(&self) -> &str {
        &self.path
    }
}

impl ProtobufAbsPath {
    pub fn root() -> ProtobufAbsPath {
        ProtobufAbsPath::new(String::new())
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

    pub fn concat(mut a: ProtobufAbsPath, b: ProtobufRelPath) -> ProtobufAbsPath {
        a.push_relative(&b.into());
        a
    }

    pub fn is_root(&self) -> bool {
        self.path.is_empty()
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

    pub fn push_relative(&mut self, relative: &ProtobufRelPath) {
        if !relative.is_empty() {
            self.path.push('.');
            self.path.push_str(&format!("{}", relative));
        }
    }

    pub fn remove_prefix(&self, prefix: &ProtobufAbsPath) -> Option<ProtobufRelPath> {
        if self.path.starts_with(&prefix.path) {
            let rem = &self.path[prefix.path.len()..];
            if rem.is_empty() {
                return Some(ProtobufRelPath::empty());
            }
            if rem.starts_with('.') {
                return Some(ProtobufRelPath::new(rem[1..].to_owned()));
            }
        }
        None
    }

    pub fn remove_suffix(&self, suffix: &ProtobufRelPath) -> Option<ProtobufAbsPath> {
        if suffix.is_empty() {
            return Some(self.clone());
        }

        if self.path.ends_with(&suffix.path) {
            let rem = &self.path[..self.path.len() - suffix.path.len()];
            if rem.is_empty() {
                return Some(ProtobufAbsPath::root());
            }
            if rem.ends_with('.') {
                return Some(ProtobufAbsPath::new(rem[..rem.len() - 1].to_owned()));
            }
        }
        None
    }

    pub fn parent(&self) -> Option<ProtobufAbsPath> {
        match self.path.rfind('.') {
            Some(pos) => Some(ProtobufAbsPath::new(self.path[..pos].to_owned())),
            None => {
                if self.path.is_empty() {
                    None
                } else {
                    Some(ProtobufAbsPath::root())
                }
            }
        }
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

    pub fn self_and_parents(&self) -> Vec<ProtobufAbsPath> {
        let mut tmp = self.clone();

        let mut r = Vec::new();

        r.push(self.clone());

        while let Some(parent) = tmp.parent() {
            r.push(parent.clone());
            tmp = parent;
        }

        r
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
            Some(ProtobufRelPath::empty()),
            ProtobufAbsPath::new(".foo".to_owned())
                .remove_prefix(&ProtobufAbsPath::new(".foo".to_owned()))
        );
        assert_eq!(
            Some(ProtobufRelPath::new("bar".to_owned())),
            ProtobufAbsPath::new(".foo.bar".to_owned())
                .remove_prefix(&ProtobufAbsPath::new(".foo".to_owned()))
        );
        assert_eq!(
            Some(ProtobufRelPath::new("baz.qux".to_owned())),
            ProtobufAbsPath::new(".foo.bar.baz.qux".to_owned())
                .remove_prefix(&ProtobufAbsPath::new(".foo.bar".to_owned()))
        );
        assert_eq!(
            None,
            ProtobufAbsPath::new(".foo.barbaz".to_owned())
                .remove_prefix(&ProtobufAbsPath::new(".foo.bar".to_owned()))
        );
    }

    #[test]
    fn self_and_parents() {
        assert_eq!(
            vec![
                ProtobufAbsPath::new(".ab.cde.fghi".to_owned()),
                ProtobufAbsPath::new(".ab.cde".to_owned()),
                ProtobufAbsPath::new(".ab".to_owned()),
                ProtobufAbsPath::root(),
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
