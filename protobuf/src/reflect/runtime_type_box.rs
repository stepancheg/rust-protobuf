use crate::reflect::EnumDescriptor;
use crate::reflect::MessageDescriptor;

/// Runtime representation of elementary protobuf type.
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
    Enum(&'static EnumDescriptor),
    /// `message`
    Message(&'static MessageDescriptor),
}
