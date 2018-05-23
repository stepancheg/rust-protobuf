use std::any::Any;


#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "bytes")]
use chars::Chars;

use core::*;
use super::*;
use super::as_any::AsAny;
use reflect::transmute_eq::transmute_eq;


/// Hack against lack of upcasting in Rust
pub trait AsProtobufValue {
    fn as_protobuf_value(&self) -> &ProtobufValue;
    fn as_protobuf_value_mut(&mut self) -> &mut ProtobufValue;
}

impl<T : ProtobufValue> AsProtobufValue for T {
    fn as_protobuf_value(&self) -> &ProtobufValue {
        self
    }

    fn as_protobuf_value_mut(&mut self) -> &mut ProtobufValue {
        self
    }
}


/// Type implemented by all protobuf singular types
/// (primitives, string, messages, enums).
///
/// Used for dynamic casting in reflection.
pub trait ProtobufValue : Any + AsAny + AsProtobufValue + 'static + Send + Sync {
}

impl ProtobufValue for u32 {
}

impl ProtobufValue for u64 {
}

impl ProtobufValue for i32 {
}

impl ProtobufValue for i64 {
}

impl ProtobufValue for f32 {
}

impl ProtobufValue for f64 {
}

impl ProtobufValue for bool {
}

impl ProtobufValue for String {
}

impl ProtobufValue for Vec<u8> {
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Bytes {
}

#[cfg(feature = "bytes")]
impl ProtobufValue for Chars {
}

// conflicting implementations, so generated code is used instead
/*
impl<E : ProtobufEnum> ProtobufValue for E {
}

impl<M : Message> ProtobufValue for M {
}
*/


#[derive(Debug)]
pub enum ReflectValueRef<'a> {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(&'a str),
    Bytes(&'a [u8]),
    Enum(&'static EnumValueDescriptor),
    Message(&'a Message),
}

impl<'a> ReflectValueRef<'a> {
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
}

#[derive(Debug, Clone)]
pub enum ReflectValueBox {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    Enum(&'static EnumValueDescriptor),
    Message(Box<Message>),
}

fn _assert_value_box_send_sync() {
    fn _assert_send_sync<T : Send + Sync>() {}
    _assert_send_sync::<ReflectValueBox>();
}

#[cfg(not(feature = "bytes"))]
type VecU8OrBytes = Vec<u8>;
#[cfg(feature = "bytes")]
type VecU8OrBytes = Vec<u8>;
#[cfg(not(feature = "bytes"))]
type StringOrChars = String;
#[cfg(feature = "bytes")]
type StringOrChars = Chars;


impl ReflectValueBox {
    pub fn as_value_ref(&self) -> ReflectValueRef {
        use std::ops::Deref;
        match *self {
            ReflectValueBox::U32(v) => ReflectValueRef::U32(v),
            ReflectValueBox::U64(v) => ReflectValueRef::U64(v),
            ReflectValueBox::I32(v) => ReflectValueRef::I32(v),
            ReflectValueBox::I64(v) => ReflectValueRef::I64(v),
            ReflectValueBox::F32(v) => ReflectValueRef::F32(v),
            ReflectValueBox::F64(v) => ReflectValueRef::F64(v),
            ReflectValueBox::Bool(v) => ReflectValueRef::Bool(v),
            ReflectValueBox::String(ref v) => ReflectValueRef::String(v.as_str()),
            ReflectValueBox::Bytes(ref v) => ReflectValueRef::Bytes(v.as_slice()),
            ReflectValueBox::Enum(v) => ReflectValueRef::Enum(v),
            ReflectValueBox::Message(ref v) => ReflectValueRef::Message(v.deref()),
        }
    }

    pub fn as_value(&self) -> &ProtobufValue {
        match self {
            ReflectValueBox::U32(v) => v,
            ReflectValueBox::U64(v) => v,
            ReflectValueBox::I32(v) => v,
            ReflectValueBox::I64(v) => v,
            ReflectValueBox::F32(v) => v,
            ReflectValueBox::F64(v) => v,
            ReflectValueBox::Bool(v) => v,
            ReflectValueBox::String(v) => v,
            ReflectValueBox::Bytes(v) => v,
            ReflectValueBox::Enum(v) => v.protobuf_value(),
            ReflectValueBox::Message(v) => v.as_protobuf_value(),
        }
    }

    pub fn as_value_mut(&mut self) -> &mut ProtobufValue {
        match self {
            ReflectValueBox::U32(v) => v,
            ReflectValueBox::U64(v) => v,
            ReflectValueBox::I32(v) => v,
            ReflectValueBox::I64(v) => v,
            ReflectValueBox::F32(v) => v,
            ReflectValueBox::F64(v) => v,
            ReflectValueBox::Bool(v) => v,
            ReflectValueBox::String(v) => v,
            ReflectValueBox::Bytes(v) => v,
            ReflectValueBox::Enum(_v) => panic!("enum value cannot be mutable"),
            ReflectValueBox::Message(v) => v.as_protobuf_value_mut(),
        }
    }

    pub fn downcast<V : 'static>(self) -> Result<V, Self> {
        match self {
            ReflectValueBox::U32(v) => transmute_eq(v).map_err(ReflectValueBox::U32),
            ReflectValueBox::U64(v) => transmute_eq(v).map_err(ReflectValueBox::U64),
            ReflectValueBox::I32(v) => transmute_eq(v).map_err(ReflectValueBox::I32),
            ReflectValueBox::I64(v) => transmute_eq(v).map_err(ReflectValueBox::I64),
            ReflectValueBox::F32(v) => transmute_eq(v).map_err(ReflectValueBox::F32),
            ReflectValueBox::F64(v) => transmute_eq(v).map_err(ReflectValueBox::F64),
            ReflectValueBox::Bool(v) => transmute_eq(v).map_err(ReflectValueBox::Bool),
            ReflectValueBox::String(v) => {
                transmute_eq::<String, _>(v)
                    .or_else(|v| transmute_eq::<StringOrChars, _>(v))
                    .map_err(ReflectValueBox::String)
            },
            ReflectValueBox::Bytes(v) => {
                transmute_eq::<Vec<u8>, _>(v)
                    .or_else(|v| transmute_eq::<VecU8OrBytes, _>(v))
                    .map_err(ReflectValueBox::Bytes)
            },
            ReflectValueBox::Enum(e) => {
                e.cast().ok_or(ReflectValueBox::Enum(e))
            }
            ReflectValueBox::Message(m) => {
                m.descriptor().cast(m).map_err(ReflectValueBox::Message)
            }
        }
    }
}

impl<'a> PartialEq for ReflectValueRef<'a> {
    fn eq(&self, other: &ReflectValueRef) -> bool {
        use self::ReflectValueRef::*;
        match (self, other) {
            (U32(a), U32(b)) => a == b,
            (U64(a), U64(b)) => a == b,
            (I32(a), I32(b)) => a == b,
            (I64(a), I64(b)) => a == b,
            // should probably NaN == NaN here
            (F32(a), F32(b)) => a == b,
            (F64(a), F64(b)) => a == b,
            (Bool(a), Bool(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Bytes(a), Bytes(b)) => a == b,
            (Enum(a), Enum(b)) => a == b,
            (Message(a), Message(b)) => {
                use std::ops::Deref;
                a.descriptor() == b.descriptor() && a.descriptor().eq(a.deref(), b.deref())
            },
            _ => false,
        }
    }
}

impl<'a> PartialEq for ReflectValueBox {
    fn eq(&self, other: &Self) -> bool {
        self.as_value_ref() == other.as_value_ref()
    }
}

impl<'a> PartialEq<ReflectValueRef<'a>> for ReflectValueBox {
    fn eq(&self, other: &ReflectValueRef) -> bool {
        self.as_value_ref() == *other
    }
}

impl<'a> PartialEq<ReflectValueBox> for ReflectValueRef<'a> {
    fn eq(&self, other: &ReflectValueBox) -> bool {
        *self == other.as_value_ref()
    }
}
