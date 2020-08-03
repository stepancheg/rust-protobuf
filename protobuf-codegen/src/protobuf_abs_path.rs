use crate::protobuf_ident::ProtobufIdent;
use crate::ProtobufRelativePath;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct ProtobufAbsolutePath {
    pub path: String,
}

impl ProtobufAbsolutePath {
    pub fn root() -> ProtobufAbsolutePath {
        ProtobufAbsolutePath::new(String::new())
    }

    pub fn new(path: String) -> ProtobufAbsolutePath {
        assert!(path.is_empty() || path.starts_with("."), path);
        assert!(!path.ends_with("."), path);
        ProtobufAbsolutePath { path }
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn from_package_path(path: Option<&str>) -> ProtobufAbsolutePath {
        match path {
            None => ProtobufAbsolutePath::root(),
            Some(path) => ProtobufAbsolutePath::from_path_without_dot(path),
        }
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
            self.path.push_str(&relative.path);
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

    fn parent(&self) -> Option<ProtobufAbsolutePath> {
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
}
