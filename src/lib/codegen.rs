use std::io::Writer;
use std::mem;
use std::fmt;
use std::collections::hash_map::HashMap;

use descriptor::*;
use misc::*;
use stream::wire_format;
use core::Message;
use rt;
use paginate::PaginatableIterator;
use strx::*;
use descriptorx::EnumWithScope;
use descriptorx::MessageWithScope;
use descriptorx::RootScope;
use descriptorx::Scope;
use descriptorx::WithScope;

#[deriving(Clone,PartialEq,Eq)]
enum RustType {
    Signed(uint),
    Unsigned(uint),
    Float(uint),
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
    Enum(String),
}

impl fmt::Show for RustType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RustType::Signed(bits)       => write!(f, "i{}", bits),
            RustType::Unsigned(bits)     => write!(f, "u{}", bits),
            RustType::Float(bits)        => write!(f, "f{}", bits),
            RustType::Bool               => write!(f, "bool"),
            RustType::Vec(ref param)     => write!(f, "::std::vec::Vec<{}>", *param),
            RustType::String             => write!(f, "::std::string::String"),
            RustType::Slice(ref param)   => write!(f, "[{}]", *param),
            RustType::Str                => write!(f, "str"),
            RustType::Option(ref param)           => write!(f, "::std::option::Option<{}>", param),
            RustType::SingularField(ref param)    => write!(f, "::protobuf::SingularField<{}>", param),
            RustType::SingularPtrField(ref param) => write!(f, "::protobuf::SingularPtrField<{}>", param),
            RustType::RepeatedField(ref param)    => write!(f, "::protobuf::RepeatedField<{}>", param),
            RustType::Uniq(ref param)             => write!(f, "::std::Box<{}>", *param),
            RustType::Ref(ref param)              => write!(f, "&{}", *param),
            RustType::Message(ref param) |
            RustType::Enum(ref param)    => write!(f, "{}", param),
        }
    }
}

impl RustType {
    fn is_ref(&self) -> bool {
        match *self {
            RustType::Ref(..) => true,
            _           => false,
        }
    }

    fn ref_str(&self, lt: &str) -> String {
        match *self {
            RustType::Ref(ref param) => format!("&'{} {}", lt, *param),
            _ => panic!("not a ref: {}", *self),
        }
    }

    fn mut_ref_str(&self, lt: &str) -> String {
        match *self {
            RustType::Ref(ref param) => format!("&'{} mut {}", lt, *param),
            _ => panic!("not a ref: {}", *self),
        }
    }

    fn ref_str_safe(&self, lt: &str) -> String {
        if self.is_ref() {
            self.ref_str(lt)
        } else {
            format!("{}", self)
        }
    }

    // default value for type
    fn default_value(&self) -> String {
        match *self {
            RustType::Ref(box RustType::Str)         => "\"\"".to_string(),
            RustType::Ref(box RustType::Slice(..))   => "[].as_slice()".to_string(), // "&[]".to_string(),
            RustType::Signed(..)                     |
            RustType::Unsigned(..)                   => "0".to_string(),
            RustType::Float(..)                      => "0.".to_string(),
            RustType::Bool(..)                       => "false".to_string(),
            RustType::Vec(..)                        => "::std::vec::Vec::new()".to_string(),
            RustType::String                         => "::std::string::String::new()".to_string(),
            RustType::Option(..)                     => "::std::option::None".to_string(),
            RustType::SingularField(..)              => "::protobuf::SingularField::none()".to_string(),
            RustType::SingularPtrField(..)           => "::protobuf::SingularPtrField::none()".to_string(),
            RustType::RepeatedField(..)              => "::protobuf::RepeatedField::new()".to_string(),
            RustType::Message(ref name)              => format!("{}::new()", name),
            RustType::Ref(box RustType::Message(ref name)) => format!("{}::default_instance()", name),
            RustType::Enum(..)                       =>
                panic!("enum default value cannot be determined by type"),
            _ => panic!("cannot create default value for: {}", *self),
        }
    }

    fn clear(&self, v: &str) -> String {
        match *self {
            RustType::Option(..) => format!("{} = ::std::option::None", v),
            RustType::Vec(..) |
            RustType::String |
            RustType::RepeatedField(..) |
            RustType::SingularField(..) |
            RustType::SingularPtrField(..) => format!("{}.clear()", v),
            ref ty => panic!("cannot clear type: {}", ty),
        }
    }

    // wrap value in storage type
    fn wrap_value(&self, value: &str) -> String {
        match *self {
            RustType::Option(..)           => format!("::std::option::Some({})", value),
            RustType::SingularField(..)    => format!("::protobuf::SingularField::some({})", value),
            RustType::SingularPtrField(..) => format!("::protobuf::SingularPtrField::some({})", value),
            _ => panic!("not a wrapper type: {}", *self),
        }
    }

    // expression to convert `v` of type `self` to type `target`
    fn into(&self, target: &RustType, v: &str) -> String {
        match (self, target) {
            (x, y) if x == y                        => format!("{}", v),
            (&RustType::Ref(ref x), y) if **x == *y => format!("*{}", v),
            (&RustType::String, &RustType::Ref(box RustType::Str))                    |
            (&RustType::Ref(box RustType::String), &RustType::Ref(box RustType::Str)) =>
                    format!("{}.as_slice()", v),
            (&RustType::Vec(ref x), &RustType::Ref(box RustType::Slice(ref y))) if x == y =>
                    format!("{}.as_slice()", v),
            (&RustType::Ref(box RustType::Vec(ref x)), &RustType::Ref(box RustType::Slice(ref y))) if x == y =>
                    format!("{}.as_slice()", v),
            (&RustType::Enum(..), &RustType::Signed(32)) =>
                    format!("{} as i32", v),
            (&RustType::Ref(box RustType::Enum(..)), &RustType::Signed(32)) =>
                    format!("*{} as i32", v),
            _ => panic!("cannot convert {} to {}", self, target),
        }
    }

    fn ref_type(&self) -> RustType {
        RustType::Ref(match self {
            &RustType::String               => box RustType::Str,
            &RustType::Vec(ref p)           |
            &RustType::RepeatedField(ref p) => box RustType::Slice(p.clone()),
            &RustType::Message(ref p)       => box RustType::Message(p.clone()),
            x => panic!("no ref type for {}", x),
        })
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
        FieldDescriptorProto_Type::TYPE_BYTES    => RustType::Vec(box RustType::Unsigned(8)),
        FieldDescriptorProto_Type::TYPE_ENUM     |
        FieldDescriptorProto_Type::TYPE_GROUP    |
        FieldDescriptorProto_Type::TYPE_MESSAGE  => panic!("there is no rust name for {}", field_type),
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
        FieldDescriptorProto_Type::TYPE_ENUM     |
        FieldDescriptorProto_Type::TYPE_GROUP    |
        FieldDescriptorProto_Type::TYPE_MESSAGE  => panic!()
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
    if field.get_type_name().starts_with(current_pkg_prefix.as_slice()) {
        let mut tn = remove_prefix(field.get_type_name(), current_pkg_prefix.as_slice()).to_string();
        match tn.as_slice().rfind('.') {
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
        let name = (if field.get_type_name().starts_with(current_pkg_prefix.as_slice()) {
            remove_prefix(field.get_type_name(), current_pkg_prefix.as_slice()).to_string()
        } else {
            // TODO: package prefix
            remove_to(field.get_type_name(), '.').to_string()
        }).replace(".", "_");
        match field.get_field_type() {
            FieldDescriptorProto_Type::TYPE_MESSAGE => RustType::Message(name),
            FieldDescriptorProto_Type::TYPE_ENUM    => RustType::Enum(name),
            _ => panic!("unknown named type: {}", field.get_field_type()),
        }
    } else if field.has_field_type() {
        rust_name(field.get_field_type())
    } else {
        panic!("neither type_name, nor field_type specified for field: {}", field.get_name());
    }
}

#[deriving(Clone)]
enum RepeatMode {
    Single,
    RepeatRegular,
    RepeatPacked,
}

#[deriving(Clone)]
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
}

impl Field {
    fn parse(field: &FieldDescriptorProto, root_scope: &RootScope, pkg: &str) -> Option<Field> {
        let type_name = field_type_name(field, pkg);
        let repeated = match field.get_label() {
            FieldDescriptorProto_Label::LABEL_REPEATED => true,
            FieldDescriptorProto_Label::LABEL_OPTIONAL |
            FieldDescriptorProto_Label::LABEL_REQUIRED => false,
        };
        let name = match field.get_name() {
            "type" => "field_type".to_string(),
            x => x.to_string(),
        };
        let packed =
            if field.has_options() {
                field.get_options().get_packed()
            } else {
                false
            };
        let repeat_mode =
            if repeated {
                if packed { RepeatMode::RepeatPacked } else { RepeatMode::RepeatRegular }
            } else {
                RepeatMode::Single
            };
        let enum_default_value = match field.get_field_type() {
            FieldDescriptorProto_Type::TYPE_ENUM => {
                let e = Enum::parse(&root_scope.find_enum(field.get_type_name()));
                let ev = if field.has_default_value() {
                    e.value_by_name(field.get_default_value()).clone()
                } else {
                    e.values.into_iter().next().unwrap()
                };
                Some(ev)
            }
            _ => None,
        };
        Some(Field {
            proto_field: field.clone(),
            name: name,
            field_type: field.get_field_type(),
            wire_type: field_type_wire_type(field.get_field_type()),
            type_name: type_name,
            type_scope_prefix: field_type_name_scope_prefix(field, pkg),
            enum_default_value: enum_default_value,
            number: field.get_number() as u32,
            repeated: repeated,
            packed: packed,
            repeat_mode: repeat_mode,
        })
    }

    fn number(&self) -> u32 {
        self.number
    }

    fn tag_size(&self) -> u32 {
        rt::tag_size(self.number)
    }

    // type of field in struct
    fn full_storage_type(&self) -> RustType {
        let c = box self.type_name.clone();
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
                RustType::Ref(box RustType::Str),
            FieldDescriptorProto_Type::TYPE_BYTES  =>
                RustType::Ref(box RustType::Slice(box RustType::Unsigned(8))),
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

    // for field `foo`, return type of `fn mut_foo(..)`
    fn mut_xxx_return_type(&self) -> RustType {
        RustType::Ref(box if self.repeated {
            self.full_storage_type()
        } else {
            self.type_name.clone()
        })
    }

    // for field `foo`, return type of `fn get_foo(..)`
    fn get_xxx_return_type(&self) -> RustType {
        match self.repeated {
            true => RustType::Ref(box RustType::Slice(box self.type_name.clone())),
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
            RustType::SingularPtrField(ty) => RustType::Option(box RustType::Ref(ty)),
            x => panic!("cannot convert {} to option", x),
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

    fn default_value_rust(&self) -> String {
        if self.enum_default_value.is_some() {
            self.enum_default_value.as_ref().unwrap().rust_name_outer()
        } else if self.proto_field.has_default_value() {
            let proto_default = self.proto_field.get_default_value();
            match self.field_type {
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
                FieldDescriptorProto_Type::TYPE_STRING   => format!("\"{}\"", proto_default.escape_default()),
                // For bytes, contains the C escaped value.  All bytes >= 128 are escaped
                FieldDescriptorProto_Type::TYPE_BYTES    => format!("b\"{}\"", proto_default),
                // TODO: resolve outer message prefix
                FieldDescriptorProto_Type::TYPE_GROUP    |
                FieldDescriptorProto_Type::TYPE_ENUM     => unreachable!(),
                FieldDescriptorProto_Type::TYPE_MESSAGE  =>
                    panic!("default value is not implemented for type: {}", self.field_type)
            }
        } else {
            self.get_xxx_return_type().default_value()
        }
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
}

#[deriving(Clone)]
struct MessageInfo<'a> {
    proto_message: DescriptorProto,
    pkg: String,
    prefix: String,
    type_name: String,
    fields: Vec<Field>,
    lite_runtime: bool,
}

impl<'a> MessageInfo<'a> {
    fn parse(message: &MessageWithScope<'a>, root_scope: &RootScope) -> MessageInfo<'a> {
        MessageInfo {
            proto_message: message.message.clone(),
            pkg: message.get_package().to_string(),
            prefix: message.scope.rust_prefix(),
            type_name: message.rust_name(),
            fields: message.message.get_field().iter().flat_map(|field| {
                Field::parse(field, root_scope, message.get_package()).into_iter()
            }).collect(),
            lite_runtime:
                message.get_file_descriptor().get_options().get_optimize_for()
                    == FileOptions_OptimizeMode::LITE_RUNTIME,
        }
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
}

struct Enum<'a> {
    //en: EnumWithScope<'a>,
    type_name: String,
    values: Vec<EnumValue>,
    lite_runtime: bool,
}

#[deriving(Clone)]
struct EnumValue {
    proto: EnumValueDescriptorProto,
    prefix: String,
    enum_rust_name: String,
}

impl<'a> Enum<'a> {
    fn parse<'a>(en: &EnumWithScope<'a>) -> Enum<'a> {
        Enum {
            //en: en.clone(),
            type_name: en.rust_name(),
            values: en.en.get_value().iter()
                .map(|p| EnumValue::parse(p, en.scope.rust_prefix().as_slice(), en.rust_name().as_slice()))
                .collect(),
            lite_runtime:
                en.get_scope().get_file_descriptor().get_options().get_optimize_for()
                    == FileOptions_OptimizeMode::LITE_RUNTIME,
        }
    }

    fn value_by_name(&'a self, name: &str) -> &'a EnumValue {
        self.values.iter().find(|v| v.name() == name).unwrap()
    }
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
        r.push_str(self.enum_rust_name.as_slice());
        r.push_str("::");
        r.push_str(self.rust_name_inner().as_slice());
        r
    }
}


struct IndentWriter<'a> {
    // TODO: add mut
    writer: &'a Writer + 'a,
    indent: String,
    msg: Option<&'a MessageInfo<'a>>,
    field: Option<&'a Field>,
    en: Option<&'a Enum<'a>>,
}

impl<'a> IndentWriter<'a> {
    fn new(writer: &'a mut Writer) -> IndentWriter<'a> {
        IndentWriter {
            writer: writer,
            indent: "".to_string(),
            msg: None,
            field: None,
            en: None,
        }
    }

    fn bind_message<T>(&self, msg: &MessageInfo, cb: |&mut IndentWriter| -> T) -> T {
        cb(&mut IndentWriter {
            writer: unsafe { mem::transmute(self.writer) },
            indent: self.indent.to_string(),
            msg: Some(msg),
            field: None,
            en: None,
        })
    }

    fn bind_field<T>(&self, field: &'a Field, cb: |&mut IndentWriter| -> T) -> T {
        assert!(self.msg.is_some());
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: self.indent.to_string(),
            msg: self.msg,
            field: Some(field),
            en: None,
        })
    }

    fn bind_enum<T>(&self, en: &Enum, cb: |&mut IndentWriter| -> T) -> T {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: self.indent.to_string(),
            msg: None,
            field: None,
            en: Some(en),
        })
    }

    fn fields(&self, cb: |&mut IndentWriter|) {
        let fields = &self.msg.as_ref().unwrap().fields;
        let mut iter = fields.iter();
        for field in iter {
            self.bind_field(field, |w| cb(w));
        }
    }

    fn required_fields(&self, cb: |&mut IndentWriter|) {
        let fields = &self.msg.as_ref().unwrap().required_fields();
        let mut iter = fields.iter();
        for field in iter {
            self.bind_field(*field, |w| cb(w));
        }
    }
    /*
    fn fields(&'a self) -> FieldsIter<'a> {
        FieldsIter { parent: self }
    }
    fn required_fields(&'a self) -> FieldsIter<'a> {
        FieldsIter { parent: self }
    }
    */


    fn field(&self) -> &'a Field {
        assert!(self.field.is_some());
        self.field.unwrap()
    }

    fn en(&self) -> &'a Enum<'a> {
        self.en.unwrap()
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

    fn if_self_field_is_some(&self, cb: |&mut IndentWriter|) {
        self.if_stmt(self.self_field_is_some(), cb);
    }

    fn if_self_field_is_not_empty(&self, cb: |&mut IndentWriter|) {
        self.if_stmt(self.self_field_is_not_empty(), cb);
    }

    fn if_self_field_is_none(&self, cb: |&mut IndentWriter|) {
        self.if_stmt(self.self_field_is_none(), cb);
    }

    fn for_self_field(&mut self, varn: &str, cb: |&mut IndentWriter, v_type: &RustType|) {
        let v_type = self.field().full_storage_iter_elem_type();
        self.for_stmt(format!("{}.iter()", self.self_field()), varn, |w| cb(w, &v_type));
    }

    fn self_field_assign<S : Str>(&self, value: S) {
        self.write_line(format!("{} = {};", self.self_field(), value.as_slice()));
    }

    fn self_field_assign_some<S : Str>(&self, value: S) {
        assert!(!self.field().repeated);
        self.self_field_assign(self.field().full_storage_type().wrap_value(value.as_slice()));
    }

    fn self_field_assign_default(&self) {
        assert!(!self.field().repeated);
        if self.field().type_is_not_trivial() {
            self.write_line(format!("{}.set_default();", self.self_field()));
        } else {
            self.self_field_assign_some(self.field().default_value_rust());
        }
    }

    fn self_field_assign_value<S : Str>(&self, value: S, ty: &RustType) {
        if self.field().repeated {
            let converted = ty.into(&self.field().full_storage_type(), value.as_slice());
            self.self_field_assign(converted);
        } else {
            let converted = ty.into(&self.field().type_name, value.as_slice());
            let wrapped = self.field().full_storage_type().wrap_value(converted.as_slice());
            self.self_field_assign(wrapped);
        }
    }

    fn self_field_push<S : Str>(&self, value: S) {
        assert!(self.field().repeated);
        self.write_line(format!("{}.push({});", self.self_field(), value.as_slice()));
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
        format!("::protobuf::rt::{}({}.as_slice())",
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
        format!("::protobuf::rt::{}({}, {}.as_slice())",
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

    fn field_default(&self) {
        let init = self.field().full_storage_type().default_value();
        self.field_entry(self.field().name.to_string(), init);
    }

    fn write_line<S : Str>(&self, line: S) {
        let mut_writer: &mut Writer = unsafe { mem::transmute(self.writer) };
        (if line.as_slice().is_empty() {
            mut_writer.write("\n".as_bytes())
        } else {
            let s = [self.indent.as_slice(), line.as_slice(), "\n"].concat();
            mut_writer.write(s.as_bytes())
        }).unwrap();
    }

    #[allow(dead_code)]
    fn write_lines(&self, lines: &[String]) {
        for line in lines.iter() {
            self.write_line(line.to_string());
        }
    }

    fn indented(&self, cb: |&mut IndentWriter|) {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: format!("{}    ", self.indent),
            msg: self.msg,
            field: self.field,
            en: self.en,
        });
    }

    #[allow(dead_code)]
    fn commented(&self, cb: |&mut IndentWriter|) {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: format!("// {}", self.indent),
            msg: self.msg,
            field: self.field,
            en: self.en,
        });
    }

    fn lazy_static<S1 : Str, S2 : Str>(&mut self, name: S1, ty: S2) {
        self.stmt_block(format!("static mut {}: ::protobuf::lazy::Lazy<{}> = ::protobuf::lazy::Lazy", name.as_slice(), ty.as_slice()), |w| {
            w.field_entry("lock", "::protobuf::lazy::ONCE_INIT");
            w.field_entry("ptr", format!("0 as *const {}", ty.as_slice()));
        });
    }

    fn lazy_static_decl_get<S1 : Str, S2 : Str>(&mut self, name: S1, ty: S2, init: |&mut IndentWriter|) {
        self.lazy_static(name.as_slice(), ty);
        self.unsafe_expr(|w| {
            w.write_line(format!("{}.get(|| {{", name.as_slice()));
            w.indented(|w| init(w));
            w.write_line(format!("}})"));
        });
    }

    fn block<S1 : Str, S2 : Str>(&self, first_line: S1, last_line: S2, cb: |&mut IndentWriter|) {
        self.write_line(first_line.as_slice());
        self.indented(cb);
        self.write_line(last_line.as_slice());
    }

    fn expr_block<S : Str>(&self, prefix: S, cb: |&mut IndentWriter|) {
        self.block(format!("{} {{", prefix.as_slice()), "}", cb);
    }

    fn stmt_block<S : Str>(&self, prefix: S, cb: |&mut IndentWriter|) {
        self.block(format!("{} {{", prefix.as_slice()), "};", cb);
    }

    fn unsafe_expr(&self, cb: |&mut IndentWriter|) {
        self.expr_block("unsafe", cb);
    }

    fn impl_block<S : Str>(&self, name: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("impl {}", name.as_slice()), cb);
    }

    fn impl_self_block<S : Str>(&self, name: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("impl<'a> {}", name.as_slice()), cb);
    }

    fn impl_for_block<S1 : Str, S2 : Str>(&self, tr: S1, ty: S2, cb: |&mut IndentWriter|) {
        self.expr_block(format!("impl {} for {}", tr.as_slice(), ty.as_slice()), cb);
    }

    fn pub_struct<S : Str>(&self, name: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("pub struct {}", name.as_slice()), cb);
    }

    fn field_entry<S1 : Str, S2 : Str>(&self, name: S1, value: S2) {
        self.write_line(format!("{}: {},", name.as_slice(), value.as_slice()));
    }

    #[allow(dead_code)]
    fn fail<S : Str>(&self, reason: S) {
        self.write_line(format!("panic!({});", reason.as_slice()));
    }

    #[allow(dead_code)]
    fn todo(&self) {
        self.fail("TODO");
    }

    fn deriving(&mut self, deriving: &[&str]) {
        let v: Vec<String> = deriving.iter().map(|&s| s.to_string()).collect();
        self.write_line(format!("#[deriving({})]", v.connect(",")));
    }

    fn allow(&mut self, what: &[&str]) {
        let v: Vec<String> = what.iter().map(|&s| s.to_string()).collect();
        self.write_line(format!("#[allow({})]", v.connect(",")));
    }

    fn comment(&self, comment: &str) {
        if comment.is_empty() {
            self.write_line("//");
        } else {
            self.write_line(format!("// {}", comment));
        }
    }

    fn pub_fn<S : Str>(&self, sig: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("pub fn {}", sig.as_slice()), cb);
    }

    fn def_fn<S : Str>(&self, sig: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("fn {}", sig.as_slice()), cb);
    }

    fn while_block<S : Str>(&self, cond: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("while {}", cond.as_slice()), cb);
    }

    fn if_stmt<S : Str>(&self, cond: S, cb: |&mut IndentWriter|) {
        self.stmt_block(format!("if {}", cond.as_slice()), cb);
    }

    fn for_stmt<S1 : Str, S2 : Str>(&self, over: S1, varn: S2, cb: |&mut IndentWriter|) {
        self.stmt_block(format!("for {} in {}", varn.as_slice(), over.as_slice()), cb)
    }

    fn match_block<S : Str>(&self, value: S, cb: |&mut IndentWriter|) {
        self.stmt_block(format!("match {}", value.as_slice()), cb);
    }

    fn match_expr<S : Str>(&self, value: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("match {}", value.as_slice()), cb);
    }

    fn case_block<S : Str>(&self, cond: S, cb: |&mut IndentWriter|) {
        self.block(format!("{} => {{", cond.as_slice()), "},", cb);
    }

    fn case_expr<S1 : Str, S2 : Str>(&self, cond: S1, body: S2) {
        self.write_line(format!("{} => {},", cond.as_slice(), body.as_slice()));
    }

    fn clear_field_func(&self) -> String {
        let mut r = "clear_".to_string();
        r.push_str(self.field.as_ref().unwrap().name.as_slice());
        r
    }

    fn clear_field(&self) {
        let clear_expr = self.field().full_storage_type().clear(self.self_field().as_slice());
        self.write_line(format!("{};", clear_expr));
    }

    fn assert_wire_type(&self, wire_type: wire_format::WireType) {
        self.if_stmt(format!("wire_type != ::protobuf::wire_format::{}", wire_type), |w| {
            // TODO: write wire type
            let message = "\"unexpected wire type\".to_string()";
            w.write_line(format!("return ::std::result::Err(::protobuf::ProtobufError::WireError({}));", message));
        });
    }
}

fn write_merge_from_field_message_string_bytes(w: &mut IndentWriter) {
    let field = w.field();
    w.assert_wire_type(wire_format::WireTypeLengthDelimited);
    if field.repeated {
        w.write_line(format!("let tmp = {}.push_default();", w.self_field()));
    } else {
        w.write_line(format!("let tmp = {}.set_default();", w.self_field()));
    }
    match field.field_type {
        FieldDescriptorProto_Type::TYPE_MESSAGE =>
            w.write_line(format!("try!(is.merge_message(tmp))")),
        FieldDescriptorProto_Type::TYPE_STRING =>
            w.write_line(format!("try!(is.read_string_into(tmp))")),
        FieldDescriptorProto_Type::TYPE_BYTES =>
            w.write_line(format!("try!(is.read_bytes_into(tmp))")),
        _ =>
            panic!(),
    }
}

fn write_merge_from_field(w: &mut IndentWriter) {
    let field = w.field();
    if field.type_is_not_trivial() {
        write_merge_from_field_message_string_bytes(w);
    } else {
        let wire_type = field_type_wire_type(field.field_type);
        let repeat_mode =
            if field.repeated {
                if wire_type == wire_format::WireTypeLengthDelimited {
                    RepeatMode::RepeatRegular
                } else {
                    RepeatMode::RepeatPacked // may be both regular or packed
                }
            } else {
                RepeatMode::Single
            };

        let read_proc0 = match field.field_type {
            FieldDescriptorProto_Type::TYPE_ENUM => format!("{}::new(try!(is.read_int32()))", field.type_name),
            t => format!("try!(is.read_{}())", protobuf_name(t)),
        };
        let read_proc = read_proc0.as_slice();

        match repeat_mode {
            RepeatMode::Single | RepeatMode::RepeatRegular => {
                w.assert_wire_type(wire_type);
                w.write_line(format!("let tmp = {};", read_proc));
                match repeat_mode {
                    RepeatMode::Single => w.self_field_assign_some("tmp"),
                    RepeatMode::RepeatRegular => w.self_field_push("tmp"),
                    _ => panic!()
                }
            },
            RepeatMode::RepeatPacked => {
                w.write_line(format!("if wire_type == ::protobuf::wire_format::{} {{", wire_format::WireTypeLengthDelimited));
                w.indented(|w| {
                    w.write_line("let len = try!(is.read_raw_varint32());");
                    w.write_line("let old_limit = is.push_limit(len);");
                    w.while_block("!try!(is.eof())", |w| {
                        w.self_field_push(read_proc);
                    });
                    w.write_line("is.pop_limit(old_limit);");
                });
                w.write_line("} else {");
                w.indented(|w| {
                    w.assert_wire_type(wire_type);
                    w.self_field_push(read_proc);
                });
                w.write_line("}");
            },
        };
    }
}

fn write_message_struct(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    let mut deriving = vec!["Clone", "Default"];
    if msg.lite_runtime {
        deriving.push("Show");
    }
    w.deriving(deriving.as_slice());
    w.pub_struct(msg.type_name.as_slice(), |w| {
        w.fields(|w| {
            let field = w.field.unwrap();
            w.field_entry(field.name.as_slice(), format!("{}", field.full_storage_type()));
        });
        w.field_entry("unknown_fields", "::protobuf::UnknownFields");
        w.field_entry("cached_size", "::std::cell::Cell<u32>");
    });
}

fn write_message_compute_size(w: &mut IndentWriter) {
    // Append sizes of messages in the tree to the specified vector.
    // First appended element is size of self, and then nested message sizes.
    // in serialization order are appended recursively.");
    w.comment("Compute sizes of nested messages");
    w.def_fn("compute_size(&self) -> u32", |w| {
        // To have access to its methods but not polute the name space.
        w.write_line("use protobuf::{Message};");
        w.write_line("let mut my_size = 0;");
        w.fields(|w| {
            let field = w.field();
            match field.repeat_mode {
                RepeatMode::Single | RepeatMode::RepeatRegular => {
                    match field_type_size(field.field_type) {
                        Some(s) => {
                            if field.repeated {
                                w.write_line(format!(
                                        "my_size += {} * {}.len() as u32;",
                                        (s + w.field().tag_size()) as int,
                                        w.self_field()));
                            } else {
                                w.if_self_field_is_some(|w| {
                                    w.write_line(format!(
                                            "my_size += {};",
                                            (s + w.field().tag_size()) as int));
                                });
                            }
                        },
                        None => {
                            w.for_self_field("value", |w, _value_type| {
                                match field.field_type {
                                    FieldDescriptorProto_Type::TYPE_MESSAGE => {
                                        w.write_line("let len = value.compute_size();");
                                        w.write_line(format!(
                                                "my_size += {} + ::protobuf::rt::compute_raw_varint32_size(len) + len;",
                                                w.field().tag_size() as uint));
                                    },
                                    FieldDescriptorProto_Type::TYPE_BYTES => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::bytes_size({}, value.as_slice());",
                                                field.number as int));
                                    },
                                    FieldDescriptorProto_Type::TYPE_STRING => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::string_size({}, value.as_slice());",
                                                field.number as int));
                                    },
                                    FieldDescriptorProto_Type::TYPE_ENUM => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::enum_size({}, *value);",
                                                field.number as int));
                                    },
                                    _ => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::value_size({}, *value, ::protobuf::wire_format::{});",
                                                field.number as int, field.wire_type));
                                    },
                                }
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
        w.write_line("my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());");
        w.write_line("self.cached_size.set(my_size);");
        w.write_line("my_size");
    });
}

fn write_message_write_field(w: &mut IndentWriter) {
    fn write_value_lines(w: &mut IndentWriter, ty: &RustType) {
        match w.field().field_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                w.write_line(format!("try!(os.write_tag({}, ::protobuf::wire_format::{}));",
                        w.field().number(),
                        wire_format::WireTypeLengthDelimited));
                w.write_line(format!("try!(os.write_raw_varint32(v.get_cached_size()));"));
                w.write_line(format!("try!(v.write_to_with_cached_sizes(os));"));
            }
            _ => {
                let param_type = w.field().os_write_fn_param_type();
                w.write_line(format!("try!(os.write_{}({}, {}));",
                    w.field().os_write_fn_suffix(),
                    w.field().number(),
                    ty.into(&param_type, "v")));
            }
        }
    }

    match w.field().repeat_mode {
        RepeatMode::Single => {
            w.match_block(w.self_field_as_option(), |w| {
                let option_type = w.field().as_option_type();
                w.case_block("Some(v)", |w| {
                    let v_type = option_type.elem_type();
                    write_value_lines(w, &v_type);
                });
                w.case_expr("None", "{}");
            });
        },
        RepeatMode::RepeatPacked => {
            w.if_self_field_is_not_empty(|w| {
                w.write_line(format!("try!(os.write_tag({}, ::protobuf::wire_format::{}));", w.field().number(), wire_format::WireTypeLengthDelimited));
                w.comment("TODO: Data size is computed again, it should be cached");
                let data_size_expr = w.self_field_vec_packed_data_size();
                w.write_line(format!("try!(os.write_raw_varint32({}));", data_size_expr));
                w.for_self_field("v", |w, v_type| {
                    let param_type = w.field().os_write_fn_param_type();
                    w.write_line(format!("try!(os.write_{}_no_tag({}));",
                        w.field().os_write_fn_suffix(), v_type.into(&param_type, "v")));
                });
            });
        },
        RepeatMode::RepeatRegular => {
            w.for_self_field("v", |w, v_type| {
                write_value_lines(w, v_type);
            });
        },
    };
}

fn write_message_write_to_with_cached_sizes(w: &mut IndentWriter) {
    w.def_fn("write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()>", |w| {
        // To have access to its methods but not polute the name space.
        w.write_line("use protobuf::{Message};");
        w.fields(|w| {
            write_message_write_field(w);
        });
        w.write_line("try!(os.write_unknown_fields(self.get_unknown_fields()));");
        w.write_line("::std::result::Ok(())");
    });
}

fn write_message_get_cached_size(w: &mut IndentWriter) {
    w.def_fn("get_cached_size(&self) -> u32", |w| {
        w.write_line("self.cached_size.get()");
    });
}

fn write_message_default_instance(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.pub_fn(format!("default_instance() -> &'static {}", msg.type_name), |w| {
        let msg = w.msg.unwrap();
        w.lazy_static_decl_get("instance", msg.type_name.as_slice(), |w| {
            w.expr_block(format!("{}", msg.type_name), |w| {
                w.fields(|w| {
                    w.field_default();
                });
                w.field_entry("unknown_fields", "::protobuf::UnknownFields::new()");
                w.field_entry("cached_size", "::std::cell::Cell::new(0)");
            });
        });
    });
}

fn write_message_field_get(w: &mut IndentWriter) {
    let get_xxx_return_type = w.field().get_xxx_return_type();
    let self_param = match get_xxx_return_type.is_ref() {
        true  => "&'a self",
        false => "&self",
    };
    let get_xxx_return_type_str = get_xxx_return_type.ref_str_safe("a");
    w.pub_fn(format!("get_{}({}) -> {}", w.field().name, self_param, get_xxx_return_type_str),
    |w| {
        if !w.field().repeated {
            if w.field().field_type == FieldDescriptorProto_Type::TYPE_MESSAGE {
                w.write_line(format!("{}.as_ref().unwrap_or_else(|| {}::default_instance())",
                        w.self_field(), w.field().type_name));
            } else {
                if get_xxx_return_type.is_ref() {
                    w.match_expr(w.self_field_as_option(), |w| {
                        let option_type = w.field().as_option_type();
                        let v_type = option_type.elem_type();
                        let r_type = w.field().get_xxx_return_type();
                        w.case_expr(
                            "Some(v)",
                            v_type.into(&r_type, "v")
                        );
                        w.case_expr(
                            "None",
                            w.field().default_value_rust(),
                        );
                    });
                } else {
                    assert!(!w.field().type_is_not_trivial());
                    w.write_line(format!(
                            "{}.unwrap_or({})",
                            w.self_field(), w.field().default_value_rust()));
                }
            }
        } else {
            w.write_line(format!("{}.as_slice()", w.self_field()));
        }
    });
}

fn write_message_single_field_accessors(w: &mut IndentWriter) {
    w.pub_fn(format!("{}(&mut self)", w.clear_field_func()), |w| {
        w.clear_field();
    });

    if !w.field().repeated {
        w.write_line("");
        w.pub_fn(format!("has_{}(&self) -> bool", w.field().name), |w| {
            w.write_line(w.self_field_is_some());
        });
    }

    let set_xxx_param_type = w.field().set_xxx_param_type();
    w.write_line("");
    w.comment("Param is passed by value, moved");
    w.pub_fn(format!("set_{}(&mut self, v: {})", w.field().name, set_xxx_param_type), |w| {
        w.self_field_assign_value("v", &set_xxx_param_type);
    });

    // mut_xxx() are pointless for primitive types
    if w.field().type_is_not_trivial() || w.field().repeated {
        let mut_xxx_return_type = w.field().mut_xxx_return_type();
        w.write_line("");
        w.comment("Mutable pointer to the field.");
        if !w.field().repeated {
            w.comment("If field is not initialized, it is initialized with default value first.");
        }
        w.pub_fn(format!("mut_{}(&'a mut self) -> {}", w.field().name, mut_xxx_return_type.mut_ref_str("a")),
        |w| {
            if !w.field().repeated {
                w.if_self_field_is_none(|w| {
                    w.self_field_assign_default();
                });
                w.write_line(format!("{}.as_mut().unwrap()", w.self_field()));
            } else {
                w.write_line(format!("&mut {}", w.self_field()));
            }
        });
    }

    w.write_line("");
    write_message_field_get(w);
}

fn write_message_field_accessors(w: &mut IndentWriter) {
    w.fields(|w| {
        w.write_line("");
        w.comment((w.field().reconstruct_def() + ";").as_slice());
        w.write_line("");
        write_message_single_field_accessors(w);
    });
}

fn write_message_impl_self(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.impl_self_block(msg.type_name.as_slice(), |w| {
        w.pub_fn(format!("new() -> {}", msg.type_name), |w| {
            w.write_line("::std::default::Default::default()");
        });

        w.write_line("");
        write_message_default_instance(w);
        write_message_field_accessors(w);
    });
}

fn write_message_unknown_fields(w: &mut IndentWriter) {
    w.def_fn("get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields", |w| {
        w.write_line("&self.unknown_fields");
    });
    w.write_line("");
    w.def_fn("mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields", |w| {
        w.write_line("&mut self.unknown_fields");
    });
}

fn write_message_merge_from(w: &mut IndentWriter) {
    w.def_fn(format!("merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()>"), |w| {
        w.while_block("!try!(is.eof())", |w| {
            w.write_line(format!("let (field_number, wire_type) = try!(is.read_tag_unpack());"));
            w.match_block("field_number", |w| {
                w.fields(|w| {
                    w.case_block(w.field().number.to_string(), |w| {
                        write_merge_from_field(w);
                    });
                });
                w.case_block("_", |w| {
                    w.write_line("let unknown = try!(is.read_unknown(wire_type));");
                    w.write_line("self.mut_unknown_fields().add_value(field_number, unknown);");
                });
            });
        });
        w.write_line("::std::result::Ok(())");
    });
}

fn write_message_descriptor_static(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.allow(&["unused_unsafe", "unused_mut"]);
    w.def_fn(format!("descriptor_static(_: ::std::option::Option<{}>) -> &'static ::protobuf::reflect::MessageDescriptor", msg.type_name), |w| {
        w.lazy_static_decl_get("descriptor", "::protobuf::reflect::MessageDescriptor", |w| {
            let vec_type_param = format!(
                    "&'static ::protobuf::reflect::FieldAccessor<{}>",
                    msg.type_name);
            w.write_line(format!("let mut fields: ::std::vec::Vec<{}> = ::std::vec::Vec::new();", vec_type_param));
            for field in msg.fields.iter() {
                let acc_name = format!("{}_{}_acc", msg.type_name, field.name);
                // TODO: transmute is because of https://github.com/mozilla/rust/issues/13887
                w.write_line(format!("fields.push(unsafe {{ ::std::mem::transmute(&{} as &'static ::protobuf::reflect::FieldAccessor<{}>) }});",
                        acc_name, msg.type_name));
            }
            w.write_line(format!("::protobuf::reflect::MessageDescriptor::new::<{}>(", msg.type_name));
            w.indented(|w| {
                w.write_line(format!("\"{}\",", msg.type_name));
                w.write_line("fields,");
                w.write_line("file_descriptor_proto()");
            });
            w.write_line(")");
        });
    });
}

fn write_message_impl_message(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.impl_for_block("::protobuf::Message", msg.type_name.as_slice(), |w| {
        w.def_fn(format!("new() -> {}", msg.type_name), |w| {
            w.write_line(format!("{}::new()", msg.type_name));
        });
        w.write_line("");
        w.def_fn(format!("is_initialized(&self) -> bool"), |w| {
            w.required_fields(|w| {
                w.if_self_field_is_none(|w| {
                    w.write_line("return false;");
                });
            });
            w.write_line("true");
        });
        w.write_line("");
        write_message_merge_from(w);
        w.write_line("");
        write_message_compute_size(w);
        w.write_line("");
        write_message_write_to_with_cached_sizes(w);
        w.write_line("");
        write_message_get_cached_size(w);
        w.write_line("");
        write_message_unknown_fields(w);
        if !msg.lite_runtime {
            w.write_line("");
            write_message_descriptor_static(w);
        }
        w.write_line("");
        w.def_fn("type_id(&self) -> ::std::intrinsics::TypeId", |w| {
            w.write_line(format!("::std::intrinsics::TypeId::of::<{}>()", msg.type_name));
        });
    });
}

fn write_message_impl_show(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.impl_for_block("::std::fmt::Show", msg.type_name.as_slice(), |w| {
        w.def_fn("fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result", |w| {
            w.write_line("use protobuf::{Message};");
            w.write_line("self.fmt_impl(f)");
        });
    });
}

fn write_message_descriptor_field(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    let field = w.field.unwrap();
    w.allow(&["non_camel_case_types"]);
    let accessor_name = format!("{}_{}_acc", msg.type_name, field.name);
    let accessor_type_name = accessor_name + "_type";
    w.write_line(format!("struct {};", accessor_type_name));
    w.write_line(format!("static {}: {} = {};", accessor_name, accessor_type_name, accessor_type_name));
    w.write_line("");
    w.impl_for_block(
            format!("::protobuf::reflect::FieldAccessor<{}>", msg.type_name), accessor_type_name,
    |w| {
        w.def_fn("name(&self) -> &'static str", |w| {
            w.write_line(format!("\"{}\"", field.name));
        });

        w.write_line("");
        if field.repeated {
            w.def_fn(format!("len_field(&self, m: &{}) -> uint", msg.type_name), |w| {
                w.write_line(format!("m.get_{}().len()", field.name));
            });
        } else {
            w.def_fn(format!("has_field(&self, m: &{}) -> bool", msg.type_name), |w| {
                w.write_line(format!("m.has_{}()", field.name));
            });
        }

        let name_suffix = match field.field_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => "message".to_string(),
            FieldDescriptorProto_Type::TYPE_ENUM    => "enum".to_string(),
            FieldDescriptorProto_Type::TYPE_STRING  => "str".to_string(),
            FieldDescriptorProto_Type::TYPE_BYTES   => "bytes".to_string(),
            _ => field.type_name.to_string(),
        };

        w.write_line("");
        if field.repeated {
            match field.field_type {
                FieldDescriptorProto_Type::TYPE_MESSAGE => {
                    w.def_fn(format!("get_rep_message_item<'a>(&self, m: &'a {}, index: uint) -> &'a ::protobuf::Message",
                            msg.type_name),
                    |w| {
                        w.write_line(format!("&m.get_{}()[index] as &'a ::protobuf::Message", field.name));
                    });
                },
                FieldDescriptorProto_Type::TYPE_ENUM => {
                    w.def_fn(format!("get_rep_enum_item<'a>(&self, m: &{}, index: uint) -> &'static ::protobuf::reflect::EnumValueDescriptor",
                            msg.type_name),
                    |w| {
                        w.write_line("use protobuf::{ProtobufEnum};");
                        w.write_line(format!("m.get_{}()[index].descriptor()", field.name));
                    });
                },
                _ => {
                    w.def_fn(format!("get_rep_{}<'a>(&self, m: &'a {}) -> {}",
                            name_suffix,
                            msg.type_name,
                            w.field().get_xxx_return_type().ref_str("a")),
                    |w| {
                        w.write_line(format!("m.get_{}()", field.name));
                    });
                },
            };
        } else {
            let get_xxx_return_type = w.field().get_xxx_return_type();
            let (lt_decl, lt_param) = match get_xxx_return_type.is_ref() {
                true  => ("<'a>", "'a "),
                false => ("", ""),
            };
            let return_type_str = get_xxx_return_type.ref_str_safe("a");
            match field.field_type {
                FieldDescriptorProto_Type::TYPE_MESSAGE => {
                    w.def_fn(format!("get_message<'a>(&self, m: &'a {}) -> &'a ::protobuf::Message",
                            msg.type_name),
                    |w| {
                        w.write_line(format!("m.get_{}() as &'a ::protobuf::Message", field.name));
                    });
                },
                FieldDescriptorProto_Type::TYPE_ENUM => {
                    w.def_fn(format!("get_enum<'a>(&self, m: &{}) -> &'static ::protobuf::reflect::EnumValueDescriptor",
                            msg.type_name),
                    |w| {
                        w.write_line("use protobuf::{ProtobufEnum};");
                        w.write_line(format!("m.get_{}().descriptor()", field.name));
                    });
                },
                _ => {
                    w.def_fn(format!("get_{}{}(&self, m: &{}{}) -> {}",
                            name_suffix,
                            lt_decl,
                            lt_param,
                            msg.type_name,
                            return_type_str),
                    |w| {
                        w.write_line(format!("m.get_{}()", field.name));
                    });
                },
            };
        }
    });
}

fn write_message_descriptor(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    for field in msg.fields.iter() {
        w.bind_field(field, |w| {
            w.write_line("");
            write_message_descriptor_field(w);
        });
    }
}

fn write_message_impl_clear(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.impl_for_block("::protobuf::Clear", msg.type_name.as_slice(), |w| {
        w.def_fn("clear(&mut self)", |w| {
            w.fields(|w| {
                w.write_line(format!("self.{}();", w.clear_field_func()));
            });
            w.write_line("self.unknown_fields.clear();");
        });
    });
}

fn write_message_impl_partial_eq(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.impl_for_block("::std::cmp::PartialEq", msg.type_name.as_slice(), |w| {
        w.def_fn(format!("eq(&self, other: &{}) -> bool", msg.type_name), |w| {
            w.fields(|w| {
                w.write_line(format!("self.{field} == other.{field} &&", field=w.field().name));
            });
            w.write_line("self.unknown_fields == other.unknown_fields");
        });
    });
}

fn write_message(m2: &MessageWithScope, root_scope: &RootScope, w: &mut IndentWriter) {
    let msg = MessageInfo::parse(m2, root_scope);

    w.bind_message(&msg, |w| {
        write_message_struct(w);
        w.write_line("");
        write_message_impl_self(w);
        w.write_line("");
        write_message_impl_message(w);
        w.write_line("");
        write_message_impl_clear(w);
        w.write_line("");
        write_message_impl_partial_eq(w);
        if !msg.lite_runtime {
            w.write_line("");
            write_message_impl_show(w);
            w.write_line("");
            write_message_descriptor(w);
        }

        let mut nested_prefix = msg.type_name.to_string();
        nested_prefix.push_str("_");

        for nested in m2.to_scope().get_messages().iter() {
            w.write_line("");
            write_message(nested, root_scope, w);
        }

        for enum_type in m2.to_scope().get_enums().iter() {
            w.write_line("");
            write_enum(enum_type, root_scope, w);
        }
    });
}

fn write_enum_struct(w: &mut IndentWriter) {
    w.deriving(&["Clone", "PartialEq", "Eq", "Show"]);
    w.expr_block(format!("pub enum {}", w.en().type_name), |w| {
        for value in w.en().values.iter() {
            w.write_line(format!("{} = {},", value.rust_name_inner(), value.number()));
        }
    });
}

fn write_enum_impl(w: &mut IndentWriter) {
    w.impl_block(w.en().type_name.as_slice(), |w| {
        w.pub_fn(format!("new(value: i32) -> {}", w.en().type_name), |w| {
            w.match_expr("value", |w| {
                for value in w.en().values.iter() {
                    w.write_line(format!("{} => {},", value.number(), value.rust_name_outer()));
                }
                w.write_line(format!("_ => panic!()"));
            });
        });
    });
}

fn write_enum_impl_enum(w: &mut IndentWriter) {
    let en = w.en.unwrap();
    w.impl_for_block("::protobuf::ProtobufEnum", w.en().type_name.as_slice(), |w| {
        w.def_fn("value(&self) -> i32", |w| {
            w.write_line("*self as i32")
        });
        if !en.lite_runtime {
            w.write_line("");
            w.def_fn(format!("enum_descriptor_static(_: Option<{}>) -> &'static ::protobuf::reflect::EnumDescriptor", w.en().type_name), |w| {
                w.lazy_static_decl_get("descriptor", "::protobuf::reflect::EnumDescriptor", |w| {
                    w.write_line(format!("::protobuf::reflect::EnumDescriptor::new(\"{}\", file_descriptor_proto())", w.en().type_name));
                });
            });
        }
    });
}

fn write_enum(enum_with_scope: &EnumWithScope, _root_scope: &RootScope, w: &mut IndentWriter) {
    let en = Enum::parse(enum_with_scope);
    w.bind_enum(&en, |w| {
        write_enum_struct(w);
        w.write_line("");
        write_enum_impl(w);
        w.write_line("");
        write_enum_impl_enum(w);
    });
}

fn proto_path_to_rust_base(path: &str) -> String {
    let without_dir = remove_to(path, '/');
    let without_suffix = remove_suffix(without_dir, ".proto");
    without_suffix.replace("-", "_")
}

pub struct GenResult {
    pub name: String,
    pub content: Vec<u8>,
}

pub struct GenOptions {
    pub dummy: bool,
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

pub fn gen(file_descriptors: &[FileDescriptorProto], files_to_generate: &[String], _: &GenOptions)
        -> Vec<GenResult>
{
    let root_scope = RootScope { file_descriptors: file_descriptors };

    let mut results: Vec<GenResult> = Vec::new();
    let files_map: HashMap<&str, &FileDescriptorProto> =
        file_descriptors.iter().map(|f| (f.get_name(), f)).collect();

    for file_name in files_to_generate.iter() {
        let file = file_descriptors.iter()
            .find(|fd| fd.get_name() == file_name.as_slice())
            .expect("no descriptor for file");
        let base = proto_path_to_rust_base(file.get_name());

        let mut v = Vec::new();

        {
            let mut os = VecWriter::new(&mut v);
            let mut w = IndentWriter::new(&mut os as &mut Writer);

            w.write_line("// This file is generated. Do not edit");

            w.write_line("");
            w.write_line("#![allow(dead_code)]");
            w.write_line("#![allow(non_camel_case_types)]");
            w.write_line("#![allow(non_upper_case_globals)]");
            w.write_line("#![allow(unused_imports)]");

            w.write_line("");
            for dep in file.get_dependency().iter() {
                for message in files_map[dep.as_slice()].get_message_type().iter() {
                    w.write_line(format!("use super::{}::{};",
                        proto_path_to_rust_base(dep.as_slice()),
                        message.get_name()));
                }
            }

            let scope = Scope {
                file_descriptor: file,
                path: Vec::new(),
            };

            for message in scope.get_messages().iter() {
                w.write_line("");
                write_message(message, &root_scope, &mut w);
            }
            for enum_type in scope.get_enums().iter() {
                w.write_line("");
                write_enum(enum_type, &root_scope, &mut w);
            }

            if file.get_options().get_optimize_for() != FileOptions_OptimizeMode::LITE_RUNTIME {
                w.write_line("");
                write_file_descriptor_data(file, &mut w);
            }
        }

        results.push(GenResult {
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

