use std::fmt;
use std::cmp;

use protobuf::descriptor::*;
use protobuf::descriptorx::*;
use super::well_known_types::is_well_known_type_full;
use ident::RustIdent;


// Represent subset of rust types used in generated code
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustType {
    // integer: signed?, size in bits
    Int(bool, u32),
    // param is size in bits
    Float(u32),
    Bool,
    Vec(Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    String,
    // [T], not &[T]
    Slice(Box<RustType>),
    // str, not &str
    Str,
    Option(Box<RustType>),
    SingularField(Box<RustType>),
    SingularPtrField(Box<RustType>),
    RepeatedField(Box<RustType>),
    // Box<T>
    Uniq(Box<RustType>),
    // &T
    Ref(Box<RustType>),
    // protobuf message
    Message(String),
    // protobuf enum, not any enum
    Enum(String, RustIdent),
    // oneof enum
    Oneof(String),
    // bytes::Bytes
    Bytes,
    // chars::Chars
    Chars,
    // group
    Group,
}

impl fmt::Display for RustType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RustType::Int(true, bits) => write!(f, "i{}", bits),
            RustType::Int(false, bits) => write!(f, "u{}", bits),
            RustType::Float(bits) => write!(f, "f{}", bits),
            RustType::Bool => write!(f, "bool"),
            RustType::Vec(ref param) => write!(f, "::std::vec::Vec<{}>", **param),
            RustType::HashMap(ref key, ref value) => {
                write!(f, "::std::collections::HashMap<{}, {}>", **key, **value)
            }
            RustType::String => write!(f, "::std::string::String"),
            RustType::Slice(ref param) => write!(f, "[{}]", **param),
            RustType::Str => write!(f, "str"),
            RustType::Option(ref param) => write!(f, "::std::option::Option<{}>", **param),
            RustType::SingularField(ref param) => {
                write!(f, "::protobuf::SingularField<{}>", **param)
            }
            RustType::SingularPtrField(ref param) => {
                write!(f, "::protobuf::SingularPtrField<{}>", **param)
            }
            RustType::RepeatedField(ref param) => {
                write!(f, "::protobuf::RepeatedField<{}>", **param)
            }
            RustType::Uniq(ref param) => write!(f, "::std::boxed::Box<{}>", **param),
            RustType::Ref(ref param) => write!(f, "&{}", **param),
            RustType::Message(ref name)
            | RustType::Enum(ref name, _)
            | RustType::Oneof(ref name) => write!(f, "{}", name),
            RustType::Group => write!(f, "<group>"),
            RustType::Bytes => write!(f, "::bytes::Bytes"),
            RustType::Chars => write!(f, "::protobuf::Chars"),
        }
    }
}

impl RustType {
    pub fn u8() -> RustType {
        RustType::Int(false, 8)
    }

    /// Type is rust primitive?
    pub fn is_primitive(&self) -> bool {
        match *self {
            RustType::Int(..) | RustType::Float(..) | RustType::Bool => true,
            _ => false,
        }
    }

    pub fn is_copy(&self) -> bool {
        if self.is_primitive() {
            true
        } else if let RustType::Enum(..) = *self {
            true
        } else {
            false
        }
    }

    fn is_str(&self) -> bool {
        match *self {
            RustType::Str => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        match *self {
            RustType::String => true,
            _ => false,
        }
    }

    fn is_slice(&self) -> bool {
        match *self {
            RustType::Slice(..) => true,
            _ => false,
        }
    }

    fn is_message(&self) -> bool {
        match *self {
            RustType::Message(..) => true,
            _ => false,
        }
    }

    fn is_enum(&self) -> bool {
        match *self {
            RustType::Enum(..) => true,
            _ => false,
        }
    }

    pub fn is_ref(&self) -> Option<&RustType> {
        match *self {
            RustType::Ref(ref v) => Some(&**v),
            _ => None,
        }
    }

    pub fn is_box(&self) -> Option<&RustType> {
        match *self {
            RustType::Uniq(ref v) => Some(&**v),
            _ => None,
        }
    }

    // default value for type
    pub fn default_value(&self) -> String {
        match *self {
            RustType::Ref(ref t) if t.is_str() => "\"\"".to_string(),
            RustType::Ref(ref t) if t.is_slice() => "&[]".to_string(),
            RustType::Int(..) => "0".to_string(),
            RustType::Float(..) => "0.".to_string(),
            RustType::Bool => "false".to_string(),
            RustType::Vec(..) => "::std::vec::Vec::new()".to_string(),
            RustType::HashMap(..) => "::std::collections::HashMap::new()".to_string(),
            RustType::String => "::std::string::String::new()".to_string(),
            RustType::Bytes => "::bytes::Bytes::new()".to_string(),
            RustType::Chars => "::protobuf::Chars::new()".to_string(),
            RustType::Option(..) => "::std::option::Option::None".to_string(),
            RustType::SingularField(..) => "::protobuf::SingularField::none()".to_string(),
            RustType::SingularPtrField(..) => "::protobuf::SingularPtrField::none()".to_string(),
            RustType::RepeatedField(..) => "::protobuf::RepeatedField::new()".to_string(),
            RustType::Message(ref name) => format!("{}::new()", name),
            RustType::Ref(ref m) if m.is_message() => {
                match **m {
                    RustType::Message(ref name) => {
                        format!("<{} as ::protobuf::Message>::default_instance()", name)
                    }
                    _ => unreachable!(),
                }
            }
            // Note: default value of enum type may not be equal to default value of field
            RustType::Enum(ref name, ref default) => format!("{}::{}", name, default),
            _ => panic!("cannot create default value for: {}", *self),
        }
    }

    pub fn default_value_typed(self) -> RustValueTyped {
        RustValueTyped {
            value: self.default_value(),
            rust_type: self,
        }
    }

    /// Emit a code to clear a variable `v`
    pub fn clear(&self, v: &str) -> String {
        match *self {
            RustType::Option(..) => format!("{} = ::std::option::Option::None", v),
            RustType::Vec(..)
            | RustType::Bytes
            | RustType::String
            | RustType::RepeatedField(..)
            | RustType::SingularField(..)
            | RustType::SingularPtrField(..)
            | RustType::HashMap(..) => format!("{}.clear()", v),
            RustType::Chars => format!("::protobuf::Clear::clear(&mut {})", v),
            RustType::Bool | RustType::Float(..) | RustType::Int(..) | RustType::Enum(..) => {
                format!("{} = {}", v, self.default_value())
            }
            ref ty => panic!("cannot clear type: {:?}", ty),
        }
    }

    // expression to convert `v` of type `self` to type `target`
    pub fn into_target(&self, target: &RustType, v: &str) -> String {
        self.try_into_target(target, v)
            .expect(&format!("failed to convert {:?} into {:?}", self, target))
    }

    fn try_into_target(&self, target: &RustType, v: &str) -> Result<String, ()> {
        {
            if let Some(t1) = self.is_ref().and_then(|t| t.is_box()) {
                if let Some(t2) = target.is_ref() {
                    if t1 == t2 {
                        return Ok(format!("&**{}", v));
                    }
                }
            }
        }

        match (self, target) {
            (x, y) if x == y => return Ok(format!("{}", v)),
            (&RustType::Ref(ref x), y) if **x == *y => return Ok(format!("*{}", v)),
            (x, &RustType::Uniq(ref y)) if *x == **y => {
                return Ok(format!("::std::boxed::Box::new({})", v))
            }
            (&RustType::Uniq(ref x), y) if **x == *y => return Ok(format!("*{}", v)),
            (&RustType::String, &RustType::Ref(ref t)) if **t == RustType::Str => {
                return Ok(format!("&{}", v))
            }
            (&RustType::Chars, &RustType::Ref(ref t)) if **t == RustType::Str => {
                return Ok(format!("&{}", v))
            }
            (&RustType::Ref(ref t1), &RustType::Ref(ref t2)) if t1.is_string() && t2.is_str() => {
                return Ok(format!("&{}", v))
            }
            (&RustType::Ref(ref t1), &RustType::String)
                if match **t1 {
                    RustType::Str => true,
                    _ => false,
                } =>
            {
                return Ok(format!("{}.to_owned()", v))
            }
            (&RustType::Ref(ref t1), &RustType::Vec(ref t2))
                if match (&**t1, &**t2) {
                    (&RustType::Slice(ref x), ref y) => **x == **y,
                    _ => false,
                } =>
            {
                return Ok(format!("{}.to_vec()", v))
            }
            (&RustType::Vec(ref x), &RustType::Ref(ref t))
                if match **t {
                    RustType::Slice(ref y) => x == y,
                    _ => false,
                } =>
            {
                return Ok(format!("&{}", v))
            }
            (&RustType::Bytes, &RustType::Ref(ref t))
                if match **t {
                    RustType::Slice(ref y) => **y == RustType::u8(),
                    _ => false,
                } =>
            {
                return Ok(format!("&{}", v))
            }
            (&RustType::Ref(ref t1), &RustType::Ref(ref t2))
                if match (&**t1, &**t2) {
<<<<<<< HEAD
                       (&RustType::Vec(ref x), &RustType::Slice(ref y)) => x == y,
                       _ => false,
                   } => return Ok(format!("&{}", v)),
            (&RustType::Enum(..), &RustType::Int(true, 32)) => {
                return Ok(format!("::protobuf::ProtobufEnum::value(&{})", v))
            },
=======
                    (&RustType::Vec(ref x), &RustType::Slice(ref y)) => x == y,
                    _ => false,
                } =>
            {
                return Ok(format!("&{}", v))
            }
            (&RustType::Enum(..), &RustType::Int(true, 32)) => return Ok(format!("{}.value()", v)),
>>>>>>> acm/master
            (&RustType::Ref(ref t), &RustType::Int(true, 32)) if t.is_enum() => {
                return Ok(format!("::protobuf::ProtobufEnum::value({})", v))
            }
            _ => (),
        };

        if let &RustType::Ref(ref s) = self {
            if let Ok(conv) = s.try_into_target(target, v) {
                return Ok(conv);
            }
        }

        Err(())
    }

    /// Type to view data of this type
    pub fn ref_type(&self) -> RustType {
        RustType::Ref(Box::new(match self {
            &RustType::String | &RustType::Chars => RustType::Str,
            &RustType::Vec(ref p) | &RustType::RepeatedField(ref p) => RustType::Slice(p.clone()),
            &RustType::Bytes => RustType::Slice(Box::new(RustType::u8())),
            &RustType::Message(ref p) => RustType::Message(p.clone()),
            &RustType::Uniq(ref p) => RustType::Uniq(p.clone()),
            x => panic!("no ref type for {}", x),
        }))
    }

    pub fn elem_type(&self) -> RustType {
        match self {
            &RustType::Option(ref ty) => (**ty).clone(),
            &RustType::SingularField(ref ty) => (**ty).clone(),
            &RustType::SingularPtrField(ref ty) => (**ty).clone(),
            x => panic!("cannot get elem type of {}", x),
        }
    }

    // type of `v` in `for v in xxx`
    pub fn iter_elem_type(&self) -> RustType {
        match self {
            &RustType::Vec(ref ty)
            | &RustType::Option(ref ty)
            | &RustType::RepeatedField(ref ty)
            | &RustType::SingularField(ref ty)
            | &RustType::SingularPtrField(ref ty) => RustType::Ref(ty.clone()),
            x => panic!("cannot iterate {}", x),
        }
    }

    pub fn value(self, value: String) -> RustValueTyped {
        RustValueTyped {
            value: value,
            rust_type: self,
        }
    }
}


/// Representation of an expression in code generator: text and type
pub struct RustValueTyped {
    pub value: String,
    pub rust_type: RustType,
}

impl RustValueTyped {
    pub fn into_type(&self, target: RustType) -> RustValueTyped {
        let target_value = self.rust_type.into_target(&target, &self.value);
        RustValueTyped {
            value: target_value,
            rust_type: target,
        }
    }

    pub fn boxed(self) -> RustValueTyped {
        self.into_type(RustType::Uniq(Box::new(self.rust_type.clone())))
    }
}


// protobuf type name for protobuf base type
pub fn protobuf_name(field_type: FieldDescriptorProto_Type) -> &'static str {
    match field_type {
        FieldDescriptorProto_Type::TYPE_DOUBLE => "double",
        FieldDescriptorProto_Type::TYPE_FLOAT => "float",
        FieldDescriptorProto_Type::TYPE_INT32 => "int32",
        FieldDescriptorProto_Type::TYPE_INT64 => "int64",
        FieldDescriptorProto_Type::TYPE_UINT32 => "uint32",
        FieldDescriptorProto_Type::TYPE_UINT64 => "uint64",
        FieldDescriptorProto_Type::TYPE_SINT32 => "sint32",
        FieldDescriptorProto_Type::TYPE_SINT64 => "sint64",
        FieldDescriptorProto_Type::TYPE_FIXED32 => "fixed32",
        FieldDescriptorProto_Type::TYPE_FIXED64 => "fixed64",
        FieldDescriptorProto_Type::TYPE_SFIXED32 => "sfixed32",
        FieldDescriptorProto_Type::TYPE_SFIXED64 => "sfixed64",
        FieldDescriptorProto_Type::TYPE_BOOL => "bool",
        FieldDescriptorProto_Type::TYPE_STRING => "string",
        FieldDescriptorProto_Type::TYPE_BYTES => "bytes",
        FieldDescriptorProto_Type::TYPE_ENUM => "enum",
        FieldDescriptorProto_Type::TYPE_MESSAGE => "message",
        FieldDescriptorProto_Type::TYPE_GROUP => "group",
    }
}


// rust type for protobuf base type
pub fn rust_name(field_type: FieldDescriptorProto_Type) -> RustType {
    match field_type {
        FieldDescriptorProto_Type::TYPE_DOUBLE => RustType::Float(64),
        FieldDescriptorProto_Type::TYPE_FLOAT => RustType::Float(32),
        FieldDescriptorProto_Type::TYPE_INT32 => RustType::Int(true, 32),
        FieldDescriptorProto_Type::TYPE_INT64 => RustType::Int(true, 64),
        FieldDescriptorProto_Type::TYPE_UINT32 => RustType::Int(false, 32),
        FieldDescriptorProto_Type::TYPE_UINT64 => RustType::Int(false, 64),
        FieldDescriptorProto_Type::TYPE_SINT32 => RustType::Int(true, 32),
        FieldDescriptorProto_Type::TYPE_SINT64 => RustType::Int(true, 64),
        FieldDescriptorProto_Type::TYPE_FIXED32 => RustType::Int(false, 32),
        FieldDescriptorProto_Type::TYPE_FIXED64 => RustType::Int(false, 64),
        FieldDescriptorProto_Type::TYPE_SFIXED32 => RustType::Int(true, 32),
        FieldDescriptorProto_Type::TYPE_SFIXED64 => RustType::Int(true, 64),
        FieldDescriptorProto_Type::TYPE_BOOL => RustType::Bool,
        FieldDescriptorProto_Type::TYPE_STRING => RustType::String,
        FieldDescriptorProto_Type::TYPE_BYTES => RustType::Vec(Box::new(RustType::Int(false, 8))),
        FieldDescriptorProto_Type::TYPE_ENUM
        | FieldDescriptorProto_Type::TYPE_GROUP
        | FieldDescriptorProto_Type::TYPE_MESSAGE => {
            panic!("there is no rust name for {:?}", field_type)
        }
    }
}

fn file_last_component(file: &str) -> &str {
    let bs = file.rfind('\\').map(|i| i + 1).unwrap_or(0);
    let fs = file.rfind('/').map(|i| i + 1).unwrap_or(0);
    &file[cmp::max(fs, bs)..]
}

#[cfg(test)]
#[test]
fn test_file_last_component() {
    assert_eq!("ab.proto", file_last_component("ab.proto"));
    assert_eq!("ab.proto", file_last_component("xx/ab.proto"));
    assert_eq!("ab.proto", file_last_component("xx\\ab.proto"));
    assert_eq!("ab.proto", file_last_component("yy\\xx\\ab.proto"));
}

fn is_descriptor_proto(file: &FileDescriptorProto) -> bool {
    file.get_package() == "google.protobuf"
        && file_last_component(file.get_name()) == "descriptor.proto"
}

pub fn type_name_to_rust_relative(
    type_name: &str,
    file: &FileDescriptorProto,
    subm: bool,
    root_scope: &RootScope,
) -> String {
    assert!(type_name.starts_with("."), "type name must start with dot: {}", type_name);
    let message_or_enum = root_scope.find_message_or_enum(type_name);
    if message_or_enum.get_scope().get_file_descriptor().get_name() == file.get_name() {
        // field type is a message or enum declared in the same file
        if subm {
            format!("super::{}", message_or_enum.rust_name())
        } else {
            format!("{}", message_or_enum.rust_name())
        }
    } else if let Some(name) = is_well_known_type_full(type_name) {
        // Well-known types are included in rust-protobuf library
        // https://developers.google.com/protocol-buffers/docs/reference/google.protobuf
        format!("::protobuf::well_known_types::{}", name)
    } else if is_descriptor_proto(message_or_enum.get_file_descriptor()) {
        // Messages defined in descriptor.proto
        format!(
            "::protobuf::descriptor::{}",
            message_or_enum.name_to_package()
        )
    } else {
        if subm {
            format!("super::super::{}", message_or_enum.rust_fq_name())
        } else {
            format!("super::{}", message_or_enum.rust_fq_name())
        }
    }
}


fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    s[..1].to_uppercase() + &s[1..]
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveTypeVariant {
    Default,
    Carllerche,
}

pub enum _CarllercheBytesType {
    Bytes,
    Chars,
}

// ProtobufType trait name
pub enum ProtobufTypeGen {
    Primitive(FieldDescriptorProto_Type, PrimitiveTypeVariant),
    Message(String),
    Enum(String),
}

impl ProtobufTypeGen {
    pub fn rust_type(&self) -> String {
        match self {
            &ProtobufTypeGen::Primitive(t, PrimitiveTypeVariant::Default) => format!(
                "::protobuf::types::ProtobufType{}",
                capitalize(protobuf_name(t))
            ),
            &ProtobufTypeGen::Primitive(
                FieldDescriptorProto_Type::TYPE_BYTES,
                PrimitiveTypeVariant::Carllerche,
            ) => format!("::protobuf::types::ProtobufTypeCarllercheBytes"),
            &ProtobufTypeGen::Primitive(
                FieldDescriptorProto_Type::TYPE_STRING,
                PrimitiveTypeVariant::Carllerche,
            ) => format!("::protobuf::types::ProtobufTypeCarllercheChars"),
            &ProtobufTypeGen::Primitive(.., PrimitiveTypeVariant::Carllerche) => unreachable!(),
            &ProtobufTypeGen::Message(ref name) => {
                format!("::protobuf::types::ProtobufTypeMessage<{}>", name)
            }
            &ProtobufTypeGen::Enum(ref name) => {
                format!("::protobuf::types::ProtobufTypeEnum<{}>", name)
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn into_target_ref_box_to_ref() {
        let t1 = RustType::Ref(
            Box::new(RustType::Uniq(
                Box::new(RustType::Message("Ab".to_owned())))));
        let t2 = RustType::Ref(
            Box::new(RustType::Message("Ab".to_owned())));

        assert_eq!("&**v", t1.into_target(&t2, "v"));
    }
}
