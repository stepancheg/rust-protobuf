use protobuf::prelude::*;

use protobuf::descriptor::*;
use protobuf::descriptorx::*;
use protobuf::wire_format;
use protobuf::rt;
use protobuf::rust;
use protobuf::text_format;
use protobuf::text_format::lexer::float;

use super::rust_types_values::*;
use super::enums::*;
use super::code_writer::CodeWriter;

use super::customize::Customize;
use super::customize::customize_from_rustproto_for_field;
use oneof::OneofField;
use map::map_entry;
use ident::RustIdent;
use code_writer::Visibility;
use protobuf::wire_format::WireType;


fn type_is_copy(field_type: FieldDescriptorProto_Type) -> bool {
    match field_type {
        FieldDescriptorProto_Type::TYPE_MESSAGE
        | FieldDescriptorProto_Type::TYPE_STRING
        | FieldDescriptorProto_Type::TYPE_BYTES => false,
        _ => true,
    }
}

trait FieldDescriptorProtoTypeExt {
    fn read(&self, is: &str) -> String;
    fn is_s_varint(&self) -> bool;
}

impl FieldDescriptorProtoTypeExt for FieldDescriptorProto_Type {
    fn read(&self, is: &str) -> String {
        format!("{}.read_{}()", is, protobuf_name(*self))
    }

    /// True if self is signed integer with zigzag encoding
    fn is_s_varint(&self) -> bool {
        match *self {
            FieldDescriptorProto_Type::TYPE_SINT32 | FieldDescriptorProto_Type::TYPE_SINT64 => true,
            _ => false,
        }
    }
}

fn field_type_wire_type(field_type: FieldDescriptorProto_Type) -> WireType {
    match field_type {
        FieldDescriptorProto_Type::TYPE_INT32 => WireType::WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_INT64 => WireType::WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_UINT32 => WireType::WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_UINT64 => WireType::WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_SINT32 => WireType::WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_SINT64 => WireType::WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_BOOL => WireType::WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_ENUM => WireType::WireTypeVarint,
        FieldDescriptorProto_Type::TYPE_FIXED32 => WireType::WireTypeFixed32,
        FieldDescriptorProto_Type::TYPE_FIXED64 => WireType::WireTypeFixed64,
        FieldDescriptorProto_Type::TYPE_SFIXED32 => WireType::WireTypeFixed32,
        FieldDescriptorProto_Type::TYPE_SFIXED64 => WireType::WireTypeFixed64,
        FieldDescriptorProto_Type::TYPE_FLOAT => WireType::WireTypeFixed32,
        FieldDescriptorProto_Type::TYPE_DOUBLE => WireType::WireTypeFixed64,
        FieldDescriptorProto_Type::TYPE_STRING => WireType::WireTypeLengthDelimited,
        FieldDescriptorProto_Type::TYPE_BYTES => WireType::WireTypeLengthDelimited,
        FieldDescriptorProto_Type::TYPE_MESSAGE => WireType::WireTypeLengthDelimited,
        FieldDescriptorProto_Type::TYPE_GROUP => WireType::WireTypeLengthDelimited, // not true
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
        FieldDescriptorProto_Type::TYPE_ENUM
        | FieldDescriptorProto_Type::TYPE_MESSAGE
        | FieldDescriptorProto_Type::TYPE_GROUP => panic!(),
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


/// Optional fields can be stored are `Option<T>`, `SingularField<T>` or `SingularPtrField<T>`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OptionKind {
    /// Field is `Option<T>`
    Option,
    /// Field is `Option<Box<T>>`
    OptionBox,
    /// Field is `SingularField<T>`
    SingularField,
    /// Field is `SingularPtrField<T>`
    SingularPtrField,
}

impl OptionKind {
    fn wrap_element(&self, element_type: RustType) -> RustType {
        let element_type = Box::new(element_type);
        match self {
            OptionKind::Option => RustType::Option(element_type),
            OptionKind::OptionBox => RustType::Option(Box::new(RustType::Uniq(element_type))),
            OptionKind::SingularField => RustType::SingularField(element_type),
            OptionKind::SingularPtrField => RustType::SingularPtrField(element_type),
        }
    }

    // Type of `as_ref()` operation
    fn as_ref_type(&self, element_type: RustType) -> RustType {
        match self {
            OptionKind::Option => {
                RustType::Option(Box::new(element_type.ref_type()))
            },
            OptionKind::OptionBox => {
                RustType::Option(
                    Box::new(RustType::Ref(
                        Box::new(RustType::Uniq(
                            Box::new(element_type))))))
            },
            OptionKind::SingularField => {
                RustType::SingularField(Box::new(element_type.ref_type()))
            },
            OptionKind::SingularPtrField => {
                RustType::SingularPtrField(Box::new(element_type.ref_type()))
            },
        }
    }

    fn _as_option_ref(&self, v: &str) -> String {
        match self {
            OptionKind::OptionBox => format!("{}.as_ref().map(|v| &**v)", v),
            OptionKind::Option |
            OptionKind::SingularField |
            OptionKind::SingularPtrField => format!("{}.as_ref()", v),
        }
    }

    fn unwrap_or_else(&self, what: &str, default_value: &str) -> String {
        match self {
            OptionKind::OptionBox => {
                format!("{}.map(|v| *v).unwrap_or_else(|| {})", what, default_value)
            },
            _ => {
                format!("{}.unwrap_or_else(|| {})", what, default_value)
            },
        }
    }

    fn unwrap_ref_or_else(&self, what: &str, default_value: &str) -> String {
        match self {
            OptionKind::OptionBox => {
                format!("{}.map(|v| v.as_ref()).unwrap_or_else(|| {})", what, default_value)
            },
            _ => {
                format!("{}.unwrap_or_else(|| {})", what, default_value)
            },
        }
    }

    fn wrap_value(&self, value: &str) -> String {
        match self {
            OptionKind::Option => {
                format!("::std::option::Option::Some({})", value)
            },
            OptionKind::OptionBox => {
                // TODO: could reuse allocated memory
                format!("::std::option::Option::Some(Box::new({}))", value)
            },
            OptionKind::SingularField => {
                format!("::protobuf::SingularField::some({})", value)
            },
            OptionKind::SingularPtrField => {
                format!("::protobuf::SingularPtrField::some({})", value)
            },
        }
    }
}


#[derive(Clone, PartialEq, Eq)]
pub enum SingularFieldFlag {
    // proto2 or proto3 message
    WithFlag { required: bool, option_kind: OptionKind },
    // proto3
    WithoutFlag,
}

impl SingularFieldFlag {
    pub fn is_required(&self) -> bool {
        match *self {
            SingularFieldFlag::WithFlag { required, .. } => required,
            SingularFieldFlag::WithoutFlag => false,
        }
    }
}

#[derive(Clone)]
pub struct SingularField {
    pub flag: SingularFieldFlag,
    pub elem: FieldElem,
}

impl SingularField {
    fn rust_storage_type(&self) -> RustType {
        match self.flag {
<<<<<<< HEAD
            SingularFieldFlag::WithFlag { option_kind, .. } => {
                option_kind.wrap_element(self.elem.rust_storage_elem_type())
            }
            SingularFieldFlag::WithoutFlag => self.elem.rust_storage_elem_type(),
=======
            SingularFieldFlag::WithFlag { .. } => match self.elem.proto_type() {
                FieldDescriptorProto_Type::TYPE_MESSAGE => {
                    RustType::SingularPtrField(Box::new(self.elem.rust_type()))
                }
                FieldDescriptorProto_Type::TYPE_STRING | FieldDescriptorProto_Type::TYPE_BYTES
                    if self.elem.primitive_type_variant() == PrimitiveTypeVariant::Default =>
                {
                    RustType::SingularField(Box::new(self.elem.rust_type()))
                }
                _ => RustType::Option(Box::new(self.elem.rust_type())),
            },
            SingularFieldFlag::WithoutFlag => self.elem.rust_type(),
>>>>>>> acm/master
        }
    }
}


/// Repeated field can be `Vec<T>` or `RepeatedField<T>`.
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum RepeatedFieldKind {
    Vec,
    RepeatedField,
}

impl RepeatedFieldKind {
    fn wrap_element(&self, element_type: RustType) -> RustType {
        let element_type = Box::new(element_type);
        match self {
            RepeatedFieldKind::Vec => RustType::Vec(element_type),
            RepeatedFieldKind::RepeatedField => RustType::RepeatedField(element_type),
        }
    }
}

#[derive(Clone)]
pub struct RepeatedField {
    pub elem: FieldElem,
    pub packed: bool,
    pub repeated_field_vec: bool,
}

impl RepeatedField {
<<<<<<< HEAD
    fn kind(&self) -> RepeatedFieldKind {
        if !self.elem.is_copy() &&
            self.elem.primitive_type_variant() != PrimitiveTypeVariant::Carllerche &&
            !self.repeated_field_vec
=======
    fn rust_type(&self) -> RustType {
        if !self.elem.is_copy()
            && self.elem.primitive_type_variant() != PrimitiveTypeVariant::Carllerche
>>>>>>> acm/master
        {
            RepeatedFieldKind::RepeatedField
        } else {
            RepeatedFieldKind::Vec
        }
    }

    fn rust_type(&self) -> RustType {
        self.kind().wrap_element(self.elem.rust_storage_elem_type())
    }
}


#[derive(Clone)]
pub struct MapField {
    name: String,
    key: FieldElem,
    value: FieldElem,
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
    fn elem(&self) -> &FieldElem {
        match self {
            &FieldKind::Singular(ref s) => &s.elem,
            &FieldKind::Repeated(ref r) => &r.elem,
            &FieldKind::Oneof(ref o) => &o.elem,
            &FieldKind::Map(..) => {
                panic!("no single elem type for map field");
            }
        }
    }

    fn primitive_type_variant(&self) -> PrimitiveTypeVariant {
        self.elem().primitive_type_variant()
    }
}

// Representation of map entry: key type and value type
#[derive(Clone, Debug)]
pub struct EntryKeyValue(FieldElem, FieldElem);

#[derive(Clone, Debug)]
pub enum FieldElem {
    Primitive(FieldDescriptorProto_Type, PrimitiveTypeVariant),
    // name, file name, entry
    Message(String, String, Option<Box<EntryKeyValue>>),
    // name, file name, default value
    Enum(String, String, RustIdent),
    Group,
}

impl FieldElem {
    fn proto_type(&self) -> FieldDescriptorProto_Type {
        match *self {
            FieldElem::Primitive(t, ..) => t,
            FieldElem::Group => FieldDescriptorProto_Type::TYPE_GROUP,
            FieldElem::Message(..) => FieldDescriptorProto_Type::TYPE_MESSAGE,
            FieldElem::Enum(..) => FieldDescriptorProto_Type::TYPE_ENUM,
        }
    }

    fn is_copy(&self) -> bool {
        type_is_copy(self.proto_type())
    }

    pub fn rust_storage_elem_type(&self) -> RustType {
        match *self {
            FieldElem::Primitive(t, PrimitiveTypeVariant::Default) => rust_name(t),
            FieldElem::Primitive(
                FieldDescriptorProto_Type::TYPE_STRING,
                PrimitiveTypeVariant::Carllerche,
            ) => RustType::Chars,
            FieldElem::Primitive(
                FieldDescriptorProto_Type::TYPE_BYTES,
                PrimitiveTypeVariant::Carllerche,
            ) => RustType::Bytes,
            FieldElem::Primitive(.., PrimitiveTypeVariant::Carllerche) => unreachable!(),
            FieldElem::Group => RustType::Group,
            FieldElem::Message(ref name, ..) => RustType::Message(name.clone()),
            FieldElem::Enum(ref name, _, ref default_value) => {
                RustType::Enum(name.clone(), default_value.clone())
            }
        }
    }

<<<<<<< HEAD
    fn protobuf_type_gen(&self) -> ProtobufTypeGen {
        match *self {
            FieldElem::Primitive(t, v) => ProtobufTypeGen::Primitive(t, v),
            FieldElem::Message(ref name, ..) => ProtobufTypeGen::Message(name.clone()),
            FieldElem::Enum(ref name, ..) => ProtobufTypeGen::Enum(name.clone()),
            FieldElem::Group => unreachable!(),
        }
=======
fn join_field_ext<A: ProtobufValue + Clone, T: ProtobufType<Value = A>>(
    source: &FieldWithContext,
    field_ext: ExtFieldOptional<FieldOptions, T>,
    message_ext: ExtFieldOptional<MessageOptions, T>,
    file_ext: ExtFieldOptional<FileOptions, T>,
) -> Option<A> {
    if let Some(v) = field_ext.get(source.field.get_options()) {
        return Some(v);
>>>>>>> acm/master
    }

    /// implementation of ProtobufType trait
    fn lib_protobuf_type(&self) -> String {
        self.protobuf_type_gen().rust_type()
    }

    fn primitive_type_variant(&self) -> PrimitiveTypeVariant {
        match self {
            &FieldElem::Primitive(_, v) => v,
            _ => PrimitiveTypeVariant::Default,
        }
    }
}

fn field_elem(
    field: &FieldWithContext,
    root_scope: &RootScope,
    parse_map: bool,
    customize: &Customize,
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
                    (parse_map, map_entry(&message_with_scope))
                {
                    Some(Box::new(EntryKeyValue(
                        field_elem(&key, root_scope, false, customize).0,
                        field_elem(&value, root_scope, false, customize).0,
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
                    customize,
                    root_scope,
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
                        RustIdent(enum_with_scope.values()[0].rust_name().to_owned()),
                    ),
                    Some(ev),
                )
            }
            _ => panic!("unknown named type: {:?}", field.field.get_field_type()),
        }
    } else if field.field.has_field_type() {
        let carllerche_for_bytes = customize.carllerche_bytes_for_bytes.unwrap_or(false);
        let carllerche_for_string = customize.carllerche_bytes_for_string.unwrap_or(false);

        let elem = match field.field.get_field_type() {
            FieldDescriptorProto_Type::TYPE_STRING if carllerche_for_string => {
                FieldElem::Primitive(
                    FieldDescriptorProto_Type::TYPE_STRING,
                    PrimitiveTypeVariant::Carllerche,
                )
            }
            FieldDescriptorProto_Type::TYPE_BYTES if carllerche_for_bytes => FieldElem::Primitive(
                FieldDescriptorProto_Type::TYPE_BYTES,
                PrimitiveTypeVariant::Carllerche,
            ),
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


pub struct AccessorFn {
    name: String,
    // function type params after first underscore
    type_params: Vec<String>,
    callback_params: Vec<String>,
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
    pub rust_name: RustIdent,
    pub proto_type: FieldDescriptorProto_Type,
    wire_type: wire_format::WireType,
    enum_default_value: Option<EnumValueGen>,
    pub kind: FieldKind,
    pub expose_field: bool,
    pub generate_accessors: bool,
    pub generate_getter: bool,
    customize: Customize,
}

impl<'a> FieldGen<'a> {
    pub fn parse(field: FieldWithContext<'a>, root_scope: &'a RootScope<'a>, customize: &Customize)
        -> FieldGen<'a>
    {
        let mut customize = customize.clone();
        customize.update_with(&customize_from_rustproto_for_field(field.field.options.get_message()));

        let (elem, enum_default_value) = field_elem(&field, root_scope, true, &customize);

        let syntax = field.message.scope.file_scope.syntax();

        let field_may_have_custo_default_value = syntax == Syntax::PROTO2
            && field.field.get_label() != FieldDescriptorProto_Label::LABEL_REPEATED
            && field.field.get_field_type() != FieldDescriptorProto_Type::TYPE_MESSAGE;

        let default_expose_field = !field_may_have_custo_default_value;
        let expose_field = customize.expose_fields.unwrap_or(default_expose_field);

        let default_generate_accessors = !expose_field;
        let generate_accessors = customize.generate_accessors.unwrap_or(default_generate_accessors)
            || field.is_oneof();

        let default_generate_getter = generate_accessors || field_may_have_custo_default_value;
        let generate_getter = customize.generate_getter.unwrap_or(default_generate_getter)
            || field.is_oneof();

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
                    elem,
                    packed: field.field.options.get_message().get_packed(),
                    repeated_field_vec: customize.repeated_field_vec.unwrap_or(false),
                }),
            }
        } else if let Some(oneof) = field.oneof() {
            FieldKind::Oneof(OneofField::parse(&oneof, field.field, elem))
        } else {
            let flag = if field.message.scope.file_scope.syntax() == Syntax::PROTO3
                && field.field.get_field_type() != FieldDescriptorProto_Type::TYPE_MESSAGE
            {
                SingularFieldFlag::WithoutFlag
            } else {
                let required = field.field.get_label() == FieldDescriptorProto_Label::LABEL_REQUIRED;
                let option_kind = match field.field.get_field_type() {
                    FieldDescriptorProto_Type::TYPE_MESSAGE => {
                        if customize.singular_field_option_box.unwrap_or(false) {
                            OptionKind::OptionBox
                        } else if customize.singular_field_option.unwrap_or(false) {
                            OptionKind::Option
                        } else {
                            OptionKind::SingularPtrField
                        }
                    },
                    FieldDescriptorProto_Type::TYPE_STRING |
                    FieldDescriptorProto_Type::TYPE_BYTES
                    if elem.primitive_type_variant() == PrimitiveTypeVariant::Default => {
                        OptionKind::SingularField
                    }
                    _ => OptionKind::Option,
                };

                SingularFieldFlag::WithFlag {
                    required,
                    option_kind,
                }
            };
            FieldKind::Singular(SingularField {
                elem,
                flag,
            })
        };

        FieldGen {
            root_scope,
            syntax: field.message.get_scope().file_scope.syntax(),
            rust_name: RustIdent(field.rust_name()),
            proto_type: field.field.get_field_type(),
            wire_type: field_type_wire_type(field.field.get_field_type()),
            enum_default_value,
            proto_field: field,
            kind,
            expose_field,
            generate_accessors,
            generate_getter,
            customize,
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

    pub fn oneof(&self) -> &OneofField {
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
    pub fn elem(&self) -> &FieldElem {
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
<<<<<<< HEAD
            FieldKind::Map(MapField { ref key, ref value, .. }) => {
                RustType::HashMap(Box::new(key.rust_storage_elem_type()), Box::new(value.rust_storage_elem_type()))
            }
            FieldKind::Singular(ref singular) => singular.rust_storage_type(),
=======
            FieldKind::Map(MapField {
                ref key, ref value, ..
            }) => RustType::HashMap(Box::new(key.rust_type()), Box::new(value.rust_type())),
            FieldKind::Singular(ref singular) => singular.rust_type(),
>>>>>>> acm/master
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    // type of `v` in `for v in field`
    fn full_storage_iter_elem_type(&self) -> RustType {
        if let FieldKind::Oneof(ref oneof) = self.kind {
            oneof.elem.rust_storage_elem_type()
        } else {
            self.full_storage_type().iter_elem_type()
        }
    }

    // suffix `xxx` as in `os.write_xxx_no_tag(..)`
    fn os_write_fn_suffix(&self) -> &str {
        protobuf_name(self.proto_type)
    }

    // type of `v` in `os.write_xxx_no_tag(v)`
    fn os_write_fn_param_type(&self) -> RustType {
        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_STRING => RustType::Ref(Box::new(RustType::Str)),
            FieldDescriptorProto_Type::TYPE_BYTES => {
                RustType::Ref(Box::new(RustType::Slice(Box::new(RustType::Int(false, 8)))))
            }
            FieldDescriptorProto_Type::TYPE_ENUM => RustType::Int(true, 32),
            t => rust_name(t),
        }
    }

    // for field `foo`, type of param of `fn set_foo(..)`
    fn set_xxx_param_type(&self) -> RustType {
        match self.kind {
<<<<<<< HEAD
            FieldKind::Singular(SingularField { ref elem, .. }) |
            FieldKind::Oneof(OneofField { ref elem, .. }) => elem.rust_storage_elem_type(),
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => self.full_storage_type(),
=======
            FieldKind::Singular(SingularField { ref elem, .. })
            | FieldKind::Oneof(OneofField { ref elem, .. }) => elem.rust_type(),
            FieldKind::Repeated(..) | FieldKind::Map(..) => self.full_storage_type(),
>>>>>>> acm/master
        }
    }

    // for field `foo`, return type if `fn take_foo(..)`
    fn take_xxx_return_type(&self) -> RustType {
        self.set_xxx_param_type()
    }

    // for field `foo`, return type of `fn mut_foo(..)`
    fn mut_xxx_return_type(&self) -> RustType {
        RustType::Ref(Box::new(match self.kind {
<<<<<<< HEAD
            FieldKind::Singular(SingularField { ref elem, .. }) |
            FieldKind::Oneof(OneofField { ref elem, .. }) => elem.rust_storage_elem_type(),
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => self.full_storage_type(),
=======
            FieldKind::Singular(SingularField { ref elem, .. })
            | FieldKind::Oneof(OneofField { ref elem, .. }) => elem.rust_type(),
            FieldKind::Repeated(..) | FieldKind::Map(..) => self.full_storage_type(),
>>>>>>> acm/master
        }))
    }

    // for field `foo`, return type of `fn get_foo(..)`
    fn get_xxx_return_type(&self) -> RustType {
        match self.kind {
<<<<<<< HEAD
            FieldKind::Singular(SingularField { ref elem, .. }) |
            FieldKind::Oneof(OneofField { ref elem, .. }) => {
                match elem.is_copy() {
                    true => elem.rust_storage_elem_type(),
                    false => elem.rust_storage_elem_type().ref_type(),
                }
            }
=======
            FieldKind::Singular(SingularField { ref elem, .. })
            | FieldKind::Oneof(OneofField { ref elem, .. }) => match elem.is_copy() {
                true => elem.rust_type(),
                false => elem.rust_type().ref_type(),
            },
>>>>>>> acm/master
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => {
                RustType::Ref(Box::new(RustType::Slice(Box::new(elem.rust_storage_elem_type()))))
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
            FieldDescriptorProto_Type::TYPE_SINT32 | FieldDescriptorProto_Type::TYPE_SINT64 => true,
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
        assert!(self.proto_field.field.has_default_value());

        let type_name = match self.proto_type {
            FieldDescriptorProto_Type::TYPE_FLOAT => "f32",
            FieldDescriptorProto_Type::TYPE_DOUBLE => "f64",
            _ => unreachable!(),
        };
        let proto_default = self.proto_field.field.get_default_value();

        let f = float::parse_protobuf_float(proto_default)
            .expect(&format!("failed to parse float: {:?}", proto_default));

        if f.is_nan() {
            format!("::std::{}::NAN", type_name)
        } else if f.is_infinite() {
            if f > 0.0 {
                format!("::std::{}::INFINITY", type_name)
            } else {
                format!("::std::{}::NEG_INFINITY", type_name)
            }
        } else {
            format!("{:?}{}", f, type_name)
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
                FieldDescriptorProto_Type::TYPE_DOUBLE | FieldDescriptorProto_Type::TYPE_FLOAT => {
                    self.defaut_value_from_proto_float()
                }
                FieldDescriptorProto_Type::TYPE_INT32
                | FieldDescriptorProto_Type::TYPE_SINT32
                | FieldDescriptorProto_Type::TYPE_SFIXED32 => format!("{}i32", proto_default),
                FieldDescriptorProto_Type::TYPE_UINT32
                | FieldDescriptorProto_Type::TYPE_FIXED32 => format!("{}u32", proto_default),
                FieldDescriptorProto_Type::TYPE_INT64
                | FieldDescriptorProto_Type::TYPE_SINT64
                | FieldDescriptorProto_Type::TYPE_SFIXED64 => format!("{}i64", proto_default),
                FieldDescriptorProto_Type::TYPE_UINT64
                | FieldDescriptorProto_Type::TYPE_FIXED64 => format!("{}u64", proto_default),

                // For booleans, "true" or "false"
                FieldDescriptorProto_Type::TYPE_BOOL => format!("{}", proto_default),
                // For strings, contains the default text contents (not escaped in any way)
                FieldDescriptorProto_Type::TYPE_STRING => rust::quote_escape_str(proto_default),
                // For bytes, contains the C escaped value.  All bytes >= 128 are escaped
                FieldDescriptorProto_Type::TYPE_BYTES => rust::quote_escape_bytes(
                    &text_format::lexer::StrLit { escaped: proto_default.to_owned() }
                        .decode_bytes().expect("decoded bytes default value")
                ),
                // TODO: resolve outer message prefix
                FieldDescriptorProto_Type::TYPE_GROUP | FieldDescriptorProto_Type::TYPE_ENUM => {
                    unreachable!()
                }
                FieldDescriptorProto_Type::TYPE_MESSAGE => panic!(
                    "default value is not implemented for type: {:?}",
                    self.proto_type
                ),
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
            .unwrap_or_else(|| self.elem().rust_storage_elem_type().default_value_typed())
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

    fn accessor_fn_map(&self, map_field: &MapField) -> AccessorFn {
        let MapField { ref key, ref value, .. } = map_field;
        AccessorFn {
            name: "make_map_accessor".to_owned(),
            type_params: vec![key.lib_protobuf_type(), value.lib_protobuf_type()],
            callback_params: self.make_accessor_fns_lambda(),
        }
    }

    fn accessor_fn_repeated(&self, repeated_field: &RepeatedField) -> AccessorFn {
        let RepeatedField { ref elem, .. } = repeated_field;
        let coll = match self.full_storage_type() {
            RustType::Vec(..) => "vec",
            RustType::RepeatedField(..) => "repeated_field",
            _ => unreachable!(),
        };
        let name = format!("make_{}_accessor", coll);
        AccessorFn {
            name: name,
            type_params: vec![elem.lib_protobuf_type()],
            callback_params: self.make_accessor_fns_lambda(),
        }
    }

    fn accessor_fn_singular_without_flag(&self, elem: &FieldElem) -> AccessorFn {
        if let &FieldElem::Message(ref name, ..) = elem {
            // TODO: old style, needed because of default instance

            AccessorFn {
                name: "make_singular_message_accessor".to_owned(),
                type_params: vec![name.clone()],
                callback_params: self.make_accessor_fns_has_get(),
            }
        } else {
            AccessorFn {
                name: "make_simple_field_accessor".to_owned(),
                type_params: vec![elem.lib_protobuf_type()],
                callback_params: self.make_accessor_fns_lambda(),
            }
        }
    }

    fn accessor_fn_singular_with_flag(&self, elem: &FieldElem, _option_kind: OptionKind) -> AccessorFn {
        match elem {
            FieldElem::Message(..) => {
                AccessorFn {
                    name: "make_option_accessor".to_owned(),
                    type_params: vec![elem.lib_protobuf_type(), "_".to_owned()],
                    callback_params: self.make_accessor_fns_lambda(),
                }
            }
            FieldElem::Primitive(FieldDescriptorProto_Type::TYPE_STRING, ..) |
            FieldElem::Primitive(FieldDescriptorProto_Type::TYPE_BYTES, ..) => {
                AccessorFn {
                    name: "make_option_get_ref_accessor".to_owned(),
                    type_params: vec![elem.lib_protobuf_type(), "_".to_owned()],
                    callback_params: self.make_accessor_fns_lambda_get(),
                }
            }
            FieldElem::Primitive(..) | FieldElem::Enum(..) => {
                AccessorFn {
                    name: "make_option_get_copy_accessor".to_owned(),
                    type_params: vec![elem.lib_protobuf_type(), "_".to_owned()],
                    callback_params: self.make_accessor_fns_lambda_get(),
                }
            }
            FieldElem::Group => {
                unreachable!("no accessor for group field");
            }
        }
    }

    fn accessor_fn_oneof(&self, oneof: &OneofField) -> AccessorFn {
        let OneofField { ref elem, .. } = oneof;
        // TODO: uses old style

        // TODO: storage type is nonsense for oneof
        if elem.rust_storage_elem_type().is_copy() {
            return AccessorFn {
                name: "make_singular_copy_has_get_set_accessor".to_owned(),
                type_params: vec![elem.protobuf_type_gen().rust_type()],
                callback_params: self.make_accessor_fns_has_get_set(),
            };
        }

        if let RustType::Message(name) = elem.rust_storage_elem_type() {
            return AccessorFn {
                name: "make_singular_message_has_get_mut_set_accessor".to_owned(),
                type_params: vec![name.clone()],
                callback_params: self.make_accessor_fns_has_get_mut_set(),
            }
        }

        // string or bytes
        AccessorFn {
            name: "make_singular_deref_has_get_set_accessor".to_owned(),
            type_params: vec![elem.protobuf_type_gen().rust_type()],
            callback_params: self.make_accessor_fns_has_get_set(),
        }
    }

    fn accessor_fn(&self) -> AccessorFn {
        match self.kind {
            FieldKind::Repeated(ref repeated_field) => self.accessor_fn_repeated(repeated_field),
            FieldKind::Map(ref map_field) => self.accessor_fn_map(map_field),
            FieldKind::Singular(SingularField {
                ref elem,
                flag: SingularFieldFlag::WithoutFlag,
            }) => {
                self.accessor_fn_singular_without_flag(elem)
            }
            FieldKind::Singular(SingularField {
                ref elem,
                flag: SingularFieldFlag::WithFlag { option_kind, .. },
            }) => {
                self.accessor_fn_singular_with_flag(elem, option_kind)
            }
            FieldKind::Oneof(ref oneof) => {
                self.accessor_fn_oneof(oneof)
            }
        }
    }

    fn make_accessor_fns_lambda(&self) -> Vec<String> {
        let message = self.proto_field.message.rust_name();
        vec![
            format!("|m: &{}| {{ &m.{} }}", message, self.rust_name),
            format!("|m: &mut {}| {{ &mut m.{} }}", message, self.rust_name),
        ]
    }

    fn make_accessor_fns_lambda_get(&self) -> Vec<String> {
        let message = self.proto_field.message.rust_name();
        vec![
            format!("|m: &{}| {{ &m.{} }}", message, self.rust_name),
            format!("|m: &mut {}| {{ &mut m.{} }}", message, self.rust_name),
            format!("{}::get_{}", message, self.rust_name),
        ]
    }

    fn make_accessor_fns_has_get(&self) -> Vec<String> {
        let message = self.proto_field.message.rust_name();
        vec![
            format!("{}::has_{}", message, self.rust_name),
            format!("{}::get_{}", message, self.rust_name),
        ]
    }

    fn make_accessor_fns_has_get_set(&self) -> Vec<String> {
        let message = self.proto_field.message.rust_name();
        vec![
            format!("{}::has_{}", message, self.rust_name),
            format!("{}::get_{}", message, self.rust_name),
            format!("{}::set_{}", message, self.rust_name),
        ]
    }

    fn make_accessor_fns_has_get_mut_set(&self) -> Vec<String> {
        let message = self.proto_field.message.rust_name();
        vec![
            format!("{}::has_{}", message, self.rust_name),
            format!("{}::get_{}", message, self.rust_name),
            format!("{}::mut_{}", message, self.rust_name),
            format!("{}::set_{}", message, self.rust_name),
        ]
    }

    pub fn write_descriptor_field(&self, fields_var: &str, w: &mut CodeWriter) {
        let accessor_fn = self.accessor_fn();
        w.write_line(&format!(
            "{}.push(::protobuf::reflect::rt::{}(",
            fields_var,
            accessor_fn.sig()
        ));
        w.indented(|w| {
            w.write_line(&format!("\"{}\",", self.proto_field.name()));
            for callback in accessor_fn.callback_params {
                w.write_line(&format!("{},", callback));
            }
        });
        w.write_line("));");
    }

    pub fn write_clear(&self, w: &mut CodeWriter) {
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
            None => match self.proto_type {
                FieldDescriptorProto_Type::TYPE_MESSAGE => panic!("not a single-liner"),
                FieldDescriptorProto_Type::TYPE_BYTES => format!(
                    "::protobuf::rt::bytes_size({}, &{})",
                    self.proto_field.number(),
                    var
                ),
                FieldDescriptorProto_Type::TYPE_STRING => format!(
                    "::protobuf::rt::string_size({}, &{})",
                    self.proto_field.number(),
                    var
                ),
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
            },
        }
    }

    // output code that writes single element to stream
    pub fn write_write_element(&self, w: &mut CodeWriter, os: &str, v: &RustValueTyped) {
        if let FieldKind::Repeated(RepeatedField { packed: true, .. }) = self.kind {
            unreachable!();
        };

        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                let param_type = RustType::Ref(Box::new(self.elem().rust_storage_elem_type()));

                w.write_line(&format!(
                    "::protobuf::rt::write_message_field_with_cached_size({}, {}, {})?;",
                    self.proto_field.number(),
                    v.into_type(param_type).value,
                    os));
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
                    v.into_type(param_type).value
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

    // field data viewed as Option
    fn self_field_as_option(&self, elem: &FieldElem, option_kind: OptionKind) -> RustValueTyped {
        match self.full_storage_type() {
            RustType::Option(ref e) if e.is_copy() => {
                return RustType::Option(e.clone()).value(self.self_field());
            }
            _ => {},
        };

//        let as_option_type = RustType::Option(Box::new(elem.rust_storage_elem_type().ref_type()));
//
//        // TODO: do not use as_option_ref, return Box for OptionBox instead (for simpler code)
//        let v = option_kind.as_option_ref(&self.self_field());
//
//        as_option_type.value(v)

        let as_option_type = option_kind.as_ref_type(elem.rust_storage_elem_type());

        as_option_type.value(format!("{}.as_ref()", self.self_field()))
    }

    /// Field visibility in message struct
    fn visibility(&self) -> Visibility {
        if self.expose_field {
            Visibility::Public
        } else {
            match self.kind {
                FieldKind::Repeated(..) => Visibility::Default,
                FieldKind::Singular(SingularField { ref flag, .. }) => {
                    match *flag {
                        SingularFieldFlag::WithFlag { .. } => Visibility::Default,
                        SingularFieldFlag::WithoutFlag => Visibility::Public,
                    }
                }
                FieldKind::Map(..) => Visibility::Public,
                FieldKind::Oneof(..) => unreachable!(),
            }
        }
    }

    pub fn write_struct_field(&self, w: &mut CodeWriter) {
        if self.proto_type == FieldDescriptorProto_Type::TYPE_GROUP {
            w.comment(&format!("{}: <group>", &self.rust_name));
        } else {
            let vis = self.visibility();
            w.field_decl_vis(
                vis,
                &self.rust_name.0,
                &self.full_storage_type().to_string(),
            );
        }
    }

    fn write_if_let_self_field_is_some<F>(&self, s: &SingularField, w: &mut CodeWriter, cb: F)
    where
        F : Fn(&RustValueTyped, &mut CodeWriter),
    {
        match s {
            SingularField {
                flag: SingularFieldFlag::WithFlag { option_kind, .. },
                ref elem,
            } => {
                let var = "v";
                let ref_prefix = match elem.rust_storage_elem_type().is_copy() {
                    true => "",
                    false => "",
                };
                let as_option = self.self_field_as_option(elem, *option_kind);
                w.if_let_stmt(
                    &format!("Some({}{})", ref_prefix, var),
                    &as_option.value,
                    |w| {
                        let v = RustValueTyped {
                            value: var.to_owned(),
                            rust_type: as_option.rust_type.elem_type(),
                        };
                        cb(&v, w);
                    },
                );
            }
            SingularField {
                flag: SingularFieldFlag::WithoutFlag,
                ref elem,
            } => {
                match *elem {
                    FieldElem::Primitive(FieldDescriptorProto_Type::TYPE_STRING, ..) |
                    FieldElem::Primitive(FieldDescriptorProto_Type::TYPE_BYTES, ..) => {
                        w.if_stmt(format!("!{}.is_empty()", self.self_field()), |w| {
                            let v = RustValueTyped {
                                value: self.self_field(),
                                rust_type: self.full_storage_type(),
                            };
                            cb(&v, w);
                        });
                    }
                    _ => {
                        w.if_stmt(
                            format!(
                                "{} != {}",
                                self.self_field(),
                                self.full_storage_type().default_value()
                            ),
                            |w| {
                                let v = RustValueTyped {
                                    value: self.self_field(),
                                    rust_type: self.full_storage_type(),
                                };
                                cb(&v, w);
                            },
                        );
                    }
                }
            }
        }
    }

    fn write_if_self_field_is_not_empty<F>(&self, w: &mut CodeWriter, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        assert!(self.is_repeated_or_map());
        let self_field_is_not_empty = self.self_field_is_not_empty();
        w.if_stmt(self_field_is_not_empty, cb);
    }

    pub fn write_if_self_field_is_none<F>(&self, w: &mut CodeWriter, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        let self_field_is_none = self.self_field_is_none();
        w.if_stmt(self_field_is_none, cb)
    }

    // repeated or singular
    pub fn write_for_self_field<F>(&self, w: &mut CodeWriter, varn: &str, cb: F)
    where
        F: Fn(&mut CodeWriter, &RustType),
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
                    |w| cb(w, &elem.rust_storage_elem_type()),
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

    fn write_self_field_assign_some(&self, w: &mut CodeWriter, s: &SingularField, value: &str) {
        match s {
            &SingularField { flag: SingularFieldFlag::WithFlag { option_kind, .. }, .. } => {
                self.write_self_field_assign(w, &option_kind.wrap_value(value));
            }
            &SingularField {
                flag: SingularFieldFlag::WithoutFlag,
                ..
            } => {
                self.write_self_field_assign(w, value);
            }
        }
    }

    fn write_self_field_assign_value_singular(
        &self,
        w: &mut CodeWriter,
        s: &SingularField,
        value: &RustValueTyped)
    {
        let SingularField { ref elem, ref flag } = s;
        let converted = value.into_type(elem.rust_storage_elem_type().clone());
        let wrapped = match flag {
            SingularFieldFlag::WithoutFlag => {
                converted.value
            }
            SingularFieldFlag::WithFlag { option_kind, .. } => {
                option_kind.wrap_value(&converted.value)
            }
        };
        self.write_self_field_assign(w, &wrapped);
    }

    fn write_self_field_assign_value(&self, w: &mut CodeWriter, value: &RustValueTyped) {
        match self.kind {
            FieldKind::Repeated(..) |
            FieldKind::Map(..) => {
                let converted = value.into_type(self.full_storage_type());
                self.write_self_field_assign(w, &converted.value);
            }
            FieldKind::Singular(ref s) => {
                self.write_self_field_assign_value_singular(w, s, value);
            }
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    fn write_self_field_assign_default(&self,
        option_kind: OptionKind, _field_elem: &FieldElem, w: &mut CodeWriter)
    {
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
            let s = self.singular();
            match option_kind {
                OptionKind::SingularField |
                OptionKind::SingularPtrField => {
                    let self_field = self.self_field();
                    w.write_line(&format!("{}.set_default();", self_field));
                }
                _ => {
                    self.write_self_field_assign_some(
                        w, s, &self.element_default_value_rust().value);
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
            "{} + ::protobuf::rt::compute_raw_varint32_size({}) + {}",
            self.tag_size(),
            self.self_field_vec_packed_fixed_data_size(),
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


    fn write_merge_from_field_message_string_bytes_repeated(
        &self,
        r: &RepeatedField,
        w: &mut CodeWriter)
    {
        let carllerche = match self.kind.primitive_type_variant() {
            PrimitiveTypeVariant::Carllerche => "carllerche_",
            PrimitiveTypeVariant::Default => "",
        };
        let type_name_for_fn = protobuf_name(self.proto_type);
        let into_what_suffix = match *r {
            RepeatedField {
                elem: FieldElem::Message(..),
                repeated_field_vec,
                ..
            } =>
            {
                if repeated_field_vec {
                    "_vec"
                } else {
                    "_repeated_field"
                }
            }
            _ => "",
        };
        w.write_line(&format!(
            "::protobuf::rt::read_repeated_{}{}_into{}(wire_type, is, &mut self.{})?;",
            carllerche,
            type_name_for_fn,
            into_what_suffix,
            self.rust_name,
        ));
    }

    fn write_merge_from_field_message_string_bytes_singular(
        &self,
        s: &SingularField,
        w: &mut CodeWriter)
    {
        let singular_or_proto3 = match s {
            SingularField { flag: SingularFieldFlag::WithFlag { .. }, .. } => {
                "singular"
            }
            SingularField { flag: SingularFieldFlag::WithoutFlag, .. } => {
                "singular_proto3"
            }
        };
        let type_params = match s.elem {
            FieldElem::Message(ref name, ..) => format!("::<{}, _>", name),
            _ => "".to_owned(),
        };
        let carllerche = match self.kind.primitive_type_variant() {
            PrimitiveTypeVariant::Carllerche => "carllerche_",
            PrimitiveTypeVariant::Default => "",
        };
        let type_name_for_fn = protobuf_name(self.proto_type);
        w.write_line(&format!(
            "::protobuf::rt::read_{}_{}{}_into{}(wire_type, is, &mut self.{})?;",
            singular_or_proto3,
            carllerche,
            type_name_for_fn,
            type_params,
            self.rust_name,
        ));
    }

    // Write `merge_from` part for this singular or repeated field
    // of type message, string or bytes
    fn write_merge_from_field_message_string_bytes(&self, w: &mut CodeWriter) {
        match self.kind {
            FieldKind::Repeated(ref r) => {
                self.write_merge_from_field_message_string_bytes_repeated(r, w)
            },
            FieldKind::Singular(ref s) => {
                self.write_merge_from_field_message_string_bytes_singular(s, w)
            },
            FieldKind::Map(..) |
            FieldKind::Oneof(..) => unreachable!(),
        };
    }

    fn write_error_unexpected_wire_type(&self, wire_type_var: &str, w: &mut CodeWriter) {
        w.write_line(&format!(
            "return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type({}));",
            wire_type_var
        ));
    }

    fn write_assert_wire_type(&self, wire_type_var: &str, w: &mut CodeWriter) {
        w.if_stmt(&format!("{} != ::protobuf::wire_format::{:?}", wire_type_var, self.wire_type), |w| {
            self.write_error_unexpected_wire_type(wire_type_var, w);
        });
    }

    // Write `merge_from` part for this oneof field
    fn write_merge_from_oneof(&self, f: &OneofField, wire_type_var: &str, w: &mut CodeWriter) {
        self.write_assert_wire_type(wire_type_var, w);

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

    // Write `merge_from` part for this map field
    fn write_merge_from_map(&self, w: &mut CodeWriter) {
        let &MapField {
            ref key, ref value, ..
        } = self.map();
        w.write_line(&format!(
            "::protobuf::rt::read_map_into::<{}, {}>(wire_type, is, &mut {})?;",
            key.lib_protobuf_type(),
            value.lib_protobuf_type(),
            self.self_field()
        ));
    }

    // Write `merge_from` part for this singular field
    fn write_merge_from_singular(
        &self,
        s: &SingularField,
        wire_type_var: &str,
        w: &mut CodeWriter)
    {
        match s.elem {
            FieldElem::Message(..) |
            FieldElem::Primitive(FieldDescriptorProto_Type::TYPE_STRING, ..) |
            FieldElem::Primitive(FieldDescriptorProto_Type::TYPE_BYTES, ..) => {
                self.write_merge_from_field_message_string_bytes(w);
            }
            FieldElem::Enum(..) => {
                let version = match s.flag {
                    SingularFieldFlag::WithFlag { .. } => "proto2",
                    SingularFieldFlag::WithoutFlag => "proto3",
                };
                w.write_line(&format!(
                    "::protobuf::rt::read_{}_enum_with_unknown_fields_into({}, is, &mut self.{}, {}, &mut self.unknown_fields)?",
                    version,
                    wire_type_var,
                    self.rust_name,
                    self.proto_field.number()
                ));
            }
            _ => {
                let read_proc = format!("{}?", self.proto_type.read("is"));

                self.write_assert_wire_type(wire_type_var, w);
                w.write_line(&format!("let tmp = {};", read_proc));
                self.write_self_field_assign_some(w, s, "tmp");
            }
        }
    }

    // Write `merge_from` part for this repeated field
    fn write_merge_from_repeated(&self, wire_type_var: &str, w: &mut CodeWriter) {
        let field = match self.kind {
            FieldKind::Repeated(ref field) => field,
            _ => panic!(),
        };

        match field.elem {
            FieldElem::Message(..) |
            FieldElem::Primitive(FieldDescriptorProto_Type::TYPE_STRING, ..) |
            FieldElem::Primitive(FieldDescriptorProto_Type::TYPE_BYTES, ..) => {
                self.write_merge_from_field_message_string_bytes(w);
            }
            FieldElem::Enum(..) => {
                w.write_line(&format!(
                    "::protobuf::rt::read_repeated_enum_with_unknown_fields_into({}, is, &mut self.{}, {}, &mut self.unknown_fields)?",
                    wire_type_var,
                    self.rust_name,
                    self.proto_field.number()
                ));
            }
            _ => {
                w.write_line(&format!(
                    "::protobuf::rt::read_repeated_{}_into({}, is, &mut self.{})?;",
                    protobuf_name(self.proto_type),
                    wire_type_var,
                    self.rust_name));
            }
        }
    }

    // Write `merge_from` part for this field
    pub fn write_merge_from_field(&self, wire_type_var: &str, w: &mut CodeWriter) {
        match self.kind {
            FieldKind::Oneof(ref f) => self.write_merge_from_oneof(&f, wire_type_var, w),
            FieldKind::Map(..) => self.write_merge_from_map(w),
            FieldKind::Singular(ref s) => self.write_merge_from_singular(s, wire_type_var, w),
            FieldKind::Repeated(..) => self.write_merge_from_repeated(wire_type_var, w),
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
            FieldKind::Singular(ref s) => {
                self.write_if_let_self_field_is_some(s, w, |v, w| {
                    self.write_write_element(w, "os", &v);
                });
            }
            FieldKind::Repeated(RepeatedField { packed: false, .. }) => {
                self.write_for_self_field(w, "v", |w, v_type| {
                    let v = RustValueTyped { value: "v".to_owned(), rust_type: v_type.clone() };
                    self.write_write_element(w, "os", &v);
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
            FieldKind::Map(MapField {
                ref key, ref value, ..
            }) => {
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
            FieldKind::Singular(ref s) => {
                self.write_if_let_self_field_is_some(s, w, |v, w| {
                    match field_type_size(self.proto_type) {
                        Some(s) => {
                            let tag_size = self.tag_size();
                            w.write_line(&format!("{} += {};", sum_var, (s + tag_size) as isize));
                        }
                        None => {
                            self.write_element_size(w, &v.value, &v.rust_type, sum_var);
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
            FieldKind::Map(MapField {
                ref key, ref value, ..
            }) => {
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

    fn write_message_field_get_singular_message(&self, s: &SingularField, w: &mut CodeWriter) {
        match s.flag {
            SingularFieldFlag::WithoutFlag => {
                unimplemented!()
            }
            SingularFieldFlag::WithFlag { option_kind, .. } => {
                let self_field = self.self_field();
                let ref field_type_name = self.elem().rust_storage_elem_type();
                w.write_line(option_kind.unwrap_ref_or_else(
                    &format!("{}.as_ref()", self_field),
                    &format!("<{} as ::protobuf::Message>::default_instance()", field_type_name),
                ));
            }
        }
    }

    fn write_message_field_get_singular(&self, s: &SingularField, w: &mut CodeWriter) {
        let get_xxx_return_type = self.get_xxx_return_type();

        if self.proto_type == FieldDescriptorProto_Type::TYPE_MESSAGE {
            self.write_message_field_get_singular_message(s, w);
        } else {
            let get_xxx_default_value_rust = self.get_xxx_default_value_rust();
            let self_field = self.self_field();
            match self.singular() {
                &SingularField { ref elem, flag: SingularFieldFlag::WithFlag { option_kind, .. }, .. } => {
                    if get_xxx_return_type.is_ref().is_some() {
                        let as_option = self.self_field_as_option(elem, option_kind);
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
                        ("ref v", elem.rust_storage_elem_type().ref_type())
                    } else {
                        ("v", elem.rust_storage_elem_type())
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
            FieldKind::Singular(ref s) => {
                self.write_message_field_get_singular(s, w);
            }
            FieldKind::Repeated(..) | FieldKind::Map(..) => {
                let self_field = self.self_field();
                w.write_line(&format!("&{}", self_field));
            }
        });
    }

    fn has_has(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(..) | FieldKind::Map(..) => false,
            FieldKind::Singular(SingularField {
                flag: SingularFieldFlag::WithFlag { .. },
                ..
            }) => true,
            FieldKind::Singular(SingularField {
                flag: SingularFieldFlag::WithoutFlag,
                ..
            }) => false,
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
            let value_typed = RustValueTyped {
                value: "v".to_owned(),
                rust_type: set_xxx_param_type.clone(),
            };
            if !self.is_oneof() {
                self.write_self_field_assign_value(w, &value_typed);
            } else {
                let self_field_oneof = self.self_field_oneof();
                let v = set_xxx_param_type.into_target(&self.oneof().rust_type(), "v");
                w.write_line(&format!("{} = ::std::option::Option::Some({}({}))",
                    self_field_oneof, self.variant_path(), v));
            }
        });
    }

    fn write_message_field_mut_singular(&self, s: &SingularField, w: &mut CodeWriter) {
        match s {
            SingularField { flag: SingularFieldFlag::WithFlag { option_kind, .. }, elem } => {
                self.write_if_self_field_is_none(
                    w,
                    |w| { self.write_self_field_assign_default(*option_kind, elem, w); },
                );
                let self_field = self.self_field();
                w.write_line(&format!("{}.as_mut().unwrap()", self_field));
            }
            SingularField { flag: SingularFieldFlag::WithoutFlag, .. } => {
                w.write_line(&format!("&mut {}", self.self_field()))
            }
        }
    }

    fn write_message_field_mut(&self, w: &mut CodeWriter) {
        let mut_xxx_return_type = self.mut_xxx_return_type();
        w.comment("Mutable pointer to the field.");
        if self.is_singular() {
            w.comment("If field is not initialized, it is initialized with default value first.");
        }
        let fn_def = match mut_xxx_return_type {
            RustType::Ref(ref param) => {
                format!("mut_{}(&mut self) -> &mut {}", self.rust_name, **param)
            }
            _ => panic!("not a ref: {}", mut_xxx_return_type),
        };
        w.pub_fn(&fn_def, |w| {
            match self.kind {
                FieldKind::Repeated(..) | FieldKind::Map(..) => {
                    let self_field = self.self_field();
                    w.write_line(&format!("&mut {}", self_field));
                }
                FieldKind::Singular(ref s) => {
                    self.write_message_field_mut_singular(s, w);
                }
                FieldKind::Oneof(..) => {
                    let self_field_oneof = self.self_field_oneof();

                    // if oneof does not contain current field
                    w.if_let_else_stmt(
                        &format!("::std::option::Option::Some({}(_))", self.variant_path())[..],
                        &self_field_oneof[..],
                        |w| {
                            // initialize it with default value
                            w.write_line(&format!(
                                "{} = ::std::option::Option::Some({}({}));",
                                self_field_oneof,
                                self.variant_path(),
                                self.element_default_value_rust()
                                    .into_type(self.oneof().rust_type())
                                    .value
                            ));
                        },
                    );

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
                    .rust_storage_elem_type()
                    .default_value_typed()
                    .into_type(take_xxx_return_type.clone())
                    .value,
            );
        });
        w.write_line("}");
    }

    fn write_message_field_take_singular(&self, s: &SingularField, w: &mut CodeWriter) {
        match s {
            SingularField {
                ref elem,
                flag: SingularFieldFlag::WithFlag { option_kind, .. },
            } => {
                if !elem.is_copy() {
                    w.write_line(&option_kind.unwrap_or_else(
                        &format!("{}.take()", self.self_field()),
                        &elem.rust_storage_elem_type().default_value()));
                } else {
                    w.write_line(&format!(
                        "{}.take().unwrap_or({})",
                        self.self_field(),
                        self.element_default_value_rust().value
                    ));
                }
            }
            SingularField { flag: SingularFieldFlag::WithoutFlag, .. } => {
                w.write_line(&format!(
                    "::std::mem::replace(&mut {}, {})",
                    self.self_field(),
                    self.full_storage_type().default_value()
                ))
            }
        }
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
                FieldKind::Repeated(..) | FieldKind::Map(..) => {
                    w.write_line(&format!(
                        "::std::mem::replace(&mut self.{}, {})",
                        self.rust_name,
                        take_xxx_return_type.default_value()
                    ));
                }
                FieldKind::Singular(ref s) => self.write_message_field_take_singular(&s, w),
            },
        );
    }

    pub fn write_message_single_field_accessors(&self, w: &mut CodeWriter) {
        if self.generate_accessors || self.generate_getter {
            w.write_line("");
            let reconstruct_def = self.reconstruct_def();
            w.comment(&(reconstruct_def + ";"));
        }

        if self.generate_getter {
            w.write_line("");
            self.write_message_field_get(w);
        }

        if !self.generate_accessors {
            return;
        }

        w.write_line("");
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
    }
}
