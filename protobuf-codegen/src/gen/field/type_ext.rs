use protobuf::descriptor::field_descriptor_proto::Type;

use crate::gen::rust_types_values::PrimitiveTypeVariant;
use crate::gen::rust_types_values::RustType;

pub(crate) trait TypeExt {
    fn read(&self, is: &str, primitive_type_variant: PrimitiveTypeVariant) -> String;
    fn _is_s_varint(&self) -> bool;
    fn is_copy(&self) -> bool;
    fn protobuf_name(&self) -> &'static str;
    fn rust_type(&self) -> RustType;
    fn os_write_fn_param_type(&self) -> RustType;
    fn encoded_size(&self) -> Option<u32>;
}

impl TypeExt for Type {
    fn read(&self, is: &str, primitive_type_variant: PrimitiveTypeVariant) -> String {
        match (self, primitive_type_variant) {
            (Type::TYPE_ENUM, _) => format!("{}.read_enum_or_unknown()", is),
            (Type::TYPE_BYTES, PrimitiveTypeVariant::TokioBytes) => {
                format!("{}.read_tokio_bytes()", is)
            }
            (Type::TYPE_STRING, PrimitiveTypeVariant::TokioBytes) => {
                format!("{}.read_tokio_chars()", is)
            }
            _ => format!("{}.read_{}()", is, self.protobuf_name()),
        }
    }

    /// True if self is signed integer with zigzag encoding
    fn _is_s_varint(&self) -> bool {
        match *self {
            Type::TYPE_SINT32 | Type::TYPE_SINT64 => true,
            _ => false,
        }
    }

    fn is_copy(&self) -> bool {
        match self {
            Type::TYPE_MESSAGE | Type::TYPE_STRING | Type::TYPE_BYTES => false,
            _ => true,
        }
    }

    fn protobuf_name(&self) -> &'static str {
        match self {
            Type::TYPE_DOUBLE => "double",
            Type::TYPE_FLOAT => "float",
            Type::TYPE_INT32 => "int32",
            Type::TYPE_INT64 => "int64",
            Type::TYPE_UINT32 => "uint32",
            Type::TYPE_UINT64 => "uint64",
            Type::TYPE_SINT32 => "sint32",
            Type::TYPE_SINT64 => "sint64",
            Type::TYPE_FIXED32 => "fixed32",
            Type::TYPE_FIXED64 => "fixed64",
            Type::TYPE_SFIXED32 => "sfixed32",
            Type::TYPE_SFIXED64 => "sfixed64",
            Type::TYPE_BOOL => "bool",
            Type::TYPE_STRING => "string",
            Type::TYPE_BYTES => "bytes",
            Type::TYPE_ENUM => "enum",
            Type::TYPE_MESSAGE => "message",
            Type::TYPE_GROUP => "group",
        }
    }

    /// Rust type for protobuf base type.
    fn rust_type(&self) -> RustType {
        match self {
            Type::TYPE_DOUBLE => RustType::Float(64),
            Type::TYPE_FLOAT => RustType::Float(32),
            Type::TYPE_INT32 => RustType::Int(true, 32),
            Type::TYPE_INT64 => RustType::Int(true, 64),
            Type::TYPE_UINT32 => RustType::Int(false, 32),
            Type::TYPE_UINT64 => RustType::Int(false, 64),
            Type::TYPE_SINT32 => RustType::Int(true, 32),
            Type::TYPE_SINT64 => RustType::Int(true, 64),
            Type::TYPE_FIXED32 => RustType::Int(false, 32),
            Type::TYPE_FIXED64 => RustType::Int(false, 64),
            Type::TYPE_SFIXED32 => RustType::Int(true, 32),
            Type::TYPE_SFIXED64 => RustType::Int(true, 64),
            Type::TYPE_BOOL => RustType::Bool,
            Type::TYPE_STRING => RustType::String,
            Type::TYPE_BYTES => RustType::Vec(Box::new(RustType::u8())),
            Type::TYPE_ENUM | Type::TYPE_GROUP | Type::TYPE_MESSAGE => {
                panic!("there is no rust name for {:?}", self)
            }
        }
    }

    // type of `v` in `os.write_xxx_no_tag(v)`
    fn os_write_fn_param_type(&self) -> RustType {
        match self {
            Type::TYPE_STRING => RustType::amp_str(),
            Type::TYPE_BYTES => RustType::amp_slice_of_u8(),
            Type::TYPE_ENUM => RustType::i32(),
            t => t.rust_type(),
        }
    }

    /// Size of value for type, None if variable.
    fn encoded_size(&self) -> Option<u32> {
        match self {
            Type::TYPE_BOOL => Some(1),
            Type::TYPE_FIXED32 => Some(4),
            Type::TYPE_FIXED64 => Some(8),
            Type::TYPE_SFIXED32 => Some(4),
            Type::TYPE_SFIXED64 => Some(8),
            Type::TYPE_FLOAT => Some(4),
            Type::TYPE_DOUBLE => Some(8),
            _ => None,
        }
    }
}
