use crate::reflect::protobuf_type_box::ProtobufTypeBox;
use crate::reflect::RuntimeFieldType;

/// Reflective representation of field type plus wire type.
pub(crate) enum ProtobufFieldType {
    /// Singular field (required, optional for proto2 or singular for proto3)
    Singular(ProtobufTypeBox),
    /// Repeated field
    Repeated(ProtobufTypeBox),
    /// Map field
    Map(ProtobufTypeBox, ProtobufTypeBox),
}

impl ProtobufFieldType {
    /// Drop wire type from the type.
    pub fn runtime(&self) -> RuntimeFieldType {
        match self {
            ProtobufFieldType::Singular(t) => RuntimeFieldType::Singular(t.runtime().clone()),
            ProtobufFieldType::Repeated(t) => RuntimeFieldType::Repeated(t.runtime().clone()),
            ProtobufFieldType::Map(kt, vt) => {
                RuntimeFieldType::Map(kt.runtime().clone(), vt.runtime().clone())
            }
        }
    }
}
