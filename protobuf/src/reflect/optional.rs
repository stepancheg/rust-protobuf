use std::mem;

use super::value::ProtobufValue;

use singular::*;

pub trait ReflectOptional<V>: 'static {
    fn to_option_typed(&self) -> Option<&V>;

    fn set_value(&mut self, value: &ProtobufValue);
}

impl<V : ProtobufValue + Clone + 'static> ReflectOptional<V> for Option<V> {
    fn to_option_typed(&self) -> Option<&V> {
        self.as_ref()
    }

    fn set_value(&mut self, value: &ProtobufValue) {
        match value.as_any().downcast_ref::<V>() {
            Some(v) => mem::replace(self, Some(v.clone())),
            None => panic!(),
        };
    }
}

impl<V : ProtobufValue + Clone + 'static> ReflectOptional<V> for SingularField<V> {
    fn to_option_typed(&self) -> Option<&V> {
        self.as_ref()
    }

    fn set_value(&mut self, value: &ProtobufValue) {
        match value.as_any().downcast_ref::<V>() {
            Some(v) => mem::replace(self, SingularField::some(v.clone())),
            None => panic!(),
        };
    }
}

impl<V : ProtobufValue + Clone + 'static> ReflectOptional<V> for SingularPtrField<V> {
    fn to_option_typed(&self) -> Option<&V> {
        self.as_ref()
    }

    fn set_value(&mut self, value: &ProtobufValue) {
        match value.as_any().downcast_ref::<V>() {
            Some(v) => mem::replace(self, SingularPtrField::some(v.clone())),
            None => panic!(),
        };
    }
}
