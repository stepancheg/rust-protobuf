use std::fmt;

use crate::protobuf_abs_path::ProtobufAbsolutePath;
use crate::protobuf_rel_path::ProtobufRelativePath;

/// Protobuf identifier can be absolute or relative.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum ProtobufPath {
    Abs(ProtobufAbsolutePath),
    Rel(ProtobufRelativePath),
}

impl ProtobufPath {
    pub fn new<S: Into<String>>(path: S) -> ProtobufPath {
        let path = path.into();
        if path.starts_with('.') {
            ProtobufPath::Abs(ProtobufAbsolutePath::new(path))
        } else {
            ProtobufPath::Rel(ProtobufRelativePath::new(path))
        }
    }

    pub fn _resolve(&self, package: &ProtobufAbsolutePath) -> ProtobufAbsolutePath {
        match self {
            ProtobufPath::Abs(p) => p.clone(),
            ProtobufPath::Rel(p) => {
                let mut package = package.clone();
                package.push_relative(p);
                package
            }
        }
    }
}

impl fmt::Display for ProtobufPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtobufPath::Abs(p) => write!(f, "{}", p),
            ProtobufPath::Rel(p) => write!(f, "{}", p),
        }
    }
}
