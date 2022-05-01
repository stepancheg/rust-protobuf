use std::fmt;

use protobuf_support::lexer::float::parse_protobuf_float;
use protobuf_support::lexer::str_lit::StrLit;

use crate::descriptor::field_descriptor_proto;
use crate::reflect::EnumDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::MessageRef;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;

/// Runtime representation of elementary protobuf type.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RuntimeType {
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

impl RuntimeType {
    pub(crate) fn default_value_ref(&self) -> ReflectValueRef<'static> {
        match self {
            RuntimeType::I32 => ReflectValueRef::I32(0),
            RuntimeType::I64 => ReflectValueRef::I64(0),
            RuntimeType::U32 => ReflectValueRef::U32(0),
            RuntimeType::U64 => ReflectValueRef::U64(0),
            RuntimeType::F32 => ReflectValueRef::F32(0.0),
            RuntimeType::F64 => ReflectValueRef::F64(0.0),
            RuntimeType::Bool => ReflectValueRef::Bool(false),
            RuntimeType::String => ReflectValueRef::String(""),
            RuntimeType::VecU8 => ReflectValueRef::Bytes(b""),
            RuntimeType::Enum(e) => ReflectValueRef::Enum(e.clone(), e.default_value().value()),
            RuntimeType::Message(m) => ReflectValueRef::Message(MessageRef::default_instance(m)),
        }
    }

    pub(crate) fn default_value_box(&self) -> ReflectValueBox {
        self.default_value_ref().to_box()
    }

    /// Rust type from protobuf type.
    ///
    /// # Panics
    ///
    /// Panics for message or enum types (because they can't be resolved without context).
    pub(crate) fn from_proto_type(t: field_descriptor_proto::Type) -> RuntimeType {
        match t {
            field_descriptor_proto::Type::TYPE_UINT32 => RuntimeType::U32,
            field_descriptor_proto::Type::TYPE_UINT64 => RuntimeType::U64,
            field_descriptor_proto::Type::TYPE_INT32 => RuntimeType::I32,
            field_descriptor_proto::Type::TYPE_INT64 => RuntimeType::I64,
            field_descriptor_proto::Type::TYPE_SINT32 => RuntimeType::I32,
            field_descriptor_proto::Type::TYPE_SINT64 => RuntimeType::I64,
            field_descriptor_proto::Type::TYPE_FIXED32 => RuntimeType::U32,
            field_descriptor_proto::Type::TYPE_FIXED64 => RuntimeType::U64,
            field_descriptor_proto::Type::TYPE_SFIXED64 => RuntimeType::I64,
            field_descriptor_proto::Type::TYPE_SFIXED32 => RuntimeType::I32,
            field_descriptor_proto::Type::TYPE_BOOL => RuntimeType::Bool,
            field_descriptor_proto::Type::TYPE_STRING => RuntimeType::String,
            field_descriptor_proto::Type::TYPE_BYTES => RuntimeType::VecU8,
            field_descriptor_proto::Type::TYPE_FLOAT => RuntimeType::F32,
            field_descriptor_proto::Type::TYPE_DOUBLE => RuntimeType::F64,
            field_descriptor_proto::Type::TYPE_ENUM
            | field_descriptor_proto::Type::TYPE_MESSAGE
            | field_descriptor_proto::Type::TYPE_GROUP => panic!(
                "{:?} cannot be converted to runtime type without context",
                t
            ),
        }
    }

    pub(crate) fn parse_proto_default_value(&self, value: &str) -> ReflectValueBox {
        match self {
            // For booleans, "true" or "false"
            RuntimeType::Bool => ReflectValueBox::Bool(if value == "true" {
                true
            } else if value == "false" {
                false
            } else {
                panic!("cannot parse bool default value: {}", value)
            }),
            RuntimeType::I32 => ReflectValueBox::I32(value.parse().unwrap()),
            RuntimeType::I64 => ReflectValueBox::I64(value.parse().unwrap()),
            RuntimeType::U32 => ReflectValueBox::U32(value.parse().unwrap()),
            RuntimeType::U64 => ReflectValueBox::U64(value.parse().unwrap()),
            RuntimeType::F32 => ReflectValueBox::F32(parse_protobuf_float(value).unwrap() as f32),
            RuntimeType::F64 => ReflectValueBox::F64(parse_protobuf_float(value).unwrap()),
            // For strings, contains the default text contents (not escaped in any way)
            RuntimeType::String => ReflectValueBox::String(value.to_owned()),
            // For bytes, contains the C escaped value.  All bytes >= 128 are escaped
            RuntimeType::VecU8 => ReflectValueBox::Bytes(
                StrLit {
                    escaped: value.to_owned(),
                }
                .decode_bytes()
                .expect("decoded bytes default value"),
            ),
            t => unimplemented!("not implemented for {:?}", t),
        }
    }
}

impl fmt::Display for RuntimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeType::I32 => write!(f, "i32"),
            RuntimeType::I64 => write!(f, "i64"),
            RuntimeType::U32 => write!(f, "u32"),
            RuntimeType::U64 => write!(f, "u64"),
            RuntimeType::F32 => write!(f, "f32"),
            RuntimeType::F64 => write!(f, "f64"),
            RuntimeType::Bool => write!(f, "bool"),
            RuntimeType::String => write!(f, "String"),
            RuntimeType::VecU8 => write!(f, "Vec<u8>"),
            RuntimeType::Enum(e) => write!(f, "{}", e.full_name()),
            RuntimeType::Message(m) => write!(f, "{}", m.full_name()),
        }
    }
}
