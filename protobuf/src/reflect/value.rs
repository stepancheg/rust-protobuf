use std::any::Any;
use std::any::TypeId;


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

#[derive(Debug)]
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
                if TypeId::of::<V>() == TypeId::of::<String>() {
                    Ok(transmute_eq(v).unwrap())
                } else if TypeId::of::<V>() == TypeId::of::<StringOrChars>() {
                    Ok(transmute_eq(StringOrChars::from(v)).unwrap())
                } else {
                    Err(ReflectValueBox::String(v))
                }
            },
            ReflectValueBox::Bytes(v) => {
                if TypeId::of::<V>() == TypeId::of::<Vec<u8>>() {
                    Ok(transmute_eq(v).unwrap())
                } else if TypeId::of::<V>() == TypeId::of::<VecU8OrBytes>() {
                    Ok(transmute_eq(VecU8OrBytes::from(v)).unwrap())
                } else {
                    Err(ReflectValueBox::Bytes(v))
                }
            },
            ReflectValueBox::Enum(e) => {
                if let Some(r) = e.cast() {
                    Ok(r)
                } else {
                    Err(ReflectValueBox::Enum(e))
                }
            }
            ReflectValueBox::Message(m) => {
                m.descriptor().cast(m).map_err(ReflectValueBox::Message)
            }
        }
    }
}

