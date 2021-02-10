use crate::ProtobufAbsolutePath;
use crate::ProtobufRelativePath;
use std::fmt;

/// Protobuf identifier can be absolute or relative.
pub enum ProtobufPath {
    Abs(ProtobufAbsolutePath),
    Rel(ProtobufRelativePath),
}

impl fmt::Display for ProtobufPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtobufPath::Abs(p) => write!(f, "{}", p),
            ProtobufPath::Rel(_) => write!(f, "{}", p),
        }
    }
}
