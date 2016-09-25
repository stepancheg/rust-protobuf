use super::value::ProtobufValue;

use singular::*;

pub trait ReflectOptional : 'static {
    fn to_option(&self) -> Option<&ProtobufValue>;
}

impl<V : ProtobufValue + 'static> ReflectOptional for Option<V> {
    fn to_option(&self) -> Option<&ProtobufValue> {
        self.as_ref().map(|v| v as &ProtobufValue)
    }
}

impl<V : ProtobufValue + 'static> ReflectOptional for SingularField<V> {
    fn to_option(&self) -> Option<&ProtobufValue> {
        self.as_ref().map(|v| v as &ProtobufValue)
    }
}

impl<V : ProtobufValue + 'static> ReflectOptional for SingularPtrField<V> {
    fn to_option(&self) -> Option<&ProtobufValue> {
        self.as_ref().map(|v| v as &ProtobufValue)
    }
}
