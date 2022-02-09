use std::any::Any;
use std::fmt;

#[cfg(feature = "bytes")]
use ::bytes::Bytes;

#[cfg(feature = "bytes")]
use crate::chars::Chars;
use crate::reflect::runtime_types::RuntimeType;
use crate::reflect::runtime_types::RuntimeTypeBool;
use crate::reflect::runtime_types::RuntimeTypeF32;
use crate::reflect::runtime_types::RuntimeTypeF64;
use crate::reflect::runtime_types::RuntimeTypeI32;
use crate::reflect::runtime_types::RuntimeTypeI64;
use crate::reflect::runtime_types::RuntimeTypeString;
#[cfg(feature = "bytes")]
use crate::reflect::runtime_types::RuntimeTypeTokioBytes;
#[cfg(feature = "bytes")]
use crate::reflect::runtime_types::RuntimeTypeTokioChars;
use crate::reflect::runtime_types::RuntimeTypeU32;
use crate::reflect::runtime_types::RuntimeTypeU64;
use crate::reflect::runtime_types::RuntimeTypeVecU8;
use crate::reflect::value::value_box::ReflectValueBox;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::value::value_ref::ReflectValueRef;
use crate::reflect::RuntimeTypeBox;

pub(crate) mod value_box;
pub(crate) mod value_ref;

/// Type implemented by all protobuf singular types
/// (primitives, string, messages, enums).
///
/// Used in reflection.
pub trait ProtobufValue:
    Any + Clone + Default + fmt::Debug + Send + Sync + Sized + 'static
{
    /// Actual implementation of type properties.
    type RuntimeType: RuntimeType<Value = Self>;

    // TODO: inline the rest

    /// Dynamic version of the type.
    fn runtime_type_box() -> RuntimeTypeBox {
        Self::RuntimeType::runtime_type_box()
    }

    /// Pointer to a dynamic reference.
    fn as_ref(value: &Self) -> ReflectValueRef {
        Self::RuntimeType::as_ref(value)
    }

    /// Mutable pointer to a dynamic mutable reference.
    fn as_mut(value: &mut Self) -> ReflectValueMut {
        Self::RuntimeType::as_mut(value)
    }

    /// Construct a value from given reflective value.
    ///
    /// # Panics
    ///
    /// If reflective value is of incompatible type.
    fn from_value_box(value_box: ReflectValueBox) -> Result<Self, ReflectValueBox> {
        Self::RuntimeType::from_value_box(value_box)
    }

    /// Write the value.
    fn set_from_value_box(target: &mut Self, value_box: ReflectValueBox) {
        Self::RuntimeType::set_from_value_box(target, value_box)
    }

    /// Default value for this type.
    fn default_value_ref() -> ReflectValueRef<'static> {
        Self::RuntimeType::default_value_ref()
    }

    /// Convert a value into a ref value if possible.
    ///
    /// # Panics
    ///
    /// For message and enum.
    fn into_static_value_ref(value: Self) -> ReflectValueRef<'static> {
        Self::RuntimeType::into_static_value_ref(value)
    }

    /// Value is non-default?
    fn is_non_zero(value: &Self) -> bool {
        Self::RuntimeType::is_non_zero(value)
    }

    /// Cast enum element data to integers.
    ///
    /// # Panics
    ///
    /// If self does not represent an enum.
    fn cast_to_enum_values(values: &[Self]) -> &[i32] {
        Self::RuntimeType::cast_to_enum_values(values)
    }
}

impl ProtobufValue for u32 {
    type RuntimeType = RuntimeTypeU32;
}

impl ProtobufValue for u64 {
    type RuntimeType = RuntimeTypeU64;
}

impl ProtobufValue for i32 {
    type RuntimeType = RuntimeTypeI32;
}

impl ProtobufValue for i64 {
    type RuntimeType = RuntimeTypeI64;
}

impl ProtobufValue for f32 {
    type RuntimeType = RuntimeTypeF32;
}

impl ProtobufValue for f64 {
    type RuntimeType = RuntimeTypeF64;
}

impl ProtobufValue for bool {
    type RuntimeType = RuntimeTypeBool;
}

impl ProtobufValue for String {
    type RuntimeType = RuntimeTypeString;
}

impl ProtobufValue for Vec<u8> {
    type RuntimeType = RuntimeTypeVecU8;
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Bytes {
    type RuntimeType = RuntimeTypeTokioBytes;
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Chars {
    type RuntimeType = RuntimeTypeTokioChars;
}

// conflicting implementations, so generated code is used instead
/*
impl<E : ProtobufEnum> ProtobufValue for E {
}

impl<M : Message> ProtobufValue for M {
}
*/
