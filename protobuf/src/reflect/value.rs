use std::any::Any;

#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "bytes")]
use chars::Chars;

use super::*;

/// Type implemented by all protobuf elementary types
/// (ints, floats, bool, string, bytes, enums, messages).
pub trait ProtobufValue: Any + 'static {
    /// As ref
    fn as_ref(&self) -> ReflectValueRef;

    /// Convert to `Any`
    fn as_any(&self) -> &Any {
        unimplemented!()
    }

    /// Is value non-zero?
    fn is_non_zero(&self) -> bool {
        self.as_ref().is_non_zero()
    }

    /// Return `ProtobufValueRef` if self is `Copy`.
    ///
    /// # Panics
    ///
    /// if `Self` is not `Copy`.
    fn as_ref_copy(&self) -> ReflectValueRef<'static>
//where Self : Copy // TODO
    {
        match self.as_ref() {
            ReflectValueRef::Bool(v) => ReflectValueRef::Bool(v),
            ReflectValueRef::U32(v) => ReflectValueRef::U32(v),
            ReflectValueRef::U64(v) => ReflectValueRef::U64(v),
            ReflectValueRef::I32(v) => ReflectValueRef::I32(v),
            ReflectValueRef::I64(v) => ReflectValueRef::I64(v),
            ReflectValueRef::F32(v) => ReflectValueRef::F32(v),
            ReflectValueRef::F64(v) => ReflectValueRef::F64(v),
            ReflectValueRef::Enum(v) => ReflectValueRef::Enum(v),
            ReflectValueRef::String(..)
            | ReflectValueRef::Bytes(..)
            | ReflectValueRef::Message(..) => unreachable!(),
        }
    }
}

impl ProtobufValue for u32 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::U32(*self)
    }
}

impl ProtobufValue for u64 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::U64(*self)
    }
}

impl ProtobufValue for i32 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::I32(*self)
    }
}

impl ProtobufValue for i64 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::I64(*self)
    }
}

impl ProtobufValue for f32 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::F32(*self)
    }
}

impl ProtobufValue for f64 {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::F64(*self)
    }
}

impl ProtobufValue for bool {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::Bool(*self)
    }
}

impl ProtobufValue for String {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::String(*&self)
    }
}

impl ProtobufValue for str {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::String(self)
    }
}

impl ProtobufValue for Vec<u8> {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::Bytes(*&self)
    }
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Bytes {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::Bytes(&*self)
    }
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Chars {
    fn as_ref(&self) -> ReflectValueRef {
        ReflectValueRef::String(&*self)
    }
}

// conflicting implementations, so generated code is used instead
/*
impl<E : ProtobufEnum> ProtobufValue for E {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Enum(self.descriptor())
    }
}

impl<M : Message> ProtobufValue for M {
    fn as_ref(&self) -> ProtobufValueRef {
        ProtobufValueRef::Message(self)
    }
}
*/

/// A reference to a value
#[derive(Debug)]
pub enum ReflectValueRef<'a> {
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `i32`
    I32(i32),
    /// `i64`
    I64(i64),
    /// `f32`
    F32(f32),
    /// `f64`
    F64(f64),
    /// `bool`
    Bool(bool),
    /// `string`
    String(&'a str),
    /// `bytes`
    Bytes(&'a [u8]),
    /// `enum`
    // TODO: change to (i32, EnumDescriptor)
    Enum(&'static EnumValueDescriptor),
    /// `message`
    Message(&'a dyn Message),
}

impl<'a> ReflectValueRef<'a> {
    /// Value is "non-zero"?
    #[doc(hidden)]
    pub fn is_non_zero(&self) -> bool {
        match *self {
            ReflectValueRef::U32(v) => v != 0,
            ReflectValueRef::U64(v) => v != 0,
            ReflectValueRef::I32(v) => v != 0,
            ReflectValueRef::I64(v) => v != 0,
            ReflectValueRef::F32(v) => v != 0.,
            ReflectValueRef::F64(v) => v != 0.,
            ReflectValueRef::Bool(v) => v,
            ReflectValueRef::String(v) => !v.is_empty(),
            ReflectValueRef::Bytes(v) => !v.is_empty(),
            ReflectValueRef::Enum(v) => v.value() != 0,
            ReflectValueRef::Message(_) => true,
        }
    }

    /// Clone to a box
    pub(crate) fn to_box(&self) -> ReflectValueBox {
        match *self {
            ReflectValueRef::U32(v) => ReflectValueBox::U32(v),
            ReflectValueRef::U64(v) => ReflectValueBox::U64(v),
            ReflectValueRef::I32(v) => ReflectValueBox::I32(v),
            ReflectValueRef::I64(v) => ReflectValueBox::I64(v),
            ReflectValueRef::F32(v) => ReflectValueBox::F32(v),
            ReflectValueRef::F64(v) => ReflectValueBox::F64(v),
            ReflectValueRef::Bool(v) => ReflectValueBox::Bool(v),
            ReflectValueRef::String(v) => ReflectValueBox::String(v.to_owned()),
            ReflectValueRef::Bytes(v) => ReflectValueBox::Bytes(v.to_owned()),
            ReflectValueRef::Enum(v) => ReflectValueBox::Enum(v),
            ReflectValueRef::Message(v) => ReflectValueBox::Message(v.descriptor().clone(v)),
        }
    }
}

/// Owner value of any elementary type
#[derive(Debug, Clone)]
pub(crate) enum ReflectValueBox {
    /// `u32`
    U32(u32),
    /// `u64`
    U64(u64),
    /// `i32`
    I32(i32),
    /// `i64`
    I64(i64),
    /// `f32`
    F32(f32),
    /// `f64`
    F64(f64),
    /// `bool`
    Bool(bool),
    /// `string`
    String(String),
    /// `bytes`
    Bytes(Vec<u8>),
    /// `enum`
    // TODO: change to (i32, EnumDescriptor)
    Enum(&'static EnumValueDescriptor),
    /// `message`
    Message(Box<dyn Message>),
}

impl From<u32> for ReflectValueBox {
    fn from(v: u32) -> Self {
        ReflectValueBox::U32(v)
    }
}

impl From<u64> for ReflectValueBox {
    fn from(v: u64) -> Self {
        ReflectValueBox::U64(v)
    }
}

impl From<i32> for ReflectValueBox {
    fn from(v: i32) -> Self {
        ReflectValueBox::I32(v)
    }
}

impl From<i64> for ReflectValueBox {
    fn from(v: i64) -> Self {
        ReflectValueBox::I64(v)
    }
}

impl From<f32> for ReflectValueBox {
    fn from(v: f32) -> Self {
        ReflectValueBox::F32(v)
    }
}

impl From<f64> for ReflectValueBox {
    fn from(v: f64) -> Self {
        ReflectValueBox::F64(v)
    }
}

impl From<bool> for ReflectValueBox {
    fn from(v: bool) -> Self {
        ReflectValueBox::Bool(v)
    }
}

impl From<String> for ReflectValueBox {
    fn from(v: String) -> Self {
        ReflectValueBox::String(v)
    }
}

impl From<Vec<u8>> for ReflectValueBox {
    fn from(v: Vec<u8>) -> Self {
        ReflectValueBox::Bytes(v)
    }
}

impl From<&'static EnumValueDescriptor> for ReflectValueBox {
    fn from(v: &'static EnumValueDescriptor) -> Self {
        ReflectValueBox::Enum(v)
    }
}

impl From<Box<dyn Message>> for ReflectValueBox {
    fn from(v: Box<dyn Message>) -> Self {
        ReflectValueBox::Message(v)
    }
}

fn _assert_value_box_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<ReflectValueBox>();
}
