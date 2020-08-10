use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectValueRef;
use crate::reflect::{EnumDescriptor, MessageRef};
use std::fmt;

/// Runtime representation of elementary protobuf type.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RuntimeTypeBox {
    /// `i32`
    I32,
    /// `i64`
    I64,
    /// `u32`
    U32,
    /// `u64`
    U64,
    /// `f32`
    F32,
    /// `f64`
    F64,
    /// `bool`
    Bool,
    /// [`String`](std::string::String)
    String,
    /// [`Vec<u8>`](std::vec::Vec)
    VecU8,
    /// `enum`
    Enum(EnumDescriptor),
    /// `message`
    Message(MessageDescriptor),
}

impl RuntimeTypeBox {
    pub(crate) fn default_value_ref(&self) -> ReflectValueRef<'static> {
        match self {
            RuntimeTypeBox::I32 => ReflectValueRef::I32(0),
            RuntimeTypeBox::I64 => ReflectValueRef::I64(0),
            RuntimeTypeBox::U32 => ReflectValueRef::U32(0),
            RuntimeTypeBox::U64 => ReflectValueRef::U64(0),
            RuntimeTypeBox::F32 => ReflectValueRef::F32(0.0),
            RuntimeTypeBox::F64 => ReflectValueRef::F64(0.0),
            RuntimeTypeBox::Bool => ReflectValueRef::Bool(false),
            RuntimeTypeBox::String => ReflectValueRef::String(""),
            RuntimeTypeBox::VecU8 => ReflectValueRef::Bytes(b""),
            RuntimeTypeBox::Enum(e) => {
                ReflectValueRef::Enum(e.clone(), e.get_default_value().value())
            }
            RuntimeTypeBox::Message(m) => ReflectValueRef::Message(MessageRef::default_instance(m)),
        }
    }
}

impl fmt::Display for RuntimeTypeBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeTypeBox::I32 => write!(f, "i32"),
            RuntimeTypeBox::I64 => write!(f, "i64"),
            RuntimeTypeBox::U32 => write!(f, "u32"),
            RuntimeTypeBox::U64 => write!(f, "u64"),
            RuntimeTypeBox::F32 => write!(f, "f32"),
            RuntimeTypeBox::F64 => write!(f, "f64"),
            RuntimeTypeBox::Bool => write!(f, "bool"),
            RuntimeTypeBox::String => write!(f, "String"),
            RuntimeTypeBox::VecU8 => write!(f, "Vec<u8>"),
            RuntimeTypeBox::Enum(e) => write!(f, "{}", e.full_name()),
            RuntimeTypeBox::Message(m) => write!(f, "{}", m.full_name()),
        }
    }
}
