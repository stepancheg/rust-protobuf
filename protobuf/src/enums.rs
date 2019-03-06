use std::fmt;

use reflect::EnumDescriptor;
use reflect::EnumValueDescriptor;
use reflect::ProtobufValue;
use std::marker;

/// Trait implemented by all protobuf enum types.
pub trait ProtobufEnum: Eq + Sized + Copy + 'static + ProtobufValue + fmt::Debug + Default {
    /// Get enum `i32` value.
    fn value(&self) -> i32;

    /// Try to create an enum from `i32` value.
    /// Return `None` if value is unknown.
    fn from_i32(v: i32) -> Option<Self>;

    /// Get all enum values for enum type.
    fn values() -> &'static [Self] {
        panic!();
    }

    /// Get enum value descriptor.
    fn descriptor(&self) -> &'static EnumValueDescriptor {
        self.enum_descriptor()
            .value_by_number(self.value())
            .unwrap()
    }

    /// Get enum descriptor.
    fn enum_descriptor(&self) -> &'static EnumDescriptor {
        Self::enum_descriptor_static()
    }

    /// Get enum descriptor by type.
    fn enum_descriptor_static() -> &'static EnumDescriptor {
        panic!();
    }
}

/// Protobuf enums with possibly unknown values are preserved in this struct.
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct ProtobufEnumOrUnknown<E: ProtobufEnum> {
    value: i32,
    _marker: marker::PhantomData<E>,
}

impl<E: ProtobufEnum> ProtobufEnumOrUnknown<E> {
    /// Construct from typed enum
    pub fn new(e: E) -> ProtobufEnumOrUnknown<E> {
        ProtobufEnumOrUnknown::from_i32(e.value())
    }

    /// Construct from any `i32` value.
    ///
    /// Note passed value is not required to be a valid enum value.
    pub fn from_i32(value: i32) -> ProtobufEnumOrUnknown<E> {
        ProtobufEnumOrUnknown {
            value,
            _marker: marker::PhantomData,
        }
    }

    /// Get contained `i32` value of enum
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Get `i32` value as typed enum. Return `None` is value is unknown.
    pub fn enum_value(&self) -> Result<E, i32> {
        E::from_i32(self.value).ok_or(self.value)
    }

    /// Get `i32` value as typed enum.
    /// Return default enum value (first value) if value is unknown.
    pub fn enum_value_or_default(&self) -> E {
        self.enum_value().unwrap_or_default()
    }

    /// Get enum descriptor by type.
    pub fn enum_descriptor_static() -> &'static EnumDescriptor {
        E::enum_descriptor_static()
    }
}

impl<E: ProtobufEnum> Default for ProtobufEnumOrUnknown<E> {
    fn default() -> ProtobufEnumOrUnknown<E> {
        ProtobufEnumOrUnknown::new(E::default())
    }
}

impl<E: ProtobufEnum> fmt::Debug for ProtobufEnumOrUnknown<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.enum_value() {
            Ok(e) => fmt::Debug::fmt(&e, f),
            Err(e) => fmt::Debug::fmt(&e, f),
        }
    }
}
