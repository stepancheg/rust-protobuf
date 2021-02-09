use crate::ProtobufAbsolutePath;
use crate::ProtobufRelativePath;

/// Protobuf identifier can be absolute or relative.
pub enum ProtobufPath {
    Abs(ProtobufAbsolutePath),
    Rel(ProtobufRelativePath),
}
