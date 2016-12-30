use std::fmt;

use wire_format;
use descriptor::*;
use descriptorx::*;
use rt;
use code_writer::*;

use super::enums::*;


fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

// Represent subset of rust types used in generated code
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum RustType {
    Int(bool, u32),
    Float(u32),
    Bool,
    Vec(Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    String,
    Slice(Box<RustType>),
    Str,
    Option(Box<RustType>),
    SingularField(Box<RustType>),
    SingularPtrField(Box<RustType>),
    RepeatedField(Box<RustType>),
    _Uniq(Box<RustType>),
    Ref(Box<RustType>),
    Message(String),
    // protobuf enum, not any enum
    Enum(String, String),
    // oneof enum
    Oneof(String),
    // group
    Group,
}

impl fmt::Display for RustType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RustType::Int(true, bits)    => write!(f, "i{}", bits),
            RustType::Int(false, bits)   => write!(f, "u{}", bits),
            RustType::Float(bits)        => write!(f, "f{}", bits),
            RustType::Bool               => write!(f, "bool"),
            RustType::Vec(ref param)     => write!(f, "::std::vec::Vec<{}>", **param),
            RustType::HashMap(ref key, ref value) =>
                write!(f, "::std::collections::HashMap<{}, {}>", **key, **value),
            RustType::String             => write!(f, "::std::string::String"),
            RustType::Slice(ref param)   => write!(f, "[{}]", **param),
            RustType::Str                => write!(f, "str"),
            RustType::Option(ref param)           => write!(f, "::std::option::Option<{}>", **param),
            RustType::SingularField(ref param)    => write!(f, "::protobuf::SingularField<{}>", **param),
            RustType::SingularPtrField(ref param) => write!(f, "::protobuf::SingularPtrField<{}>", **param),
            RustType::RepeatedField(ref param)    => write!(f, "::protobuf::RepeatedField<{}>", **param),
            RustType::_Uniq(ref param)             => write!(f, "::std::Box<{}>", **param),
            RustType::Ref(ref param)              => write!(f, "&{}", **param),
            RustType::Message(ref name) |
            RustType::Enum(ref name, _)    |
            RustType::Oneof(ref name)   => write!(f, "{}", name),
            RustType::Group             => write!(f, "<group>"),
        }
    }
}

impl RustType {
    fn is_primitive(&self) -> bool {
        match *self {
            RustType::Int(..)      |
            RustType::Float(..)    |
            RustType::Bool         => true,
            _                      => false,
        }
    }

    fn is_str(&self) -> bool {
        match *self {
            RustType::Str => true,
            _ => false
        }
    }

    fn is_string(&self) -> bool {
        match *self {
            RustType::String => true,
            _ => false
        }
    }

    fn is_slice(&self) -> bool {
        match *self {
            RustType::Slice(..) => true,
            _ => false
        }
    }

    fn is_message(&self) -> bool {
        match *self {
            RustType::Message(..) => true,
            _ => false
        }
    }

    fn is_enum(&self) -> bool {
        match *self {
            RustType::Enum(..) => true,
            _ => false
        }
    }

    fn is_u8(&self) -> bool {
        match *self {
            RustType::Int(false, 8) => true,
            _ => false
        }
    }

    fn is_ref(&self) -> bool {
        match *self {
            RustType::Ref(..) => true,
            _           => false,
        }
    }

    // default value for type
    fn default_value(&self) -> String {
        match *self {
            RustType::Ref(ref t) if t.is_str()       => "\"\"".to_string(),
            RustType::Ref(ref t) if t.is_slice()     => "&[]".to_string(),
            RustType::Int(..)                        => "0".to_string(),
            RustType::Float(..)                      => "0.".to_string(),
            RustType::Bool                           => "false".to_string(),
            RustType::Vec(..)                        => "::std::vec::Vec::new()".to_string(),
            RustType::HashMap(..)                    => "::std::collections::HashMap::new()".to_string(),
            RustType::String                         => "::std::string::String::new()".to_string(),
            RustType::Option(..)                     => "::std::option::Option::None".to_string(),
            RustType::SingularField(..)              => "::protobuf::SingularField::none()".to_string(),
            RustType::SingularPtrField(..)           => "::protobuf::SingularPtrField::none()".to_string(),
            RustType::RepeatedField(..)              => "::protobuf::RepeatedField::new()".to_string(),
            RustType::Message(ref name)              => format!("{}::new()", name),
            RustType::Ref(ref m) if m.is_message()   => match **m {
                RustType::Message(ref name) => format!("{}::default_instance()", name),
                _ => unreachable!()
            },
            // Note: default value of enum type may not be equal to default value of field
            RustType::Enum(ref name, ref default)    => format!("{}::{}", name, default),
            _ => panic!("cannot create default value for: {}", *self),
        }
    }

    /// Emit a code to clear a variable `v`
    fn clear(&self, v: &str) -> String {
        match *self {
            RustType::Option(..) => format!("{} = ::std::option::Option::None", v),
            RustType::Vec(..) |
            RustType::String |
            RustType::RepeatedField(..)    |
            RustType::SingularField(..)    |
            RustType::SingularPtrField(..) |
            RustType::HashMap(..)          => format!("{}.clear()", v),
            RustType::Bool      |
            RustType::Float(..) |
            RustType::Int(..)   |
            RustType::Enum(..)  => format!("{} = {}", v, self.default_value()),
            ref ty => panic!("cannot clear type: {:?}", ty),
        }
    }

    // wrap value in storage type
    fn wrap_value(&self, value: &str) -> String {
        match *self {
            RustType::Option(..)           => format!("::std::option::Option::Some({})", value),
            RustType::SingularField(..)    => format!("::protobuf::SingularField::some({})", value),
            RustType::SingularPtrField(..) => format!("::protobuf::SingularPtrField::some({})", value),
            _ => panic!("not a wrapper type: {}", *self),
        }
    }

    // expression to convert `v` of type `self` to type `target`
    fn into_target(&self, target: &RustType, v: &str) -> String {
        match (self, target) {
            (x, y) if x == y                        => format!("{}", v),
            (&RustType::Ref(ref x), y) if **x == *y => format!("*{}", v),
            (&RustType::String, &RustType::Ref(ref t)) if t.is_str() =>
                    format!("&{}", v),
            (&RustType::Ref(ref t1), &RustType::Ref(ref t2)) if t1.is_string() && t2.is_str() =>
                    format!("&{}", v),
            (&RustType::Vec(ref x), &RustType::Ref(ref t))
                if match **t { RustType::Slice(ref y) => x == y, _ => false } =>
                    format!("&{}", v),
            (&RustType::Ref(ref t1), &RustType::Ref(ref t2))
                if match (&**t1, &**t2) {
                    (&RustType::Vec(ref x), &RustType::Slice(ref y)) => x == y,
                    _ => false
                } => format!("&{}", v),
            (&RustType::Enum(..), &RustType::Int(true, 32)) =>
                    format!("{}.value()", v),
            (&RustType::Ref(ref t), &RustType::Int(true, 32)) if t.is_enum() =>
                    format!("{}.value()", v),
            _ => panic!("cannot convert {} to {}", self, target),
        }
    }

    /// Type to view data of this type
    fn ref_type(&self) -> RustType {
        RustType::Ref(Box::new(match self {
            &RustType::String               => RustType::Str,
            &RustType::Vec(ref p)           |
            &RustType::RepeatedField(ref p) => RustType::Slice(p.clone()),
            &RustType::Message(ref p)       => RustType::Message(p.clone()),
            x => panic!("no ref type for {}", x),
        }))
    }

    fn elem_type(&self) -> RustType {
        match self {
            &RustType::Option(ref ty) => (**ty).clone(),
            x => panic!("cannot get elem type of {}", x),
        }
    }

    // type of `v` in `for v in xxx`
    fn iter_elem_type(&self) -> RustType {
        match self {
            &RustType::Vec(ref ty)              |
            &RustType::Option(ref ty)           |
            &RustType::RepeatedField(ref ty)    |
            &RustType::SingularField(ref ty)    |
            &RustType::SingularPtrField(ref ty) => RustType::Ref(ty.clone()),
            x => panic!("cannot iterate {}", x),
        }
    }
}


// rust type for protobuf base type
fn rust_name(field_type: FieldDescriptorProto_Type) -> RustType {
    match field_type {
        FieldDescriptorProto_Type::TYPE_DOUBLE   => RustType::Float(64),
        FieldDescriptorProto_Type::TYPE_FLOAT    => RustType::Float(32),
        FieldDescriptorProto_Type::TYPE_INT32    => RustType::Int(true, 32),
        FieldDescriptorProto_Type::TYPE_INT64    => RustType::Int(true, 64),
        FieldDescriptorProto_Type::TYPE_UINT32   => RustType::Int(false, 32),
        FieldDescriptorProto_Type::TYPE_UINT64   => RustType::Int(false, 64),
        FieldDescriptorProto_Type::TYPE_SINT32   => RustType::Int(true, 32),
        FieldDescriptorProto_Type::TYPE_SINT64   => RustType::Int(true, 64),
        FieldDescriptorProto_Type::TYPE_FIXED32  => RustType::Int(false, 32),
        FieldDescriptorProto_Type::TYPE_FIXED64  => RustType::Int(false, 64),
        FieldDescriptorProto_Type::TYPE_SFIXED32 => RustType::Int(true, 32),
        FieldDescriptorProto_Type::TYPE_SFIXED64 => RustType::Int(true, 64),
        FieldDescriptorProto_Type::TYPE_BOOL     => RustType::Bool,
        FieldDescriptorProto_Type::TYPE_STRING   => RustType::String,
        FieldDescriptorProto_Type::TYPE_BYTES    => RustType::Vec(Box::new(RustType::Int(false, 8))),
        FieldDescriptorProto_Type::TYPE_ENUM     |
        FieldDescriptorProto_Type::TYPE_GROUP    |
        FieldDescriptorProto_Type::TYPE_MESSAGE  => panic!("there is no rust name for {:?}", field_type),
    }
}

fn type_is_copy(field_type: FieldDescriptorProto_Type) -> bool {
    match field_type {
        FieldDescriptorProto_Type::TYPE_MESSAGE |
        FieldDescriptorProto_Type::TYPE_STRING |
        FieldDescriptorProto_Type::TYPE_BYTES => false,
        _ => true,
    }
}

impl FieldDescriptorProto_Type {
    fn read(&self, is: &str) -> String {
        format!("{}.read_{}()", is, protobuf_name(*self))
    }

    /// True if self is signed integer with zigzag encoding
    fn is_s_varint(&self) -> bool {
        match *self {
            FieldDescriptorProto_Type::TYPE_SINT32 |
            FieldDescriptorProto_Type::TYPE_SINT64 => true,
            _ => false,
        }
    }
}

// protobuf type name for protobuf base type
fn protobuf_name(field_type: FieldDescriptorProto_Type) -> &'static str {
    match field_type {
        FieldDescriptorProto_Type::TYPE_DOUBLE   => "double",
        FieldDescriptorProto_Type::TYPE_FLOAT    => "float",
        FieldDescriptorProto_Type::TYPE_INT32    => "int32",
        FieldDescriptorProto_Type::TYPE_INT64    => "int64",
        FieldDescriptorProto_Type::TYPE_UINT32   => "uint32",
        FieldDescriptorProto_Type::TYPE_UINT64   => "uint64",
        FieldDescriptorProto_Type::TYPE_SINT32   => "sint32",
        FieldDescriptorProto_Type::TYPE_SINT64   => "sint64",
        FieldDescriptorProto_Type::TYPE_FIXED32  => "fixed32",
        FieldDescriptorProto_Type::TYPE_FIXED64  => "fixed64",
        FieldDescriptorProto_Type::TYPE_SFIXED32 => "sfixed32",
        FieldDescriptorProto_Type::TYPE_SFIXED64 => "sfixed64",
        FieldDescriptorProto_Type::TYPE_BOOL     => "bool",
        FieldDescriptorProto_Type::TYPE_STRING   => "string",
        FieldDescriptorProto_Type::TYPE_BYTES    => "bytes",
        FieldDescriptorProto_Type::TYPE_ENUM     => "enum",
        FieldDescriptorProto_Type::TYPE_MESSAGE  => "message",
        FieldDescriptorProto_Type::TYPE_GROUP    => "group",
    }
}

fn field_type_wire_type(field_type: FieldDescriptorProto_Type) -> wire_format::WireType {
    use stream::wire_format::*;
    match field_type {
        FieldDescriptorProto_Type::TYPE_INT32    => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_INT64    => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_UINT32   => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_UINT64   => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_SINT32   => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_SINT64   => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_BOOL     => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_ENUM     => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_FIXED32  => WireTypeFixed32,
        FieldDescriptorProto_Type::TYPE_FIXED64  => WireTypeFixed64,
        FieldDescriptorProto_Type::TYPE_SFIXED32 => WireTypeFixed32,
        FieldDescriptorProto_Type::TYPE_SFIXED64 => WireTypeFixed64,
        FieldDescriptorProto_Type::TYPE_FLOAT    => WireTypeFixed32,
        FieldDescriptorProto_Type::TYPE_DOUBLE   => WireTypeFixed64,
        FieldDescriptorProto_Type::TYPE_STRING   => WireTypeLengthDelimited,
        FieldDescriptorProto_Type::TYPE_BYTES    => WireTypeLengthDelimited,
        FieldDescriptorProto_Type::TYPE_MESSAGE  => WireTypeLengthDelimited,
        FieldDescriptorProto_Type::TYPE_GROUP    => WireTypeLengthDelimited, // not true
    }
}

fn type_protobuf_name(field_type: FieldDescriptorProto_Type) -> &'static str {
    match field_type {
        FieldDescriptorProto_Type::TYPE_INT32    => "int32",
        FieldDescriptorProto_Type::TYPE_INT64    => "int64",
        FieldDescriptorProto_Type::TYPE_UINT32   => "uint32",
        FieldDescriptorProto_Type::TYPE_UINT64   => "uint64",
        FieldDescriptorProto_Type::TYPE_SINT32   => "sint32",
        FieldDescriptorProto_Type::TYPE_SINT64   => "sint64",
        FieldDescriptorProto_Type::TYPE_BOOL     => "bool",
        FieldDescriptorProto_Type::TYPE_FIXED32  => "fixed32",
        FieldDescriptorProto_Type::TYPE_FIXED64  => "fixed64",
        FieldDescriptorProto_Type::TYPE_SFIXED32 => "sfixed32",
        FieldDescriptorProto_Type::TYPE_SFIXED64 => "sfixed64",
        FieldDescriptorProto_Type::TYPE_FLOAT    => "float",
        FieldDescriptorProto_Type::TYPE_DOUBLE   => "double",
        FieldDescriptorProto_Type::TYPE_STRING   => "string",
        FieldDescriptorProto_Type::TYPE_BYTES    => "bytes",
        FieldDescriptorProto_Type::TYPE_ENUM     |
        FieldDescriptorProto_Type::TYPE_MESSAGE  |
        FieldDescriptorProto_Type::TYPE_GROUP    => panic!()
    }
}

fn field_type_protobuf_name<'a>(field: &'a FieldDescriptorProto) -> &'a str {
    if field.has_type_name() {
        field.get_type_name()
    } else {
        type_protobuf_name(field.get_field_type())
    }
}

// size of value for type, None if variable
fn field_type_size(field_type: FieldDescriptorProto_Type) -> Option<u32> {
    match field_type {
        FieldDescriptorProto_Type::TYPE_BOOL => Some(1),
        t if field_type_wire_type(t) == wire_format::WireTypeFixed32 => Some(4),
        t if field_type_wire_type(t) == wire_format::WireTypeFixed64 => Some(8),
        _ => None
    }
}


#[derive(Clone,Debug)]
struct EntryKeyValue(GenProtobufType, GenProtobufType);

fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    s[..1].to_uppercase() + &s[1..]
}

#[derive(Clone,Debug)]
enum GenProtobufType {
    Primitive(FieldDescriptorProto_Type),
    // name, entry
    Message(String),
    // name, default value
    Enum(String, String),
    Group,
}

impl GenProtobufType {
    fn proto_type(&self) -> FieldDescriptorProto_Type {
        match *self {
            GenProtobufType::Primitive(t) => t,
            GenProtobufType::Group => FieldDescriptorProto_Type::TYPE_GROUP,
            GenProtobufType::Message(..) => FieldDescriptorProto_Type::TYPE_MESSAGE,
            GenProtobufType::Enum(..) => FieldDescriptorProto_Type::TYPE_ENUM,
        }
    }

    fn is_copy(&self) -> bool {
        type_is_copy(self.proto_type())
    }

    fn rust_type(&self) -> RustType {
        match *self {
            GenProtobufType::Primitive(t) => rust_name(t),
            GenProtobufType::Group => RustType::Group,
            GenProtobufType::Message(ref name) => RustType::Message(name.clone()),
            GenProtobufType::Enum(ref name, ref default_value) => {
                RustType::Enum(name.clone(), default_value.clone())
            }
        }
    }

    /// implementation of ProtobufType trait
    fn lib_protobuf_type(&self) -> String {
        match *self {
            GenProtobufType::Primitive(t) => {
                format!("::protobuf::types::ProtobufType{}", capitalize(protobuf_name(t)))
            },
            GenProtobufType::Message(ref name) => {
                format!("::protobuf::types::ProtobufTypeMessage<{}>", name)
            },
            GenProtobufType::Enum(ref name, _) => {
                format!("::protobuf::types::ProtobufTypeEnum<{}>", name)
            },
            GenProtobufType::Group => unreachable!(),
        }
    }
}

#[derive(Clone,PartialEq,Eq)]
enum SingularFieldFlag {
    // proto2 or proto3 message
    WithFlag { required: bool },
    // proto3
    WithoutFlag,
}

impl SingularFieldFlag {
    fn is_required(&self) -> bool {
        match *self {
            SingularFieldFlag::WithFlag { required } => required,
            SingularFieldFlag::WithoutFlag => false,
        }
    }
}

#[derive(Clone)]
struct SingularField {
    flag: SingularFieldFlag,
    elem: GenProtobufType,
}

#[derive(Clone)]
struct OneofField {
    elem: GenProtobufType,
    oneof_name: String,
    oneof_type_name: RustType,
}

impl OneofField {
    fn parse(oneof: &OneofWithContext, elem: GenProtobufType) -> OneofField {
        OneofField {
            elem: elem,
            oneof_name: oneof.name().to_string(),
            oneof_type_name: RustType::Oneof(oneof.rust_name()),
        }
    }
}

#[derive(Clone)]
struct RepeatedField {
    elem: GenProtobufType,
    packed: bool,
}

#[derive(Clone)]
struct MapField {
    name: String,
    key: GenProtobufType,
    value: GenProtobufType,
}

#[derive(Clone)]
enum FieldKind {
    // optional or required
    Singular(SingularField),
    // repeated except map
    Repeated(RepeatedField),
    // map
    Map(MapField),
    // part of oneof
    Oneof(OneofField),
}

#[derive(Clone)]
struct FieldGen<'a> {
    root_scope: &'a RootScope<'a>,
    syntax: Syntax,
    proto_field: FieldDescriptorProto,
    // field name in generated code
    rust_name: String,
    proto_type: FieldDescriptorProto_Type,
    wire_type: wire_format::WireType,
    enum_default_value: Option<EnumValueGen>,
    number: u32,
    kind: FieldKind,
}

enum FieldElem {
    Primitive(FieldDescriptorProto_Type),
    // name, entry
    Message(String, Option<Box<EntryKeyValue>>),
    // name, default value
    Enum(String, String),
    Group,
}

impl FieldElem {
    fn into_type(self) -> GenProtobufType {
        match self {
            FieldElem::Primitive(t) => GenProtobufType::Primitive(t),
            FieldElem::Message(name, None) => GenProtobufType::Message(name),
            // TODO: replace with unreachable
            FieldElem::Message(name, Some(..)) => GenProtobufType::Message(name),
            FieldElem::Enum(name, default_value) => GenProtobufType::Enum(name, default_value),
            FieldElem::Group => GenProtobufType::Group,
        }
    }
}

fn field_elem(field: &FieldWithContext, root_scope: &RootScope, parse_map: bool)
    -> (FieldElem, Option<EnumValueGen>)
{
    if field.field.get_field_type() == FieldDescriptorProto_Type::TYPE_GROUP {
        (FieldElem::Group, None)
    } else if field.field.has_type_name() {
        let message_or_enum = root_scope.find_message_or_enum(field.field.get_type_name());
        let rust_name =
            if message_or_enum.get_scope().get_file_descriptor().get_name() ==
                field.message.get_scope().get_file_descriptor().get_name()
            {
                // field type is a message or enum declared in the same file
                message_or_enum.rust_name()
            } else {
                format!("super::{}", message_or_enum.rust_fq_name())
            };
        match (field.field.get_field_type(), message_or_enum) {
            (FieldDescriptorProto_Type::TYPE_MESSAGE, MessageOrEnumWithScope::Message(message_with_scope)) => {
                let entry_key_value =
                    if let (true, Some((key, value))) = (parse_map, message_with_scope.map_entry()) {
                        Some(Box::new(EntryKeyValue(
                            field_elem(&key, root_scope, false).0.into_type(),
                            field_elem(&value, root_scope, false).0.into_type(),
                        )))
                    } else {
                        None
                    };
                (FieldElem::Message(rust_name, entry_key_value), None)
            }
            (FieldDescriptorProto_Type::TYPE_ENUM, MessageOrEnumWithScope::Enum(enum_with_scope)) => {
                let e = EnumGen::new(&enum_with_scope, field.message.get_scope().get_file_descriptor());
                let ev = if field.field.has_default_value() {
                    e.value_by_name(field.field.get_default_value()).clone()
                } else {
                    e.values_unique().into_iter().next().unwrap()
                };
                (
                    FieldElem::Enum(rust_name, enum_with_scope.values()[0].get_name().to_owned()),
                    Some(ev),
                )
            }
            _ => panic!("unknown named type: {:?}", field.field.get_field_type()),
        }
    } else if field.field.has_field_type() {
        (FieldElem::Primitive(field.field.get_field_type()), None)
    } else {
        panic!("neither type_name, nor field_type specified for field: {}", field.field.get_name());
    }
}

struct AccessorFn {
    name: String,
    for_reflect_suffix: bool,
    type_params: Vec<String>,
    accessors: Vec<String>,
}

impl AccessorFn {
    fn sig(&self) -> String {
        let mut s = self.name.clone();
        s.push_str("::<_");
        for p in &self.type_params {
            s.push_str(", ");
            s.push_str(&p);
        }
        s.push_str(">");
        s
    }
}

impl<'a> FieldGen<'a> {
    fn parse(field: &FieldWithContext, root_scope: &'a RootScope<'a>) -> FieldGen<'a> {
        let (elem, enum_default_value) = field_elem(field, root_scope, true);

        let kind =
            if field.field.get_label() == FieldDescriptorProto_Label::LABEL_REPEATED {
                match (elem, true) {
                    (FieldElem::Message(name, Some(key_value)), true) =>
                        FieldKind::Map(MapField {
                            name: name,
                            key: key_value.0.clone(),
                            value: key_value.1.clone(),
                        }),
                    (elem, _) =>
                        FieldKind::Repeated(RepeatedField {
                            elem: elem.into_type(),
                            packed: field.field.get_options().get_packed()
                        })
                }
            } else if let Some(oneof) = field.oneof() {
                FieldKind::Oneof(OneofField::parse(&oneof, elem.into_type()))
            } else {
                let flag =
                    if field.message.scope.file_scope.syntax() == Syntax::PROTO3 &&
                        field.field.get_field_type() != FieldDescriptorProto_Type::TYPE_MESSAGE
                    {
                        SingularFieldFlag::WithoutFlag
                    } else {
                        SingularFieldFlag::WithFlag {
                            required: field.field.get_label() == FieldDescriptorProto_Label::LABEL_REQUIRED
                        }
                    };
                FieldKind::Singular(SingularField {
                    elem: elem.into_type(),
                    flag: flag,
                })
            };

        FieldGen {
            root_scope: root_scope,
            syntax: field.message.get_scope().file_scope.syntax(),
            proto_field: field.field.clone(),
            rust_name: field.rust_name(),
            proto_type: field.field.get_field_type(),
            wire_type: field_type_wire_type(field.field.get_field_type()),
            enum_default_value: enum_default_value,
            number: field.field.get_number() as u32,
            kind: kind,
        }
    }

    fn number(&self) -> u32 {
        self.number
    }

    fn tag_size(&self) -> u32 {
        rt::tag_size(self.number)
    }

    fn is_oneof(&self) -> bool {
        match self.kind {
            FieldKind::Oneof(..) => true,
            _ => false,
        }
    }

    fn oneof(&self) -> &OneofField {
        match self.kind {
            FieldKind::Oneof(ref oneof) => &oneof,
            _ => panic!("not a oneof field: {}", self.reconstruct_def()),
        }
    }

    fn is_singular(&self) -> bool {
        match self.kind {
            FieldKind::Singular(..) => true,
            _ => false,
        }
    }

    fn is_repeated_not_map(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(..) => true,
            _ => false,
        }
    }

    fn is_repeated_or_map(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(..) | FieldKind::Map(..) => true,
            _ => false,
        }
    }

    fn is_repeated_packed(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(RepeatedField { packed: true, .. }) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn repeated(&self) -> &RepeatedField {
        match self.kind {
            FieldKind::Repeated(ref repeated) => &repeated,
            _ => panic!("not a repeated field: {}", self.reconstruct_def()),
        }
    }

    fn singular(&self) -> &SingularField {
        match self.kind {
            FieldKind::Singular(ref singular) => &singular,
            _ => panic!("not a singular field: {}", self.reconstruct_def()),
        }
    }

    fn map(&self) -> &MapField {
        match self.kind {
            FieldKind::Map(ref map) => &map,
            _ => panic!("not a map field: {}", self.reconstruct_def()),
        }
    }

    fn variant_path(&self) -> String {
        // TODO: should reuse code from OneofVariantGen
        format!("{}::{}", self.oneof().oneof_type_name, self.rust_name)
    }

    // TODO: drop it
    fn elem(&self) -> &GenProtobufType {
        match self.kind {
            FieldKind::Singular(SingularField { ref elem, .. }) => &elem,
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => &elem,
            FieldKind::Oneof(OneofField { ref elem, .. }) => &elem,
            FieldKind::Map(..) => unreachable!(),
        }
    }

    // type of field in struct
    fn full_storage_type(&self) -> RustType {
        match self.kind {
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => {
                if !elem.is_copy() {
                    RustType::RepeatedField(Box::new(elem.rust_type()))
                } else {
                    RustType::Vec(Box::new(elem.rust_type()))
                }
            }
            FieldKind::Map(MapField { ref key, ref value, .. }) => {
                RustType::HashMap(Box::new(key.rust_type()), Box::new(value.rust_type()))
            }
            FieldKind::Singular(SingularField { ref elem, flag: SingularFieldFlag::WithFlag { .. }}) => {
                match elem.proto_type() {
                    FieldDescriptorProto_Type::TYPE_MESSAGE =>
                        RustType::SingularPtrField(Box::new(elem.rust_type())),
                    FieldDescriptorProto_Type::TYPE_STRING |
                    FieldDescriptorProto_Type::TYPE_BYTES  =>
                        RustType::SingularField(Box::new(elem.rust_type())),
                    _ =>
                        RustType::Option(Box::new(elem.rust_type()))
                }

            }
            FieldKind::Singular(SingularField { ref elem, flag: SingularFieldFlag::WithoutFlag }) => {
                elem.rust_type()
            }
            FieldKind::Oneof(..) => {
                unreachable!()
            }
        }
    }

    // type of `v` in `for v in field`
    fn full_storage_iter_elem_type(&self) -> RustType {
        self.full_storage_type().iter_elem_type()
    }

    // suffix `xxx` as in `os.write_xxx_no_tag(..)`
    fn os_write_fn_suffix(&self) -> &str {
        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => "message",
            FieldDescriptorProto_Type::TYPE_ENUM    => "enum",
            ty => protobuf_name(ty),
        }
    }

    // type of `v` in `os.write_xxx_no_tag(v)`
    fn os_write_fn_param_type(&self) -> RustType {
        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_STRING =>
                RustType::Ref(Box::new(RustType::Str)),
            FieldDescriptorProto_Type::TYPE_BYTES  =>
                RustType::Ref(Box::new(RustType::Slice(Box::new(RustType::Int(false, 8))))),
            FieldDescriptorProto_Type::TYPE_ENUM   =>
                RustType::Int(true, 32),
            t => rust_name(t),
        }
    }

    // for field `foo`, type of param of `fn set_foo(..)`
    fn set_xxx_param_type(&self) -> RustType {
        match self.kind {
            FieldKind::Singular(SingularField { ref elem, .. }) |
            FieldKind::Oneof(OneofField { ref elem, .. })       => elem.rust_type(),
            FieldKind::Repeated(..) |
            FieldKind::Map(..)      => self.full_storage_type(),
        }
    }

    // for field `foo`, return type if `fn take_foo(..)`
    fn take_xxx_return_type(&self) -> RustType {
        self.set_xxx_param_type()
    }

    // for field `foo`, return type of `fn mut_foo(..)`
    fn mut_xxx_return_type(&self) -> RustType {
        RustType::Ref(Box::new(match self.kind {
            FieldKind::Singular(SingularField { ref elem, .. }) |
            FieldKind::Oneof(OneofField { ref elem, .. })       => elem.rust_type(),
            FieldKind::Repeated(..) |
            FieldKind::Map (..)     => self.full_storage_type()
        }))
    }

    // for field `foo`, return type of `fn get_foo(..)`
    fn get_xxx_return_type(&self) -> RustType {
        match self.kind {
            FieldKind::Singular(SingularField { ref elem, .. }) |
            FieldKind::Oneof(OneofField { ref elem, .. }) => {
                match elem.is_copy() {
                    true => elem.rust_type(),
                    false => elem.rust_type().ref_type(),
                }
            }
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => {
                RustType::Ref(Box::new(RustType::Slice(Box::new(elem.rust_type()))))
            }
            FieldKind::Map(..) => {
                RustType::Ref(Box::new(self.full_storage_type()))
            }
        }
    }

    // suffix to convert field value to option
    // like `.as_ref()` in `self.xx.as_ref()`
    fn as_option(&self) -> &'static str {
        assert!(self.is_singular());
        match self.full_storage_type() {
            RustType::Option(..) => "",
            _                    => ".as_ref()"
        }
    }

    // type of expression returned by `as_option()`
    fn as_option_type(&self) -> RustType {
        assert!(self.is_singular());
        match self.full_storage_type() {
            r @ RustType::Option(..)       => r,
            RustType::SingularField(ty)    |
            RustType::SingularPtrField(ty) => RustType::Option(Box::new(RustType::Ref(ty))),
            x => panic!("cannot convert {} to option", x),
        }
    }

    // fixed size type?
    fn is_fixed(&self) -> bool {
        field_type_size(self.proto_type).is_some()
    }

    // must use zigzag encoding?
    fn is_zigzag(&self) -> bool {
        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_SINT32 |
            FieldDescriptorProto_Type::TYPE_SINT64 => true,
            _ => false,
        }
    }

    // data is enum
    fn is_enum(&self) -> bool {
        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_ENUM => true,
            _ => false,
        }
    }

    // elem data is not stored in heap
    fn elem_type_is_copy(&self) -> bool {
        type_is_copy(self.proto_type)
    }

    fn defaut_value_from_proto_float(&self) -> String {
        let type_name = match self.proto_type {
            FieldDescriptorProto_Type::TYPE_FLOAT  => "f32",
            FieldDescriptorProto_Type::TYPE_DOUBLE => "f64",
            _ => unreachable!()
        };
        let proto_default = self.proto_field.get_default_value();

        fn parse_special_float(s: &str) -> Option<&'static str> {
            if s == "nan" {
                Some("NAN")
            } else if s == "inf" {
                Some("INFINITY")
            } else if s == "-inf" {
                Some("NEG_INFINITY")
            } else {
                None
            }
        }

        match parse_special_float(proto_default) {
            Some(special) => format!("::std::{}::{}", type_name, special),
            // hope it is decimal float
            None          => format!("{}{}", proto_default, type_name),
        }
    }

    fn default_value_from_proto(&self) -> Option<String> {
        assert!(self.is_singular() || self.is_oneof());
        if self.enum_default_value.is_some() {
            Some(self.enum_default_value.as_ref().unwrap().rust_name_outer())
        } else if self.proto_field.has_default_value() {
            let proto_default = self.proto_field.get_default_value();
            Some(match self.proto_type {
                // For numeric types, contains the original text representation of the value
                FieldDescriptorProto_Type::TYPE_DOUBLE   |
                FieldDescriptorProto_Type::TYPE_FLOAT    => self.defaut_value_from_proto_float(),
                FieldDescriptorProto_Type::TYPE_INT32    |
                FieldDescriptorProto_Type::TYPE_SINT32   |
                FieldDescriptorProto_Type::TYPE_SFIXED32 => format!("{}i32", proto_default),
                FieldDescriptorProto_Type::TYPE_UINT32   |
                FieldDescriptorProto_Type::TYPE_FIXED32  => format!("{}u32", proto_default),
                FieldDescriptorProto_Type::TYPE_INT64    |
                FieldDescriptorProto_Type::TYPE_SINT64   |
                FieldDescriptorProto_Type::TYPE_SFIXED64 => format!("{}i64", proto_default),
                FieldDescriptorProto_Type::TYPE_UINT64   |
                FieldDescriptorProto_Type::TYPE_FIXED64  => format!("{}u64", proto_default),

                // For booleans, "true" or "false"
                FieldDescriptorProto_Type::TYPE_BOOL     => format!("{}", proto_default),
                // For strings, contains the default text contents (not escaped in any way)
                FieldDescriptorProto_Type::TYPE_STRING   => format!("\"{}\"", escape_default(proto_default)),
                // For bytes, contains the C escaped value.  All bytes >= 128 are escaped
                FieldDescriptorProto_Type::TYPE_BYTES    => format!("b\"{}\"", proto_default),
                // TODO: resolve outer message prefix
                FieldDescriptorProto_Type::TYPE_GROUP    |
                FieldDescriptorProto_Type::TYPE_ENUM     => unreachable!(),
                FieldDescriptorProto_Type::TYPE_MESSAGE  =>
                    panic!("default value is not implemented for type: {:?}", self.proto_type)
            })
        } else {
            None
        }
    }

    // default value to be returned from fn get_xxx
    fn get_xxx_default_value_rust(&self) -> String {
        assert!(self.is_singular() || self.is_oneof());
        self.default_value_from_proto().unwrap_or_else(|| self.get_xxx_return_type().default_value())
    }

    // default to be assigned to field
    fn element_default_value_rust(&self) -> String {
        assert!(self.is_singular() || self.is_oneof(), "field is not singular: {}", self.reconstruct_def());
        self.default_value_from_proto().unwrap_or_else(|| self.elem().rust_type().default_value())
    }

    fn reconstruct_def(&self) -> String {
        let prefix = match (self.proto_field.get_label(), self.syntax) {
            (FieldDescriptorProto_Label::LABEL_REPEATED, _) => "repeated ",
            (_,                             Syntax::PROTO3) => "",
            (FieldDescriptorProto_Label::LABEL_OPTIONAL, _) => "optional ",
            (FieldDescriptorProto_Label::LABEL_REQUIRED, _) => "required ",
        };
        format!("{}{} {} = {}",
            prefix,
            field_type_protobuf_name(&self.proto_field),
            self.proto_field.get_name(),
            self.proto_field.get_number())
    }

    fn accessor_fn(&self) -> AccessorFn {
        match self.kind {
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => {
                let coll =
                    match self.full_storage_type() {
                        RustType::Vec(..) => "vec",
                        RustType::RepeatedField(..) => "repeated_field",
                        _ => unreachable!(),
                    };
                let name = format!("make_{}_accessor", coll);
                AccessorFn {
                    name: name,
                    type_params: vec![elem.lib_protobuf_type()],
                    for_reflect_suffix: true,
                    accessors: vec![
                        format!("get_{}_for_reflect", self.rust_name),
                        format!("mut_{}_for_reflect", self.rust_name),
                    ]
                }
            }
            FieldKind::Map(MapField { ref key, ref value, .. }) => {
                AccessorFn {
                    name: "make_map_accessor".to_owned(),
                    type_params: vec![key.lib_protobuf_type(), value.lib_protobuf_type()],
                    for_reflect_suffix: true,
                    accessors: vec![
                        format!("get_{}_for_reflect", self.rust_name),
                        format!("mut_{}_for_reflect", self.rust_name),
                    ]
                }
            }
            FieldKind::Singular(SingularField { ref elem, flag: SingularFieldFlag::WithoutFlag }) => {
                if let &GenProtobufType::Message(ref name) = elem {
                    // TODO: old style, needed because of default instance

                    AccessorFn {
                        name: "make_singular_message_accessor".to_owned(),
                        type_params: vec![
                            name.clone(),
                        ],
                        for_reflect_suffix: false,
                        accessors: vec![
                            format!("has_{}", self.rust_name),
                            format!("get_{}", self.rust_name),
                        ]
                    }
                } else {
                    AccessorFn {
                        name: "make_simple_field_accessor".to_owned(),
                        type_params: vec![elem.lib_protobuf_type()],
                        for_reflect_suffix: true,
                        accessors: vec![
                            format!("get_{}_for_reflect", self.rust_name),
                            format!("mut_{}_for_reflect", self.rust_name),
                        ]
                    }
                }
            }
            FieldKind::Singular(SingularField { ref elem, flag: SingularFieldFlag::WithFlag { .. } }) => {
                let coll =
                    match self.full_storage_type() {
                        RustType::Option(..) => "option",
                        RustType::SingularField(..) => "singular_field",
                        RustType::SingularPtrField(..) => "singular_ptr_field",
                        _ => unreachable!(),
                    };
                let name = format!("make_{}_accessor", coll);
                AccessorFn {
                    name: name,
                    type_params: vec![elem.lib_protobuf_type()],
                    for_reflect_suffix: true,
                    accessors: vec![
                        format!("get_{}_for_reflect", self.rust_name),
                        format!("mut_{}_for_reflect", self.rust_name),
                    ]
                }
            }
            FieldKind::Oneof(OneofField { ref elem, .. }) => {
                // TODO: uses old style

                let suffix = match &self.elem().rust_type() {
                    t if t.is_primitive()                     => format!("{}", t),
                    &RustType::String                         => "string".to_string(),
                    &RustType::Vec(ref t) if t.is_u8()        => "bytes".to_string(),
                    &RustType::Enum(..)                       => "enum".to_string(),
                    &RustType::Message(..)                    => "message".to_string(),
                    t => panic!("unexpected field type: {}", t),
                };

                let name = format!("make_singular_{}_accessor", suffix);

                let mut type_params = Vec::new();
                match elem {
                    &GenProtobufType::Message(ref name) | &GenProtobufType::Enum(ref name, _) => {
                        type_params.push(name.to_owned());
                    }
                    _ => (),
                }

                AccessorFn {
                    name: name,
                    type_params: type_params,
                    for_reflect_suffix: false,
                    accessors: vec![
                        format!("has_{}", self.rust_name),
                        format!("get_{}", self.rust_name),
                    ]
                }
            }
        }
    }

    fn write_clear(&self, w: &mut CodeWriter) {
        if self.is_oneof() {
            w.write_line(&format!("self.{} = ::std::option::Option::None;", self.oneof().oneof_name));
        } else {
            let clear_expr = self.full_storage_type().clear(&self.self_field());
            w.write_line(&format!("{};", clear_expr));
        }
    }

    // expression that returns size of data is variable
    fn element_size(&self, var: &str, var_type: &RustType) -> String {
        assert!(!self.is_repeated_packed());

        match field_type_size(self.proto_type) {
            Some(data_size) => {
                format!("{}", data_size + self.tag_size())
            },
            None => {
                match self.proto_type {
                    FieldDescriptorProto_Type::TYPE_MESSAGE => panic!("not a single-liner"),
                    FieldDescriptorProto_Type::TYPE_BYTES => {
                        format!("::protobuf::rt::bytes_size({}, &{})", self.number as isize, var)
                    },
                    FieldDescriptorProto_Type::TYPE_STRING => {
                        format!("::protobuf::rt::string_size({}, &{})", self.number as isize, var)
                    },
                    FieldDescriptorProto_Type::TYPE_ENUM => {
                        let param_type = match var_type {
                            &RustType::Ref(ref t) => (**t).clone(),
                            t => t.clone(),
                        };
                        format!("::protobuf::rt::enum_size({}, {})",
                                self.number as isize, var_type.into_target(&param_type, var))
                    },
                    _ => {
                        let param_type = match var_type {
                            &RustType::Ref(ref t) => (**t).clone(),
                            t => t.clone(),
                        };
                        if self.proto_type.is_s_varint() {
                            format!("::protobuf::rt::value_varint_zigzag_size({}, {})",
                                    self.number, var_type.into_target(&param_type, var))
                        } else {
                            format!("::protobuf::rt::value_size({}, {}, ::protobuf::wire_format::{:?})",
                                    self.number, var_type.into_target(&param_type, var), self.wire_type)
                        }
                    }
                }
            },
        }
    }

    // output code that writes single element to stream
    fn write_write_element(&self, w: &mut CodeWriter, os: &str, var: &str, ty: &RustType) {
        if let FieldKind::Repeated(RepeatedField { packed: true, .. }) = self.kind {
            unreachable!();
        };

        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                w.write_line(&format!("try!({}.write_tag({}, ::protobuf::wire_format::{:?}));",
                        os, self.number, wire_format::WireTypeLengthDelimited));
                w.write_line(&format!("try!({}.write_raw_varint32({}.get_cached_size()));",
                        os, var));
                w.write_line(&format!("try!({}.write_to_with_cached_sizes({}));",
                        var, os));
            }
            _ => {
                let param_type = self.os_write_fn_param_type();
                let os_write_fn_suffix = self.os_write_fn_suffix();
                let number = self.number();
                w.write_line(&format!("try!({}.write_{}({}, {}));",
                    os,
                    os_write_fn_suffix,
                    number,
                    ty.into_target(&param_type, var)));
            }
        }
    }

    fn self_field(&self) -> String {
        format!("self.{}", self.rust_name)
    }

    fn self_field_is_some(&self) -> String {
        assert!(self.is_singular());
        format!("{}.is_some()", self.self_field())
    }

    fn self_field_is_not_empty(&self) -> String {
        assert!(self.is_repeated_or_map());
        format!("!{}.is_empty()", self.self_field())
    }

    fn self_field_is_none(&self) -> String {
        assert!(self.is_singular());
        format!("{}.is_none()", self.self_field())
    }

    // field data viewed as Option
    fn self_field_as_option(&self) -> String {
        format!("{}{}", self.self_field(), self.as_option())
    }

    fn write_if_let_self_field_is_some<F>(&self, w: &mut CodeWriter, cb: F)
        where F : Fn(&str, &RustType, &mut CodeWriter)
    {
        match self.kind {
            FieldKind::Repeated(..) | FieldKind::Map(..) => panic!(),
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. }) => {
                let var = "v";
                w.if_let_stmt(&format!("Some({})", var), &self.self_field_as_option(), |w| {
                    let option_type = self.as_option_type();
                    let v_type = option_type.elem_type();
                    cb(var, &v_type, w);
                });
            }
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithoutFlag, .. }) => {
                w.if_stmt(format!("{} != {}", self.self_field(), self.full_storage_type().default_value()), |w| {
                    cb(&self.self_field(), &self.full_storage_type(), w);
                });
            }
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    fn write_if_self_field_is_not_empty<F>(&self, w: &mut CodeWriter, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        assert!(self.is_repeated_or_map());
        let self_field_is_not_empty = self.self_field_is_not_empty();
        w.if_stmt(self_field_is_not_empty, cb);
    }

    fn write_if_self_field_is_none<F>(&self, w: &mut CodeWriter, cb: F)
        where F : Fn(&mut CodeWriter)
    {
        let self_field_is_none = self.self_field_is_none();
        w.if_stmt(self_field_is_none, cb)
    }

    // repeated or singular
    fn write_for_self_field<F>(&self, w: &mut CodeWriter, varn: &str, cb: F)
        where F : Fn(&mut CodeWriter, &RustType)
    {
        let v_type = self.full_storage_iter_elem_type();
        let self_field = self.self_field();
        w.for_stmt(&format!("&{}", self_field), varn, |w| cb(w, &v_type));
    }

    fn write_self_field_assign(&self, w: &mut CodeWriter, value: &str) {
        let self_field = self.self_field();
        w.write_line(&format!("{} = {};", self_field, value));
    }

    fn write_self_field_assign_some(&self, w: &mut CodeWriter, value: &str) {
        let full_storage_type = self.full_storage_type();
        match self.singular() {
            &SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. } => {
                self.write_self_field_assign(w, &full_storage_type.wrap_value(value.as_ref()));
            }
            &SingularField { flag: SingularFieldFlag::WithoutFlag, .. } => {
                self.write_self_field_assign(w, value);
            }
        }
    }

    fn write_self_field_assign_value(&self,
        w: &mut CodeWriter, value: &str, ty: &RustType)
    {
        match self.kind {
            FieldKind::Repeated(..) | FieldKind::Map(..) => {
                let converted = ty.into_target(&self.full_storage_type(), value.as_ref());
                self.write_self_field_assign(w, &converted);
            }
            FieldKind::Singular(SingularField { ref elem, ref flag }) => {
                let converted = ty.into_target(&elem.rust_type(), value.as_ref());
                let wrapped =
                    if *flag == SingularFieldFlag::WithoutFlag {
                        converted
                    } else {
                        self.full_storage_type().wrap_value(&converted)
                    };
                self.write_self_field_assign(w, &wrapped);
            }
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    fn write_self_field_assign_default(&self, w: &mut CodeWriter) {
        assert!(self.is_singular());
        if self.is_oneof() {
            let self_field_oneof = self.self_field_oneof();
            w.write_line(
                format!("{} = ::std::option::Option::Some({}({}))",
                self_field_oneof,
                self.variant_path(),
                // TODO: default from .proto is not needed here
                self.element_default_value_rust()));
        } else {
            if !self.elem_type_is_copy() {
                let self_field = self.self_field();
                w.write_line(&format!("{}.set_default();", self_field));
            } else {
                self.write_self_field_assign_some(w, &self.element_default_value_rust());
            }
        }
    }

    fn self_field_vec_packed_fixed_data_size(&self) -> String {
        assert!(self.is_fixed());
        format!("({}.len() * {}) as u32",
            self.self_field(), field_type_size(self.proto_type).unwrap())
    }

    fn self_field_vec_packed_varint_data_size(&self) -> String {
        assert!(!self.is_fixed());
        let fn_name = if self.is_enum() {
            "vec_packed_enum_data_size".to_string()
        } else {
            let zigzag_suffix = if self.is_zigzag() { "_zigzag" } else { "" };
            format!("vec_packed_varint{}_data_size", zigzag_suffix)
        };
        format!("::protobuf::rt::{}(&{})",
            fn_name, self.self_field())
    }

    fn self_field_vec_packed_data_size(&self) -> String {
        assert!(self.is_repeated_not_map());
        if self.is_fixed() {
            self.self_field_vec_packed_fixed_data_size()
        } else {
            self.self_field_vec_packed_varint_data_size()
        }
    }

    fn self_field_vec_packed_fixed_size(&self) -> String {
        // zero is filtered outside
        format!("{} + ::protobuf::rt::compute_raw_varint32_size({}.len() as u32) + {}",
            self.tag_size(),
            self.self_field(),
            self.self_field_vec_packed_fixed_data_size())
    }

    fn self_field_vec_packed_varint_size(&self) -> String {
        // zero is filtered outside
        assert!(!self.is_fixed());
        let fn_name = if self.is_enum() {
            "vec_packed_enum_size".to_string()
        } else {
            let zigzag_suffix = if self.is_zigzag() { "_zigzag" } else { "" };
            format!("vec_packed_varint{}_size", zigzag_suffix)
        };
        format!("::protobuf::rt::{}({}, &{})",
            fn_name, self.number, self.self_field())
    }

    fn self_field_oneof(&self) -> String {
        format!("self.{}", self.oneof().oneof_name)
    }

    fn clear_field_func(&self) -> String {
        format!("clear_{}", self.rust_name)
    }


    fn write_merge_from_field_message_string_bytes(&self, w: &mut CodeWriter) {
        let singular_or_repeated = match self.kind {
            FieldKind::Repeated(..) => "repeated",
            FieldKind::Map(..) => "repeated", // TODO
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. }) => {
                "singular"
            },
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithoutFlag, .. }) => {
                "singular_proto3"
            },
            FieldKind::Oneof(..) => unreachable!(),
        };
        w.write_line(&format!(
            "try!(::protobuf::rt::read_{}_{}_into(wire_type, is, &mut self.{}));",
            singular_or_repeated,
            protobuf_name(self.proto_type),
            self.rust_name));
    }

    fn write_merge_from_oneof(&self, w: &mut CodeWriter) {
        w.assert_wire_type(self.wire_type);
        // TODO: split long line
        w.write_line(&format!("self.{} = ::std::option::Option::Some({}(try!({})));",
                             self.oneof().oneof_name,
                             self.variant_path(),
                             self.proto_type.read("is")));
    }

    fn write_merge_from_map(&self, w: &mut CodeWriter) {
        let &MapField { ref key, ref value, .. } = self.map();
        w.write_line(&format!("try!(::protobuf::rt::read_map_into::<{}, {}>(wire_type, is, &mut {}));",
            key.lib_protobuf_type(),
            value.lib_protobuf_type(),
            self.self_field()));
    }

    fn write_merge_from_field(&self, w: &mut CodeWriter) {
        match self.kind {
            FieldKind::Oneof(..) => self.write_merge_from_oneof(w),
            FieldKind::Map(..) => self.write_merge_from_map(w),
            _ => {
                if !self.elem_type_is_copy() {
                    self.write_merge_from_field_message_string_bytes(w);
                } else {
                    let wire_type = field_type_wire_type(self.proto_type);
                    let read_proc = format!("try!(is.read_{}())", protobuf_name(self.proto_type));

                    match self.kind {
                        FieldKind::Singular(..) => {
                            w.assert_wire_type(wire_type);
                            w.write_line(&format!("let tmp = {};", read_proc));
                            self.write_self_field_assign_some(w, "tmp");
                        }
                        FieldKind::Repeated(..) => {
                            w.write_line(&format!(
                                "try!(::protobuf::rt::read_repeated_{}_into(wire_type, is, &mut self.{}));",
                                protobuf_name(self.proto_type),
                                self.rust_name));
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    fn self_field_vec_packed_size(&self) -> String {
        match self.kind {
            FieldKind::Repeated(RepeatedField { packed: true, .. }) => {
                // zero is filtered outside
                if self.is_fixed() {
                    self.self_field_vec_packed_fixed_size()
                } else {
                    self.self_field_vec_packed_varint_size()
                }
            }
            _ => {
                panic!("not packed");
            }
        }
    }

    fn write_element_size(&self, w: &mut CodeWriter, item_var: &str, item_var_type: &RustType, sum_var: &str) {
        assert!(!self.is_repeated_packed());

        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                w.write_line(&format!("let len = {}.compute_size();", item_var));
                let tag_size = self.tag_size();
                w.write_line(&format!(
                        "{} += {} + ::protobuf::rt::compute_raw_varint32_size(len) + len;",
                        sum_var, tag_size));
            },
            _ => {
                w.write_line(&format!(
                        "{} += {};", sum_var, self.element_size(item_var, item_var_type)));
            },
        }
    }

    fn write_message_write_field(&self, w: &mut CodeWriter) {
        match self.kind {
            FieldKind::Singular(..) => {
                self.write_if_let_self_field_is_some(w, |v, v_type, w| {
                    self.write_write_element(w, "os", v, v_type);
                });
            },
            FieldKind::Repeated(RepeatedField { packed: false, .. }) => {
                self.write_for_self_field(w, "v", |w, v_type| {
                    self.write_write_element(w, "os", "v", v_type);
                });
            },
            FieldKind::Repeated(RepeatedField { packed: true, .. }) => {
                self.write_if_self_field_is_not_empty(w, |w| {
                    let number = self.number();
                    w.write_line(&format!("try!(os.write_tag({}, ::protobuf::wire_format::{:?}));", number, wire_format::WireTypeLengthDelimited));
                    w.comment("TODO: Data size is computed again, it should be cached");
                    let data_size_expr = self.self_field_vec_packed_data_size();
                    w.write_line(&format!("try!(os.write_raw_varint32({}));", data_size_expr));
                    self.write_for_self_field(w, "v", |w, v_type| {
                        let param_type = self.os_write_fn_param_type();
                        let os_write_fn_suffix = self.os_write_fn_suffix();
                        w.write_line(&format!("try!(os.write_{}_no_tag({}));",
                            os_write_fn_suffix, v_type.into_target(&param_type, "v")));
                    });
                });
            },
            FieldKind::Map(MapField { ref key, ref value, .. }) => {
                w.write_line(&format!("try!(::protobuf::rt::write_map_with_cached_sizes::<{}, {}>({}, &{}, os));",
                    key.lib_protobuf_type(),
                    value.lib_protobuf_type(),
                    self.number,
                    self.self_field()));
            }
            FieldKind::Oneof(..) => unreachable!(),
        };
    }

    fn write_message_compute_field_size(&self, sum_var: &str, w: &mut CodeWriter) {
        match self.kind {
            FieldKind::Singular(..) => {
                self.write_if_let_self_field_is_some(w, |v, v_type, w| {
                    match field_type_size(self.proto_type) {
                        Some(s) => {
                                let tag_size = self.tag_size();
                                w.write_line(&format!(
                                    "{} += {};",
                                    sum_var,
                                    (s + tag_size) as isize));
                        },
                        None => {
                            self.write_element_size(w, v, v_type, sum_var);
                        },
                    };
                });
            }
            FieldKind::Repeated(RepeatedField { packed: false, .. }) => {
                match field_type_size(self.proto_type) {
                    Some(s) => {
                        let tag_size = self.tag_size();
                        let self_field = self.self_field();
                        w.write_line(&format!(
                            "{} += {} * {}.len() as u32;",
                            sum_var,
                            (s + tag_size) as isize,
                            self_field));
                    },
                    None => {
                        self.write_for_self_field(w, "value", |w, value_type| {
                            self.write_element_size(w, "value", value_type, sum_var);
                        });
                    },
                };
            },
            FieldKind::Map(MapField { ref key, ref value, .. }) => {
                w.write_line(&format!("{} += ::protobuf::rt::compute_map_size::<{}, {}>({}, &{});",
                    sum_var,
                    key.lib_protobuf_type(),
                    value.lib_protobuf_type(),
                    self.number,
                    self.self_field()));
            }
            FieldKind::Repeated(RepeatedField { packed: true, .. }) => {
                self.write_if_self_field_is_not_empty(w, |w| {
                    let size_expr = self.self_field_vec_packed_size();
                    w.write_line(&format!("{} += {};", sum_var, size_expr));
                });
            },
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    fn write_message_field_get_singular(&self, w: &mut CodeWriter) {
        let get_xxx_return_type = self.get_xxx_return_type();

        if self.proto_type == FieldDescriptorProto_Type::TYPE_MESSAGE {
            let self_field = self.self_field();
            let ref field_type_name = self.elem().rust_type();
            w.write_line(&format!("{}.as_ref().unwrap_or_else(|| {}::default_instance())",
                    self_field, field_type_name));
        } else {
            let get_xxx_default_value_rust = self.get_xxx_default_value_rust();
            let self_field = self.self_field();
            match self.singular() {
                &SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. } => {
                    if get_xxx_return_type.is_ref() {
                        let self_field_as_option = self.self_field_as_option();
                        w.match_expr(self_field_as_option, |w| {
                            let option_type = self.as_option_type();
                            let v_type = option_type.elem_type();
                            let r_type = self.get_xxx_return_type();
                            w.case_expr(
                                "Some(v)",
                                v_type.into_target(&r_type, "v")
                            );
                            let get_xxx_default_value_rust = self.get_xxx_default_value_rust();
                            w.case_expr(
                                "None",
                                get_xxx_default_value_rust
                            );
                        });
                    } else {
                        w.write_line(&format!(
                            "{}.unwrap_or({})", self_field, get_xxx_default_value_rust));
                    }
                }
                &SingularField { flag: SingularFieldFlag::WithoutFlag, .. } => {
                    w.write_line(self.full_storage_type().into_target(&get_xxx_return_type, &self_field));
                }
            }
        }
    }

    fn write_message_field_get(&self, w: &mut CodeWriter) {
        let get_xxx_return_type = self.get_xxx_return_type();
        let fn_def = format!("get_{}(&self) -> {}",  self.rust_name, get_xxx_return_type);

        w.pub_fn(&fn_def,
        |w| {
            match self.kind {
                FieldKind::Oneof(OneofField { ref elem, .. }) => {
                    let self_field_oneof = self.self_field_oneof();
                    w.match_expr(self_field_oneof, |w| {
                        let (refv, vtype) =
                            if !self.elem_type_is_copy() {
                                ("ref v", elem.rust_type().ref_type())
                            } else {
                                ("v", elem.rust_type())
                            };
                        w.case_expr(format!(
                                "::std::option::Option::Some({}({}))",
                                self.variant_path(),
                                refv),
                            vtype.into_target(&get_xxx_return_type, "v"));
                        w.case_expr("_", self.get_xxx_default_value_rust());
                    })
                }
                FieldKind::Singular(..) => {
                    self.write_message_field_get_singular(w);
                }
                FieldKind::Repeated(..) | FieldKind::Map(..) => {
                    let self_field = self.self_field();
                    w.write_line(&format!("&{}", self_field));
                }
            }
        });
    }

    fn write_message_field_get_for_reflect(&self, w: &mut CodeWriter) {
        let sig = format!("get_{}_for_reflect(&self) -> &{}", self.rust_name, self.full_storage_type());
        w.def_fn(&sig, |w| {
            w.write_line(&format!("&{}", self.self_field()))
        });
    }

    fn write_message_field_mut_for_reflect(&self, w: &mut CodeWriter) {
        let sig = format!("mut_{}_for_reflect(&mut self) -> &mut {}", self.rust_name, self.full_storage_type());
        w.def_fn(&sig, |w| {
            w.write_line(&format!("&mut {}", self.self_field()))
        });
    }

    fn has_has(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(..) | FieldKind::Map(..) => false,
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. }) => true,
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithoutFlag, .. }) => false,
            FieldKind::Oneof(..) => true,
        }
    }

    fn has_mut(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(..) | FieldKind::Map(..) => true,
            // TODO: string should be public, and mut is not needed
            FieldKind::Singular(..) | FieldKind::Oneof(..) => !self.elem_type_is_copy(),
        }
    }

    fn has_take(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(..) | FieldKind::Map(..) => true,
            // TODO: string should be public, and mut is not needed
            FieldKind::Singular(..) | FieldKind::Oneof(..) => !self.elem_type_is_copy(),
        }

    }

    fn has_name(&self) -> String {
        format!("has_{}", self.rust_name)
    }

    fn write_message_field_has(&self, w: &mut CodeWriter) {
        w.pub_fn(&format!("{}(&self) -> bool", self.has_name()), |w| {
            if !self.is_oneof() {
                let self_field_is_some = self.self_field_is_some();
                w.write_line(self_field_is_some);
            } else {
                let self_field_oneof = self.self_field_oneof();
                w.match_expr(self_field_oneof, |w| {
                    w.case_expr(format!(
                            "::std::option::Option::Some({}(..))",
                            self.variant_path()),
                        "true");
                    w.case_expr("_", "false");
                });
            }
        });
    }

    fn write_message_field_set(&self, w: &mut CodeWriter) {
        let set_xxx_param_type = self.set_xxx_param_type();
        w.comment("Param is passed by value, moved");
        let ref name = self.rust_name;
        w.pub_fn(&format!("set_{}(&mut self, v: {})", name, set_xxx_param_type), |w| {
            if !self.is_oneof() {
                self.write_self_field_assign_value(w, "v", &set_xxx_param_type);
            } else {
                let self_field_oneof = self.self_field_oneof();
                w.write_line(&format!("{} = ::std::option::Option::Some({}(v))",
                    self_field_oneof, self.variant_path()));
            }
        });
    }

    fn write_message_field_mut(&self, w: &mut CodeWriter) {
        let mut_xxx_return_type = self.mut_xxx_return_type();
        w.comment("Mutable pointer to the field.");
        if self.is_singular() {
            w.comment("If field is not initialized, it is initialized with default value first.");
        }
        let fn_def = match mut_xxx_return_type {
            RustType::Ref(ref param) => format!("mut_{}(&mut self) -> &mut {}", self.rust_name, **param),
            _ => panic!("not a ref: {}", mut_xxx_return_type),
        };
        w.pub_fn(&fn_def,
        |w| {
            match self.kind {
                FieldKind::Repeated(..) | FieldKind::Map(..) => {
                    let self_field = self.self_field();
                    w.write_line(&format!("&mut {}", self_field));
                }
                FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. }) => {
                    self.write_if_self_field_is_none(w, |w| {
                        self.write_self_field_assign_default(w);
                    });
                    let self_field = self.self_field();
                    w.write_line(&format!("{}.as_mut().unwrap()", self_field));
                }
                FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithoutFlag, .. }) => {
                    w.write_line(&format!("&mut {}", self.self_field()))
                }
                FieldKind::Oneof(..) => {
                    let self_field_oneof = self.self_field_oneof();

                    // if oneof does not contain current field
                    w.if_let_else_stmt(&format!(
                                "::std::option::Option::Some({}(_))",
                                self.variant_path())[..], &self_field_oneof[..],
                    |w|
                    {
                        // initialize it with default value
                        w.write_line(&format!(
                            "{} = ::std::option::Option::Some({}({}));",
                            self_field_oneof,
                            self.variant_path(),
                            self.element_default_value_rust()));
                    });

                    // extract field
                    w.match_expr(self_field_oneof, |w| {
                        w.case_expr(format!(
                                "::std::option::Option::Some({}(ref mut v))",
                                self.variant_path()),
                            "v");
                        w.case_expr("_", "panic!()");
                    });
                }
            }
        });
    }

    fn write_message_field_take(&self, w: &mut CodeWriter) {
        let take_xxx_return_type = self.take_xxx_return_type();
        w.comment("Take field");
        w.pub_fn(&format!("take_{}(&mut self) -> {}", self.rust_name, take_xxx_return_type), |w| {
            match self.kind {
                FieldKind::Oneof(..) => {
                    // TODO: replace with if let
                    w.write_line(&format!("if self.{}() {{", self.has_name()));
                    w.indented(|w| {
                        let self_field_oneof = self.self_field_oneof();
                        w.match_expr(format!("{}.take()", self_field_oneof), |w| {
                            w.case_expr(format!("::std::option::Option::Some({}(v))", self.variant_path()), "v");
                            w.case_expr("_", "panic!()");
                        });
                    });
                    w.write_line("} else {");
                    w.indented(|w| {
                        w.write_line(self.element_default_value_rust());
                    });
                    w.write_line("}");
                }
                FieldKind::Repeated(..) | FieldKind::Map(..) => {
                    w.write_line(&format!("::std::mem::replace(&mut self.{}, {})",
                        self.rust_name,
                        take_xxx_return_type.default_value()));

                }
                FieldKind::Singular(SingularField { ref elem, flag: SingularFieldFlag::WithFlag { .. } }) => {
                    if !elem.is_copy() {
                        w.write_line(&format!("{}.take().unwrap_or_else(|| {})",
                            self.self_field(), elem.rust_type().default_value()));
                    } else {
                        w.write_line(&format!("{}.take().unwrap_or({})",
                            self.self_field(), self.element_default_value_rust()));
                    }
                }
                FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithoutFlag, .. }) => {
                    w.write_line(&format!("::std::mem::replace(&mut {}, {})",
                        self.self_field(), self.full_storage_type().default_value()))
                }
            }
        });
    }

    fn write_message_single_field_accessors(&self, w: &mut CodeWriter) {
        let clear_field_func = self.clear_field_func();
        w.pub_fn(&format!("{}(&mut self)", clear_field_func), |w| {
            self.write_clear(w);
        });

        if self.has_has() {
            w.write_line("");
            self.write_message_field_has(w);
        }

        w.write_line("");
        self.write_message_field_set(w);

        if self.has_mut() {
            w.write_line("");
            self.write_message_field_mut(w);
        }

        if self.has_take() {
            w.write_line("");
            self.write_message_field_take(w);
        }

        w.write_line("");
        self.write_message_field_get(w);

        if self.accessor_fn().for_reflect_suffix {
            w.write_line("");
            self.write_message_field_get_for_reflect(w);
            w.write_line("");
            self.write_message_field_mut_for_reflect(w);
        }
    }
}

#[derive(Clone)]
struct OneofVariantGen<'a> {
    oneof: &'a OneofGen<'a>,
    variant: OneofVariantWithContext<'a>,
    // TODO: OneofField
    field: FieldGen<'a>,
    path: String,
}

impl<'a> OneofVariantGen<'a> {
    fn parse(oneof: &'a OneofGen<'a>, variant: OneofVariantWithContext<'a>, field: &'a FieldGen) -> OneofVariantGen<'a> {
        OneofVariantGen {
            oneof: oneof,
            variant: variant,
            field: field.clone(),
            path: format!("{}::{}", oneof.type_name, field.rust_name),
        }
    }

    fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Clone)]
struct OneofGen<'a> {
    message: &'a MessageGen<'a>,
    oneof: OneofWithContext<'a>,
    type_name: RustType,
}

impl<'a> OneofGen<'a> {
    fn parse(message: &'a MessageGen, oneof: OneofWithContext<'a>) -> OneofGen<'a> {
        let rust_name = oneof.rust_name();
        OneofGen {
            message: message,
            oneof: oneof,
            type_name: RustType::Oneof(rust_name),
        }
    }

    fn name(&self) -> &str {
        match self.oneof.oneof.get_name() {
            "type" => "field_type",
            "box" => "field_box",
            x => x,
        }
    }

    fn variants(&'a self) -> Vec<OneofVariantGen<'a>> {
        self.oneof.variants().into_iter()
            .map(|v| {
                let field = self.message.fields.iter()
                    .filter(|f| f.proto_field.get_name() == v.field_name())
                    .next()
                    .unwrap();
                OneofVariantGen::parse(self, v, field)
            })
            .collect()
    }

    fn full_storage_type(&self) -> RustType {
        RustType::Option(Box::new(self.type_name.clone()))
    }
}

fn write_message_oneof(oneof: &OneofGen, w: &mut CodeWriter) {
    let mut derive = vec!["Clone", "PartialEq"];
    if false /* lite_runtime */ {
        derive.push("Debug");
    }
    w.derive(&derive);
    w.pub_enum(&oneof.type_name.to_string(), |w| {
        for variant in oneof.variants() {
            w.write_line(&format!("{}({}),", variant.field.rust_name, &variant.field.elem().rust_type().to_string()));
        }
    });
}

/// Message info for codegen
pub struct MessageGen<'a> {
    message: &'a MessageWithScope<'a>,
    root_scope: &'a RootScope<'a>,
    type_name: String,
    fields: Vec<FieldGen<'a>>,
    lite_runtime: bool,
}

impl<'a> MessageGen<'a> {
    pub fn new(message: &'a MessageWithScope<'a>, root_scope: &'a RootScope<'a>)
        -> MessageGen<'a>
    {
        let fields: Vec<_> = message.fields().iter().map(|field| {
            FieldGen::parse(field, root_scope)
        }).collect();
        MessageGen {
            message: message,
            root_scope: root_scope,
            type_name: message.rust_name(),
            fields: fields,
            lite_runtime:
                message.get_file_descriptor().get_options().get_optimize_for()
                    == FileOptions_OptimizeMode::LITE_RUNTIME,
        }
    }

    fn oneofs(&'a self) -> Vec<OneofGen<'a>> {
        self.message.oneofs().into_iter().map(|oneof| {
            OneofGen::parse(self, oneof)
        }).collect()
    }

    fn required_fields(&'a self) -> Vec<&'a FieldGen> {
        self.fields.iter().filter(|f| match f.kind {
            FieldKind::Singular(ref singular) => singular.flag.is_required(),
            _ => false,
        }).collect()
    }

    fn fields_except_oneof(&'a self) -> Vec<&'a FieldGen> {
        self.fields.iter()
            .filter(|f| !f.is_oneof())
            .collect()
    }

    fn fields_except_group(&'a self) -> Vec<&'a FieldGen> {
        self.fields.iter()
            .filter(|f| f.proto_type != FieldDescriptorProto_Type::TYPE_GROUP)
            .collect()
    }

    fn fields_except_oneof_and_group(&'a self) -> Vec<&'a FieldGen> {
        self.fields.iter()
            .filter(|f| !f.is_oneof() && f.proto_type != FieldDescriptorProto_Type::TYPE_GROUP)
            .collect()
    }


    fn write_match_each_oneof_variant<F>(&self, w: &mut CodeWriter, cb: F)
        where F: Fn(&mut CodeWriter, &OneofVariantGen, &str, &RustType)
    {
        for oneof in self.oneofs() {
            w.if_let_stmt("::std::option::Option::Some(ref v)", &format!("self.{}", oneof.name())[..], |w| {
                w.match_block("v", |w| {
                    for variant in oneof.variants() {
                        let ref field = variant.field;
                        let (refv, vtype) =
                            if !field.elem_type_is_copy() {
                                ("ref v", field.elem().rust_type().ref_type())
                            } else {
                                ("v", field.elem().rust_type())
                            };
                        w.case_block(format!("&{}({})", variant.path(), refv), |w| {
                            cb(w, &variant, "v", &vtype);
                        });
                    }
                });
            });
        }
    }

    fn write_write_to_with_cached_sizes(&self, w: &mut CodeWriter) {
        w.def_fn("write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()>", |w| {
            // To have access to its methods but not polute the name space.
            for f in self.fields_except_oneof_and_group() {
                f.write_message_write_field(w);
            }
            self.write_match_each_oneof_variant(w, |w, variant, v, v_type| {
                variant.field.write_write_element(w, "os", v, v_type);
            });
            w.write_line("try!(os.write_unknown_fields(self.get_unknown_fields()));");
            w.write_line("::std::result::Result::Ok(())");
        });
    }

    fn write_default_instance(&self, w: &mut CodeWriter) {
        w.pub_fn(&format!("default_instance() -> &'static {}", self.type_name), |w| {
            w.lazy_static_decl_get_simple(
                "instance",
                &self.type_name,
                &format!("{}::new", self.type_name));
        });
    }

    fn write_compute_size(&self, w: &mut CodeWriter) {
        // Append sizes of messages in the tree to the specified vector.
        // First appended element is size of self, and then nested message sizes.
        // in serialization order are appended recursively.");
        w.comment("Compute sizes of nested messages");
        // there are unused variables in oneof
        w.allow(&["unused_variables"]);
        w.def_fn("compute_size(&self) -> u32", |w| {
            // To have access to its methods but not polute the name space.
            w.write_line("let mut my_size = 0;");
            for field in self.fields_except_oneof_and_group() {
                field.write_message_compute_field_size("my_size", w);
            }
            self.write_match_each_oneof_variant(w, |w, variant, v, vtype| {
                variant.field.write_element_size(w, v, vtype, "my_size");
            });
            w.write_line("self.set_cached_size(my_size)");
        });
    }

    fn write_field_accessors(&self, w: &mut CodeWriter) {
        for f in self.fields_except_group() {
            w.write_line("");
            let reconstruct_def = f.reconstruct_def();
            w.comment(&(reconstruct_def + ";"));
            w.write_line("");
            f.write_message_single_field_accessors(w);
        }
    }

    fn write_impl_self(&self, w: &mut CodeWriter) {
        w.impl_self_block(&self.type_name, |w| {
            w.pub_fn(&format!("new() -> {}", self.type_name), |w| {
                w.write_line("::std::default::Default::default()");
            });

            w.write_line("");
            self.write_default_instance(w);
            self.write_field_accessors(w);
        });
    }

    fn write_special_fields(&self, w: &mut CodeWriter) {
        w.def_fn("get_special_fields(&self) -> &::protobuf::SpecialFields", |w| {
            w.write_line("&self.special_fields");
        });
        w.write_line("");
        w.def_fn("mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields", |w| {
            w.write_line("&mut self.special_fields");
        });
    }

    fn write_merge_from(&self, w: &mut CodeWriter) {
        w.def_fn(&format!("merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()>"), |w| {
            w.while_block("!try!(is.eof())", |w| {
                w.write_line(&format!("let (field_number, wire_type) = try!(is.read_tag_unpack());"));
                w.match_block("field_number", |w| {
                    for f in &self.fields_except_group() {
                        let number = f.number;
                        w.case_block(number.to_string(), |w| {
                            f.write_merge_from_field(w);
                        });
                    }
                    w.case_block("_", |w| {
                        w.write_line("try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));");
                    });
                });
            });
            w.write_line("::std::result::Result::Ok(())");
        });
    }

    fn write_descriptor_field(&self, fields_var: &str, field: &FieldGen, w: &mut CodeWriter) {
        let accessor_fn = field.accessor_fn();
        w.write_line(&format!("{}.push(::protobuf::reflect::accessor::{}(",
            fields_var,
            accessor_fn.sig()));
        w.indented(|w| {
            w.write_line(&format!("\"{}\",", field.proto_field.get_name()));
            for acc in &accessor_fn.accessors {
                w.write_line(&format!("{}::{},", self.type_name, acc));
            }
        });
        w.write_line("));");
    }

    fn write_descriptor_static(&self, w: &mut CodeWriter) {
        w.def_fn(&format!("descriptor_static(_: ::std::option::Option<{}>) -> &'static ::protobuf::reflect::MessageDescriptor", self.type_name), |w| {
            w.lazy_static_decl_get("descriptor", "::protobuf::reflect::MessageDescriptor", |w| {
                let fields = self.fields_except_group();
                if fields.is_empty() {
                    w.write_line(&format!("let fields = ::std::vec::Vec::new();"));
                } else {
                    w.write_line(&format!("let mut fields = ::std::vec::Vec::new();"));
                }
                for field in fields {
                    self.write_descriptor_field("fields", field, w);;
                }
                w.write_line(&format!(
                    "::protobuf::reflect::MessageDescriptor::new::<{}>(", self.type_name));
                w.indented(|w| {
                    w.write_line(&format!("\"{}\",", self.type_name));
                    w.write_line("fields,");
                    w.write_line("file_descriptor_proto()");
                });
                w.write_line(")");
            });
        });
    }

    fn write_impl_message(&self, w: &mut CodeWriter) {
        w.impl_for_block("::protobuf::Message", &self.type_name, |w| {
            w.def_fn(&format!("is_initialized(&self) -> bool"), |w| {
                for f in self.required_fields() {
                    f.write_if_self_field_is_none(w, |w| {
                        w.write_line("return false;");
                    });
                }
                w.write_line("true");
            });
            w.write_line("");
            self.write_merge_from(w);
            w.write_line("");
            self.write_compute_size(w);
            w.write_line("");
            self.write_write_to_with_cached_sizes(w);
            w.write_line("");
            self.write_special_fields(w);
            w.write_line("");
            w.def_fn("as_any(&self) -> &::std::any::Any", |w| {
                w.write_line("self as &::std::any::Any");
            });
            w.write_line("");
            w.def_fn("descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor", |w| {
                w.write_line("::protobuf::MessageStatic::descriptor_static(None::<Self>)");
            });
        });
    }

    fn write_impl_message_static(&self, w: &mut CodeWriter) {
        w.impl_for_block("::protobuf::MessageStatic", &self.type_name, |w| {
            w.def_fn(&format!("new() -> {}", self.type_name), |w| {
                w.write_line(&format!("{}::new()", self.type_name));
            });
            if !self.lite_runtime {
                w.write_line("");
                self.write_descriptor_static(w);
            }
        });
    }

    fn write_impl_value(&self, w: &mut CodeWriter) {
        w.impl_for_block("::protobuf::reflect::ProtobufValue", &self.type_name, |w| {
            if !self.lite_runtime {
                w.def_fn("as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef", |w| {
                    w.write_line("::protobuf::reflect::ProtobufValueRef::Message(self)")
                })
            }
        })
    }

    fn write_impl_show(&self, w: &mut CodeWriter) {
        w.impl_for_block("::std::fmt::Debug", &self.type_name, |w| {
            w.def_fn("fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result", |w| {
                w.write_line("::protobuf::text_format::fmt(self, f)");
            });
        });
    }

    fn write_impl_clear(&self, w: &mut CodeWriter) {
        w.impl_for_block("::protobuf::Clear", &self.type_name, |w| {
            w.def_fn("clear(&mut self)", |w| {
                // TODO: no need to clear oneof fields in loop
                for f in self.fields_except_group() {
                    let clear_field_func = f.clear_field_func();
                    w.write_line(&format!("self.{}();", clear_field_func));
                }
                w.write_line("self.mut_unknown_fields().clear();");
            });
        });
    }

    fn write_struct(&self, w: &mut CodeWriter) {
        let mut derive = vec!["Clone", "Default", "PartialEq"];
        if self.lite_runtime {
            derive.push("Debug");
        }
        w.derive(&derive);
        w.pub_struct(&self.type_name, |w| {
            if !self.fields_except_oneof().is_empty() {
                w.comment("message fields");
                for field in self.fields_except_oneof() {
                    if field.proto_type == FieldDescriptorProto_Type::TYPE_GROUP {
                        w.comment(&format!("{}: <group>", &field.rust_name));
                    } else {
                        match field.kind {
                            FieldKind::Repeated(..)                |
                            FieldKind::Map(..)                     |
                            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. }) => {
                                w.field_decl(&field.rust_name, &field.full_storage_type().to_string());
                            }
                            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithoutFlag, .. }) => {
                                w.pub_field_decl(&field.rust_name, &field.full_storage_type().to_string());
                            }
                            FieldKind::Oneof(..) => unreachable!(),
                        }
                    }
                }
            }
            if !self.oneofs().is_empty() {
                w.comment("message oneof groups");
                for oneof in self.oneofs() {
                    w.field_decl(oneof.name(), &oneof.full_storage_type().to_string());
                }
            }
            w.comment("special fields");
            w.field_decl("special_fields", "::protobuf::SpecialFields");
        });
    }

    pub fn write(&self, w: &mut CodeWriter) {
        self.write_struct(w);

        // Cell<u32> (which stores cached size) is not Sync
        // so messages do not implicitly implement sync.
        // `cached_size` could be of type `AtomicUsize`, which could be updated
        // with `Ordering::Relaxed`, however:
        // * usize is twice as large as u32 on 64-bit, and rust has no `AtomicU32`
        // * there's small performance degradation when using `AtomicUsize`, which is
        //   probably related to https://github.com/rust-lang/rust/pull/30962
        // Anyway, `cached_size` is always read after updated from the same thread
        // so even in theory the code is incorrect, `u32` write is atomic on all platforms.
        w.write_line("");
        w.comment("see codegen.rs for the explanation why impl Sync explicitly");
        w.unsafe_impl("::std::marker::Sync", &self.type_name);
        for oneof in self.oneofs() {
            w.write_line("");
            write_message_oneof(&oneof, w);
        }

        w.write_line("");
        self.write_impl_self(w);
        w.write_line("");
        self.write_impl_message(w);
        w.write_line("");
        self.write_impl_message_static(w);
        w.write_line("");
        self.write_impl_clear(w);
        if !self.lite_runtime {
            w.write_line("");
            self.write_impl_show(w);
            w.write_line("");
            self.write_impl_value(w);
        }

        let mut nested_prefix = self.type_name.to_string();
        nested_prefix.push_str("_");

        for nested in &self.message.to_scope().get_messages() {
            // ignore map entries, because they are not used in map fields
            if nested.map_entry().is_none() {
                w.write_line("");
                MessageGen::new(nested, self.root_scope).write(w);
            }
        }

        for enum_type in &self.message.to_scope().get_enums() {
            w.write_line("");
            EnumGen::new(enum_type, self.message.get_scope().get_file_descriptor()).write(w);
        }
    }
}
