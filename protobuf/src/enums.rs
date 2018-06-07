use std::hash::{Hash,Hasher};

use reflect::EnumDescriptor;
use reflect::EnumValueDescriptor;
use reflect::ProtobufValue;

/// A helper trait for construct `ProtobufEnum`.
pub trait ProtoEnum: Sized + Copy + Send + Sync + ProtobufValue + 'static {
    fn from_i32(v: i32) -> Result<Self, i32>;
    fn value(&self) -> i32;
    fn values() -> &'static [Self];
    /// Get enum descriptor by type.
    fn enum_descriptor_static() -> &'static EnumDescriptor;
}

#[derive(Debug, Copy, Clone)]
pub struct ProtobufEnum<E>(Result<E, i32>);

impl<E: ProtoEnum> ProtobufEnum<E> {
    /// Get enum `i32` value.
    pub fn value(&self) -> i32 {
        match self.0 {
            Ok(e) => e.value(),
            Err(v) => v,
        }
    }

    /// Create an enum from `i32` value.
    pub fn from_i32(v: i32) -> Self {
        ProtobufEnum(E::from_i32(v))
    }

    /// Get all enum values for enum type.
    pub fn values() -> &'static [E] {
        E::values()
    }

    /// Get enum value descriptor.
    pub fn descriptor(&self) -> &'static EnumValueDescriptor {
        E::enum_descriptor_static().value_by_number(self.value()).unwrap()
    }

    pub fn result(self) -> Result<E, i32> {
        self.0
    }

    pub fn unwrap(&self) -> E {
        self.result().unwrap()
    }

    pub fn from_enum(e: E) -> Self {
        ProtobufEnum(Ok(e))
    }
}

impl<E: ProtoEnum + Default> Default for ProtobufEnum<E> {
    fn default() -> Self {
        ProtobufEnum(Ok(E::default()))
    }
}

impl<E: ProtoEnum> PartialEq for ProtobufEnum<E> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl<E: ProtoEnum> Hash for ProtobufEnum<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.value())
    }
}

impl<E: ProtoEnum> ProtobufValue for ProtobufEnum<E> {}
