use std::collections::hash_map::HashMap;
use std::fmt;
use std::io::Write;
use std::convert::AsRef;
use std::collections::HashSet;

use descriptor::*;
use misc::*;
use stream::wire_format;
use core::Message;
use compiler_plugin;
use rt;
use paginate::PaginatableIterator;
use strx::*;
use descriptorx::EnumWithScope;
use descriptorx::MessageWithScope;
use descriptorx::FieldWithContext;
use descriptorx::OneofWithContext;
use descriptorx::OneofVariantWithContext;
use descriptorx::FileScope;
use descriptorx::RootScope;
use descriptorx::WithScope;

fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

#[derive(Clone,PartialEq,Eq)]
enum RustType {
    Signed(u32),
    Unsigned(u32),
    Float(u32),
    Bool,
    Vec(Box<RustType>),
    String,
    Slice(Box<RustType>),
    Str,
    Option(Box<RustType>),
    SingularField(Box<RustType>),
    SingularPtrField(Box<RustType>),
    RepeatedField(Box<RustType>),
    Uniq(Box<RustType>),
    Ref(Box<RustType>),
    Message(String),
    // protobuf enum, not any enum
    Enum(String),
    // oneof enum
    Oneof(String),
}

impl fmt::Debug for RustType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RustType::Signed(bits)       => write!(f, "i{}", bits),
            RustType::Unsigned(bits)     => write!(f, "u{}", bits),
            RustType::Float(bits)        => write!(f, "f{}", bits),
            RustType::Bool               => write!(f, "bool"),
            RustType::Vec(ref param)     => write!(f, "::std::vec::Vec<{:?}>", **param),
            RustType::String             => write!(f, "::std::string::String"),
            RustType::Slice(ref param)   => write!(f, "[{:?}]", **param),
            RustType::Str                => write!(f, "str"),
            RustType::Option(ref param)           => write!(f, "::std::option::Option<{:?}>", **param),
            RustType::SingularField(ref param)    => write!(f, "::protobuf::SingularField<{:?}>", **param),
            RustType::SingularPtrField(ref param) => write!(f, "::protobuf::SingularPtrField<{:?}>", **param),
            RustType::RepeatedField(ref param)    => write!(f, "::protobuf::RepeatedField<{:?}>", **param),
            RustType::Uniq(ref param)             => write!(f, "::std::Box<{:?}>", **param),
            RustType::Ref(ref param)              => write!(f, "&{:?}", **param),
            RustType::Message(ref name) |
            RustType::Enum(ref name)    |
            RustType::Oneof(ref name)   => write!(f, "{}", name),
        }
    }
}

impl RustType {
    fn is_primitive(&self) -> bool {
        match *self {
            RustType::Signed(..)   |
            RustType::Unsigned(..) |
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
            RustType::Unsigned(8) => true,
            _ => false
        }
    }

    fn is_ref(&self) -> bool {
        match *self {
            RustType::Ref(..) => true,
            _           => false,
        }
    }

    fn ref_str(&self, lt: &str) -> String {
        match *self {
            RustType::Ref(ref param) => format!("&'{} {:?}", lt, **param),
            _ => panic!("not a ref: {:?}", *self),
        }
    }

    fn mut_ref_str(&self, lt: &str) -> String {
        match *self {
            RustType::Ref(ref param) => format!("&'{} mut {:?}", lt, **param),
            _ => panic!("not a ref: {:?}", *self),
        }
    }

    fn ref_str_safe(&self, lt: &str) -> String {
        if self.is_ref() {
            self.ref_str(lt)
        } else {
            format!("{:?}", self)
        }
    }

    // default value for type
    fn default_value(&self) -> String {
        match *self {
            RustType::Ref(ref t) if t.is_str()       => "\"\"".to_string(),
            RustType::Ref(ref t) if t.is_slice()     => "&[]".to_string(),
            RustType::Signed(..)                     |
            RustType::Unsigned(..)                   => "0".to_string(),
            RustType::Float(..)                      => "0.".to_string(),
            RustType::Bool(..)                       => "false".to_string(),
            RustType::Vec(..)                        => "::std::vec::Vec::new()".to_string(),
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
            RustType::Enum(..)                       =>
                panic!("enum default value cannot be determined by type"),
            _ => panic!("cannot create default value for: {:?}", *self),
        }
    }

    fn clear(&self, v: &str) -> String {
        match *self {
            RustType::Option(..) => format!("{} = ::std::option::Option::None", v),
            RustType::Vec(..) |
            RustType::String |
            RustType::RepeatedField(..) |
            RustType::SingularField(..) |
            RustType::SingularPtrField(..) => format!("{}.clear()", v),
            ref ty => panic!("cannot clear type: {:?}", ty),
        }
    }

    // wrap value in storage type
    fn wrap_value(&self, value: &str) -> String {
        match *self {
            RustType::Option(..)           => format!("::std::option::Option::Some({})", value),
            RustType::SingularField(..)    => format!("::protobuf::SingularField::some({})", value),
            RustType::SingularPtrField(..) => format!("::protobuf::SingularPtrField::some({})", value),
            _ => panic!("not a wrapper type: {:?}", *self),
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
            (&RustType::Enum(..), &RustType::Signed(32)) =>
                    format!("{} as i32", v),
            (&RustType::Ref(ref t), &RustType::Signed(32)) if t.is_enum() =>
                    format!("*{} as i32", v),
            _ => panic!("cannot convert {:?} to {:?}", self, target),
        }
    }

    fn ref_type(&self) -> RustType {
        RustType::Ref(match self {
            &RustType::String               => Box::new(RustType::Str),
            &RustType::Vec(ref p)           |
            &RustType::RepeatedField(ref p) => Box::new(RustType::Slice(p.clone())),
            &RustType::Message(ref p)       => Box::new(RustType::Message(p.clone())),
            x => panic!("no ref type for {:?}", x),
        })
    }

    fn elem_type(&self) -> RustType {
        match self {
            &RustType::Option(ref ty) => (**ty).clone(),
            x => panic!("cannot get elem type of {:?}", x),
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
            x => panic!("cannot iterate {:?}", x),
        }
    }
}

// rust type for protobuf base type
fn rust_name(field_type: FieldDescriptorProto_Type) -> RustType {
    match field_type {
        FieldDescriptorProto_Type::TYPE_DOUBLE   => RustType::Float(64),
        FieldDescriptorProto_Type::TYPE_FLOAT    => RustType::Float(32),
        FieldDescriptorProto_Type::TYPE_INT32    => RustType::Signed(32),
        FieldDescriptorProto_Type::TYPE_INT64    => RustType::Signed(64),
        FieldDescriptorProto_Type::TYPE_UINT32   => RustType::Unsigned(32),
        FieldDescriptorProto_Type::TYPE_UINT64   => RustType::Unsigned(64),
        FieldDescriptorProto_Type::TYPE_SINT32   => RustType::Signed(32),
        FieldDescriptorProto_Type::TYPE_SINT64   => RustType::Signed(64),
        FieldDescriptorProto_Type::TYPE_FIXED32  => RustType::Unsigned(32),
        FieldDescriptorProto_Type::TYPE_FIXED64  => RustType::Unsigned(64),
        FieldDescriptorProto_Type::TYPE_SFIXED32 => RustType::Signed(32),
        FieldDescriptorProto_Type::TYPE_SFIXED64 => RustType::Signed(64),
        FieldDescriptorProto_Type::TYPE_BOOL     => RustType::Bool,
        FieldDescriptorProto_Type::TYPE_STRING   => RustType::String,
        FieldDescriptorProto_Type::TYPE_BYTES    => RustType::Vec(Box::new(RustType::Unsigned(8))),
        FieldDescriptorProto_Type::TYPE_ENUM     |
        FieldDescriptorProto_Type::TYPE_GROUP    |
        FieldDescriptorProto_Type::TYPE_MESSAGE  => panic!("there is no rust name for {:?}", field_type),
    }
}

impl FieldDescriptorProto_Type {
    fn merge(&self, is: &str, var: &str) -> String {
        match *self {
            FieldDescriptorProto_Type::TYPE_MESSAGE =>
                format!("{}.merge_message({})", is, var),
            FieldDescriptorProto_Type::TYPE_STRING =>
                format!("{}.read_string_into({})", is, var),
            FieldDescriptorProto_Type::TYPE_BYTES =>
                format!("{}.read_bytes_into({})", is, var),
            _ =>
                panic!("unknown type: {:?}", *self),
        }
    }

    fn read(&self, is: &str) -> String {
        format!("{}.read_{}()", is, protobuf_name(*self))
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
        FieldDescriptorProto_Type::TYPE_GROUP    => panic!()
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
        FieldDescriptorProto_Type::TYPE_GROUP    => panic!()
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

fn field_type_name_scope_prefix(field: &FieldDescriptorProto, pkg: &str) -> String {
    if !field.has_type_name() {
        return "".to_string();
    }
    let current_pkg_prefix = if pkg.is_empty() {
        ".".to_string()
    } else {
        format!(".{}.", pkg)
    };
    if field.get_type_name().starts_with(&current_pkg_prefix) {
        let mut tn = remove_prefix(field.get_type_name(), &current_pkg_prefix).to_string();
        match tn.rfind('.') {
            Some(pos) => { tn.truncate(pos + 1); tn }.replace(".", "_"),
            None => "".to_string(),
        }
    } else {
        // TODO: package prefix
        "".to_string()
    }
}

fn field_type_name(field: &FieldDescriptorProto, pkg: &str) -> RustType {
    if field.has_type_name() {
        let current_pkg_prefix = if pkg.is_empty() {
            ".".to_string()
        } else {
            format!(".{}.", pkg)
        };
        let name = (if field.get_type_name().starts_with(&current_pkg_prefix) {
            remove_prefix(field.get_type_name(), &current_pkg_prefix).to_string()
        } else {
            // TODO: package prefix
            remove_to(field.get_type_name(), '.').to_string()
        }).replace(".", "_");
        match field.get_field_type() {
            FieldDescriptorProto_Type::TYPE_MESSAGE => RustType::Message(name),
            FieldDescriptorProto_Type::TYPE_ENUM    => RustType::Enum(name),
            _ => panic!("unknown named type: {:?}", field.get_field_type()),
        }
    } else if field.has_field_type() {
        rust_name(field.get_field_type())
    } else {
        panic!("neither type_name, nor field_type specified for field: {}", field.get_name());
    }
}

#[derive(Clone,PartialEq,Eq)]
enum RepeatMode {
    Single,
    RepeatRegular,
    RepeatPacked,
}

#[derive(Clone)]
struct FieldOneofInfo {
    name: String,
    type_name: RustType,
}

impl FieldOneofInfo {
    fn parse(oneof: &OneofWithContext) -> FieldOneofInfo {
        FieldOneofInfo {
            name: oneof.name().to_string(),
            type_name: RustType::Oneof(oneof.rust_name()),
        }
    }
}

#[derive(Clone)]
struct Field {
    proto_field: FieldDescriptorProto,
    name: String,
    field_type: FieldDescriptorProto_Type,
    wire_type: wire_format::WireType,
    type_scope_prefix: String,
    type_name: RustType,
    enum_default_value: Option<EnumValue>,
    number: u32,
    repeated: bool,
    packed: bool,
    repeat_mode: RepeatMode,
    oneof: Option<FieldOneofInfo>,
}

impl Field {
    fn parse(field: &FieldWithContext, root_scope: &RootScope, pkg: &str) -> Field {
        let type_name = field_type_name(field.field, pkg);
        let repeated = match field.field.get_label() {
            FieldDescriptorProto_Label::LABEL_REPEATED => true,
            FieldDescriptorProto_Label::LABEL_OPTIONAL |
            FieldDescriptorProto_Label::LABEL_REQUIRED => false,
        };
        let name = match field.field.get_name() {
            "type" => "field_type".to_string(),
            x => x.to_string(),
        };
        let packed =
            if field.field.has_options() {
                field.field.get_options().get_packed()
            } else {
                false
            };
        let repeat_mode =
            if repeated {
                if packed { RepeatMode::RepeatPacked } else { RepeatMode::RepeatRegular }
            } else {
                RepeatMode::Single
            };
        let enum_default_value = match field.field.get_field_type() {
            FieldDescriptorProto_Type::TYPE_ENUM => {
                let enum_with_scope = root_scope.find_enum(field.field.get_type_name());
                let e = EnumContext::new(&enum_with_scope, root_scope);
                let ev = if field.field.has_default_value() {
                    e.value_by_name(field.field.get_default_value()).clone()
                } else {
                    e.values().into_iter().next().unwrap()
                };
                Some(ev)
            }
            _ => None,
        };
        Field {
            proto_field: field.field.clone(),
            name: name,
            field_type: field.field.get_field_type(),
            wire_type: field_type_wire_type(field.field.get_field_type()),
            type_name: type_name,
            type_scope_prefix: field_type_name_scope_prefix(field.field, pkg),
            enum_default_value: enum_default_value,
            number: field.field.get_number() as u32,
            repeated: repeated,
            packed: packed,
            repeat_mode: repeat_mode,

            oneof: field.oneof().map(|oneof| FieldOneofInfo::parse(&oneof)),
        }
    }

    fn number(&self) -> u32 {
        self.number
    }

    fn tag_size(&self) -> u32 {
        rt::tag_size(self.number)
    }

    fn is_oneof(&self) -> bool {
        self.oneof.is_some()
    }

    fn variant_path(&self) -> String {
        // TODO: should reuse code from OneofVariantContext
        format!("{:?}::{}", self.oneof.as_ref().unwrap().type_name, self.name)
    }

    // type of field in struct
    fn full_storage_type(&self) -> RustType {
        if self.is_oneof() {
            panic!("field is not oneof: {}", self.name);
        }
        let c = Box::new(self.type_name.clone());
        if self.repeated {
            if self.type_is_not_trivial() {
                RustType::RepeatedField(c)
            } else {
                RustType::Vec(c)
            }
        } else {
            if self.field_type == FieldDescriptorProto_Type::TYPE_MESSAGE {
                RustType::SingularPtrField(c)
            } else if self.field_type == FieldDescriptorProto_Type::TYPE_STRING ||
                    self.field_type == FieldDescriptorProto_Type::TYPE_BYTES
            {
                RustType::SingularField(c)
            } else {
                RustType::Option(c)
            }
        }
    }

    // type of `v` in `for v in field`
    fn full_storage_iter_elem_type(&self) -> RustType {
        self.full_storage_type().iter_elem_type()
    }

    // suffix `xxx` as in `os.write_xxx_no_tag(..)`
    fn os_write_fn_suffix(&self) -> &str {
        match self.field_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => "message",
            FieldDescriptorProto_Type::TYPE_ENUM    => "enum",
            ty => protobuf_name(ty),
        }
    }

    // type of `v` in `os.write_xxx_no_tag(v)`
    fn os_write_fn_param_type(&self) -> RustType {
        match self.field_type {
            FieldDescriptorProto_Type::TYPE_STRING =>
                RustType::Ref(Box::new(RustType::Str)),
            FieldDescriptorProto_Type::TYPE_BYTES  =>
                RustType::Ref(Box::new(RustType::Slice(Box::new(RustType::Unsigned(8))))),
            FieldDescriptorProto_Type::TYPE_ENUM   =>
                RustType::Signed(32),
            t => rust_name(t),
        }
    }

    // for field `foo`, type of param of `fn set_foo(..)`
    fn set_xxx_param_type(&self) -> RustType {
        if self.repeated {
            self.full_storage_type()
        } else {
            self.type_name.clone()
        }
    }

    // for field `foo`, return type if `fn take_foo(..)`
    fn take_xxx_return_type(&self) -> RustType {
        self.set_xxx_param_type()
    }

    // for field `foo`, return type of `fn mut_foo(..)`
    fn mut_xxx_return_type(&self) -> RustType {
        RustType::Ref(Box::new(if self.repeated {
            self.full_storage_type()
        } else {
            self.type_name.clone()
        }))
    }

    // for field `foo`, return type of `fn get_foo(..)`
    fn get_xxx_return_type(&self) -> RustType {
        match self.repeated {
            true => RustType::Ref(Box::new(RustType::Slice(Box::new(self.type_name.clone())))),
            false => match self.type_is_not_trivial() {
                true => self.type_name.ref_type(),
                false => self.type_name.clone(),
            }
        }
    }

    // suffix to convert field value to option
    // like `.as_ref()` in `self.xx.as_ref()`
    fn as_option(&self) -> &'static str {
        assert!(!self.repeated);
        match self.full_storage_type() {
            RustType::Option(..) => "",
            _                    => ".as_ref()"
        }
    }

    // type of expression returned by `as_option()`
    fn as_option_type(&self) -> RustType {
        assert!(!self.repeated);
        match self.full_storage_type() {
            r @ RustType::Option(..)       => r,
            RustType::SingularField(ty)    |
            RustType::SingularPtrField(ty) => RustType::Option(Box::new(RustType::Ref(ty))),
            x => panic!("cannot convert {:?} to option", x),
        }
    }

    // fixed size type?
    fn is_fixed(&self) -> bool {
        field_type_size(self.field_type).is_some()
    }

    // must use zigzag encoding?
    fn is_zigzag(&self) -> bool {
        match self.field_type {
            FieldDescriptorProto_Type::TYPE_SINT32 |
            FieldDescriptorProto_Type::TYPE_SINT64 => true,
            _ => false,
        }
    }

    // data is enum
    fn is_enum(&self) -> bool {
        match self.field_type {
            FieldDescriptorProto_Type::TYPE_ENUM => true,
            _ => false,
        }
    }

    // data is stored in heap
    fn type_is_not_trivial(&self) -> bool {
        match self.field_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE |
            FieldDescriptorProto_Type::TYPE_STRING |
            FieldDescriptorProto_Type::TYPE_BYTES => true,
            _ => false,
        }
    }

    fn default_value_from_proto(&self) -> Option<String> {
        assert!(!self.repeated);
        if self.enum_default_value.is_some() {
            Some(self.enum_default_value.as_ref().unwrap().rust_name_outer())
        } else if self.proto_field.has_default_value() {
            let proto_default = self.proto_field.get_default_value();
            Some(match self.field_type {
                // For numeric types, contains the original text representation of the value
                FieldDescriptorProto_Type::TYPE_DOUBLE   => format!("{}f64", proto_default),
                FieldDescriptorProto_Type::TYPE_FLOAT    => format!("{}f32", proto_default),
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
                    panic!("default value is not implemented for type: {:?}", self.field_type)
            })
        } else {
            None
        }
    }

    // default value to be returned from fn get_xxx
    fn get_xxx_default_value_rust(&self) -> String {
        assert!(!self.repeated);
        self.default_value_from_proto().unwrap_or_else(|| self.get_xxx_return_type().default_value())
    }

    // default to be assigned to field
    fn element_default_value_rust(&self) -> String {
        assert!(!self.repeated);
        self.default_value_from_proto().unwrap_or_else(|| self.type_name.default_value())
    }

    fn reconstruct_def(&self) -> String {
        let prefix = match self.proto_field.get_label() {
            FieldDescriptorProto_Label::LABEL_OPTIONAL => "optional",
            FieldDescriptorProto_Label::LABEL_REQUIRED => "required",
            FieldDescriptorProto_Label::LABEL_REPEATED => "repeated",
        };
        format!("{} {} {} = {}",
            prefix,
            field_type_protobuf_name(&self.proto_field),
            self.proto_field.get_name(),
            self.proto_field.get_number())
    }

    // name of function in protobuf::reflect::accessor
    // that generates accessor for this field
    fn make_accessor_fn(&self) -> String {
        let repeated_or_signular = match self.repeated {
            true  => "repeated",
            false => "singular",
        };
        let suffix = match &self.type_name {
            t if t.is_primitive()                     => format!("{:?}", t),
            &RustType::String                         => "string".to_string(),
            &RustType::Vec(ref t) if t.is_u8()        => "bytes".to_string(),
            &RustType::Enum(..)                       => "enum".to_string(),
            &RustType::Message(..)                    => "message".to_string(),
            t => panic!("unexpected field type: {:?}", t),
        };
        format!("make_{}_{}_accessor", repeated_or_signular, suffix)
    }

    // accessor function function params
    fn make_accessor_fn_fn_params(&self) -> Vec<&'static str> {
        let mut r = Vec::new();
        if !self.repeated {
            r.push("has");
        }
        r.push("get");
        r
    }

    fn write_clear(&self, w: &mut IndentWriter) {
        if self.is_oneof() {
            w.write_line(format!("self.{} = ::std::option::Option::None;", self.oneof.as_ref().unwrap().name));
        } else {
            let clear_expr = self.full_storage_type().clear(&w.self_field());
            w.write_line(format!("{};", clear_expr));
        }
    }

    fn write_element_size(&self, w: &mut IndentWriter, item_var: &str, item_var_type: &RustType, sum_var: &str) {
        assert!(self.repeat_mode != RepeatMode::RepeatPacked);
        match self.field_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                w.write_line(format!("let len = {}.compute_size();", item_var));
                let tag_size = self.tag_size();
                w.write_line(format!(
                        "{} += {} + ::protobuf::rt::compute_raw_varint32_size(len) + len;",
                        sum_var, tag_size));
            },
            _ => {
                w.write_line(format!(
                        "{} += {};", sum_var, self.element_size(item_var, item_var_type)));
            },
        }
    }

    // expression that returns size of data is variable
    fn element_size(&self, var: &str, var_type: &RustType) -> String {
        assert!(self.repeat_mode != RepeatMode::RepeatPacked);
        match field_type_size(self.field_type) {
            Some(data_size) => {
                format!("{}", data_size + self.tag_size())
            },
            None => {
                match self.field_type {
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
                        format!("::protobuf::rt::value_size({}, {}, ::protobuf::wire_format::{:?})",
                                self.number, var_type.into_target(&param_type, var), self.wire_type)
                    }
                }
            },
        }
    }

    // output code that writes single element to stream
    fn write_write_element(&self, w: &mut IndentWriter, os: &str, var: &str, ty: &RustType) {
        assert!(self.repeat_mode != RepeatMode::RepeatPacked);
        match self.field_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                w.write_line(format!("try!({}.write_tag({}, ::protobuf::wire_format::{:?}));",
                        os, self.number, wire_format::WireTypeLengthDelimited));
                w.write_line(format!("try!({}.write_raw_varint32({}.get_cached_size()));",
                        os, var));
                w.write_line(format!("try!({}.write_to_with_cached_sizes({}));",
                        var, os));
            }
            _ => {
                let param_type = self.os_write_fn_param_type();
                let os_write_fn_suffix = self.os_write_fn_suffix();
                let number = self.number();
                w.write_line(format!("try!({}.write_{}({}, {}));",
                    os,
                    os_write_fn_suffix,
                    number,
                    ty.into_target(&param_type, var)));
            }
        }
    }

}


#[derive(Clone)]
struct OneofVariantContext<'a> {
    oneof: &'a OneofContext<'a>,
    variant: OneofVariantWithContext<'a>,
    field: Field,
    path: String,
}

impl<'a> OneofVariantContext<'a> {
    fn parse(oneof: &'a OneofContext<'a>, variant: OneofVariantWithContext<'a>, field: &'a Field) -> OneofVariantContext<'a> {
        OneofVariantContext {
            oneof: oneof,
            variant: variant,
            field: field.clone(),
            path: format!("{:?}::{}", oneof.type_name, field.name),
        }
    }

    fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Clone)]
struct OneofContext<'a> {
    message: &'a MessageContext<'a>,
    oneof: OneofWithContext<'a>,
    type_name: RustType,
}

impl<'a> OneofContext<'a> {
    fn parse(message: &'a MessageContext, oneof: OneofWithContext<'a>) -> OneofContext<'a> {
        let rust_name = oneof.rust_name();
        OneofContext {
            message: message,
            oneof: oneof,
            type_name: RustType::Oneof(rust_name),
        }
    }

    fn name(&self) -> &str {
        self.oneof.oneof.get_name()
    }

    fn variants(&'a self) -> Vec<OneofVariantContext<'a>> {
        self.oneof.variants().into_iter()
            .map(|v| {
                let field = self.message.fields.iter()
                    .filter(|f| f.name == v.field_name())
                    .next()
                    .unwrap();
                OneofVariantContext::parse(self, v, field)
            })
            .collect()
    }

    fn full_storage_type(&self) -> RustType {
        RustType::Option(Box::new(self.type_name.clone()))
    }
}



struct IndentWriter<'a> {
    writer: &'a mut (Write + 'a),
    indent: String,
    field: Option<&'a Field>,
}

impl<'a> IndentWriter<'a> {
    fn new(writer: &'a mut Write) -> IndentWriter<'a> {
        IndentWriter {
            writer: writer,
            indent: "".to_string(),
            field: None,
        }
    }

    fn bind_field<T, F>(&mut self, field: &Field, cb: F) -> T
        where F : Fn(&mut IndentWriter) -> T
    {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: self.indent.to_string(),
            field: Some(field),
        })
    }

    fn field(&self) -> &'a Field {
        assert!(self.field.is_some());
        self.field.unwrap()
    }

    fn self_field(&self) -> String {
        format!("self.{}", self.field().name)
    }

    fn self_field_is_some(&self) -> String {
        assert!(!self.field().repeated);
        format!("{}.is_some()", self.self_field())
    }

    fn self_field_is_not_empty(&self) -> String {
        assert!(self.field().repeated);
        format!("!{}.is_empty()", self.self_field())
    }

    fn self_field_is_none(&self) -> String {
        assert!(!self.field().repeated);
        format!("{}.is_none()", self.self_field())
    }

    // field data viewed as Option
    fn self_field_as_option(&self) -> String {
        format!("{}{}", self.self_field(), self.field().as_option())
    }

    fn if_self_field_is_some<F>(&mut self, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        let self_field_is_some = self.self_field_is_some();
        self.if_stmt(self_field_is_some, cb);
    }

    fn if_self_field_is_not_empty<F>(&mut self, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        let self_field_is_not_empty = self.self_field_is_not_empty();
        self.if_stmt(self_field_is_not_empty, cb);
    }

    fn if_self_field_is_none<F>(&mut self, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        let self_field_is_none = self.self_field_is_none();
        self.if_stmt(self_field_is_none, cb)
    }

    fn for_self_field<F>(&mut self, varn: &str, cb: F)
        where F : Fn(&mut IndentWriter, &RustType)
    {
        let v_type = self.field().full_storage_iter_elem_type();
        let self_field = self.self_field();
        self.for_stmt(format!("{}.iter()", self_field), varn, |w| cb(w, &v_type));
    }

    fn self_field_assign<S : AsRef<str>>(&mut self, value: S) {
        let self_field = self.self_field();
        self.write_line(format!("{} = {};", self_field, value.as_ref()));
    }

    fn self_field_assign_some<S : AsRef<str>>(&mut self, value: S) {
        assert!(!self.field().repeated);
        let full_storage_type = self.field().full_storage_type();
        self.self_field_assign(full_storage_type.wrap_value(value.as_ref()));
    }

    fn self_field_assign_default(&mut self) {
        let field = self.field();
        assert!(!field.repeated);
        if field.is_oneof() {
            let self_field_oneof = self.self_field_oneof();
            self.write_line(
                format!("{} = ::std::option::Option::Some({}({}))",
                self_field_oneof,
                field.variant_path(),
                // TODO: default from .proto is not needed here
                field.element_default_value_rust()));
        } else {
            if field.type_is_not_trivial() {
                let self_field = self.self_field();
                self.write_line(format!("{}.set_default();", self_field));
            } else {
                self.self_field_assign_some(field.element_default_value_rust());
            }
        }
    }

    fn self_field_assign_value<S : AsRef<str>>(&mut self, value: S, ty: &RustType) {
        if self.field().repeated {
            let converted = ty.into_target(&self.field().full_storage_type(), value.as_ref());
            self.self_field_assign(converted);
        } else {
            let converted = ty.into_target(&self.field().type_name, value.as_ref());
            let wrapped = self.field().full_storage_type().wrap_value(&converted);
            self.self_field_assign(wrapped);
        }
    }

    fn self_field_vec_packed_fixed_data_size(&self) -> String {
        assert!(self.field().is_fixed());
        format!("({}.len() * {}) as u32",
            self.self_field(), field_type_size(self.field().field_type).unwrap())
    }

    fn self_field_vec_packed_varint_data_size(&self) -> String {
        assert!(!self.field().is_fixed());
        let fn_name = if self.field().is_enum() {
            "vec_packed_enum_data_size".to_string()
        } else {
            let zigzag_suffix = if self.field().is_zigzag() { "_zigzag" } else { "" };
            format!("vec_packed_varint{}_data_size", zigzag_suffix)
        };
        format!("::protobuf::rt::{}(&{})",
            fn_name, self.self_field())
    }

    fn self_field_vec_packed_data_size(&self) -> String {
        assert!(self.field().repeated);
        if self.field().is_fixed() {
            self.self_field_vec_packed_fixed_data_size()
        } else {
            self.self_field_vec_packed_varint_data_size()
        }
    }

    fn self_field_vec_packed_fixed_size(&self) -> String {
        // zero is filtered outside
        format!("{} + ::protobuf::rt::compute_raw_varint32_size({}.len() as u32) + {}",
            self.field().tag_size(),
            self.self_field(),
            self.self_field_vec_packed_fixed_data_size())
    }

    fn self_field_vec_packed_varint_size(&self) -> String {
        // zero is filtered outside
        assert!(!self.field().is_fixed());
        let fn_name = if self.field().is_enum() {
            "vec_packed_enum_size".to_string()
        } else {
            let zigzag_suffix = if self.field().is_zigzag() { "_zigzag" } else { "" };
            format!("vec_packed_varint{}_size", zigzag_suffix)
        };
        format!("::protobuf::rt::{}({}, &{})",
            fn_name, self.field().number, self.self_field())
    }

    fn self_field_vec_packed_size(&mut self) -> String {
        assert!(self.field.unwrap().packed);
        // zero is filtered outside
        if self.field.unwrap().is_fixed() {
            self.self_field_vec_packed_fixed_size()
        } else {
            self.self_field_vec_packed_varint_size()
        }
    }

    fn self_field_oneof(&self) -> String {
        format!("self.{}", self.field().oneof.as_ref().unwrap().name)
    }

    fn write_line<S : AsRef<str>>(&mut self, line: S) {
        (if line.as_ref().is_empty() {
            self.writer.write_all("\n".as_bytes())
        } else {
            let s: String = [self.indent.as_ref(), line.as_ref(), "\n"].concat();
            self.writer.write_all(s.as_bytes())
        }).unwrap();
    }

    fn todo(&mut self, message: &str) {
        self.write_line(format!("panic!(\"TODO: {}\");", message));
    }

    fn indented<F>(&mut self, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: format!("{}    ", self.indent),
            field: self.field,
        });
    }

    #[allow(dead_code)]
    fn commented<F>(&mut self, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: format!("// {}", self.indent),
            field: self.field,
        });
    }

    fn lazy_static<S1 : AsRef<str>, S2 : AsRef<str>>(&mut self, name: S1, ty: S2) {
        self.stmt_block(format!("static mut {}: ::protobuf::lazy::Lazy<{}> = ::protobuf::lazy::Lazy", name.as_ref(), ty.as_ref()), |w| {
            w.field_entry("lock", "::protobuf::lazy::ONCE_INIT");
            w.field_entry("ptr", format!("0 as *const {}", ty.as_ref()));
        });
    }

    fn lazy_static_decl_get<S1 : AsRef<str>, S2 : AsRef<str>, F>(&mut self, name: S1, ty: S2, init: F)
        where F : Fn(&mut IndentWriter)
    {
        self.lazy_static(name.as_ref(), ty);
        self.unsafe_expr(|w| {
            w.write_line(format!("{}.get(|| {{", name.as_ref()));
            w.indented(|w| init(w));
            w.write_line(format!("}})"));
        });
    }

    fn block<S1 : AsRef<str>, S2 : AsRef<str>, F>(&mut self, first_line: S1, last_line: S2, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.write_line(first_line.as_ref());
        self.indented(cb);
        self.write_line(last_line.as_ref());
    }

    fn expr_block<S : AsRef<str>, F>(&mut self, prefix: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.block(format!("{} {{", prefix.as_ref()), "}", cb);
    }

    fn stmt_block<S : AsRef<str>, F>(&mut self, prefix: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.block(format!("{} {{", prefix.as_ref()), "};", cb);
    }

    fn unsafe_expr<F>(&mut self, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.expr_block("unsafe", cb);
    }

    fn impl_self_block<S : AsRef<str>, F>(&mut self, name: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.expr_block(format!("impl {}", name.as_ref()), cb);
    }

    fn impl_for_block<S1 : AsRef<str>, S2 : AsRef<str>, F>(&mut self, tr: S1, ty: S2, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.expr_block(format!("impl {} for {}", tr.as_ref(), ty.as_ref()), cb);
    }

    fn pub_struct<S : AsRef<str>, F>(&mut self, name: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.expr_block(format!("pub struct {}", name.as_ref()), cb);
    }

    fn pub_enum<F>(&mut self, name: &str, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.expr_block(format!("pub enum {}", name), cb);
    }

    fn field_entry<S1 : AsRef<str>, S2 : AsRef<str>>(&mut self, name: S1, value: S2) {
        self.write_line(format!("{}: {},", name.as_ref(), value.as_ref()));
    }

    fn field_decl<S : AsRef<str>>(&mut self, name: S, field_type: &RustType) {
        self.field_entry(name, format!("{:?}", field_type));
    }

    fn derive(&mut self, derive: &[&str]) {
        let v: Vec<String> = derive.iter().map(|&s| s.to_string()).collect();
        self.write_line(format!("#[derive({})]", v.connect(",")));
    }

    fn allow(&mut self, what: &[&str]) {
        let v: Vec<String> = what.iter().map(|&s| s.to_string()).collect();
        self.write_line(format!("#[allow({})]", v.connect(",")));
    }

    fn comment(&mut self, comment: &str) {
        if comment.is_empty() {
            self.write_line("//");
        } else {
            self.write_line(format!("// {}", comment));
        }
    }

    fn pub_fn<S : AsRef<str>, F>(&mut self, sig: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.expr_block(format!("pub fn {}", sig.as_ref()), cb);
    }

    fn def_fn<S : AsRef<str>, F>(&mut self, sig: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.expr_block(format!("fn {}", sig.as_ref()), cb);
    }

    fn while_block<S : AsRef<str>, F>(&mut self, cond: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.expr_block(format!("while {}", cond.as_ref()), cb);
    }

    // if ... { ... }
    fn if_stmt<S : AsRef<str>, F>(&mut self, cond: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.stmt_block(format!("if {}", cond.as_ref()), cb);
    }

    // if ... {} else { ... }
    fn if_else_stmt<S : AsRef<str>, F>(&mut self, cond: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.write_line(format!("if {} {{", cond.as_ref()));
        self.write_line("} else {");
        self.indented(cb);
        self.write_line("}");
    }

    // if let ... = ... { ... }
    fn if_let_stmt<F>(&mut self, decl: &str, expr: &str, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.if_stmt(format!("let {} = {}", decl, expr), cb);
    }

    // if let ... = ... { } else { ... }
    fn if_let_else_stmt<F>(&mut self, decl: &str, expr: &str, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.if_else_stmt(format!("let {} = {}", decl, expr), cb);
    }

    fn for_stmt<S1 : AsRef<str>, S2 : AsRef<str>, F>(&mut self, over: S1, varn: S2, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.stmt_block(format!("for {} in {}", varn.as_ref(), over.as_ref()), cb)
    }

    fn match_block<S : AsRef<str>, F>(&mut self, value: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.stmt_block(format!("match {}", value.as_ref()), cb);
    }

    fn match_expr<S : AsRef<str>, F>(&mut self, value: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.expr_block(format!("match {}", value.as_ref()), cb);
    }

    fn case_block<S : AsRef<str>, F>(&mut self, cond: S, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        self.block(format!("{} => {{", cond.as_ref()), "},", cb);
    }

    fn case_expr<S1 : AsRef<str>, S2 : AsRef<str>>(&mut self, cond: S1, body: S2) {
        self.write_line(format!("{} => {},", cond.as_ref(), body.as_ref()));
    }

    fn clear_field_func(&mut self) -> String {
        let mut r = "clear_".to_string();
        r.push_str(&self.field.as_ref().unwrap().name);
        r
    }

    fn error_wire_type(&mut self, _wire_type: wire_format::WireType) {
        // TODO: write wire type
        let message = "\"unexpected wire type\".to_string()";
        self.write_line(format!(
                "return ::std::result::Result::Err(::protobuf::ProtobufError::WireError({}));",
                message));
    }

    fn assert_wire_type(&mut self, wire_type: wire_format::WireType) {
        self.if_stmt(format!("wire_type != ::protobuf::wire_format::{:?}", wire_type), |w| {
            w.error_wire_type(wire_type);
        });
    }
}

fn write_merge_from_field_message_string_bytes(w: &mut IndentWriter) {
    let field = w.field();
    if field.repeated {
        w.write_line(format!(
            "try!(::protobuf::rt::read_repeated_{}_into(wire_type, is, &mut self.{}));",
                protobuf_name(field.field_type),
                field.name));
    } else {
        w.assert_wire_type(wire_format::WireTypeLengthDelimited);
        let self_field = w.self_field();
        w.write_line(format!("let tmp = {}.set_default();", self_field));
        w.write_line(format!("try!({})", field.field_type.merge("is", "tmp")));
    }
}

fn write_merge_from_oneof(field: &Field, w: &mut IndentWriter) {
    w.assert_wire_type(field.wire_type);
    // TODO: split long line
    w.write_line(format!("self.{} = ::std::option::Option::Some({}(try!({})));",
        field.oneof.as_ref().unwrap().name,
        field.variant_path(),
        field.field_type.read("is")));
}

fn write_merge_from_field(field: &Field, w: &mut IndentWriter) {
    if field.is_oneof() {
        write_merge_from_oneof(field, w);
    } else if field.type_is_not_trivial() {
        write_merge_from_field_message_string_bytes(w);
    } else {
        if field.is_oneof() {
            w.todo("oneof");
            return;
        }
        let wire_type = field_type_wire_type(field.field_type);
        let read_proc = format!("try!(is.read_{}())", protobuf_name(field.field_type));

        match field.repeated {
            false => {
                w.assert_wire_type(wire_type);
                w.write_line(format!("let tmp = {};", read_proc));
                w.self_field_assign_some("tmp");
            },
            true => {
                w.write_line(format!(
                    "try!(::protobuf::rt::read_repeated_{}_into(wire_type, is, &mut self.{}));",
                        protobuf_name(field.field_type),
                        field.name));
            },
        };
    }
}

fn write_message_write_field(w: &mut IndentWriter) {
    match w.field().repeat_mode {
        RepeatMode::Single => {
            let self_field_as_option = w.self_field_as_option();
            w.if_let_stmt("Some(v)", &self_field_as_option, |w| {
                let option_type = w.field().as_option_type();
                let v_type = option_type.elem_type();
                w.field().write_write_element(w, "os", "v", &v_type);
            });
        },
        RepeatMode::RepeatPacked => {
            w.if_self_field_is_not_empty(|w| {
                let number = w.field().number();
                w.write_line(format!("try!(os.write_tag({}, ::protobuf::wire_format::{:?}));", number, wire_format::WireTypeLengthDelimited));
                w.comment("TODO: Data size is computed again, it should be cached");
                let data_size_expr = w.self_field_vec_packed_data_size();
                w.write_line(format!("try!(os.write_raw_varint32({}));", data_size_expr));
                w.for_self_field("v", |w, v_type| {
                    let param_type = w.field().os_write_fn_param_type();
                    let os_write_fn_suffix = w.field().os_write_fn_suffix();
                    w.write_line(format!("try!(os.write_{}_no_tag({}));",
                        os_write_fn_suffix, v_type.into_target(&param_type, "v")));
                });
            });
        },
        RepeatMode::RepeatRegular => {
            w.for_self_field("v", |w, v_type| {
                w.field().write_write_element(w, "os", "v", v_type);
            });
        },
    };
}

fn write_message_field_get(w: &mut IndentWriter) {
    let field = w.field();

    let get_xxx_return_type = w.field().get_xxx_return_type();
    let self_param = match get_xxx_return_type.is_ref() {
        true  => "&'a self",
        false => "&self",
    };
    let get_xxx_return_type_str = get_xxx_return_type.ref_str_safe("a");
    // TODO: 'a is not needed when function does not return a reference
    let ref name = field.name;
    w.pub_fn(format!("get_{}<'a>({}) -> {}", name, self_param, get_xxx_return_type_str),
    |w| {
        if field.is_oneof() {
            let self_field_oneof = w.self_field_oneof();
            w.match_expr(self_field_oneof, |w| {
                let (refv, vtype) =
                    if field.type_is_not_trivial() {
                        ("ref v", field.type_name.ref_type())
                    } else {
                        ("v", field.type_name.clone())
                    };
                w.case_expr(format!(
                        "::std::option::Option::Some({}({}))",
                        field.variant_path(),
                        refv),
                    vtype.into_target(&get_xxx_return_type, "v"));
                w.case_expr("_", field.get_xxx_default_value_rust());
            });
        } else if !w.field().repeated {
            if w.field().field_type == FieldDescriptorProto_Type::TYPE_MESSAGE {
                let self_field = w.self_field();
                let ref field_type_name = w.field().type_name;
                w.write_line(format!("{}.as_ref().unwrap_or_else(|| {:?}::default_instance())",
                        self_field, field_type_name));
            } else {
                if get_xxx_return_type.is_ref() {
                    let self_field_as_option = w.self_field_as_option();
                    w.match_expr(self_field_as_option, |w| {
                        let option_type = w.field().as_option_type();
                        let v_type = option_type.elem_type();
                        let r_type = w.field().get_xxx_return_type();
                        w.case_expr(
                            "Some(v)",
                            v_type.into_target(&r_type, "v")
                        );
                        let get_xxx_default_value_rust = w.field().get_xxx_default_value_rust();
                        w.case_expr(
                            "None",
                            get_xxx_default_value_rust
                        );
                    });
                } else {
                    assert!(!w.field().type_is_not_trivial());
                    let get_xxx_default_value_rust = w.field().get_xxx_default_value_rust();
                    let self_field = w.self_field();
                    w.write_line(format!(
                            "{}.unwrap_or({})", self_field, get_xxx_default_value_rust));
                }
            }
        } else {
            let self_field = w.self_field();
            w.write_line(format!("&{}", self_field));
        }
    });
}

fn write_message_field_has(w: &mut IndentWriter) {
    let field = w.field();
    let ref name = w.field().name;
    w.pub_fn(format!("has_{}(&self) -> bool", name), |w| {
        if !field.is_oneof() {
            let self_field_is_some = w.self_field_is_some();
            w.write_line(self_field_is_some);
        } else {
            let self_field_oneof = w.self_field_oneof();
            w.match_expr(self_field_oneof, |w| {
                w.case_expr(format!(
                        "::std::option::Option::Some({}(..))",
                        field.variant_path()),
                    "true");
                w.case_expr("_", "false");
            });
        }
    });
}

fn write_message_field_set(w: &mut IndentWriter) {
    let field = w.field();
    let set_xxx_param_type = w.field().set_xxx_param_type();
    w.comment("Param is passed by value, moved");
    let ref name = w.field().name;
    w.pub_fn(format!("set_{}(&mut self, v: {:?})", name, set_xxx_param_type), |w| {
        if !field.is_oneof() {
            w.self_field_assign_value("v", &set_xxx_param_type);
        } else {
            let self_field_oneof = w.self_field_oneof();
            w.write_line(format!("{} = ::std::option::Option::Some({}(v))",
                self_field_oneof, field.variant_path()));
        }
    });
}

fn write_message_field_mut_take(w: &mut IndentWriter) {
    let field = w.field();
    let mut_xxx_return_type = field.mut_xxx_return_type();
    w.comment("Mutable pointer to the field.");
    if !field.repeated {
        w.comment("If field is not initialized, it is initialized with default value first.");
    }
    // TODO: 'a is not needed when function does not return a reference
    w.pub_fn(format!("mut_{}<'a>(&'a mut self) -> {}", field.name, mut_xxx_return_type.mut_ref_str("a")),
    |w| {
        if field.is_oneof() {
            let self_field_oneof = w.self_field_oneof();

            // if oneof does not contain current field
            w.if_let_else_stmt(&format!(
                        "::std::option::Option::Some({}(_))",
                        field.variant_path())[..], &self_field_oneof[..],
            |w|
            {
                // initialize it with default value
                w.write_line(format!(
                    "{} = ::std::option::Option::Some({}({}));",
                    self_field_oneof,
                    field.variant_path(),
                    field.element_default_value_rust()));
            });

            // extract field
            w.match_expr(self_field_oneof, |w| {
                w.case_expr(format!(
                        "::std::option::Option::Some({}(ref mut v))",
                        field.variant_path()),
                    "v");
                w.case_expr("_", "panic!()");
            });
        } else if !field.repeated {
            w.if_self_field_is_none(|w| {
                w.self_field_assign_default();
            });
            let self_field = w.self_field();
            w.write_line(format!("{}.as_mut().unwrap()", self_field));
        } else {
            let self_field = w.self_field();
            w.write_line(format!("&mut {}", self_field));
        }
    });
    w.write_line("");
    w.comment("Take field");
    let take_xxx_return_type = field.take_xxx_return_type();
    w.pub_fn(format!("take_{}(&mut self) -> {:?}", field.name, take_xxx_return_type), |w| {
        if field.is_oneof() {
            // TODO: replace with if let
            w.write_line(format!("if self.has_{}() {{", field.name));
            w.indented(|w| {
                let self_field_oneof = w.self_field_oneof();
                w.match_expr(format!("{}.take()", self_field_oneof), |w| {
                    w.case_expr(format!("::std::option::Option::Some({}(v))", field.variant_path()), "v");
                    w.case_expr("_", "panic!()");
                });
            });
            w.write_line("} else {");
            w.indented(|w| {
                w.write_line(field.element_default_value_rust());
            });
            w.write_line("}");
        } else if !field.repeated {
            if w.field().type_is_not_trivial() {
                w.write_line(format!("self.{}.take().unwrap_or_else(|| {})",
                    field.name, field.type_name.default_value()));
            } else {
                w.write_line(format!("self.{}.take().unwrap_or({})",
                    field.name, field.element_default_value_rust()));
            }
        } else {
            w.write_line(format!("::std::mem::replace(&mut self.{}, {})",
                    field.name,
                    take_xxx_return_type.default_value()));
        }
    });
}

fn write_message_single_field_accessors(w: &mut IndentWriter) {
    let clear_field_func = w.clear_field_func();
    w.pub_fn(format!("{}(&mut self)", clear_field_func), |w| {
        w.field().write_clear(w);
    });

    if !w.field().repeated {
        w.write_line("");
        write_message_field_has(w);
    }

    w.write_line("");
    write_message_field_set(w);

    // mut_xxx() are pointless for primitive types
    if w.field().type_is_not_trivial() || w.field().repeated {
        w.write_line("");
        write_message_field_mut_take(w);
    }

    w.write_line("");
    write_message_field_get(w);
}

fn write_message_oneof(oneof: &OneofContext, w: &mut IndentWriter) {
    let mut derive = vec!["Clone", "PartialEq"];
    if false /* lite_runtime */ {
        derive.push("Debug");
    }
    w.derive(&derive);
    w.pub_enum(&format!("{:?}", oneof.type_name)[..], |w| {
        for variant in oneof.variants() {
            w.write_line(format!("{}({:?}),", variant.field.name, variant.field.type_name));
        }
    });
}

/// Message info for codegen
struct MessageContext<'a> {
    message: &'a MessageWithScope<'a>,
    root_scope: &'a RootScope<'a>,
    type_name: String,
    fields: Vec<Field>,
    lite_runtime: bool,
}

impl<'a> MessageContext<'a> {
    fn new(message: &'a MessageWithScope<'a>, root_scope: &'a RootScope<'a>)
        -> MessageContext<'a>
    {
        let fields: Vec<_> = message.fields().iter().map(|field| {
            Field::parse(field, root_scope, message.get_package())
        }).collect();
        MessageContext {
            message: message,
            root_scope: root_scope,
            type_name: message.rust_name(),
            fields: fields,
            lite_runtime:
                message.get_file_descriptor().get_options().get_optimize_for()
                    == FileOptions_OptimizeMode::LITE_RUNTIME,
        }
    }

    fn oneofs(&'a self) -> Vec<OneofContext<'a>> {
        self.message.oneofs().into_iter().map(|oneof| {
            OneofContext::parse(self, oneof)
        }).collect()
    }

    fn required_fields(&'a self) -> Vec<&'a Field> {
        let mut r = Vec::new();
        for field in self.fields.iter() {
            if field.proto_field.get_label() == FieldDescriptorProto_Label::LABEL_REQUIRED {
                r.push(field);
            }
        }
        r
    }

    fn fields_except_oneof(&'a self) -> Vec<&'a Field> {
        self.fields.iter().filter(|f| !f.is_oneof()).collect()
    }

    fn each_field<F>(&self, w: &mut IndentWriter, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        for field in &self.fields {
            w.bind_field(field, |w| cb(w));
        }
    }

    fn each_field_except_oneof<F>(&self, w: &mut IndentWriter, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        for field in self.fields_except_oneof() {
            w.bind_field(field, |w| cb(w));
        }
    }

    fn each_required_field<F>(&self, w: &mut IndentWriter, cb: F)
        where F : Fn(&mut IndentWriter)
    {
        for field in self.required_fields() {
            w.bind_field(&field, |w| cb(w));
        }
    }


    fn write_match_each_oneof_variant<F>(&self, w: &mut IndentWriter, cb: F)
        where F: Fn(&mut IndentWriter, &OneofVariantContext, &str, &RustType)
    {
        for oneof in self.oneofs() {
            w.if_let_stmt("::std::option::Option::Some(ref v)", &format!("self.{}", oneof.name())[..], |w| {
                w.match_block("v", |w| {
                    for variant in oneof.variants() {
                        let ref field = variant.field;
                        let (refv, vtype) =
                            if field.type_is_not_trivial() {
                                ("ref v", field.type_name.ref_type())
                            } else {
                                ("v", field.type_name.clone())
                            };
                        w.case_block(format!("&{}({})", variant.path(), refv), |w| {
                            cb(w, &variant, "v", &vtype);
                        });
                    }
                });
            });
        }
    }

    fn write_write_to_with_cached_sizes(&self, w: &mut IndentWriter) {
        w.def_fn("write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()>", |w| {
            // To have access to its methods but not polute the name space.
            self.each_field_except_oneof(w, |w| {
                write_message_write_field(w);
            });
            self.write_match_each_oneof_variant(w, |w, variant, v, v_type| {
                variant.field.write_write_element(w, "os", v, v_type);
            });
            w.write_line("try!(os.write_unknown_fields(self.get_unknown_fields()));");
            w.write_line("::std::result::Result::Ok(())");
        });
    }

    fn write_get_cached_size(&self, w: &mut IndentWriter) {
        w.def_fn("get_cached_size(&self) -> u32", |w| {
            w.write_line("self.cached_size.get()");
        });
    }

    fn write_default_instance(&self, w: &mut IndentWriter) {
        w.pub_fn(format!("default_instance() -> &'static {}", self.type_name), |w| {
            w.lazy_static_decl_get("instance", &self.type_name, |w| {
                w.expr_block(format!("{}", self.type_name), |w| {
                    for field in self.fields_except_oneof() {
                        let init = field.full_storage_type().default_value();
                        w.field_entry(field.name.to_string(), init);
                    }
                    for oneof in self.oneofs() {
                        let init = oneof.full_storage_type().default_value();
                        w.field_entry(oneof.name(), init);
                    }
                    w.field_entry("unknown_fields", "::protobuf::UnknownFields::new()");
                    w.field_entry("cached_size", "::std::cell::Cell::new(0)");
                });
            });
        });
    }

    fn write_compute_size(&self, w: &mut IndentWriter) {
        // Append sizes of messages in the tree to the specified vector.
        // First appended element is size of self, and then nested message sizes.
        // in serialization order are appended recursively.");
        w.comment("Compute sizes of nested messages");
        // there are unused variables in oneof
        w.allow(&["unused_variables"]);
        w.def_fn("compute_size(&self) -> u32", |w| {
            // To have access to its methods but not polute the name space.
            w.write_line("let mut my_size = 0;");
            self.each_field_except_oneof(w, |w| {
                let field = w.field();
                match field.repeat_mode {
                    RepeatMode::Single | RepeatMode::RepeatRegular => {
                        match field_type_size(field.field_type) {
                            Some(s) => {
                                if field.repeated {
                                    let tag_size = w.field().tag_size();
                                    let self_field = w.self_field();
                                    w.write_line(format!(
                                            "my_size += {} * {}.len() as u32;",
                                            (s + tag_size) as isize,
                                            self_field));
                                } else {
                                    w.if_self_field_is_some(|w| {
                                        let tag_size = w.field().tag_size();
                                        w.write_line(format!(
                                                "my_size += {};",
                                                (s + tag_size) as isize));
                                    });
                                }
                            },
                            None => {
                                w.for_self_field("value", |w, value_type| {
                                    field.write_element_size(w, "value", value_type, "my_size");
                                });
                            },
                        };
                    },
                    RepeatMode::RepeatPacked => {
                        w.if_self_field_is_not_empty(|w| {
                            let size_expr = w.self_field_vec_packed_size();
                            w.write_line(format!("my_size += {};", size_expr));
                        });
                    },
                };
            });
            self.write_match_each_oneof_variant(w, |w, variant, v, vtype| {
                variant.field.write_element_size(w, v, vtype, "my_size");
            });
            w.write_line("my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());");
            w.write_line("self.cached_size.set(my_size);");
            w.write_line("my_size");
        });
    }

    fn write_field_accessors(&self, w: &mut IndentWriter) {
        self.each_field(w, |w| {
            w.write_line("");
            let reconstruct_def = w.field().reconstruct_def();
            w.comment(&(reconstruct_def + ";"));
            w.write_line("");
            write_message_single_field_accessors(w);
        });
    }

    fn write_impl_self(&self, w: &mut IndentWriter) {
        w.impl_self_block(&self.type_name, |w| {
            w.pub_fn(format!("new() -> {}", self.type_name), |w| {
                w.write_line("::std::default::Default::default()");
            });

            w.write_line("");
            self.write_default_instance(w);
            self.write_field_accessors(w);
        });
    }

    fn write_unknown_fields(&self, w: &mut IndentWriter) {
        w.def_fn("get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields", |w| {
            w.write_line("&self.unknown_fields");
        });
        w.write_line("");
        w.def_fn("mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields", |w| {
            w.write_line("&mut self.unknown_fields");
        });
    }

    fn write_merge_from(&self, w: &mut IndentWriter) {
        w.def_fn(format!("merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()>"), |w| {
            w.while_block("!try!(is.eof())", |w| {
                w.write_line(format!("let (field_number, wire_type) = try!(is.read_tag_unpack());"));
                w.match_block("field_number", |w| {
                    self.each_field(w, |w| {
                        let number = w.field().number;
                        w.case_block(number.to_string(), |w| {
                            write_merge_from_field(w.field(), w);
                        });
                    });
                    w.case_block("_", |w| {
                        w.write_line("let unknown = try!(is.read_unknown(wire_type));");
                        w.write_line("self.mut_unknown_fields().add_value(field_number, unknown);");
                    });
                });
            });
            w.write_line("::std::result::Result::Ok(())");
        });
    }

    fn write_descriptor_static(&self, w: &mut IndentWriter) {
        w.def_fn(format!("descriptor_static(_: ::std::option::Option<{}>) -> &'static ::protobuf::reflect::MessageDescriptor", self.type_name), |w| {
            w.lazy_static_decl_get("descriptor", "::protobuf::reflect::MessageDescriptor", |w| {
                if self.fields.is_empty() {
                    w.write_line(format!("let fields = ::std::vec::Vec::new();"));
                } else {
                    w.write_line(format!("let mut fields = ::std::vec::Vec::new();"));
                }
                for field in self.fields.iter() {
                    w.write_line(format!("fields.push(::protobuf::reflect::accessor::{}(", field.make_accessor_fn()));
                    w.indented(|w| {
                        w.write_line(format!("\"{}\",", field.name));
                        for f in field.make_accessor_fn_fn_params().iter() {
                            w.write_line(format!("{}::{}_{},",
                                    self.type_name,
                                    f,
                                    field.name,
                                ));
                        }
                    });
                    w.write_line("));");
                }
                w.write_line(format!(
                    "::protobuf::reflect::MessageDescriptor::new::<{}>(", self.type_name));
                w.indented(|w| {
                    w.write_line(format!("\"{}\",", self.type_name));
                    w.write_line("fields,");
                    w.write_line("file_descriptor_proto()");
                });
                w.write_line(")");
            });
        });
    }

    fn write_impl_message(&self, w: &mut IndentWriter) {
        w.impl_for_block("::protobuf::Message", &self.type_name, |w| {
            w.def_fn(format!("is_initialized(&self) -> bool"), |w| {
                self.each_required_field(w, |w| {
                    w.if_self_field_is_none(|w| {
                        w.write_line("return false;");
                    });
                });
                w.write_line("true");
            });
            w.write_line("");
            self.write_merge_from(w);
            w.write_line("");
            self.write_compute_size(w);
            w.write_line("");
            self.write_write_to_with_cached_sizes(w);
            w.write_line("");
            self.write_get_cached_size(w);
            w.write_line("");
            self.write_unknown_fields(w);
            w.write_line("");
            w.def_fn("type_id(&self) -> ::std::any::TypeId", |w| {
                w.write_line(format!("::std::any::TypeId::of::<{}>()", self.type_name));
            });
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

    fn write_impl_message_static(&self, w: &mut IndentWriter) {
        w.impl_for_block("::protobuf::MessageStatic", &self.type_name, |w| {
            w.def_fn(format!("new() -> {}", self.type_name), |w| {
                w.write_line(format!("{}::new()", self.type_name));
            });
            if !self.lite_runtime {
                w.write_line("");
                self.write_descriptor_static(w);
            }
        });
    }

    fn write_impl_show(&self, w: &mut IndentWriter) {
        w.impl_for_block("::std::fmt::Debug", &self.type_name, |w| {
            w.def_fn("fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result", |w| {
                w.write_line("::protobuf::text_format::fmt(self, f)");
            });
        });
    }

    fn write_impl_clear(&self, w: &mut IndentWriter) {
        w.impl_for_block("::protobuf::Clear", &self.type_name, |w| {
            w.def_fn("clear(&mut self)", |w| {
                // TODO: no need to clear oneof fields in loop
                self.each_field(w, |w| {
                    let clear_field_func = w.clear_field_func();
                    w.write_line(format!("self.{}();", clear_field_func));
                });
                w.write_line("self.unknown_fields.clear();");
            });
        });
    }

    // cannot use `#[derive(PartialEq)]` because of `cached_size` field
    fn write_impl_partial_eq(&self, w: &mut IndentWriter) {
        w.impl_for_block("::std::cmp::PartialEq", &self.type_name, |w| {
            w.def_fn(format!("eq(&self, other: &{}) -> bool", self.type_name), |w| {
                self.each_field_except_oneof(w, |w| {
                    let ref field_name = w.field().name;
                    w.write_line(format!("self.{field} == other.{field} &&", field=field_name));
                });
                for oneof in self.oneofs() {
                    w.write_line(format!("self.{oneof} == other.{oneof} &&", oneof=oneof.name()));
                }
                w.write_line("self.unknown_fields == other.unknown_fields");
            });
        });
    }

    fn write_struct(&self, w: &mut IndentWriter) {
        let mut derive = vec!["Clone", "Default"];
        if self.lite_runtime {
            derive.push("Debug");
        }
        w.derive(&derive);
        w.pub_struct(&self.type_name, |w| {
            if !self.fields.is_empty() {
                w.comment("message fields");
                for field in self.fields_except_oneof() {
                    w.field_decl(&field.name[..], &field.full_storage_type());
                }
            }
            if !self.oneofs().is_empty() {
                w.comment("message oneof groups");
                for oneof in self.oneofs() {
                    w.field_decl(oneof.name(), &oneof.full_storage_type());
                }
            }
            w.comment("special fields");
            w.field_entry("unknown_fields", "::protobuf::UnknownFields");
            w.field_entry("cached_size", "::std::cell::Cell<u32>");
        });
    }

    fn write(&self, w: &mut IndentWriter) {
        self.write_struct(w);
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
        w.write_line("");
        self.write_impl_partial_eq(w);
        if !self.lite_runtime {
            w.write_line("");
            self.write_impl_show(w);
        }

        let mut nested_prefix = self.type_name.to_string();
        nested_prefix.push_str("_");

        for nested in self.message.to_scope().get_messages().iter() {
            w.write_line("");
            MessageContext::new(nested, self.root_scope).write(w);
        }

        for enum_type in self.message.to_scope().get_enums().iter() {
            w.write_line("");
            EnumContext::new(enum_type, self.root_scope).write(w);
        }
    }
}


#[derive(Clone)]
struct EnumValue {
    proto: EnumValueDescriptorProto,
    prefix: String,
    enum_rust_name: String,
}

impl EnumValue {
    fn parse(proto: &EnumValueDescriptorProto, prefix: &str, enum_rust_name: &str) -> EnumValue {
        EnumValue {
            proto: proto.clone(),
            prefix: prefix.to_string(),
            enum_rust_name: enum_rust_name.to_string(),
        }
    }

    // value name
    fn name<'a>(&'a self) -> &'a str {
        self.proto.get_name()
    }

    // enum value
    fn number(&self) -> i32 {
        self.proto.get_number()
    }

    fn rust_name_inner(&self) -> String {
        self.name().to_string()
    }

    fn rust_name_outer(&self) -> String {
        let mut r = String::new();
        r.push_str(&self.enum_rust_name);
        r.push_str("::");
        r.push_str(&self.rust_name_inner());
        r
    }
}


struct EnumContext<'a> {
    enum_with_scope: &'a EnumWithScope<'a>,
    type_name: String,
    lite_runtime: bool,
}

impl<'a> EnumContext<'a> {
    fn new(enum_with_scope: &'a EnumWithScope<'a>, _root_scope: &RootScope) -> EnumContext<'a> {
        let rust_name = enum_with_scope.rust_name();
        EnumContext {
            enum_with_scope: enum_with_scope,
            type_name: rust_name.clone(),
            lite_runtime:
                enum_with_scope.get_scope().get_file_descriptor().get_options().get_optimize_for()
                    == FileOptions_OptimizeMode::LITE_RUNTIME,
        }
    }

    fn values(&self) -> Vec<EnumValue> {
        let mut used = HashSet::new();
        let mut r = Vec::new();
        for p in self.enum_with_scope.values() {
            // skipping non-unique enums
            // TODO: should support it
            if !used.insert(p.get_number()) {
                continue;
            }
            r.push(
                EnumValue::parse(
                    p,
                    &self.enum_with_scope.scope.rust_prefix(),
                    &self.type_name)
            );
        }
        r
    }

    // find enum value by name
    fn value_by_name(&'a self, name: &str) -> EnumValue {
        self.values().into_iter().find(|v| v.name() == name).unwrap()
    }

    fn write(&self, w: &mut IndentWriter) {
        self.write_struct(w);
        w.write_line("");
        self.write_impl_enum(w);
        w.write_line("");
        self.write_impl_copy(w);
    }

    fn write_struct(&self, w: &mut IndentWriter) {
        w.derive(&["Clone", "PartialEq", "Eq", "Debug", "Hash"]);
        let ref type_name = self.type_name;
        w.expr_block(format!("pub enum {}", type_name), |w| {
            for value in self.values().iter() {
                w.write_line(format!("{} = {},", value.rust_name_inner(), value.number()));
            }
        });
    }

    fn write_impl_enum(&self, w: &mut IndentWriter) {
        let ref type_name = self.type_name;
        w.impl_for_block("::protobuf::ProtobufEnum", &type_name, |w| {
            w.def_fn("value(&self) -> i32", |w| {
                w.write_line("*self as i32")
            });
            w.write_line("");
            let ref type_name = self.type_name;
            w.def_fn(format!("from_i32(value: i32) -> ::std::option::Option<{}>", type_name), |w| {
                w.match_expr("value", |w| {
                    let values = self.values();
                    for value in values.iter() {
                        w.write_line(format!("{} => ::std::option::Option::Some({}),",
                            value.number(), value.rust_name_outer()));
                    }
                    w.write_line(format!("_ => ::std::option::Option::None"));
                });
            });
            if !self.lite_runtime {
                w.write_line("");
                let ref type_name = self.type_name;
                w.def_fn(format!("enum_descriptor_static(_: Option<{}>) -> &'static ::protobuf::reflect::EnumDescriptor", type_name), |w| {
                    w.lazy_static_decl_get("descriptor", "::protobuf::reflect::EnumDescriptor", |w| {
                        let ref type_name = self.type_name;
                        w.write_line(format!("::protobuf::reflect::EnumDescriptor::new(\"{}\", file_descriptor_proto())", type_name));
                    });
                });
            }
        });
    }

    fn write_impl_copy(&self, w: &mut IndentWriter) {
        let ref type_name = self.type_name;
        w.impl_for_block("::std::marker::Copy", &type_name, |_w| {
        });
    }

}

// Copy-pasted from libsyntax.
fn ident_start(c: char) -> bool {
    (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || c == '_'
}

// Copy-pasted from libsyntax.
fn ident_continue(c: char) -> bool {
    (c >= 'a' && c <= 'z')
        || (c >= 'A' && c <= 'Z')
        || (c >= '0' && c <= '9')
        || c == '_'
}

fn proto_path_to_rust_base(path: &str) -> String {
    let without_dir = remove_to(path, '/');
    let without_suffix = remove_suffix(without_dir, ".proto");
    without_suffix.chars().enumerate().map(|(i, c)| {
        let valid = if i == 0 { ident_start(c) } else { ident_continue(c) };
        if valid { c } else { '_' }
    }).collect()
}


fn write_file_descriptor_data(file: &FileDescriptorProto, w: &mut IndentWriter) {
    let fdp_bytes = file.write_to_bytes().unwrap();
    w.write_line("static file_descriptor_proto_data: &'static [u8] = &[");
    for groups in fdp_bytes.iter().paginate(16) {
        let fdp_bytes_str = groups.iter()
                .map(|&b| format!("0x{:02x}", *b))
                .collect::<Vec<String>>()
                .connect(", ");
        w.write_line(format!("    {},", fdp_bytes_str));
    }
    w.write_line("];");
    w.write_line("");
    w.lazy_static("file_descriptor_proto_lazy", "::protobuf::descriptor::FileDescriptorProto");
    w.write_line("");
    w.def_fn("parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto", |w| {
        w.write_line("::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()");
    });
    w.write_line("");
    w.pub_fn("file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto", |w| {
        w.unsafe_expr(|w| {
            w.block("file_descriptor_proto_lazy.get(|| {", "})", |w| {
                w.write_line("parse_descriptor_proto()");
            });
        });
    });
}

pub fn gen(file_descriptors: &[FileDescriptorProto], files_to_generate: &[String])
        -> Vec<compiler_plugin::GenResult>
{
    let root_scope = RootScope { file_descriptors: file_descriptors };

    let mut results: Vec<compiler_plugin::GenResult> = Vec::new();
    let files_map: HashMap<&str, &FileDescriptorProto> =
        file_descriptors.iter().map(|f| (f.get_name(), f)).collect();

    for file_name in files_to_generate.iter() {
        let file = file_descriptors.iter()
            .find(|fd| fd.get_name() == &file_name[..])
            .expect("no descriptor for file");
        let base = proto_path_to_rust_base(file.get_name());

        let mut v = Vec::new();

        {
            let mut os = VecWriter::new(&mut v);
            let mut w = IndentWriter::new(&mut os);

            w.write_line("// This file is generated. Do not edit");

            // https://secure.phabricator.com/T784
            w.write_line("// @generated");

            w.write_line("");
            w.write_line("#![allow(dead_code)]");
            w.write_line("#![allow(non_camel_case_types)]");
            w.write_line("#![allow(non_snake_case)]");
            w.write_line("#![allow(non_upper_case_globals)]");
            w.write_line("#![allow(unused_imports)]");

            w.write_line("");
            w.write_line("use protobuf::Message as Message_imported_for_functions;");
            w.write_line("use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;");
            for dep in file.get_dependency().iter() {
                // TODO: should use absolute paths in file instead of global uses
                let files = files_map[&dep as &str];
                for message in files.get_message_type().iter() {
                    w.write_line(format!("use super::{}::{};",
                        proto_path_to_rust_base(&dep),
                        message.get_name()));
                }
                for en in files.get_enum_type().iter() {
                    w.write_line(format!("use super::{}::{};",
                        proto_path_to_rust_base(&dep),
                        en.get_name()));
                }
            }

            let scope = FileScope { file_descriptor: file } .to_scope();

            for message in scope.get_messages().iter() {
                w.write_line("");
                MessageContext::new(message, &root_scope).write(&mut w);
            }
            for enum_type in scope.get_enums().iter() {
                w.write_line("");
                EnumContext::new(enum_type, &root_scope).write(&mut w);
            }

            if file.get_options().get_optimize_for() != FileOptions_OptimizeMode::LITE_RUNTIME {
                w.write_line("");
                write_file_descriptor_data(file, &mut w);
            }
        }

        results.push(compiler_plugin::GenResult {
            name: {
                let mut r = base.to_string();
                r.push_str(".rs");
                r
            },
            content: v,
        });
    }
    results
}

pub fn protoc_gen_rust_main() {
    compiler_plugin::plugin_main(gen);
}
