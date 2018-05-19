use std::any::Any;


#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "bytes")]
use chars::Chars;

use core::*;
use super::*;
use super::as_any::AsAny;


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
}

