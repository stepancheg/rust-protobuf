#![doc(hidden)]

use std::fmt;
use std::ops::Deref;

use crate::protobuf_ident::ProtobufIdent;
use crate::protobuf_rel_path::ProtobufRelativePath;

/// Protobuf absolute name (e. g. `.foo.Bar`).
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[doc(hidden)]
pub struct ProtobufAbsolutePath {
    pub path: String,
}

impl Default for ProtobufAbsolutePath {
    fn default() -> ProtobufAbsolutePath {
        ProtobufAbsolutePath::root()
    }
}

impl Deref for ProtobufAbsolutePath {
    type Target = str;

    fn deref(&self) -> &str {
        &self.path
    }
}

impl ProtobufAbsolutePath {
    pub fn root() -> ProtobufAbsolutePath {
        ProtobufAbsolutePath::new(String::new())
    }

    /// If given name is an fully quialified protobuf name.
    pub fn is_abs(path: &str) -> bool {
        path.is_empty() || path.starts_with(".")
    }

    pub fn new<S: Into<String>>(path: S) -> ProtobufAbsolutePath {
        let path = path.into();
        assert!(
            ProtobufAbsolutePath::is_abs(&path),
            "path is not absolute: `{}`",
            path
        );
        assert!(!path.ends_with("."), "{}", path);
        ProtobufAbsolutePath { path }
    }

    pub fn concat(mut a: ProtobufAbsolutePath, b: ProtobufRelativePath) -> ProtobufAbsolutePath {
        a.push_relative(&b.into());
        a
    }

    pub fn is_root(&self) -> bool {
        self.path.is_empty()
    }

    pub fn from_path_without_dot(path: &str) -> ProtobufAbsolutePath {
        assert!(!path.is_empty());
        assert!(!path.starts_with("."));
        assert!(!path.ends_with("."));
        ProtobufAbsolutePath::new(format!(".{}", path))
    }

    pub fn from_path_maybe_dot(path: &str) -> ProtobufAbsolutePath {
        if path.starts_with(".") {
            ProtobufAbsolutePath::new(path.to_owned())
        } else {
            ProtobufAbsolutePath::from_path_without_dot(path)
        }
    }

    pub fn push_simple(&mut self, simple: ProtobufIdent) {
        self.path.push('.');
        self.path.push_str(simple.get());
    }

    pub fn push_relative(&mut self, relative: &ProtobufRelativePath) {
        if !relative.is_empty() {
            self.path.push('.');
            self.path.push_str(&format!("{}", relative));
        }
    }

    pub fn remove_prefix(&self, prefix: &ProtobufAbsolutePath) -> Option<ProtobufRelativePath> {
        if self.path.starts_with(&prefix.path) {
            let rem = &self.path[prefix.path.len()..];
            if rem.is_empty() {
                return Some(ProtobufRelativePath::empty());
            }
            if rem.starts_with('.') {
                return Some(ProtobufRelativePath::new(rem[1..].to_owned()));
            }
        }
        None
    }

    pub fn parent(&self) -> Option<ProtobufAbsolutePath> {
        match self.path.rfind('.') {
            Some(pos) => Some(ProtobufAbsolutePath::new(self.path[..pos].to_owned())),
            None => {
                if self.path.is_empty() {
                    None
                } else {
                    Some(ProtobufAbsolutePath::root())
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

    pub fn self_and_parents(&self) -> Vec<ProtobufAbsolutePath> {
        let mut tmp = self.clone();

        let mut r = Vec::new();

        r.push(self.clone());

        while let Some(parent) = tmp.parent() {
            r.push(parent.clone());
            tmp = parent;
        }

        r
    }

    pub fn to_root_rel(&self) -> ProtobufRelativePath {
        if self == &Self::root() {
            ProtobufRelativePath::empty()
        } else {
            ProtobufRelativePath::new(&self.path[1..])
        }
    }

    pub fn starts_with(&self, that: &ProtobufAbsolutePath) -> bool {
        self.remove_prefix(that).is_some()
    }
}

impl From<&'_ str> for ProtobufAbsolutePath {
    fn from(s: &str) -> Self {
        ProtobufAbsolutePath::new(s.to_owned())
    }
}

impl From<String> for ProtobufAbsolutePath {
    fn from(s: String) -> Self {
        ProtobufAbsolutePath::new(s)
    }
}

impl fmt::Display for ProtobufAbsolutePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.path, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn absolute_path_push_simple() {
        let mut foo = ProtobufAbsolutePath::new(".foo".to_owned());
        foo.push_simple(ProtobufIdent::from("bar"));
        assert_eq!(ProtobufAbsolutePath::new(".foo.bar".to_owned()), foo);

        let mut foo = ProtobufAbsolutePath::root();
        foo.push_simple(ProtobufIdent::from("bar"));
        assert_eq!(ProtobufAbsolutePath::new(".bar".to_owned()), foo);
    }

    #[test]
    fn absolute_path_remove_prefix() {
        assert_eq!(
            Some(ProtobufRelativePath::empty()),
            ProtobufAbsolutePath::new(".foo".to_owned())
                .remove_prefix(&ProtobufAbsolutePath::new(".foo".to_owned()))
        );
        assert_eq!(
            Some(ProtobufRelativePath::new("bar".to_owned())),
            ProtobufAbsolutePath::new(".foo.bar".to_owned())
                .remove_prefix(&ProtobufAbsolutePath::new(".foo".to_owned()))
        );
        assert_eq!(
            Some(ProtobufRelativePath::new("baz.qux".to_owned())),
            ProtobufAbsolutePath::new(".foo.bar.baz.qux".to_owned())
                .remove_prefix(&ProtobufAbsolutePath::new(".foo.bar".to_owned()))
        );
        assert_eq!(
            None,
            ProtobufAbsolutePath::new(".foo.barbaz".to_owned())
                .remove_prefix(&ProtobufAbsolutePath::new(".foo.bar".to_owned()))
        );
    }

    #[test]
    fn self_and_parents() {
        assert_eq!(
            vec![
                ProtobufAbsolutePath::new(".ab.cde.fghi".to_owned()),
                ProtobufAbsolutePath::new(".ab.cde".to_owned()),
                ProtobufAbsolutePath::new(".ab".to_owned()),
                ProtobufAbsolutePath::root(),
            ],
            ProtobufAbsolutePath::new(".ab.cde.fghi".to_owned()).self_and_parents()
        );
    }
}
