use descriptor::*;
use descriptorx::*;
use rustproto;
use wire_format;
use rt;
use rust;
use text_format;
use code_writer::CodeWriter;
use types::ProtobufType;
use reflect::ProtobufValue;

use ext::ExtFieldOptional;

use super::message::*;
use super::rust_types_values::*;
use super::enums::*;



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

fn field_type_wire_type(field_type: FieldDescriptorProto_Type) -> wire_format::WireType {
    use stream::wire_format::*;
    match field_type {
        FieldDescriptorProto_Type::TYPE_INT32 => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_INT64 => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_UINT32 => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_UINT64 => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_SINT32 => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_SINT64 => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_BOOL => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_ENUM => WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_FIXED32 => WireTypeFixed32,
        FieldDescriptorProto_Type::TYPE_FIXED64 => WireTypeFixed64,
        FieldDescriptorProto_Type::TYPE_SFIXED32 => WireTypeFixed32,
        FieldDescriptorProto_Type::TYPE_SFIXED64 => WireTypeFixed64,
        FieldDescriptorProto_Type::TYPE_FLOAT => WireTypeFixed32,
        FieldDescriptorProto_Type::TYPE_DOUBLE => WireTypeFixed64,
        FieldDescriptorProto_Type::TYPE_STRING => WireTypeLengthDelimited,
        FieldDescriptorProto_Type::TYPE_BYTES => WireTypeLengthDelimited,
        FieldDescriptorProto_Type::TYPE_MESSAGE => WireTypeLengthDelimited,
        FieldDescriptorProto_Type::TYPE_GROUP => WireTypeLengthDelimited, // not true
    }
}

fn type_protobuf_name(field_type: FieldDescriptorProto_Type) -> &'static str {
    match field_type {
        FieldDescriptorProto_Type::TYPE_INT32 => "int32",
        FieldDescriptorProto_Type::TYPE_INT64 => "int64",
        FieldDescriptorProto_Type::TYPE_UINT32 => "uint32",
        FieldDescriptorProto_Type::TYPE_UINT64 => "uint64",
        FieldDescriptorProto_Type::TYPE_SINT32 => "sint32",
        FieldDescriptorProto_Type::TYPE_SINT64 => "sint64",
        FieldDescriptorProto_Type::TYPE_BOOL => "bool",
        FieldDescriptorProto_Type::TYPE_FIXED32 => "fixed32",
        FieldDescriptorProto_Type::TYPE_FIXED64 => "fixed64",
        FieldDescriptorProto_Type::TYPE_SFIXED32 => "sfixed32",
        FieldDescriptorProto_Type::TYPE_SFIXED64 => "sfixed64",
        FieldDescriptorProto_Type::TYPE_FLOAT => "float",
        FieldDescriptorProto_Type::TYPE_DOUBLE => "double",
        FieldDescriptorProto_Type::TYPE_STRING => "string",
        FieldDescriptorProto_Type::TYPE_BYTES => "bytes",
        FieldDescriptorProto_Type::TYPE_ENUM |
        FieldDescriptorProto_Type::TYPE_MESSAGE |
        FieldDescriptorProto_Type::TYPE_GROUP => panic!(),
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
        _ => None,
    }
}


#[derive(Clone, Debug)]
struct EntryKeyValue(GenProtobufType, GenProtobufType);

#[derive(Clone, Debug)]
pub enum GenProtobufType {
    Primitive(FieldDescriptorProto_Type, PrimitiveTypeVariant),
    // name, file name
    Message(String, String),
    // name, file name, default value
    Enum(String, String, String),
    Group,
}

impl GenProtobufType {
    fn proto_type(&self) -> FieldDescriptorProto_Type {
        match *self {
            GenProtobufType::Primitive(t, ..) => t,
            GenProtobufType::Group => FieldDescriptorProto_Type::TYPE_GROUP,
            GenProtobufType::Message(..) => FieldDescriptorProto_Type::TYPE_MESSAGE,
            GenProtobufType::Enum(..) => FieldDescriptorProto_Type::TYPE_ENUM,
        }
    }

    fn is_copy(&self) -> bool {
        type_is_copy(self.proto_type())
    }

    pub fn rust_type(&self) -> RustType {
        match *self {
            GenProtobufType::Primitive(t, PrimitiveTypeVariant::Default) => rust_name(t),
            GenProtobufType::Primitive(
                FieldDescriptorProto_Type::TYPE_STRING,
                PrimitiveTypeVariant::Carllerche,
            ) => RustType::Chars,
            GenProtobufType::Primitive(
                FieldDescriptorProto_Type::TYPE_BYTES,
                PrimitiveTypeVariant::Carllerche,
            ) => RustType::Bytes,
            GenProtobufType::Primitive(.., PrimitiveTypeVariant::Carllerche) => unreachable!(),
            GenProtobufType::Group => RustType::Group,
            GenProtobufType::Message(ref name, _) => RustType::Message(name.clone()),
            GenProtobufType::Enum(ref name, _, ref default_value) => {
                RustType::Enum(name.clone(), default_value.clone())
            }
        }
    }

    fn protobuf_type_gen(&self) -> ProtobufTypeGen {
        match *self {
            GenProtobufType::Primitive(t, v) => ProtobufTypeGen::Primitive(t, v),
            GenProtobufType::Message(ref name, ..) => ProtobufTypeGen::Message(name.clone()),
            GenProtobufType::Enum(ref name, ..) => ProtobufTypeGen::Enum(name.clone()),
            GenProtobufType::Group => unreachable!(),
        }
    }

    /// implementation of ProtobufType trait
    fn lib_protobuf_type(&self) -> String {
        self.protobuf_type_gen().rust_type()
    }

    fn primitive_type_variant(&self) -> PrimitiveTypeVariant {
        match self {
            &GenProtobufType::Primitive(_, v) => v,
            _ => PrimitiveTypeVariant::Default,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum SingularFieldFlag {
    // proto2 or proto3 message
    WithFlag { required: bool },
    // proto3
    WithoutFlag,
}

impl SingularFieldFlag {
    pub fn is_required(&self) -> bool {
        match *self {
            SingularFieldFlag::WithFlag { required } => required,
            SingularFieldFlag::WithoutFlag => false,
        }
    }
}

#[derive(Clone)]
pub struct SingularField {
    pub flag: SingularFieldFlag,
    pub elem: GenProtobufType,
}

impl SingularField {
    fn rust_type(&self) -> RustType {
        match self.flag {
            SingularFieldFlag::WithFlag { .. } => {
                match self.elem.proto_type() {
                    FieldDescriptorProto_Type::TYPE_MESSAGE => RustType::SingularPtrField(
                        Box::new(self.elem.rust_type()),
                    ),
                    FieldDescriptorProto_Type::TYPE_STRING |
                    FieldDescriptorProto_Type::TYPE_BYTES
                        if self.elem.primitive_type_variant() == PrimitiveTypeVariant::Default => {
                        RustType::SingularField(Box::new(self.elem.rust_type()))
                    }
                    _ => RustType::Option(Box::new(self.elem.rust_type())),
                }
            }
            SingularFieldFlag::WithoutFlag => self.elem.rust_type(),
        }
    }
}

// oneof one { ... }
#[derive(Clone)]
pub struct OneofField {
    elem: GenProtobufType,
    oneof_name: String,
    oneof_type_name: RustType,
    boxed: bool,
}

impl OneofField {
    fn parse(
        oneof: &OneofWithContext,
        _field: &FieldDescriptorProto,
        elem: GenProtobufType,
    ) -> OneofField {
        // detecting recursion
        let boxed = if let &GenProtobufType::Message(ref name, ..) = &elem {
            if *name == oneof.message.rust_name() {
                true
            } else {
                false
            }
        } else {
            false
        };

        OneofField {
            elem: elem,
            oneof_name: oneof.name().to_string(),
            oneof_type_name: RustType::Oneof(oneof.rust_name()),
            boxed: boxed,
        }
    }

    fn rust_type(&self) -> RustType {
        let t = self.elem.rust_type();

        if self.boxed {
            RustType::Uniq(Box::new(t))
        } else {
            t
        }
    }
}

#[derive(Clone)]
pub struct RepeatedField {
    pub elem: GenProtobufType,
    pub packed: bool,
}

impl RepeatedField {
    fn rust_type(&self) -> RustType {
        if !self.elem.is_copy() &&
            self.elem.primitive_type_variant() != PrimitiveTypeVariant::Carllerche
        {
            RustType::RepeatedField(Box::new(self.elem.rust_type()))
        } else {
            RustType::Vec(Box::new(self.elem.rust_type()))
        }
    }
}

#[derive(Clone)]
pub struct MapField {
    name: String,
    key: GenProtobufType,
    value: GenProtobufType,
}

#[derive(Clone)]
pub enum FieldKind {
    // optional or required
    Singular(SingularField),
    // repeated except map
    Repeated(RepeatedField),
    // map
    Map(MapField),
    // part of oneof
    Oneof(OneofField),
}

impl FieldKind {
    fn primitive_type_variant(&self) -> PrimitiveTypeVariant {
        match self {
            &FieldKind::Singular(ref s) => s.elem.primitive_type_variant(),
            &FieldKind::Repeated(ref r) => r.elem.primitive_type_variant(),
            _ => panic!("not a primitive type"), // TODO: map
        }
    }
}

enum FieldElem {
    Primitive(FieldDescriptorProto_Type, PrimitiveTypeVariant),
    // name, file name, entry
    Message(String, String, Option<Box<EntryKeyValue>>),
    // name, file name, default value
    Enum(String, String, String),
    Group,
}

impl FieldElem {
    fn into_type(self) -> GenProtobufType {
        match self {
            FieldElem::Primitive(t, v) => GenProtobufType::Primitive(t, v),
            FieldElem::Message(name, file_name, None) => GenProtobufType::Message(name, file_name),
            // TODO: replace with unreachable
            FieldElem::Message(name, file_name, Some(..)) => {
                GenProtobufType::Message(name, file_name)
            }
            FieldElem::Enum(name, file_name, default_value) => {
                GenProtobufType::Enum(name, file_name, default_value)
            }
            FieldElem::Group => GenProtobufType::Group,
        }
    }
}

fn join_field_ext<A : ProtobufValue + Clone, T : ProtobufType<Value = A>>(
    source: &FieldWithContext,
    field_ext: ExtFieldOptional<FieldOptions, T>,
    message_ext: ExtFieldOptional<MessageOptions, T>,
    file_ext: ExtFieldOptional<FileOptions, T>,
) -> Option<A> {
    if let Some(v) = field_ext.get(source.field.get_options()) {
        return Some(v);
    }
    for m in source.containing_messages() {
        if let Some(v) = message_ext.get(m.get_options()) {
            return Some(v);
        }
    }
    return file_ext.get(source.message.scope.get_file_descriptor().get_options());
}

fn field_elem(
    field: &FieldWithContext,
    root_scope: &RootScope,
    parse_map: bool,
) -> (FieldElem, Option<EnumValueGen>) {
    if field.field.get_field_type() == FieldDescriptorProto_Type::TYPE_GROUP {
        (FieldElem::Group, None)
    } else if field.field.has_type_name() {
        let message_or_enum = root_scope.find_message_or_enum(field.field.get_type_name());
        let file_name = message_or_enum
            .get_scope()
            .file_scope
            .file_descriptor
            .get_name()
            .to_owned();
        let rust_relative_name = type_name_to_rust_relative(
            field.field.get_type_name(),
            field.message.get_scope().file_scope.file_descriptor,
            false,
            root_scope,
        );
        match (field.field.get_field_type(), message_or_enum) {
            (
                FieldDescriptorProto_Type::TYPE_MESSAGE,
                MessageOrEnumWithScope::Message(message_with_scope),
            ) => {
                let entry_key_value = if let (true, Some((key, value))) =
                    (parse_map, message_with_scope.map_entry())
                {
                    Some(Box::new(EntryKeyValue(
                        field_elem(&key, root_scope, false).0.into_type(),
                        field_elem(&value, root_scope, false).0.into_type(),
                    )))
                } else {
                    None
                };
                (
                    FieldElem::Message(rust_relative_name, file_name, entry_key_value),
                    None,
                )
            }
            (
                FieldDescriptorProto_Type::TYPE_ENUM,
                MessageOrEnumWithScope::Enum(enum_with_scope),
            ) => {
                let e = EnumGen::new(
                    &enum_with_scope,
                    field.message.get_scope().get_file_descriptor(),
                );
                let ev = if field.field.has_default_value() {
                    e.value_by_name(field.field.get_default_value()).clone()
                } else {
                    e.values_unique().into_iter().next().unwrap()
                };
                (
                    FieldElem::Enum(
                        rust_relative_name,
                        file_name,
                        enum_with_scope.values()[0].get_name().to_owned(),
                    ),
                    Some(ev),
                )
            }
            _ => panic!("unknown named type: {:?}", field.field.get_field_type()),
        }
    } else if field.field.has_field_type() {
        let carllerche_for_bytes = join_field_ext(
            field,
            rustproto::exts::carllerche_bytes_for_bytes_field,
            rustproto::exts::carllerche_bytes_for_bytes,
            rustproto::exts::carllerche_bytes_for_bytes_all,
        ).unwrap_or(false);
        let carllerche_for_string = join_field_ext(
            field,
            rustproto::exts::carllerche_bytes_for_string_field,
            rustproto::exts::carllerche_bytes_for_string,
            rustproto::exts::carllerche_bytes_for_string_all,
        ).unwrap_or(false);

        let elem = match field.field.get_field_type() {
            FieldDescriptorProto_Type::TYPE_STRING if carllerche_for_string => {
                FieldElem::Primitive(
                    FieldDescriptorProto_Type::TYPE_STRING,
                    PrimitiveTypeVariant::Carllerche,
                )
            }
            FieldDescriptorProto_Type::TYPE_BYTES if carllerche_for_bytes => {
                FieldElem::Primitive(
                    FieldDescriptorProto_Type::TYPE_BYTES,
                    PrimitiveTypeVariant::Carllerche,
                )
            }
            t => FieldElem::Primitive(t, PrimitiveTypeVariant::Default),
        };

        (elem, None)
    } else {
        panic!(
            "neither type_name, nor field_type specified for field: {}",
            field.field.get_name()
        );
    }
}

pub enum AccessorStyle {
    Lambda,
    HasGet,
}

pub struct AccessorFn {
    name: String,
    type_params: Vec<String>,
    pub style: AccessorStyle,
}

impl AccessorFn {
    pub fn sig(&self) -> String {
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

#[derive(Clone)]
pub struct FieldGen<'a> {
    root_scope: &'a RootScope<'a>,
    syntax: Syntax,
    pub proto_field: FieldWithContext<'a>,
    // field name in generated code
    pub rust_name: String,
    pub proto_type: FieldDescriptorProto_Type,
    wire_type: wire_format::WireType,
    enum_default_value: Option<EnumValueGen>,
    pub kind: FieldKind,
    pub expose_field: bool,
    pub generate_accessors: bool,
}

impl<'a> FieldGen<'a> {
    pub fn parse(field: FieldWithContext<'a>, root_scope: &'a RootScope<'a>) -> FieldGen<'a> {
        let (elem, enum_default_value) = field_elem(&field, root_scope, true);

        let default_expose_field = field.message.scope.file_scope.syntax() == Syntax::PROTO3;

        let expose_field = join_field_ext(
            &field,
            rustproto::exts::expose_fields_field,
            rustproto::exts::expose_fields,
            rustproto::exts::expose_fields_all,
        ).unwrap_or(default_expose_field);

        let generate_accessors = join_field_ext(
            &field,
            rustproto::exts::generate_accessors_field,
            rustproto::exts::generate_accessors,
            rustproto::exts::generate_accessors_all,
        ).unwrap_or(true);

        let kind = if field.field.get_label() == FieldDescriptorProto_Label::LABEL_REPEATED {
            match (elem, true) {
                // map field
                (FieldElem::Message(name, _, Some(key_value)), true) => FieldKind::Map(MapField {
                    name: name,
                    key: key_value.0.clone(),
                    value: key_value.1.clone(),
                }),
                // regular repeated field
                (elem, _) => FieldKind::Repeated(RepeatedField {
                    elem: elem.into_type(),
                    packed: field.field.get_options().get_packed(),
                }),
            }
        } else if let Some(oneof) = field.oneof() {
            FieldKind::Oneof(OneofField::parse(&oneof, field.field, elem.into_type()))
        } else {
            let flag = if field.message.scope.file_scope.syntax() == Syntax::PROTO3 &&
                field.field.get_field_type() != FieldDescriptorProto_Type::TYPE_MESSAGE
            {
                SingularFieldFlag::WithoutFlag
            } else {
                SingularFieldFlag::WithFlag {
                    required: field.field.get_label() == FieldDescriptorProto_Label::LABEL_REQUIRED,
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
            rust_name: field.rust_name(),
            proto_type: field.field.get_field_type(),
            wire_type: field_type_wire_type(field.field.get_field_type()),
            enum_default_value: enum_default_value,
            proto_field: field,
            kind: kind,
            expose_field: expose_field,
            generate_accessors: generate_accessors,
        }
    }

    fn tag_size(&self) -> u32 {
        rt::tag_size(self.proto_field.number())
    }

    pub fn is_oneof(&self) -> bool {
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
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => true,
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
    pub fn elem(&self) -> &GenProtobufType {
        match self.kind {
            FieldKind::Singular(SingularField { ref elem, .. }) => &elem,
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => &elem,
            FieldKind::Oneof(OneofField { ref elem, .. }) => &elem,
            FieldKind::Map(..) => unreachable!(),
        }
    }

    // type of field in struct
    pub fn full_storage_type(&self) -> RustType {
        match self.kind {
            FieldKind::Repeated(ref repeated) => repeated.rust_type(),
            FieldKind::Map(MapField { ref key, ref value, .. }) => {
                RustType::HashMap(Box::new(key.rust_type()), Box::new(value.rust_type()))
            }
            FieldKind::Singular(ref singular) => singular.rust_type(),
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    // type of `v` in `for v in field`
    fn full_storage_iter_elem_type(&self) -> RustType {
        if let FieldKind::Oneof(ref oneof) = self.kind {
            oneof.elem.rust_type()
        } else {
            self.full_storage_type().iter_elem_type()
        }
    }

    // suffix `xxx` as in `os.write_xxx_no_tag(..)`
    fn os_write_fn_suffix(&self) -> &str {
        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => "message",
            FieldDescriptorProto_Type::TYPE_ENUM => "enum",
            ty => protobuf_name(ty),
        }
    }

    // type of `v` in `os.write_xxx_no_tag(v)`
    fn os_write_fn_param_type(&self) -> RustType {
        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_STRING => RustType::Ref(Box::new(RustType::Str)),
            FieldDescriptorProto_Type::TYPE_BYTES => RustType::Ref(
                Box::new(RustType::Slice(Box::new(RustType::Int(false, 8)))),
            ),
            FieldDescriptorProto_Type::TYPE_ENUM => RustType::Int(true, 32),
            t => rust_name(t),
        }
    }

    // for field `foo`, type of param of `fn set_foo(..)`
    fn set_xxx_param_type(&self) -> RustType {
        match self.kind {
            FieldKind::Singular(SingularField { ref elem, .. }) |
            FieldKind::Oneof(OneofField { ref elem, .. }) => elem.rust_type(),
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => self.full_storage_type(),
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
            FieldKind::Oneof(OneofField { ref elem, .. }) => elem.rust_type(),
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => self.full_storage_type(),
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
            FieldKind::Map(..) => RustType::Ref(Box::new(self.full_storage_type())),
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
    pub fn elem_type_is_copy(&self) -> bool {
        type_is_copy(self.proto_type)
    }

    fn defaut_value_from_proto_float(&self) -> String {
        let type_name = match self.proto_type {
            FieldDescriptorProto_Type::TYPE_FLOAT => "f32",
            FieldDescriptorProto_Type::TYPE_DOUBLE => "f64",
            _ => unreachable!(),
        };
        let proto_default = self.proto_field.field.get_default_value();

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
            None => format!("{}{}", proto_default, type_name),
        }
    }

    fn default_value_from_proto(&self) -> Option<String> {
        assert!(self.is_singular() || self.is_oneof());
        if self.enum_default_value.is_some() {
            Some(self.enum_default_value.as_ref().unwrap().rust_name_outer())
        } else if self.proto_field.field.has_default_value() {
            let proto_default = self.proto_field.field.get_default_value();
            Some(match self.proto_type {
                // For numeric types, contains the original text representation of the value
                FieldDescriptorProto_Type::TYPE_DOUBLE |
                FieldDescriptorProto_Type::TYPE_FLOAT => self.defaut_value_from_proto_float(),
                FieldDescriptorProto_Type::TYPE_INT32 |
                FieldDescriptorProto_Type::TYPE_SINT32 |
                FieldDescriptorProto_Type::TYPE_SFIXED32 => format!("{}i32", proto_default),
                FieldDescriptorProto_Type::TYPE_UINT32 |
                FieldDescriptorProto_Type::TYPE_FIXED32 => format!("{}u32", proto_default),
                FieldDescriptorProto_Type::TYPE_INT64 |
                FieldDescriptorProto_Type::TYPE_SINT64 |
                FieldDescriptorProto_Type::TYPE_SFIXED64 => format!("{}i64", proto_default),
                FieldDescriptorProto_Type::TYPE_UINT64 |
                FieldDescriptorProto_Type::TYPE_FIXED64 => format!("{}u64", proto_default),

                // For booleans, "true" or "false"
                FieldDescriptorProto_Type::TYPE_BOOL => format!("{}", proto_default),
                // For strings, contains the default text contents (not escaped in any way)
                FieldDescriptorProto_Type::TYPE_STRING => rust::quote_escape_str(proto_default),
                // For bytes, contains the C escaped value.  All bytes >= 128 are escaped
                FieldDescriptorProto_Type::TYPE_BYTES => rust::quote_escape_bytes(
                    &text_format::unescape_string(proto_default),
                ),
                // TODO: resolve outer message prefix
                FieldDescriptorProto_Type::TYPE_GROUP |
                FieldDescriptorProto_Type::TYPE_ENUM => unreachable!(),
                FieldDescriptorProto_Type::TYPE_MESSAGE => {
                    panic!(
                        "default value is not implemented for type: {:?}",
                        self.proto_type
                    )
                }
            })
        } else {
            None
        }
    }

    fn default_value_from_proto_typed(&self) -> Option<RustValueTyped> {
        self.default_value_from_proto().map(|v| {
            let default_value_type = match self.proto_type {
                FieldDescriptorProto_Type::TYPE_STRING => RustType::Ref(Box::new(RustType::Str)),
                FieldDescriptorProto_Type::TYPE_BYTES => {
                    RustType::Ref(Box::new(RustType::Slice(Box::new(RustType::u8()))))
                }
                _ => self.full_storage_iter_elem_type(),
            };

            RustValueTyped {
                value: v,
                rust_type: default_value_type,
            }
        })
    }

    // default value to be returned from fn get_xxx
    fn get_xxx_default_value_rust(&self) -> String {
        assert!(self.is_singular() || self.is_oneof());
        self.default_value_from_proto()
            .unwrap_or_else(|| self.get_xxx_return_type().default_value())
    }

    // default to be assigned to field
    fn element_default_value_rust(&self) -> RustValueTyped {
        assert!(
            self.is_singular() || self.is_oneof(),
            "field is not singular: {}",
            self.reconstruct_def()
        );
        self.default_value_from_proto_typed()
            .unwrap_or_else(|| self.elem().rust_type().default_value_typed())
    }

    pub fn reconstruct_def(&self) -> String {
        let prefix = match (self.proto_field.field.get_label(), self.syntax) {
            (FieldDescriptorProto_Label::LABEL_REPEATED, _) => "repeated ",
            (_, Syntax::PROTO3) => "",
            (FieldDescriptorProto_Label::LABEL_OPTIONAL, _) => "optional ",
            (FieldDescriptorProto_Label::LABEL_REQUIRED, _) => "required ",
        };
        format!(
            "{}{} {} = {}",
            prefix,
            field_type_protobuf_name(&self.proto_field.field),
            self.proto_field.name(),
            self.proto_field.number()
        )
    }

    pub fn accessor_fn(&self) -> AccessorFn {
        match self.kind {
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => {
                let coll = match self.full_storage_type() {
                    RustType::Vec(..) => "vec",
                    RustType::RepeatedField(..) => "repeated_field",
                    _ => unreachable!(),
                };
                let name = format!("make_{}_accessor", coll);
                AccessorFn {
                    name: name,
                    type_params: vec![elem.lib_protobuf_type()],
                    style: AccessorStyle::Lambda,
                }
            }
            FieldKind::Map(MapField { ref key, ref value, .. }) => {
                AccessorFn {
                    name: "make_map_accessor".to_owned(),
                    type_params: vec![key.lib_protobuf_type(), value.lib_protobuf_type()],
                    style: AccessorStyle::Lambda,
                }
            }
            FieldKind::Singular(SingularField {
                ref elem,
                flag: SingularFieldFlag::WithoutFlag,
            }) => {
                if let &GenProtobufType::Message(ref name, _) = elem {
                    // TODO: old style, needed because of default instance

                    AccessorFn {
                        name: "make_singular_message_accessor".to_owned(),
                        type_params: vec![name.clone()],
                        style: AccessorStyle::HasGet,
                    }
                } else {
                    AccessorFn {
                        name: "make_simple_field_accessor".to_owned(),
                        type_params: vec![elem.lib_protobuf_type()],
                        style: AccessorStyle::Lambda,
                    }
                }
            }
            FieldKind::Singular(SingularField {
                ref elem,
                flag: SingularFieldFlag::WithFlag { .. },
            }) => {
                let coll = match self.full_storage_type() {
                    RustType::Option(..) => "option",
                    RustType::SingularField(..) => "singular_field",
                    RustType::SingularPtrField(..) => "singular_ptr_field",
                    _ => unreachable!(),
                };
                let name = format!("make_{}_accessor", coll);
                AccessorFn {
                    name: name,
                    type_params: vec![elem.lib_protobuf_type()],
                    style: AccessorStyle::Lambda,
                }
            }
            FieldKind::Oneof(OneofField { ref elem, .. }) => {
                // TODO: uses old style

                let suffix = match &self.elem().rust_type() {
                    t if t.is_primitive() => format!("{}", t),
                    &RustType::String => "string".to_string(),
                    &RustType::Vec(ref t) if t.is_u8() => "bytes".to_string(),
                    &RustType::Enum(..) => "enum".to_string(),
                    &RustType::Message(..) => "message".to_string(),
                    t => panic!("unexpected field type: {}", t),
                };

                let name = format!("make_singular_{}_accessor", suffix);

                let mut type_params = Vec::new();
                match elem {
                    &GenProtobufType::Message(ref name, _) |
                    &GenProtobufType::Enum(ref name, _, _) => {
                        type_params.push(name.to_owned());
                    }
                    _ => (),
                }

                AccessorFn {
                    name: name,
                    type_params: type_params,
                    style: AccessorStyle::HasGet,
                }
            }
        }
    }

    fn write_clear(&self, w: &mut CodeWriter) {
        if self.is_oneof() {
            w.write_line(&format!(
                "self.{} = ::std::option::Option::None;",
                self.oneof().oneof_name
            ));
        } else {
            let clear_expr = self.full_storage_type().clear(&self.self_field());
            w.write_line(&format!("{};", clear_expr));
        }
    }

    // expression that returns size of data is variable
    fn element_size(&self, var: &str, var_type: &RustType) -> String {
        assert!(!self.is_repeated_packed());

        match field_type_size(self.proto_type) {
            Some(data_size) => format!("{}", data_size + self.tag_size()),
            None => {
                match self.proto_type {
                    FieldDescriptorProto_Type::TYPE_MESSAGE => panic!("not a single-liner"),
                    FieldDescriptorProto_Type::TYPE_BYTES => {
                        format!(
                            "::protobuf::rt::bytes_size({}, &{})",
                            self.proto_field.number(),
                            var
                        )
                    }
                    FieldDescriptorProto_Type::TYPE_STRING => {
                        format!(
                            "::protobuf::rt::string_size({}, &{})",
                            self.proto_field.number(),
                            var
                        )
                    }
                    FieldDescriptorProto_Type::TYPE_ENUM => {
                        let param_type = match var_type {
                            &RustType::Ref(ref t) => (**t).clone(),
                            t => t.clone(),
                        };
                        format!(
                            "::protobuf::rt::enum_size({}, {})",
                            self.proto_field.number(),
                            var_type.into_target(&param_type, var)
                        )
                    }
                    _ => {
                        let param_type = match var_type {
                            &RustType::Ref(ref t) => (**t).clone(),
                            t => t.clone(),
                        };
                        if self.proto_type.is_s_varint() {
                            format!(
                                "::protobuf::rt::value_varint_zigzag_size({}, {})",
                                self.proto_field.number(),
                                var_type.into_target(&param_type, var)
                            )
                        } else {
                            format!(
                                "::protobuf::rt::value_size({}, {}, ::protobuf::wire_format::{:?})",
                                self.proto_field.number(),
                                var_type.into_target(&param_type, var),
                                self.wire_type
                            )
                        }
                    }
                }
            }
        }
    }

    // output code that writes single element to stream
    pub fn write_write_element(&self, w: &mut CodeWriter, os: &str, var: &str, ty: &RustType) {
        if let FieldKind::Repeated(RepeatedField { packed: true, .. }) = self.kind {
            unreachable!();
        };

        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                w.write_line(&format!(
                    "{}.write_tag({}, ::protobuf::wire_format::{:?})?;",
                    os,
                    self.proto_field.number(),
                    wire_format::WireTypeLengthDelimited
                ));
                w.write_line(&format!(
                    "{}.write_raw_varint32({}.get_cached_size())?;",
                    os,
                    var
                ));
                w.write_line(&format!("{}.write_to_with_cached_sizes({})?;", var, os));
            }
            _ => {
                let param_type = self.os_write_fn_param_type();
                let os_write_fn_suffix = self.os_write_fn_suffix();
                let number = self.proto_field.number();
                w.write_line(&format!(
                    "{}.write_{}({}, {})?;",
                    os,
                    os_write_fn_suffix,
                    number,
                    ty.into_target(&param_type, var)
                ));
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

    // type of expression returned by `as_option()`
    fn as_option_type(&self) -> RustType {
        assert!(self.is_singular());
        match self.full_storage_type() {
            RustType::Option(ref e) if e.is_copy() => RustType::Option(e.clone()),
            RustType::Option(e) => RustType::Option(Box::new(e.ref_type())),
            RustType::SingularField(ty) |
            RustType::SingularPtrField(ty) => RustType::Option(Box::new(RustType::Ref(ty))),
            x => panic!("cannot convert {} to option", x),
        }
    }

    // field data viewed as Option
    fn self_field_as_option(&self) -> RustValueTyped {
        assert!(self.is_singular());

        let suffix = match self.full_storage_type() {
            RustType::Option(ref e) if e.is_copy() => "",
            _ => ".as_ref()",
        };

        self.as_option_type()
            .value(format!("{}{}", self.self_field(), suffix))
    }

    fn write_if_let_self_field_is_some<F>(&self, w: &mut CodeWriter, cb: F)
    where
        F : Fn(&str, &RustType, &mut CodeWriter),
    {
        match self.kind {
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => panic!("field is not singular"),
            FieldKind::Singular(SingularField {
                flag: SingularFieldFlag::WithFlag { .. },
                ref elem,
            }) => {
                let var = "v";
                let ref_prefix = match elem.rust_type().is_copy() {
                    true => "",
                    false => "ref ",
                };
                let as_option = self.self_field_as_option();
                w.if_let_stmt(
                    &format!("Some({}{})", ref_prefix, var),
                    &as_option.value,
                    |w| {
                        let v_type = as_option.rust_type.elem_type();
                        cb(var, &v_type, w);
                    },
                );
            }
            FieldKind::Singular(SingularField {
                flag: SingularFieldFlag::WithoutFlag,
                ref elem,
            }) => {
                match *elem {
                    GenProtobufType::Primitive(FieldDescriptorProto_Type::TYPE_STRING, ..) |
                    GenProtobufType::Primitive(FieldDescriptorProto_Type::TYPE_BYTES, ..) => {
                        w.if_stmt(format!("!{}.is_empty()", self.self_field()), |w| {
                            cb(&self.self_field(), &self.full_storage_type(), w);
                        });
                    }
                    _ => {
                        w.if_stmt(
                            format!(
                                "{} != {}",
                                self.self_field(),
                                self.full_storage_type().default_value()
                            ),
                            |w| { cb(&self.self_field(), &self.full_storage_type(), w); },
                        );
                    }
                }
            }
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    fn write_if_self_field_is_not_empty<F>(&self, w: &mut CodeWriter, cb: F)
    where
        F : Fn(&mut CodeWriter),
    {
        assert!(self.is_repeated_or_map());
        let self_field_is_not_empty = self.self_field_is_not_empty();
        w.if_stmt(self_field_is_not_empty, cb);
    }

    pub fn write_if_self_field_is_none<F>(&self, w: &mut CodeWriter, cb: F)
    where
        F : Fn(&mut CodeWriter),
    {
        let self_field_is_none = self.self_field_is_none();
        w.if_stmt(self_field_is_none, cb)
    }

    // repeated or singular
    pub fn write_for_self_field<F>(&self, w: &mut CodeWriter, varn: &str, cb: F)
    where
        F : Fn(&mut CodeWriter, &RustType),
    {
        match self.kind {
            FieldKind::Oneof(OneofField {
                ref elem,
                ref oneof_type_name,
                ..
            }) => {
                let cond = format!(
                    "Some({}::{}(ref {}))",
                    oneof_type_name,
                    self.rust_name,
                    varn
                );
                w.if_let_stmt(
                    &cond,
                    &self.self_field_oneof(),
                    |w| cb(w, &elem.rust_type()),
                )
            }
            _ => {
                let v_type = self.full_storage_iter_elem_type();
                let self_field = self.self_field();
                w.for_stmt(&format!("&{}", self_field), varn, |w| cb(w, &v_type));
            }
        }
    }

    fn write_self_field_assign(&self, w: &mut CodeWriter, value: &str) {
        let self_field = self.self_field();
        w.write_line(&format!("{} = {};", self_field, value));
    }

    fn write_self_field_assign_some(&self, w: &mut CodeWriter, value: &str) {
        let full_storage_type = self.full_storage_type();
        match self.singular() {
            &SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. } => {
                self.write_self_field_assign(w, &full_storage_type.wrap_value(value));
            }
            &SingularField { flag: SingularFieldFlag::WithoutFlag, .. } => {
                self.write_self_field_assign(w, value);
            }
        }
    }

    fn write_self_field_assign_value(&self, w: &mut CodeWriter, value: &str, ty: &RustType) {
        match self.kind {
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => {
                let converted = ty.into_target(&self.full_storage_type(), value);
                self.write_self_field_assign(w, &converted);
            }
            FieldKind::Singular(SingularField { ref elem, ref flag }) => {
                let converted = ty.into_target(&elem.rust_type(), value);
                let wrapped = if *flag == SingularFieldFlag::WithoutFlag {
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
            w.write_line(format!(
                "{} = ::std::option::Option::Some({}({}))",
                self_field_oneof,
                self.variant_path(),
                // TODO: default from .proto is not needed here (?)
                self.element_default_value_rust()
                    .into_type(self.full_storage_iter_elem_type())
                    .value
            ));
        } else {
            match self.full_storage_type() {
                RustType::SingularField(..) |
                RustType::SingularPtrField(..) => {
                    let self_field = self.self_field();
                    w.write_line(&format!("{}.set_default();", self_field));
                }
                _ => {
                    self.write_self_field_assign_some(w, &self.element_default_value_rust().value);
                }
            }
        }
    }

    fn self_field_vec_packed_fixed_data_size(&self) -> String {
        assert!(self.is_fixed());
        format!(
            "({}.len() * {}) as u32",
            self.self_field(),
            field_type_size(self.proto_type).unwrap()
        )
    }

    fn self_field_vec_packed_varint_data_size(&self) -> String {
        assert!(!self.is_fixed());
        let fn_name = if self.is_enum() {
            "vec_packed_enum_data_size".to_string()
        } else {
            let zigzag_suffix = if self.is_zigzag() { "_zigzag" } else { "" };
            format!("vec_packed_varint{}_data_size", zigzag_suffix)
        };
        format!("::protobuf::rt::{}(&{})", fn_name, self.self_field())
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
        format!(
            "{} + ::protobuf::rt::compute_raw_varint32_size({}.len() as u32) + {}",
            self.tag_size(),
            self.self_field(),
            self.self_field_vec_packed_fixed_data_size()
        )
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
        format!(
            "::protobuf::rt::{}({}, &{})",
            fn_name,
            self.proto_field.number(),
            self.self_field()
        )
    }

    fn self_field_oneof(&self) -> String {
        format!("self.{}", self.oneof().oneof_name)
    }

    pub fn clear_field_func(&self) -> String {
        format!("clear_{}", self.rust_name)
    }


    fn write_merge_from_field_message_string_bytes(&self, w: &mut CodeWriter) {
        let singular_or_repeated = match self.kind {
            FieldKind::Repeated(..) => "repeated",
            FieldKind::Map(..) => "repeated", // TODO
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. }) => {
                "singular"
            }
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithoutFlag, .. }) => {
                "singular_proto3"
            }
            FieldKind::Oneof(..) => unreachable!(),
        };
        let carllerche = match self.kind.primitive_type_variant() {
            PrimitiveTypeVariant::Carllerche => "carllerche_",
            PrimitiveTypeVariant::Default => "",
        };
        let type_name_for_fn = protobuf_name(self.proto_type);
        w.write_line(&format!(
            "::protobuf::rt::read_{}_{}{}_into(wire_type, is, &mut self.{})?;",
            singular_or_repeated,
            carllerche,
            type_name_for_fn,
            self.rust_name
        ));
    }

    fn write_merge_from_oneof(&self, f: &OneofField, w: &mut CodeWriter) {
        w.assert_wire_type(self.wire_type);

        let typed = RustValueTyped {
            value: format!("{}?", self.proto_type.read("is")),
            rust_type: self.full_storage_iter_elem_type(),
        };

        let maybe_boxed = if f.boxed { typed.boxed() } else { typed };

        w.write_line(&format!(
            "self.{} = ::std::option::Option::Some({}({}));",
            self.oneof().oneof_name,
            self.variant_path(),
            maybe_boxed.value
        )); // TODO: into_type
    }

    fn write_merge_from_map(&self, w: &mut CodeWriter) {
        let &MapField { ref key, ref value, .. } = self.map();
        w.write_line(&format!(
            "::protobuf::rt::read_map_into::<{}, {}>(wire_type, is, &mut {})?;",
            key.lib_protobuf_type(),
            value.lib_protobuf_type(),
            self.self_field()
        ));
    }

    pub fn write_merge_from_field(&self, w: &mut CodeWriter) {
        match self.kind {
            FieldKind::Oneof(ref f) => self.write_merge_from_oneof(&f, w),
            FieldKind::Map(..) => self.write_merge_from_map(w),
            _ => {
                if !self.elem_type_is_copy() {
                    self.write_merge_from_field_message_string_bytes(w);
                } else {
                    let wire_type = field_type_wire_type(self.proto_type);
                    let read_proc = format!("is.read_{}()?", protobuf_name(self.proto_type));

                    match self.kind {
                        FieldKind::Singular(..) => {
                            w.assert_wire_type(wire_type);
                            w.write_line(&format!("let tmp = {};", read_proc));
                            self.write_self_field_assign_some(w, "tmp");
                        }
                        FieldKind::Repeated(..) => {
                            w.write_line(&format!(
                                "::protobuf::rt::read_repeated_{}_into(wire_type, is, &mut self.{})?;",
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

    pub fn write_element_size(
        &self,
        w: &mut CodeWriter,
        item_var: &str,
        item_var_type: &RustType,
        sum_var: &str,
    ) {
        assert!(!self.is_repeated_packed());

        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                w.write_line(&format!("let len = {}.compute_size();", item_var));
                let tag_size = self.tag_size();
                w.write_line(&format!(
                    "{} += {} + ::protobuf::rt::compute_raw_varint32_size(len) + len;",
                    sum_var,
                    tag_size
                ));
            }
            _ => {
                w.write_line(&format!(
                    "{} += {};",
                    sum_var,
                    self.element_size(item_var, item_var_type)
                ));
            }
        }
    }

    pub fn write_message_write_field(&self, w: &mut CodeWriter) {
        match self.kind {
            FieldKind::Singular(..) => {
                self.write_if_let_self_field_is_some(w, |v, v_type, w| {
                    self.write_write_element(w, "os", v, v_type);
                });
            }
            FieldKind::Repeated(RepeatedField { packed: false, .. }) => {
                self.write_for_self_field(w, "v", |w, v_type| {
                    self.write_write_element(w, "os", "v", v_type);
                });
            }
            FieldKind::Repeated(RepeatedField { packed: true, .. }) => {
                self.write_if_self_field_is_not_empty(w, |w| {
                    let number = self.proto_field.number();
                    w.write_line(&format!(
                        "os.write_tag({}, ::protobuf::wire_format::{:?})?;",
                        number,
                        wire_format::WireTypeLengthDelimited
                    ));
                    w.comment("TODO: Data size is computed again, it should be cached");
                    let data_size_expr = self.self_field_vec_packed_data_size();
                    w.write_line(&format!("os.write_raw_varint32({})?;", data_size_expr));
                    self.write_for_self_field(w, "v", |w, v_type| {
                        let param_type = self.os_write_fn_param_type();
                        let os_write_fn_suffix = self.os_write_fn_suffix();
                        w.write_line(&format!(
                            "os.write_{}_no_tag({})?;",
                            os_write_fn_suffix,
                            v_type.into_target(&param_type, "v")
                        ));
                    });
                });
            }
            FieldKind::Map(MapField { ref key, ref value, .. }) => {
                w.write_line(&format!(
                    "::protobuf::rt::write_map_with_cached_sizes::<{}, {}>({}, &{}, os)?;",
                    key.lib_protobuf_type(),
                    value.lib_protobuf_type(),
                    self.proto_field.number(),
                    self.self_field()
                ));
            }
            FieldKind::Oneof(..) => unreachable!(),
        };
    }

    pub fn write_message_compute_field_size(&self, sum_var: &str, w: &mut CodeWriter) {
        match self.kind {
            FieldKind::Singular(..) => {
                self.write_if_let_self_field_is_some(w, |v, v_type, w| {
                    match field_type_size(self.proto_type) {
                        Some(s) => {
                            let tag_size = self.tag_size();
                            w.write_line(&format!("{} += {};", sum_var, (s + tag_size) as isize));
                        }
                        None => {
                            self.write_element_size(w, v, v_type, sum_var);
                        }
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
                            self_field
                        ));
                    }
                    None => {
                        self.write_for_self_field(w, "value", |w, value_type| {
                            self.write_element_size(w, "value", value_type, sum_var);
                        });
                    }
                };
            }
            FieldKind::Map(MapField { ref key, ref value, .. }) => {
                w.write_line(&format!(
                    "{} += ::protobuf::rt::compute_map_size::<{}, {}>({}, &{});",
                    sum_var,
                    key.lib_protobuf_type(),
                    value.lib_protobuf_type(),
                    self.proto_field.number(),
                    self.self_field()
                ));
            }
            FieldKind::Repeated(RepeatedField { packed: true, .. }) => {
                self.write_if_self_field_is_not_empty(w, |w| {
                    let size_expr = self.self_field_vec_packed_size();
                    w.write_line(&format!("{} += {};", sum_var, size_expr));
                });
            }
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    fn write_message_field_get_singular(&self, w: &mut CodeWriter) {
        let get_xxx_return_type = self.get_xxx_return_type();

        if self.proto_type == FieldDescriptorProto_Type::TYPE_MESSAGE {
            let self_field = self.self_field();
            let ref field_type_name = self.elem().rust_type();
            w.write_line(&format!(
                "{}.as_ref().unwrap_or_else(|| {}::default_instance())",
                self_field,
                field_type_name
            ));
        } else {
            let get_xxx_default_value_rust = self.get_xxx_default_value_rust();
            let self_field = self.self_field();
            match self.singular() {
                &SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. } => {
                    if get_xxx_return_type.is_ref() {
                        let as_option = self.self_field_as_option();
                        w.match_expr(&as_option.value, |w| {
                            let v_type = as_option.rust_type.elem_type();
                            let r_type = self.get_xxx_return_type();
                            w.case_expr("Some(v)", v_type.into_target(&r_type, "v"));
                            let get_xxx_default_value_rust = self.get_xxx_default_value_rust();
                            w.case_expr("None", get_xxx_default_value_rust);
                        });
                    } else {
                        w.write_line(&format!(
                            "{}.unwrap_or({})",
                            self_field,
                            get_xxx_default_value_rust
                        ));
                    }
                }
                &SingularField { flag: SingularFieldFlag::WithoutFlag, .. } => {
                    w.write_line(
                        self.full_storage_type()
                            .into_target(&get_xxx_return_type, &self_field),
                    );
                }
            }
        }
    }

    fn write_message_field_get(&self, w: &mut CodeWriter) {
        let get_xxx_return_type = self.get_xxx_return_type();
        let fn_def = format!("get_{}(&self) -> {}", self.rust_name, get_xxx_return_type);

        w.pub_fn(&fn_def, |w| match self.kind {
            FieldKind::Oneof(OneofField { ref elem, .. }) => {
                let self_field_oneof = self.self_field_oneof();
                w.match_expr(self_field_oneof, |w| {
                    let (refv, vtype) = if !self.elem_type_is_copy() {
                        ("ref v", elem.rust_type().ref_type())
                    } else {
                        ("v", elem.rust_type())
                    };
                    w.case_expr(
                        format!(
                            "::std::option::Option::Some({}({}))",
                            self.variant_path(),
                            refv
                        ),
                        vtype.into_target(&get_xxx_return_type, "v"),
                    );
                    w.case_expr("_", self.get_xxx_default_value_rust());
                })
            }
            FieldKind::Singular(..) => {
                self.write_message_field_get_singular(w);
            }
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => {
                let self_field = self.self_field();
                w.write_line(&format!("&{}", self_field));
            }
        });
    }

    fn has_has(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => false,
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. }) => {
                true
            }
            FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithoutFlag, .. }) => {
                false
            }
            FieldKind::Oneof(..) => true,
        }
    }

    fn has_mut(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => true,
            // TODO: string should be public, and mut is not needed
            FieldKind::Singular(..) |
            FieldKind::Oneof(..) => !self.elem_type_is_copy(),
        }
    }

    fn has_take(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => true,
            // TODO: string should be public, and mut is not needed
            FieldKind::Singular(..) |
            FieldKind::Oneof(..) => !self.elem_type_is_copy(),
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
                    w.case_expr(
                        format!("::std::option::Option::Some({}(..))", self.variant_path()),
                        "true",
                    );
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
                let v = set_xxx_param_type.into_target(&self.oneof().rust_type(), "v");
                w.write_line(&format!("{} = ::std::option::Option::Some({}({}))",
                    self_field_oneof, self.variant_path(), v));
            }
        });
    }

    fn write_message_field_mut(&self, w: &mut CodeWriter) {
        let mut_xxx_return_type = self.mut_xxx_return_type();
        w.comment("Mutable pointer to the field.");
        if self.is_singular() {
            w.comment(
                "If field is not initialized, it is initialized with default value first.",
            );
        }
        let fn_def = match mut_xxx_return_type {
            RustType::Ref(ref param) => {
                format!("mut_{}(&mut self) -> &mut {}", self.rust_name, **param)
            }
            _ => panic!("not a ref: {}", mut_xxx_return_type),
        };
        w.pub_fn(&fn_def, |w| {
            match self.kind {
                FieldKind::Repeated(..) |
                FieldKind::Map(..) => {
                    let self_field = self.self_field();
                    w.write_line(&format!("&mut {}", self_field));
                }
                FieldKind::Singular(
                    SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. },
                ) => {
                    self.write_if_self_field_is_none(
                        w,
                        |w| { self.write_self_field_assign_default(w); },
                    );
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
                            self.element_default_value_rust()
                                .into_type(self.oneof().rust_type())
                                .value));
                    });

                    // extract field
                    w.match_expr(self_field_oneof, |w| {
                        w.case_expr(
                            format!(
                                "::std::option::Option::Some({}(ref mut v))",
                                self.variant_path()
                            ),
                            "v",
                        );
                        w.case_expr("_", "panic!()");
                    });
                }
            }
        });
    }

    fn write_message_field_take_oneof(&self, w: &mut CodeWriter) {
        let take_xxx_return_type = self.take_xxx_return_type();

        // TODO: replace with if let
        w.write_line(&format!("if self.{}() {{", self.has_name()));
        w.indented(|w| {
            let self_field_oneof = self.self_field_oneof();
            w.match_expr(format!("{}.take()", self_field_oneof), |w| {
                let value_in_some = self.oneof().rust_type().value("v".to_owned());
                let converted = value_in_some.into_type(self.take_xxx_return_type());
                w.case_expr(
                    format!("::std::option::Option::Some({}(v))", self.variant_path()),
                    &converted.value,
                );
                w.case_expr("_", "panic!()");
            });
        });
        w.write_line("} else {");
        w.indented(|w| {
            w.write_line(
                self.elem()
                    .rust_type()
                    .default_value_typed()
                    .into_type(take_xxx_return_type.clone())
                    .value,
            );
        });
        w.write_line("}");
    }

    fn write_message_field_take(&self, w: &mut CodeWriter) {
        let take_xxx_return_type = self.take_xxx_return_type();
        w.comment("Take field");
        w.pub_fn(
            &format!(
                "take_{}(&mut self) -> {}",
                self.rust_name,
                take_xxx_return_type
            ),
            |w| match self.kind {
                FieldKind::Oneof(..) => {
                    self.write_message_field_take_oneof(w);
                }
                FieldKind::Repeated(..) |
                FieldKind::Map(..) => {
                    w.write_line(&format!(
                        "::std::mem::replace(&mut self.{}, {})",
                        self.rust_name,
                        take_xxx_return_type.default_value()
                    ));

                }
                FieldKind::Singular(SingularField {
                    ref elem,
                    flag: SingularFieldFlag::WithFlag { .. },
                }) => {
                    if !elem.is_copy() {
                        w.write_line(&format!(
                            "{}.take().unwrap_or_else(|| {})",
                            self.self_field(),
                            elem.rust_type().default_value()
                        ));
                    } else {
                        w.write_line(&format!(
                            "{}.take().unwrap_or({})",
                            self.self_field(),
                            self.element_default_value_rust().value
                        ));
                    }
                }
                FieldKind::Singular(SingularField { flag: SingularFieldFlag::WithoutFlag, .. }) => {
                    w.write_line(&format!(
                        "::std::mem::replace(&mut {}, {})",
                        self.self_field(),
                        self.full_storage_type().default_value()
                    ))
                }
            },
        );
    }

    pub fn write_message_single_field_accessors(&self, w: &mut CodeWriter) {
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
    }
}

#[derive(Clone)]
pub struct OneofVariantGen<'a> {
    oneof: &'a OneofGen<'a>,
    variant: OneofVariantWithContext<'a>,
    oneof_field: OneofField,
    pub field: FieldGen<'a>,
    path: String,
}

impl<'a> OneofVariantGen<'a> {
    fn parse(
        oneof: &'a OneofGen<'a>,
        variant: OneofVariantWithContext<'a>,
        field: &'a FieldGen,
    ) -> OneofVariantGen<'a> {
        OneofVariantGen {
            oneof: oneof,
            variant: variant.clone(),
            field: field.clone(),
            path: format!("{}::{}", oneof.type_name, field.rust_name),
            oneof_field: OneofField::parse(
                variant.oneof,
                variant.field,
                field.oneof().elem.clone(),
            ),
        }
    }

    fn rust_type(&self) -> RustType {
        self.oneof_field.rust_type()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}

#[derive(Clone)]
pub struct OneofGen<'a> {
    // Message containing this oneof
    message: &'a MessageGen<'a>,
    oneof: OneofWithContext<'a>,
    type_name: RustType,
    lite_runtime: bool,
}

impl<'a> OneofGen<'a> {
    pub fn parse(message: &'a MessageGen, oneof: OneofWithContext<'a>) -> OneofGen<'a> {
        let rust_name = oneof.rust_name();
        OneofGen {
            message: message,
            oneof: oneof,
            type_name: RustType::Oneof(rust_name),
            lite_runtime: message.lite_runtime,
        }
    }

    pub fn name(&self) -> &str {
        match self.oneof.oneof.get_name() {
            "type" => "field_type",
            "box" => "field_box",
            x => x,
        }
    }

    pub fn variants(&'a self) -> Vec<OneofVariantGen<'a>> {
        self.oneof
            .variants()
            .into_iter()
            .map(|v| {
                let field = self.message
                    .fields
                    .iter()
                    .filter(|f| f.proto_field.name() == v.field.get_name())
                    .next()
                    .expect(&format!("field not found by name: {}", v.field.get_name()));
                OneofVariantGen::parse(self, v, field)
            })
            .collect()
    }

    pub fn full_storage_type(&self) -> RustType {
        RustType::Option(Box::new(self.type_name.clone()))
    }

    pub fn write_enum(&self, w: &mut CodeWriter) {
        let mut derive = vec!["Clone", "PartialEq"];
        if self.lite_runtime {
            derive.push("Debug");
        }
        w.derive(&derive);
        w.pub_enum(&self.type_name.to_string(), |w| {
            for variant in self.variants() {
                w.write_line(&format!(
                    "{}({}),",
                    variant.field.rust_name,
                    &variant.rust_type().to_string()
                ));
            }
        });
    }
}
