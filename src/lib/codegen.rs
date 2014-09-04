use std::io::Writer;
use std::mem;
use std::fmt;
use std::collections::hashmap::HashMap;

use descriptor::*;
use misc::*;
use core::wire_format;
use core::Message;
use rt;
use paginate::PaginatableIterator;
use strx::*;

#[deriving(Clone,PartialEq,Eq)]
enum RustType {
    RustSigned(uint),
    RustUnsigned(uint),
    RustFloat(uint),
    RustBool,
    RustVec(Box<RustType>),
    RustString,
    RustSlice(Box<RustType>),
    RustStr,
    RustOption(Box<RustType>),
    RustSingularField(Box<RustType>),
    RustSingularPtrField(Box<RustType>),
    RustRepeatedField(Box<RustType>),
    RustUniq(Box<RustType>),
    RustRef(Box<RustType>),
    RustMessage(String),
    RustEnum(String),
}

impl fmt::Show for RustType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RustSigned(bits)       => write!(f, "i{}", bits),
            RustUnsigned(bits)     => write!(f, "u{}", bits),
            RustFloat(bits)        => write!(f, "f{}", bits),
            RustBool               => write!(f, "bool"),
            RustVec(ref param)     => write!(f, "Vec<{}>", *param),
            RustString             => write!(f, "String"),
            RustSlice(ref param)   => write!(f, "[{}]", *param),
            RustStr                => write!(f, "str"),
            RustOption(ref param)           => write!(f, "Option<{}>", param),
            RustSingularField(ref param)    => write!(f, "::protobuf::SingularField<{}>", param),
            RustSingularPtrField(ref param) => write!(f, "::protobuf::SingularPtrField<{}>", param),
            RustRepeatedField(ref param)    => write!(f, "::protobuf::RepeatedField<{}>", param),
            RustUniq(ref param)             => write!(f, "Box<{}>", *param),
            RustRef(ref param)              => write!(f, "&{}", *param),
            RustMessage(ref param) |
            RustEnum(ref param)    => write!(f, "{}", param),
        }
    }
}

impl RustType {
    fn is_ref(&self) -> bool {
        match *self {
            RustRef(..) => true,
            _           => false,
        }
    }

    fn ref_str(&self, lt: &str) -> String {
        match *self {
            RustRef(ref param) => format!("&'{} {}", lt, *param),
            _ => fail!("not a ref: {}", *self),
        }
    }

    fn mut_ref_str(&self, lt: &str) -> String {
        match *self {
            RustRef(ref param) => format!("&'{} mut {}", lt, *param),
            _ => fail!("not a ref: {}", *self),
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
            RustRef(box RustStr)               => "\"\"".to_string(),
            //RustRef(box RustSlice(..))         => "&[]".to_string(),
            RustRef(box RustSlice(ref rtype))  => format!("{{ let t: &[{}] = &[]; t }}", rtype), // XXX: workaround
            RustSigned(..) | RustUnsigned(..)  => "0".to_string(),
            RustFloat(..)                      => "0.".to_string(),
            RustBool(..)                       => "false".to_string(),
            RustVec(..)                        => "Vec::new()".to_string(),
            RustString                         => "String::new()".to_string(),
            RustOption(..)                     => "None".to_string(),
            RustSingularField(..)              => "::protobuf::SingularField::none()".to_string(),
            RustSingularPtrField(..)           => "::protobuf::SingularPtrField::none()".to_string(),
            RustRepeatedField(..)              => "::protobuf::RepeatedField::new()".to_string(),
            RustMessage(ref name)              => format!("{}::new()", name),
            RustRef(box RustMessage(ref name)) => format!("{}::default_instance()", name),
            // TODO: use proper constant
            RustEnum(ref name)                 => format!("{}::new(0)", name),
            _ => fail!("cannot create default value for: {}", *self),
        }
    }

    // wrap value in storage type
    fn wrap_value(&self, value: &str) -> String {
        match *self {
            RustOption(..)           => format!("Some({})", value),
            RustSingularField(..)    => format!("::protobuf::SingularField::some({})", value),
            RustSingularPtrField(..) => format!("::protobuf::SingularPtrField::some({})", value),
            _ => fail!("not a wrapper type: {}", *self),
        }
    }

    fn view_as(&self, target: &RustType, v: &str) -> String {
        match (self, target) {
            (&RustString,  &RustRef(box RustStr)) => format!("{}.as_slice()", v),
            (&RustVec(..), &RustRef(box RustSlice(..))) => format!("{}.as_slice()", v),
            _ => v.to_string(),
        }
    }

    fn into(&self, target: &RustType, v: &str) -> String {
        match (self, target) {
            (x, y) if x == y => v.to_string(),
            _ => fail!("internal error: cannot convert {} to {}", self, target),
        }
    }

    fn ref_type(&self) -> RustType {
        RustRef(match self {
            &RustString               => box RustStr,
            &RustVec(ref p)           |
            &RustRepeatedField(ref p) => box RustSlice(p.clone()),
            &RustMessage(ref p)       => box RustMessage(p.clone()),
            x => fail!("no ref type for {}", x),
        })
    }
}

// rust type for protobuf base type
fn rust_name(field_type: FieldDescriptorProto_Type) -> RustType {
    match field_type {
        FieldDescriptorProto_TYPE_DOUBLE   => RustFloat(64),
        FieldDescriptorProto_TYPE_FLOAT    => RustFloat(32),
        FieldDescriptorProto_TYPE_INT32    => RustSigned(32),
        FieldDescriptorProto_TYPE_INT64    => RustSigned(64),
        FieldDescriptorProto_TYPE_UINT32   => RustUnsigned(32),
        FieldDescriptorProto_TYPE_UINT64   => RustUnsigned(64),
        FieldDescriptorProto_TYPE_SINT32   => RustSigned(32),
        FieldDescriptorProto_TYPE_SINT64   => RustSigned(64),
        FieldDescriptorProto_TYPE_FIXED32  => RustUnsigned(32),
        FieldDescriptorProto_TYPE_FIXED64  => RustUnsigned(64),
        FieldDescriptorProto_TYPE_SFIXED32 => RustSigned(32),
        FieldDescriptorProto_TYPE_SFIXED64 => RustSigned(64),
        FieldDescriptorProto_TYPE_BOOL     => RustBool,
        FieldDescriptorProto_TYPE_STRING   => RustString,
        FieldDescriptorProto_TYPE_BYTES    => RustVec(box RustUnsigned(8)),
        FieldDescriptorProto_TYPE_ENUM |
        FieldDescriptorProto_TYPE_GROUP |
        FieldDescriptorProto_TYPE_MESSAGE => fail!()
    }
}

// protobuf type name for protobuf base type
fn protobuf_name(field_type: FieldDescriptorProto_Type) -> &'static str {
    match field_type {
        FieldDescriptorProto_TYPE_DOUBLE   => "double",
        FieldDescriptorProto_TYPE_FLOAT    => "float",
        FieldDescriptorProto_TYPE_INT32    => "int32",
        FieldDescriptorProto_TYPE_INT64    => "int64",
        FieldDescriptorProto_TYPE_UINT32   => "uint32",
        FieldDescriptorProto_TYPE_UINT64   => "uint64",
        FieldDescriptorProto_TYPE_SINT32   => "sint32",
        FieldDescriptorProto_TYPE_SINT64   => "sint64",
        FieldDescriptorProto_TYPE_FIXED32  => "fixed32",
        FieldDescriptorProto_TYPE_FIXED64  => "fixed64",
        FieldDescriptorProto_TYPE_SFIXED32 => "sfixed32",
        FieldDescriptorProto_TYPE_SFIXED64 => "sfixed64",
        FieldDescriptorProto_TYPE_BOOL     => "bool",
        FieldDescriptorProto_TYPE_STRING   => "string",
        FieldDescriptorProto_TYPE_BYTES    => "bytes",
        FieldDescriptorProto_TYPE_ENUM     |
        FieldDescriptorProto_TYPE_GROUP    |
        FieldDescriptorProto_TYPE_MESSAGE  => fail!()
    }
}

fn field_type_wire_type(field_type: FieldDescriptorProto_Type) -> wire_format::WireType {
    use core::wire_format::*;
    match field_type {
        FieldDescriptorProto_TYPE_INT32    => WireTypeVarint,
        FieldDescriptorProto_TYPE_INT64    => WireTypeVarint,
        FieldDescriptorProto_TYPE_UINT32   => WireTypeVarint,
        FieldDescriptorProto_TYPE_UINT64   => WireTypeVarint,
        FieldDescriptorProto_TYPE_SINT32   => WireTypeVarint,
        FieldDescriptorProto_TYPE_SINT64   => WireTypeVarint,
        FieldDescriptorProto_TYPE_BOOL     => WireTypeVarint,
        FieldDescriptorProto_TYPE_ENUM     => WireTypeVarint,
        FieldDescriptorProto_TYPE_FIXED32  => WireTypeFixed32,
        FieldDescriptorProto_TYPE_FIXED64  => WireTypeFixed64,
        FieldDescriptorProto_TYPE_SFIXED32 => WireTypeFixed32,
        FieldDescriptorProto_TYPE_SFIXED64 => WireTypeFixed64,
        FieldDescriptorProto_TYPE_FLOAT    => WireTypeFixed32,
        FieldDescriptorProto_TYPE_DOUBLE   => WireTypeFixed64,
        FieldDescriptorProto_TYPE_STRING   => WireTypeLengthDelimited,
        FieldDescriptorProto_TYPE_BYTES    => WireTypeLengthDelimited,
        FieldDescriptorProto_TYPE_MESSAGE  => WireTypeLengthDelimited,
        FieldDescriptorProto_TYPE_GROUP    => fail!()
    }
}

// size of value for type, None if variable
fn field_type_size(field_type: FieldDescriptorProto_Type) -> Option<u32> {
    match field_type {
        FieldDescriptorProto_TYPE_BOOL => Some(1),
        t if field_type_wire_type(t) == wire_format::WireTypeFixed32 => Some(4),
        t if field_type_wire_type(t) == wire_format::WireTypeFixed64 => Some(8),
        _ => None
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
            remove_to(field.get_type_name(), '.').to_string()
        }).replace(".", "_");
        match field.get_field_type() {
            FieldDescriptorProto_TYPE_MESSAGE => RustMessage(name),
            FieldDescriptorProto_TYPE_ENUM    => RustEnum(name),
            _ => fail!("unknown named type: {}", field.get_field_type()),
        }
    } else if field.has_field_type() {
        rust_name(field.get_field_type())
    } else {
        fail!("neither type_name, nor field_type specified for field: {}", field.get_name());
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
    type_name: RustType,
    number: u32,
    repeated: bool,
    packed: bool,
    repeat_mode: RepeatMode,
}

impl Field {
    fn parse(field: &FieldDescriptorProto, pkg: &str) -> Option<Field> {
        let type_name = field_type_name(field, pkg);
        let repeated = match field.get_label() {
            FieldDescriptorProto_LABEL_REPEATED => true,
            FieldDescriptorProto_LABEL_OPTIONAL |
            FieldDescriptorProto_LABEL_REQUIRED => false,
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
                if packed { RepeatPacked } else { RepeatRegular }
            } else {
                Single
            };
        Some(Field {
            proto_field: field.clone(),
            name: name,
            field_type: field.get_field_type(),
            wire_type: field_type_wire_type(field.get_field_type()),
            type_name: type_name,
            number: field.get_number() as u32,
            repeated: repeated,
            packed: packed,
            repeat_mode: repeat_mode,
        })
    }

    fn tag_size(&self) -> u32 {
        rt::tag_size(self.number)
    }

    // type of field in struct
    fn full_storage_type(&self) -> RustType {
        let c = box self.type_name.clone();
        if self.repeated {
            if self.type_is_not_trivial() {
                RustRepeatedField(c)
            } else {
                RustVec(c)
            }
        } else {
            if self.field_type == FieldDescriptorProto_TYPE_MESSAGE {
                RustSingularPtrField(c)
            } else if self.field_type == FieldDescriptorProto_TYPE_STRING ||
                    self.field_type == FieldDescriptorProto_TYPE_BYTES
            {
                RustSingularField(c)
            } else {
                RustOption(c)
            }
        }
    }

    // for field `foo`, type of param of `fn set_foo(..)`
    fn set_xxx_param_type(&self) -> RustType {
        if self.repeated {
            self.full_storage_type()
        } else if self.field_type == FieldDescriptorProto_TYPE_STRING {
            RustString
        } else {
            self.type_name.clone()
        }
    }

    // for field `foo`, return type of `fn mut_foo(..)`
    fn mut_xxx_return_type(&self) -> RustType {
        RustRef(box if self.repeated {
            self.full_storage_type()
        } else {
            self.type_name.clone()
        })
    }

    // for field `foo`, return type of `fn get_foo(..)`
    fn get_xxx_return_type(&self) -> RustType {
        match self.repeated {
            true => RustRef(box RustSlice(box self.type_name.clone())),
            false => match self.type_is_not_trivial() {
                true => self.type_name.ref_type(),
                false => self.type_name.clone(),
            }
        }
    }

    // fixed size type?
    fn is_fixed(&self) -> bool {
        field_type_size(self.field_type).is_some()
    }

    // must use zigzag encoding?
    fn is_zigzag(&self) -> bool {
        match self.field_type {
            FieldDescriptorProto_TYPE_SINT32 |
            FieldDescriptorProto_TYPE_SINT64 => true,
            _ => false,
        }
    }

    // data is stored in heap
    fn type_is_not_trivial(&self) -> bool {
        match self.field_type {
            FieldDescriptorProto_TYPE_MESSAGE |
            FieldDescriptorProto_TYPE_STRING |
            FieldDescriptorProto_TYPE_BYTES => true,
            _ => false,
        }
    }
}

#[deriving(Clone)]
struct MessageInfo {
    proto_message: DescriptorProto,
    pkg: String,
    prefix: String,
    type_name: String,
    fields: Vec<Field>,
}

impl<'a> MessageInfo {
    fn parse(proto_message: &DescriptorProto, pkg: &str, prefix: &str) -> MessageInfo {
        MessageInfo {
            proto_message: proto_message.clone(),
            pkg: pkg.to_string(),
            prefix: prefix.to_string(),
            type_name: prefix.to_string().append(proto_message.get_name()),
            fields: proto_message.get_field().iter().flat_map(|field| {
                Field::parse(field, pkg).move_iter()
            }).collect(),
        }
    }

    fn has_any_message_field(&self) -> bool {
        for field in self.fields.iter() {
            if field.field_type == FieldDescriptorProto_TYPE_MESSAGE {
                return true;
            }
        }
        false
    }

    fn required_fields(&'a self) -> Vec<&'a Field> {
        let mut r = Vec::new();
        for field in self.fields.iter() {
            if field.proto_field.get_label() == FieldDescriptorProto_LABEL_REQUIRED {
                r.push(field);
            }
        }
        r
    }
}

struct Enum {
    //pkg: String,
    //prefix: String,
    type_name: String,
    values: Vec<EnumValue>,
}

struct EnumValue {
    proto: EnumValueDescriptorProto,
    prefix: String,
}

impl Enum {
    fn parse(proto: &EnumDescriptorProto, _pkg: &str, prefix: &str) -> Enum {
        Enum {
            //pkg: pkg.to_string(),
            //prefix: prefix.to_string(),
            type_name: prefix.to_string().append(proto.get_name()),
            values: proto.get_value().iter().map(|p| EnumValue::parse(p, prefix)).collect(),
        }
    }
}

impl EnumValue {
    fn parse(proto: &EnumValueDescriptorProto, prefix: &str) -> EnumValue {
        EnumValue {
            proto: proto.clone(),
            prefix: prefix.to_string(),
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

    fn rust_name(&self) -> String {
        self.prefix.to_string().append(self.name())
    }
}


struct IndentWriter<'a> {
    // TODO: add mut
    writer: &'a Writer + 'a,
    indent: String,
    msg: Option<&'a MessageInfo>,
    field: Option<&'a Field>,
    en: Option<&'a Enum>,
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

    fn bind_enum<T>(&self, en: &'a Enum, cb: |&mut IndentWriter| -> T) -> T {
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

    fn en(&self) -> &'a Enum {
        self.en.unwrap()
    }

    fn self_field(&self) -> String {
        format!("self.{:s}", self.field().name)
    }

    fn self_field_is_some(&self) -> String {
        assert!(!self.field().repeated);
        format!("{:s}.is_some()", self.self_field())
    }

    fn self_field_is_not_empty(&self) -> String {
        assert!(self.field().repeated);
        format!("!{:s}.is_empty()", self.self_field())
    }

    fn self_field_is_none(&self) -> String {
        assert!(!self.field().repeated);
        format!("{:s}.is_none()", self.self_field())
    }

    // field data viewed as Option
    fn self_field_as_option(&self) -> String {
        assert!(!self.field().repeated);
        // TODO: make it RustType function
        if self.field().type_is_not_trivial() {
            // Singular*Field.as_ref()
            format!("{}.as_ref()", self.self_field())
        } else {
            self.self_field()
        }
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

    fn for_self_field(&mut self, varn: &str, cb: |&mut IndentWriter|) {
        self.for_stmt(format!("{}.iter()", self.self_field()), varn, cb);
    }

    fn self_field_assign<S : Str>(&self, value: S) {
        self.write_line(format!("{:s} = {:s};", self.self_field(), value));
    }

    fn self_field_assign_none(&self) {
        assert!(!self.field().repeated);
        self.self_field_assign("None");
    }

    fn self_field_assign_some<S : Str>(&self, value: S) {
        assert!(!self.field().repeated);
        self.self_field_assign(format!("Some({:s})", value));
    }

    fn self_field_assign_default(&self) {
        assert!(!self.field().repeated);
        if self.field().type_is_not_trivial() {
            self.write_line(format!("{:s}.set_default();", self.self_field()));
        } else {
            self.self_field_assign_some(self.field().type_name.default_value());
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
        self.write_line(format!("{:s}.push({:s});", self.self_field(), value));
    }

    fn self_field_vec_packed_fixed_data_size(&self) -> String {
        assert!(self.field().is_fixed());
        format!("({}.len() * {}) as u32",
            self.self_field(), field_type_size(self.field().field_type).unwrap())
    }

    fn self_field_vec_packed_varint_data_size(&self) -> String {
        assert!(!self.field().is_fixed());
        let zigzag_suffix = if self.field().is_zigzag() { "_zigzag" } else { "" };
        format!("::protobuf::rt::vec_packed_varint{}_data_size({:s}.as_slice())",
            zigzag_suffix, self.self_field())
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
        let zigzag_suffix = if self.field().is_zigzag() { "_zigzag" } else { "" };
        format!("::protobuf::rt::vec_packed_varint{}_size({:u}, {:s}.as_slice())",
            zigzag_suffix, self.field().number, self.self_field())
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

    fn write_lines(&self, lines: &[String]) {
        for line in lines.iter() {
            self.write_line(line.to_string());
        }
    }

    fn indented(&self, cb: |&mut IndentWriter|) {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: format!("{:s}    ", self.indent),
            msg: self.msg,
            field: self.field,
            en: self.en,
        });
    }

    #[allow(dead_code)]
    fn commented(&self, cb: |&mut IndentWriter|) {
        cb(&mut IndentWriter {
            writer: self.writer,
            indent: format!("// {:s}", self.indent),
            msg: self.msg,
            field: self.field,
            en: self.en,
        });
    }

    fn lazy_static<S1 : Str, S2 : Str>(&mut self, name: S1, ty: S2) {
        self.write_line(format!("static mut {:s}: ::protobuf::lazy::Lazy<{:s}> = ::protobuf::lazy::Lazy {{ lock: ::protobuf::lazy::ONCE_INIT, ptr: 0 as *const {:s} }};", name, ty, ty));
    }

    fn lazy_static_decl_get<S1 : Str, S2 : Str>(&mut self, name: S1, ty: S2, init: |&mut IndentWriter|) {
        self.lazy_static(name.as_slice(), ty);
        self.unsafe_expr(|w| {
            w.write_line(format!("{:s}.get(|| {{", name));
            w.indented(|w| init(w));
            w.write_line(format!("}})"));
        });
    }

    fn block<S1 : Str, S2 : Str>(&self, first_line: S1, last_line: S2, cb: |&mut IndentWriter|) {
        self.write_line(first_line);
        self.indented(cb);
        self.write_line(last_line);
    }

    fn expr_block<S : Str>(&self, prefix: S, cb: |&mut IndentWriter|) {
        self.block(format!("{:s} {{", prefix), "}", cb);
    }

    fn stmt_block<S : Str>(&self, prefix: S, cb: |&mut IndentWriter|) {
        self.block(format!("{:s} {{", prefix), "};", cb);
    }

    fn unsafe_expr(&self, cb: |&mut IndentWriter|) {
        self.expr_block("unsafe", cb);
    }

    fn impl_block<S : Str>(&self, name: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("impl {:s}", name), cb);
    }

    fn impl_self_block<S : Str>(&self, name: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("impl<'a> {:s}", name), cb);
    }

    fn impl_for_block<S1 : Str, S2 : Str>(&self, tr: S1, ty: S2, cb: |&mut IndentWriter|) {
        self.expr_block(format!("impl {:s} for {:s}", tr, ty), cb);
    }

    fn pub_struct<S : Str>(&self, name: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("pub struct {:s}", name), cb);
    }

    fn field_entry<S1 : Str, S2 : Str>(&self, name: S1, value: S2) {
        self.write_line(format!("{:s}: {:s},", name, value));
    }

    #[allow(dead_code)]
    fn fail<S : Str>(&self, reason: S) {
        self.write_line(format!("fail!({:?});", reason));
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
        self.expr_block(format!("pub fn {:s}", sig), cb);
    }

    fn def_fn<S : Str>(&self, sig: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("fn {:s}", sig), cb);
    }

    fn while_block<S : Str>(&self, cond: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("while {:s}", cond), cb);
    }

    fn if_stmt<S : Str>(&self, cond: S, cb: |&mut IndentWriter|) {
        self.stmt_block(format!("if {:s}", cond), cb);
    }

    fn for_stmt<S1 : Str, S2 : Str>(&self, over: S1, varn: S2, cb: |&mut IndentWriter|) {
        self.stmt_block(format!("for {:s} in {:s}", varn, over), cb)
    }

    fn match_block<S : Str>(&self, value: S, cb: |&mut IndentWriter|) {
        self.stmt_block(format!("match {:s}", value), cb);
    }

    fn match_expr<S : Str>(&self, value: S, cb: |&mut IndentWriter|) {
        self.expr_block(format!("match {:s}", value), cb);
    }

    fn case_block<S : Str>(&self, cond: S, cb: |&mut IndentWriter|) {
        self.block(format!("{:s} => {{", cond), "},", cb);
    }

    fn case_expr<S1 : Str, S2 : Str>(&self, cond: S1, body: S2) {
        self.write_line(format!("{:s} => {:s},", cond, body));
    }

    fn clear_field_func(&self) -> String {
        "clear_".to_string().append(self.field.as_ref().unwrap().name.as_slice())
    }

    fn clear_field(&self) {
        if self.field().repeated || self.field().type_is_not_trivial() {
            self.write_line(format!("{:s}.clear();", self.self_field()));
        } else {
            self.self_field_assign_none();
        }
    }
}

fn write_merge_from_field_message_string_bytes(w: &mut IndentWriter) {
    let field = w.field();
    w.write_line(format!("assert_eq!(::protobuf::wire_format::{:?}, wire_type);",
            wire_format::WireTypeLengthDelimited));
    if field.repeated {
        w.write_line(format!("let tmp = {}.push_default();", w.self_field()));
    } else {
        w.write_line(format!("let tmp = {}.set_default();", w.self_field()));
    }
    match field.field_type {
        FieldDescriptorProto_TYPE_MESSAGE =>
            w.write_line(format!("is.merge_message(tmp)")),
        FieldDescriptorProto_TYPE_STRING =>
            w.write_line(format!("is.read_string_into(tmp)")),
        FieldDescriptorProto_TYPE_BYTES =>
            w.write_line(format!("is.read_bytes_into(tmp)")),
        _ =>
            fail!(),
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
                    RepeatRegular
                } else {
                    RepeatPacked // may be both regular or packed
                }
            } else {
                Single
            };

        let read_proc0 = match field.field_type {
            FieldDescriptorProto_TYPE_ENUM => format!("{}::new(is.read_int32())", field.type_name),
            t => format!("is.read_{}()", protobuf_name(t)),
        };
        let read_proc = read_proc0.as_slice();

        match repeat_mode {
            Single | RepeatRegular => {
                w.write_line(format!("assert_eq!(::protobuf::wire_format::{:?}, wire_type);", wire_type));
                w.write_line(format!("let tmp = {:s};", read_proc));
                match repeat_mode {
                    Single => w.self_field_assign_some("tmp"),
                    RepeatRegular => w.self_field_push("tmp"),
                    _ => fail!()
                }
            },
            RepeatPacked => {
                w.write_line(format!("if wire_type == ::protobuf::wire_format::{:?} {{", wire_format::WireTypeLengthDelimited));
                w.indented(|w| {
                    w.write_line("let len = is.read_raw_varint32();");
                    w.write_line("let old_limit = is.push_limit(len);");
                    w.while_block("!is.eof()", |w| {
                        w.self_field_push(read_proc);
                    });
                    w.write_line("is.pop_limit(old_limit);");
                });
                w.write_line("} else {");
                w.indented(|w| {
                    w.write_line(format!("assert_eq!(::protobuf::wire_format::{:?}, wire_type);", wire_type));
                    w.self_field_push(read_proc);
                });
                w.write_line("}");
            },
        };
    }
}

fn write_message_struct(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.deriving(["Clone", "PartialEq", "Default"]);
    w.pub_struct(msg.type_name.as_slice(), |w| {
        w.fields(|w| {
            let field = w.field.unwrap();
            if !format!("{}", field.type_name).as_slice().contains_char('.') {
                w.field_entry(field.name.as_slice(), format!("{}", field.full_storage_type()));
            }
        });
        w.field_entry("unknown_fields", "::protobuf::UnknownFields");
    });
}

fn write_message_compute_sizes(w: &mut IndentWriter) {
    // Append sizes of messages in the tree to the specified vector.
    // First appended element is size of self, and then nested message sizes.
    // in serialization order are appended recursively.");
    w.comment("Compute sizes of nested messages");
    w.def_fn("compute_sizes(&self, sizes: &mut Vec<u32>) -> u32", |w| {
        // To have access to its methods but not polute the name space.
        w.write_line("use protobuf::{Message};");
        w.write_line("let pos = sizes.len();");
        w.write_line("sizes.push(0);");
        w.write_line("let mut my_size = 0;");
        w.fields(|w| {
            let field = w.field();
            match field.repeat_mode {
                Single | RepeatRegular => {
                    match field_type_size(field.field_type) {
                        Some(s) => {
                            if field.repeated {
                                w.write_line(format!(
                                        "my_size += {:d} * {:s}.len() as u32;",
                                        (s + w.field().tag_size()) as int,
                                        w.self_field()));
                            } else {
                                w.if_self_field_is_some(|w| {
                                    w.write_line(format!(
                                            "my_size += {:d};",
                                            (s + w.field().tag_size()) as int));
                                });
                            }
                        },
                        None => {
                            w.for_self_field("value", |w| {
                                match field.field_type {
                                    FieldDescriptorProto_TYPE_MESSAGE => {
                                        w.write_line("let len = value.compute_sizes(sizes);");
                                        w.write_line(format!(
                                                "my_size += {:u} + ::protobuf::rt::compute_raw_varint32_size(len) + len;",
                                                w.field().tag_size() as uint));
                                    },
                                    FieldDescriptorProto_TYPE_BYTES => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::bytes_size({:d}, value.as_slice());",
                                                field.number as int));
                                    },
                                    FieldDescriptorProto_TYPE_STRING => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::string_size({:d}, value.as_slice());",
                                                field.number as int));
                                    },
                                    FieldDescriptorProto_TYPE_ENUM => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::enum_size({:d}, *value);",
                                                field.number as int));
                                    },
                                    _ => {
                                        w.write_line(format!(
                                                "my_size += ::protobuf::rt::value_size({:d}, *value, ::protobuf::wire_format::{:?});",
                                                field.number as int, field.wire_type));
                                    },
                                }
                            });
                        },
                    };
                },
                RepeatPacked => {
                    w.if_self_field_is_not_empty(|w| {
                        let size_expr = w.self_field_vec_packed_size();
                        w.write_line(format!("my_size += {};", size_expr));
                    });
                },
            };
        });
        w.write_line("my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());");
        w.write_line("*sizes.get_mut(pos) = my_size;");
        w.comment("value is returned for convenience");
        w.write_line("my_size");
    });
}

fn write_message_write_field(w: &mut IndentWriter) {
    let field = w.field();
    let field_type = field.field_type;
    let write_method_suffix = match field_type {
        FieldDescriptorProto_TYPE_MESSAGE => "message",
        FieldDescriptorProto_TYPE_ENUM => "enum",
        t => protobuf_name(t),
    };
    let field_number = field.proto_field.get_number();
    let vv = match field.field_type {
        FieldDescriptorProto_TYPE_MESSAGE => "v", // TODO: as &Message
        FieldDescriptorProto_TYPE_ENUM => "*v as i32",
        FieldDescriptorProto_TYPE_BYTES |
        FieldDescriptorProto_TYPE_STRING => "v.as_slice()",
        _ => "*v",
    };
    let write_value_lines = match field.field_type {
        FieldDescriptorProto_TYPE_MESSAGE => vec!(
            format!("os.write_tag({:d}, ::protobuf::wire_format::{:?});",
                    field_number as int, wire_format::WireTypeLengthDelimited),
            format!("os.write_raw_varint32(sizes[*sizes_pos]);"),
            format!("*sizes_pos += 1;"),
            format!("v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos);"),
        ),
        _ => vec!(
            format!("os.write_{:s}({:d}, {:s});", write_method_suffix, field_number as int, vv),
        ),
    };
    match field.repeat_mode {
        Single => {
            w.match_block(w.self_field_as_option(), |w| {
                w.case_block("Some(ref v)", |w| {
                    w.write_lines(write_value_lines.as_slice());
                });
                w.case_expr("None", "{}");
            });
        },
        RepeatPacked => {
            w.if_self_field_is_not_empty(|w| {
                w.write_line(format!("os.write_tag({:d}, ::protobuf::wire_format::{:?});", field_number as int, wire_format::WireTypeLengthDelimited));
                // Data size is computed again here,
                // probably it should be cached in `sizes` vec
                let data_size_expr = w.self_field_vec_packed_data_size();
                w.write_line(format!("os.write_raw_varint32({});", data_size_expr));
                w.for_self_field("v", |w| {
                    w.write_line(format!("os.write_{:s}_no_tag({:s});", write_method_suffix, vv));
                });
            });
        },
        RepeatRegular => {
            w.for_self_field("v", |w| {
                w.write_lines(write_value_lines.as_slice());
            });
        },
    };
}

fn write_message_write_to_with_computed_sizes(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    if !msg.has_any_message_field() {
        // `sizes` and `sizes_pos` are unused
        w.allow(["unused_variable"]);
    }
    w.def_fn("write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint)", |w| {
        // To have access to its methods but not polute the name space.
        w.write_line("use protobuf::{Message};");
        w.fields(|w| {
            write_message_write_field(w);
        });
        w.write_line("os.write_unknown_fields(self.get_unknown_fields());");
    });
}

fn write_message_default_instance(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.pub_fn(format!("default_instance() -> &'static {:s}", msg.type_name), |w| {
        let msg = w.msg.unwrap();
        w.lazy_static_decl_get("instance", msg.type_name.as_slice(), |w| {
            w.expr_block(format!("{:s}", msg.type_name), |w| {
                w.fields(|w| {
                    w.field_default();
                });
                w.field_entry("unknown_fields", "::protobuf::UnknownFields::new()");
            });
        });
    });
}

fn write_message_field_accessors(w: &mut IndentWriter) {
    w.fields(|w| {
        w.write_line("");
        w.pub_fn(format!("{:s}(&mut self)", w.clear_field_func()), |w| {
            w.clear_field();
        });

        if !w.field().repeated {
            w.write_line("");
            w.pub_fn(format!("has_{:s}(&self) -> bool", w.field().name), |w| {
                w.write_line(w.self_field_is_some());
            });
        }

        let set_xxx_param_type = w.field().set_xxx_param_type();
        w.write_line("");
        w.comment("Param is passed by value, moved");
        w.pub_fn(format!("set_{:s}(&mut self, v: {})", w.field().name, set_xxx_param_type), |w| {
            w.self_field_assign_value("v", &set_xxx_param_type);
        });

        let mut_xxx_return_type = w.field().mut_xxx_return_type();
        w.write_line("");
        w.comment("Mutable pointer to the field.");
        if !w.field().repeated {
            w.comment("If field is not initialized, it is initialized with default value first.");
        }
        w.pub_fn(format!("mut_{:s}(&'a mut self) -> {}", w.field().name, mut_xxx_return_type.mut_ref_str("a")),
        |w| {
            if !w.field().repeated {
                w.if_self_field_is_none(|w| {
                    w.self_field_assign_default();
                });
                w.write_line(format!("{:s}.as_mut().unwrap()", w.self_field()));
            } else {
                w.write_line(format!("&mut {:s}", w.self_field()));
            }
        });

        w.write_line("");
        let get_xxx_return_type = w.field().get_xxx_return_type();
        let self_param = match get_xxx_return_type.is_ref() {
            true  => "&'a self",
            false => "&self",
        };
        let get_xxx_return_type_str = get_xxx_return_type.ref_str_safe("a");
        w.pub_fn(format!("get_{:s}({:s}) -> {:s}", w.field().name, self_param, get_xxx_return_type_str),
        |w| {
            if !w.field().repeated {
                if w.field().field_type == FieldDescriptorProto_TYPE_MESSAGE {
                    w.write_line(format!("{:s}.as_ref().unwrap_or_else(|| {}::default_instance())",
                            w.self_field(), w.field().type_name));
                } else {
                    if get_xxx_return_type.is_ref() {
                        w.match_expr(w.self_field_as_option(), |w| {
                            w.case_expr(
                                "Some(ref v)",
                                w.field().type_name.view_as(&w.field().get_xxx_return_type(), "v")
                            );
                            w.case_expr(
                                "None",
                                w.field().get_xxx_return_type().default_value()
                            );
                        });
                    } else {
                        w.write_line(format!(
                                "{:s}.unwrap_or_else(|| {:s})",
                                w.self_field(), w.field().get_xxx_return_type().default_value()));
                    }
                }
            } else {
                w.write_line(format!("{:s}.as_slice()", w.self_field()));
            }
        });

        if w.field().repeated {
            w.write_line("");
            w.pub_fn(format!("add_{:s}(&mut self, v: {})",
                    w.field().name, w.field().type_name),
            |w| {
                w.self_field_push("v");
            });
        }
    });
}

fn write_message_impl_self(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.impl_self_block(msg.type_name.as_slice(), |w| {
        w.pub_fn(format!("new() -> {:s}", msg.type_name), |w| {
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
    w.def_fn(format!("merge_from(&mut self, is: &mut ::protobuf::CodedInputStream)"), |w| {
        w.while_block("!is.eof()", |w| {
            w.write_line(format!("let (field_number, wire_type) = is.read_tag_unpack();"));
            w.match_block("field_number", |w| {
                w.fields(|w| {
                    w.case_block(w.field().number.to_string(), |w| {
                        write_merge_from_field(w);
                    });
                });
                w.case_block("_", |w| {
                    w.write_line("let unknown = is.read_unknown(wire_type);");
                    w.write_line("self.mut_unknown_fields().add_value(field_number, unknown);");
                });
            });
        });
    });
}

fn write_message_impl_message(w: &mut IndentWriter) {
    let msg = w.msg.unwrap();
    w.impl_for_block("::protobuf::Message", msg.type_name.as_slice(), |w| {
        w.def_fn(format!("new() -> {:s}", msg.type_name), |w| {
            w.write_line(format!("{:s}::new()", msg.type_name));
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
        write_message_compute_sizes(w);
        w.write_line("");
        write_message_write_to_with_computed_sizes(w);
        w.write_line("");
        write_message_unknown_fields(w);
        w.write_line("");
        w.allow(["unused_unsafe", "unused_mut"]);
        w.def_fn(format!("descriptor_static(_: Option<{}>) -> &'static ::protobuf::reflect::MessageDescriptor", msg.type_name), |w| {
            w.lazy_static_decl_get("descriptor", "::protobuf::reflect::MessageDescriptor", |w| {
                let vec_type_param = format!(
                        "&'static ::protobuf::reflect::FieldAccessor<{}>",
                        msg.type_name);
                w.write_line(format!("let mut fields: Vec<{}> = Vec::new();", vec_type_param));
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
    w.allow(["non_camel_case_types"]);
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
            FieldDescriptorProto_TYPE_MESSAGE => "message".to_string(),
            FieldDescriptorProto_TYPE_ENUM    => "enum".to_string(),
            FieldDescriptorProto_TYPE_STRING  => "str".to_string(),
            FieldDescriptorProto_TYPE_BYTES   => "bytes".to_string(),
            _ => field.type_name.to_string(),
        };

        w.write_line("");
        if field.repeated {
            match field.field_type {
                FieldDescriptorProto_TYPE_MESSAGE => {
                    w.def_fn(format!("get_rep_message_item<'a>(&self, m: &'a {}, index: uint) -> &'a ::protobuf::Message",
                            msg.type_name),
                    |w| {
                        w.write_line(format!("&m.get_{}()[index] as &'a ::protobuf::Message", field.name));
                    });
                },
                FieldDescriptorProto_TYPE_ENUM => {
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
                FieldDescriptorProto_TYPE_MESSAGE => {
                    w.def_fn(format!("get_message<'a>(&self, m: &'a {}) -> &'a ::protobuf::Message",
                            msg.type_name),
                    |w| {
                        w.write_line(format!("m.get_{}() as &'a ::protobuf::Message", field.name));
                    });
                },
                FieldDescriptorProto_TYPE_ENUM => {
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
                w.write_line(format!("self.{:s}();", w.clear_field_func()));
            });
            w.write_line("self.unknown_fields.clear();");
        });
    });
}

fn write_message(msg: &MessageInfo, w: &mut IndentWriter) {
    let pkg = msg.pkg.as_slice();
    let message_type = &msg.proto_message;

    w.bind_message(msg, |w| {
        write_message_struct(w);
        w.write_line("");
        write_message_impl_self(w);
        w.write_line("");
        write_message_impl_message(w);
        w.write_line("");
        write_message_impl_clear(w);
        w.write_line("");
        write_message_impl_show(w);
        w.write_line("");
        write_message_descriptor(w);

        for nested_type in message_type.get_nested_type().iter() {
            w.write_line("");
            write_message(&MessageInfo::parse(nested_type, pkg.as_slice(), msg.type_name.to_string().append("_").as_slice()), w);
        }

        for enum_type in message_type.get_enum_type().iter() {
            w.write_line("");
            write_enum(&Enum::parse(enum_type, pkg, msg.type_name.to_string().append("_").as_slice()), w);
        }
    });
}

fn write_enum_struct(w: &mut IndentWriter) {
    w.deriving(["Clone", "PartialEq", "Eq", "Show"]);
    w.write_line(format!("pub enum {:s} {{", w.en().type_name));
    for value in w.en().values.iter() {
        w.write_line(format!("    {:s} = {:i},", value.rust_name(), value.number()));
    }
    w.write_line(format!("}}"));
}

fn write_enum_impl(w: &mut IndentWriter) {
    w.impl_block(w.en().type_name.as_slice(), |w| {
        w.pub_fn(format!("new(value: i32) -> {:s}", w.en().type_name), |w| {
            w.match_expr("value", |w| {
                for value in w.en().values.iter() {
                    w.write_line(format!("{:d} => {:s},", value.number(), value.rust_name()));
                }
                w.write_line(format!("_ => fail!()"));
            });
        });
    });
}

fn write_enum_impl_enum(w: &mut IndentWriter) {
    w.impl_for_block("::protobuf::ProtobufEnum", w.en().type_name.as_slice(), |w| {
        w.def_fn("value(&self) -> i32", |w| {
            w.write_line("*self as i32")
        });
        w.write_line("");
        w.def_fn(format!("enum_descriptor_static(_: Option<{}>) -> &'static ::protobuf::reflect::EnumDescriptor", w.en().type_name), |w| {
            w.lazy_static_decl_get("descriptor", "::protobuf::reflect::EnumDescriptor", |w| {
                w.write_line(format!("::protobuf::reflect::EnumDescriptor::new(\"{}\", file_descriptor_proto())", w.en().type_name));
            });
        });
    });
}

fn write_enum(en: &Enum, w: &mut IndentWriter) {
    w.bind_enum(en, |w| {
        write_enum_struct(w);
        w.write_line("");
        write_enum_impl(w);
        w.write_line("");
        write_enum_impl_enum(w);
    });
}

fn proto_path_to_rust_base<'s>(path: &'s str) -> &'s str {
    remove_suffix(remove_to(path, '/'), ".proto")
}

pub struct GenResult {
    pub name: String,
    pub content: Vec<u8>,
}

pub struct GenOptions {
    pub dummy: bool,
}

pub fn gen(files: &[FileDescriptorProto], _: &GenOptions) -> Vec<GenResult> {
    let mut results: Vec<GenResult> = Vec::new();
    let files_map: HashMap<&str, &FileDescriptorProto> = files.iter().map(|f| (f.get_name(), f)).collect();

    for file in files.iter() {
        let base = proto_path_to_rust_base(file.get_name());

        let mut os = VecWriter::new();

        {
            let mut w = IndentWriter::new(&mut os as &mut Writer);

            w.write_line("// This file is generated. Do not edit");

            w.write_line("");
            w.write_line("#![allow(dead_code)]");
            w.write_line("#![allow(non_camel_case_types)]");
            w.write_line("#![allow(unused_imports)]");

            w.write_line("");
            for dep in file.get_dependency().iter() {
                for message in files_map[dep.as_slice()].get_message_type().iter() {
                    w.write_line(format!("use super::{:s}::{:s};", proto_path_to_rust_base(dep.as_slice()), message.get_name()));
                }
            }

            {
                w.write_line("");
                let fdp_bytes = file.write_to_bytes();
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
                    w.write_line("::protobuf::parse_from_bytes(file_descriptor_proto_data)");
                });
                w.write_line("");
                // XXX: this broke due to the glob import and recent rust changes
                //w.pub_fn("file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto", |w| {
                w.def_fn("file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto", |w| {
                    w.unsafe_expr(|w| {
                        w.block("file_descriptor_proto_lazy.get(|| {", "})", |w| {
                            w.write_line("parse_descriptor_proto()");
                        });
                    });
                });
            }

            for message_type in file.get_message_type().iter() {
                w.write_line("");
                write_message(&MessageInfo::parse(message_type, file.get_package(), ""), &mut w);
            }
            for enum_type in file.get_enum_type().iter() {
                w.write_line("");
                write_enum(&Enum::parse(enum_type, file.get_package(), ""), &mut w);
            }
        }

        results.push(GenResult {
            name: base.to_string().append(".rs"),
            content: os.vec,
        });
    }
    results
}

