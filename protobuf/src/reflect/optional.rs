use std::mem;

use super::value::ProtobufValue;
use crate::SingularField;
use crate::SingularPtrField;

pub trait ReflectOptional: 'static {
    fn to_option(&self) -> Option<&ProtobufValue>;

    fn set_value(&mut self, value: &ProtobufValue);
}

impl<V: ProtobufValue + Clone + 'static> ReflectOptional for Option<V> {
    fn to_option(&self) -> Option<&ProtobufValue> {
        self.as_ref().map(|v| v as &ProtobufValue)
    }

    fn set_value(&mut self, value: &ProtobufValue) {
        *self = Some(value.as_ref().to_box().downcast().unwrap());
    }
}

impl<V: ProtobufValue + Clone + 'static> ReflectOptional for SingularField<V> {
    fn to_option(&self) -> Option<&ProtobufValue> {
        self.as_ref().map(|v| v as &ProtobufValue)
    }

    fn set_value(&mut self, value: &ProtobufValue) {
        *self = SingularField::some(value.as_ref().to_box().downcast().unwrap());
    }
}

impl<V: ProtobufValue + Clone + 'static> ReflectOptional for SingularPtrField<V> {
    fn to_option(&self) -> Option<&ProtobufValue> {
        self.as_ref().map(|v| v as &ProtobufValue)
    }

    fn set_value(&mut self, value: &ProtobufValue) {
        *self = SingularPtrField::some(value.as_ref().to_box().downcast().unwrap());
    }
}
