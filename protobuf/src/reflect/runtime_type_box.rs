use std::fmt;

use crate::descriptor::field_descriptor_proto;
use crate::reflect::EnumDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::MessageRef;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::text_format;
use crate::text_format::lexer::float::parse_protobuf_float;

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

    pub(crate) fn default_value_box(&self) -> ReflectValueBox {
        self.default_value_ref().to_box()
    }

    /// Rust type from protobuf type.
    ///
    /// # Panics
    ///
    /// Panics for message or enum types (because they can't be resolved without context).
    pub(crate) fn from_proto_type(t: field_descriptor_proto::Type) -> RuntimeTypeBox {
        match t {
            field_descriptor_proto::Type::TYPE_UINT32 => RuntimeTypeBox::U32,
            field_descriptor_proto::Type::TYPE_UINT64 => RuntimeTypeBox::U64,
            field_descriptor_proto::Type::TYPE_INT32 => RuntimeTypeBox::I32,
            field_descriptor_proto::Type::TYPE_INT64 => RuntimeTypeBox::I64,
            field_descriptor_proto::Type::TYPE_SINT32 => RuntimeTypeBox::I32,
            field_descriptor_proto::Type::TYPE_SINT64 => RuntimeTypeBox::I64,
            field_descriptor_proto::Type::TYPE_FIXED32 => RuntimeTypeBox::U32,
            field_descriptor_proto::Type::TYPE_FIXED64 => RuntimeTypeBox::U64,
            field_descriptor_proto::Type::TYPE_SFIXED64 => RuntimeTypeBox::I64,
            field_descriptor_proto::Type::TYPE_SFIXED32 => RuntimeTypeBox::I32,
            field_descriptor_proto::Type::TYPE_BOOL => RuntimeTypeBox::Bool,
            field_descriptor_proto::Type::TYPE_STRING => RuntimeTypeBox::String,
            field_descriptor_proto::Type::TYPE_BYTES => RuntimeTypeBox::VecU8,
            field_descriptor_proto::Type::TYPE_FLOAT => RuntimeTypeBox::F32,
            field_descriptor_proto::Type::TYPE_DOUBLE => RuntimeTypeBox::F64,
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
            RuntimeTypeBox::Bool => ReflectValueBox::Bool(if value == "true" {
                true
            } else if value == "false" {
                false
            } else {
                panic!("cannot parse bool default value: {}", value)
            }),
            RuntimeTypeBox::I32 => ReflectValueBox::I32(value.parse().unwrap()),
            RuntimeTypeBox::I64 => ReflectValueBox::I64(value.parse().unwrap()),
            RuntimeTypeBox::U32 => ReflectValueBox::U32(value.parse().unwrap()),
            RuntimeTypeBox::U64 => ReflectValueBox::U64(value.parse().unwrap()),
            RuntimeTypeBox::F32 => {
                ReflectValueBox::F32(parse_protobuf_float(value).unwrap() as f32)
            }
            RuntimeTypeBox::F64 => ReflectValueBox::F64(parse_protobuf_float(value).unwrap()),
            // For strings, contains the default text contents (not escaped in any way)
            RuntimeTypeBox::String => ReflectValueBox::String(value.to_owned()),
            // For bytes, contains the C escaped value.  All bytes >= 128 are escaped
            RuntimeTypeBox::VecU8 => ReflectValueBox::Bytes(
                text_format::lexer::StrLit {
                    escaped: value.to_owned(),
                }
                .decode_bytes()
                .expect("decoded bytes default value"),
            ),
            t => unimplemented!("not implemented for {:?}", t),
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
