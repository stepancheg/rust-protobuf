mod accessor;
pub(crate) mod elem;
mod option_kind;
mod repeated;
mod singular;
mod tag;
pub(crate) mod type_ext;

use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::descriptor::*;
use protobuf::reflect::ReflectValueRef;
use protobuf::reflect::RuntimeFieldType;
use protobuf::reflect::Syntax;
use protobuf::rt;
use protobuf::rt::WireType;
use protobuf_parse::camel_case;
use protobuf_parse::ProtobufAbsPath;

use crate::customize::ctx::CustomizeElemCtx;
use crate::customize::rustproto_proto::customize_from_rustproto_for_field;
use crate::customize::Customize;
use crate::gen::code_writer::CodeWriter;
use crate::gen::code_writer::Visibility;
use crate::gen::field::elem::field_elem;
use crate::gen::field::elem::FieldElem;
use crate::gen::field::elem::FieldElemEnum;
use crate::gen::field::elem::HowToGetMessageSize;
use crate::gen::field::option_kind::OptionKind;
use crate::gen::field::repeated::RepeatedField;
use crate::gen::field::singular::SingularField;
use crate::gen::field::singular::SingularFieldFlag;
use crate::gen::field::tag::make_tag;
use crate::gen::field::type_ext::TypeExt;
use crate::gen::file_and_mod::FileAndMod;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::map::map_entry;
use crate::gen::oneof::OneofField;
use crate::gen::protoc_insertion_point::write_protoc_insertion_point_for_field;
use crate::gen::rust::ident::RustIdent;
use crate::gen::rust::quote::quote_escape_bytes;
use crate::gen::rust::quote::quote_escape_str;
use crate::gen::rust::snippets::EXPR_NONE;
use crate::gen::rust_types_values::PrimitiveTypeVariant;
use crate::gen::rust_types_values::RustType;
use crate::gen::rust_types_values::RustValueTyped;
use crate::gen::scope::FieldWithContext;
use crate::gen::scope::MessageWithScope;
use crate::gen::scope::RootScope;

fn field_type_protobuf_name<'a>(field: &'a FieldDescriptorProto) -> &'a str {
    if field.has_type_name() {
        field.type_name()
    } else {
        field.type_().protobuf_name()
    }
}

#[derive(Clone)]
pub struct MapField<'a> {
    _message: MessageWithScope<'a>,
    key: FieldElem<'a>,
    value: FieldElem<'a>,
}

#[derive(Clone)]
pub(crate) enum FieldKind<'a> {
    // optional or required
    Singular(SingularField<'a>),
    // repeated except map
    Repeated(RepeatedField<'a>),
    // map
    Map(MapField<'a>),
    // part of oneof
    Oneof(OneofField<'a>),
}

impl<'a> FieldKind<'a> {
    pub(crate) fn default(
        &self,
        customize: &Customize,
        reference: &FileAndMod,
        const_expr: bool,
    ) -> String {
        match self {
            FieldKind::Singular(s) => s.default_value(customize, reference, const_expr),
            FieldKind::Repeated(r) => r.default(),
            FieldKind::Oneof(..) => EXPR_NONE.to_owned(),
            FieldKind::Map(..) => panic!("map fields cannot have field value"),
        }
    }
}

#[derive(Clone)]
pub(crate) enum SingularOrOneofField<'a> {
    Singular(SingularField<'a>),
    Oneof(OneofField<'a>),
}

impl<'a> SingularOrOneofField<'a> {
    fn elem(&self) -> &FieldElem {
        match self {
            SingularOrOneofField::Singular(SingularField { ref elem, .. }) => elem,
            SingularOrOneofField::Oneof(OneofField { ref elem, .. }) => elem,
        }
    }

    // Type of `xxx` function for singular type.
    pub(crate) fn getter_return_type(&self, reference: &FileAndMod) -> RustType {
        let elem = self.elem();
        if let FieldElem::Enum(ref en) = elem {
            en.enum_rust_type(reference)
        } else if elem.is_copy() {
            elem.rust_storage_elem_type(reference)
        } else {
            elem.rust_storage_elem_type(reference).ref_type()
        }
    }
}

#[derive(Clone)]
pub(crate) struct FieldGen<'a> {
    syntax: Syntax,
    pub proto_field: FieldWithContext<'a>,
    // field name in generated code
    pub rust_name: RustIdent,
    pub proto_type: Type,
    wire_type: WireType,
    pub kind: FieldKind<'a>,
    pub generate_accessors: bool,
    pub generate_getter: bool,
    customize: Customize,
    path: Vec<i32>,
    info: Option<&'a SourceCodeInfo>,
}

impl<'a> FieldGen<'a> {
    pub(crate) fn parse(
        field: FieldWithContext<'a>,
        root_scope: &'a RootScope<'a>,
        parent_customize: &CustomizeElemCtx<'a>,
        path: Vec<i32>,
        info: Option<&'a SourceCodeInfo>,
    ) -> anyhow::Result<FieldGen<'a>> {
        let customize = parent_customize
            .child(
                &customize_from_rustproto_for_field(field.field.proto().options.get_or_default()),
                &field.field,
            )
            .for_elem;

        let syntax = field.message.scope.file_scope.syntax();

        let field_may_have_custom_default_value = syntax == Syntax::Proto2
            && field.field.proto().label() != field_descriptor_proto::Label::LABEL_REPEATED
            && field.field.proto().type_() != Type::TYPE_MESSAGE;

        let generate_accessors = customize
            .generate_accessors
            .unwrap_or(field_may_have_custom_default_value)
            || field.is_oneof();

        let default_generate_getter = generate_accessors || field_may_have_custom_default_value;
        let generate_getter =
            customize.generate_getter.unwrap_or(default_generate_getter) || field.is_oneof();

        let kind = match field.field.runtime_field_type() {
            RuntimeFieldType::Map(..) => {
                let message = root_scope
                    .find_message(&ProtobufAbsPath::from(field.field.proto().type_name()));

                let (key, value) = map_entry(&message).unwrap();

                let key = field_elem(&key, root_scope, &customize);
                let value = field_elem(&value, root_scope, &customize);

                FieldKind::Map(MapField {
                    _message: message,
                    key,
                    value,
                })
            }
            RuntimeFieldType::Repeated(..) => {
                let elem = field_elem(&field, root_scope, &customize);
                let primitive = match field.field.proto().type_() {
                    Type::TYPE_DOUBLE
                    | Type::TYPE_FLOAT
                    | Type::TYPE_INT64
                    | Type::TYPE_UINT64
                    | Type::TYPE_INT32
                    | Type::TYPE_FIXED64
                    | Type::TYPE_FIXED32
                    | Type::TYPE_BOOL
                    | Type::TYPE_UINT32
                    | Type::TYPE_SFIXED32
                    | Type::TYPE_SFIXED64
                    | Type::TYPE_SINT32
                    | Type::TYPE_SINT64
                    | Type::TYPE_ENUM => true,
                    Type::TYPE_STRING
                    | Type::TYPE_GROUP
                    | Type::TYPE_MESSAGE
                    | Type::TYPE_BYTES => false,
                };
                let packed = field
                    .field
                    .proto()
                    .options
                    .get_or_default()
                    .packed
                    .unwrap_or(match field.message.scope.file_scope.syntax() {
                        Syntax::Proto2 => false,
                        // in proto3, repeated primitive types are packed by default
                        Syntax::Proto3 => primitive,
                    });
                if packed && !primitive {
                    anyhow::bail!(
                        "[packed = true] can only be specified for repeated primitive fields"
                    );
                }
                FieldKind::Repeated(RepeatedField { elem, packed })
            }
            RuntimeFieldType::Singular(..) => {
                let elem = field_elem(&field, root_scope, &customize);

                if let Some(oneof) = field.oneof() {
                    FieldKind::Oneof(OneofField::parse(&oneof, &field.field, elem, root_scope))
                } else {
                    let flag = if field.message.scope.file_scope.syntax() == Syntax::Proto3
                        && field.field.proto().type_() != field_descriptor_proto::Type::TYPE_MESSAGE
                        && !field.field.proto().proto3_optional()
                    {
                        SingularFieldFlag::WithoutFlag
                    } else {
                        let required = field.field.proto().label()
                            == field_descriptor_proto::Label::LABEL_REQUIRED;
                        let option_kind = match field.field.proto().type_() {
                            field_descriptor_proto::Type::TYPE_MESSAGE => OptionKind::MessageField,
                            _ => OptionKind::Option,
                        };

                        SingularFieldFlag::WithFlag {
                            required,
                            option_kind,
                        }
                    };
                    FieldKind::Singular(SingularField { elem, flag })
                }
            }
        };

        Ok(FieldGen {
            syntax: field.message.message.file_descriptor().syntax(),
            rust_name: rust_field_name_for_protobuf_field_name(&field.field.name()),
            proto_type: field.field.proto().type_(),
            wire_type: WireType::for_type(field.field.proto().type_()),
            proto_field: field,
            kind,
            generate_accessors,
            generate_getter,
            customize,
            path,
            info,
        })
    }

    // for message level
    fn file_and_mod(&self) -> FileAndMod {
        self.proto_field
            .message
            .scope
            .file_and_mod(self.customize.clone())
    }

    fn tag_size(&self) -> u32 {
        rt::tag_size(self.proto_field.number() as u32) as u32
    }

    fn is_singular(&self) -> bool {
        match self.kind {
            FieldKind::Singular(..) => true,
            _ => false,
        }
    }

    fn is_repeated_packed(&self) -> bool {
        match self.kind {
            FieldKind::Repeated(RepeatedField { packed: true, .. }) => true,
            _ => false,
        }
    }

    pub(crate) fn elem(&self) -> &FieldElem {
        match self.kind {
            FieldKind::Singular(SingularField { ref elem, .. }) => &elem,
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => &elem,
            FieldKind::Oneof(OneofField { ref elem, .. }) => &elem,
            FieldKind::Map(..) => unreachable!(),
        }
    }

    // type of field in struct
    pub(crate) fn full_storage_type(&self, reference: &FileAndMod) -> RustType {
        match self.kind {
            FieldKind::Repeated(ref repeated) => repeated.rust_type(reference),
            FieldKind::Map(MapField {
                ref key, ref value, ..
            }) if self.customize.btreemap == Some(true) => RustType::BTreeMap(
                Box::new(key.rust_storage_elem_type(reference)),
                Box::new(value.rust_storage_elem_type(reference)),
            ),
            FieldKind::Map(MapField {
                ref key, ref value, ..
            }) => RustType::HashMap(
                Box::new(key.rust_storage_elem_type(reference)),
                Box::new(value.rust_storage_elem_type(reference)),
            ),
            FieldKind::Singular(ref singular) => singular.rust_storage_type(reference),
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    // type of `v` in `for v in field`
    fn full_storage_iter_elem_type(&self, reference: &FileAndMod) -> RustType {
        if let FieldKind::Oneof(ref oneof) = self.kind {
            oneof.elem.rust_storage_elem_type(reference)
        } else {
            self.full_storage_type(reference).iter_elem_type()
        }
    }

    // suffix `xxx` as in `os.write_xxx_no_tag(..)`
    fn os_write_fn_suffix(&self) -> &str {
        self.proto_type.protobuf_name()
    }

    fn os_write_fn_suffix_with_unknown_for_enum(&self) -> &str {
        if self.proto_type == field_descriptor_proto::Type::TYPE_ENUM {
            "enum_or_unknown"
        } else {
            self.os_write_fn_suffix()
        }
    }

    // for field `foo`, type of param of `fn set_foo(..)`
    fn set_xxx_param_type(&self, reference: &FileAndMod) -> RustType {
        match self.kind {
            FieldKind::Singular(SingularField { ref elem, .. })
            | FieldKind::Oneof(OneofField { ref elem, .. }) => {
                elem.rust_set_xxx_param_type(reference)
            }
            FieldKind::Repeated(..) | FieldKind::Map(..) => self.full_storage_type(reference),
        }
    }

    // for field `foo`, return type if `fn take_foo(..)`
    fn take_xxx_return_type(&self, reference: &FileAndMod) -> RustType {
        self.set_xxx_param_type(reference)
    }

    // for field `foo`, return type of `fn mut_foo(..)`
    fn mut_xxx_return_type(&self, reference: &FileAndMod) -> RustType {
        RustType::Ref(Box::new(match self.kind {
            FieldKind::Singular(SingularField { ref elem, .. })
            | FieldKind::Oneof(OneofField { ref elem, .. }) => {
                elem.rust_storage_elem_type(reference)
            }
            FieldKind::Repeated(..) | FieldKind::Map(..) => self.full_storage_type(reference),
        }))
    }

    // for field `foo`, return type of `fn foo(..)`
    fn getter_return_type(&self) -> RustType {
        let reference = self
            .proto_field
            .message
            .scope
            .file_and_mod(self.customize.clone());
        match &self.kind {
            FieldKind::Singular(s) => {
                SingularOrOneofField::Singular(s.clone()).getter_return_type(&reference)
            }
            FieldKind::Oneof(o) => {
                SingularOrOneofField::Oneof(o.clone()).getter_return_type(&reference)
            }
            FieldKind::Repeated(RepeatedField { ref elem, .. }) => RustType::Ref(Box::new(
                RustType::Slice(Box::new(elem.rust_storage_elem_type(&reference))),
            )),
            FieldKind::Map(..) => RustType::Ref(Box::new(self.full_storage_type(&reference))),
        }
    }

    // elem data is not stored in heap
    pub(crate) fn elem_type_is_copy(&self) -> bool {
        self.proto_type.is_copy()
    }

    fn defaut_value_from_proto_float(f: f64, type_name: &str) -> String {
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

    fn singular_or_oneof_default_value_from_proto(&self, elem: &FieldElem) -> Option<String> {
        if !self.proto_field.field.proto().has_default_value() {
            return None;
        }

        let default_value = self.proto_field.field.singular_default_value();
        Some(match default_value {
            ReflectValueRef::Bool(b) => format!("{}", b),
            ReflectValueRef::I32(v) => format!("{}i32", v),
            ReflectValueRef::I64(v) => format!("{}i64", v),
            ReflectValueRef::U32(v) => format!("{}u32", v),
            ReflectValueRef::U64(v) => format!("{}u64", v),
            ReflectValueRef::String(v) => quote_escape_str(v),
            ReflectValueRef::Bytes(v) => quote_escape_bytes(v),
            ReflectValueRef::F32(v) => Self::defaut_value_from_proto_float(v as f64, "f32"),
            ReflectValueRef::F64(v) => Self::defaut_value_from_proto_float(v as f64, "f64"),
            ReflectValueRef::Enum(_e, _v) => {
                if let &FieldElem::Enum(ref e) = elem {
                    format!("{}", e.default_value_rust_expr(&self.file_and_mod()))
                } else {
                    unreachable!()
                }
            }
            t => panic!("default value is not implemented for type: {:?}", t),
        })
    }

    fn default_value_from_proto(&self) -> Option<String> {
        match self.kind {
            FieldKind::Oneof(OneofField { ref elem, .. })
            | FieldKind::Singular(SingularField { ref elem, .. }) => {
                self.singular_or_oneof_default_value_from_proto(elem)
            }
            _ => unreachable!(),
        }
    }

    fn default_value_from_proto_typed(&self) -> Option<RustValueTyped> {
        self.default_value_from_proto().map(|v| {
            let default_value_type = match self.proto_type {
                field_descriptor_proto::Type::TYPE_STRING => RustType::Ref(Box::new(RustType::Str)),
                field_descriptor_proto::Type::TYPE_BYTES => {
                    RustType::Ref(Box::new(RustType::Slice(Box::new(RustType::u8()))))
                }
                _ => self.full_storage_iter_elem_type(
                    &self
                        .proto_field
                        .message
                        .scope
                        .file_and_mod(self.customize.clone()),
                ),
            };

            RustValueTyped {
                value: v,
                rust_type: default_value_type,
            }
        })
    }

    // default value to be returned from `fn xxx` for field `xxx`.
    fn xxx_default_value_rust(&self) -> String {
        match self.kind {
            FieldKind::Singular(..) | FieldKind::Oneof(..) => {
                self.default_value_from_proto().unwrap_or_else(|| {
                    self.getter_return_type()
                        .default_value(&self.customize, false)
                })
            }
            _ => unreachable!(),
        }
    }

    // default to be assigned to field
    fn element_default_value_rust(&self) -> RustValueTyped {
        match self.kind {
            FieldKind::Singular(..) | FieldKind::Oneof(..) => {
                self.default_value_from_proto_typed().unwrap_or_else(|| {
                    self.elem()
                        .rust_storage_elem_type(
                            &self
                                .proto_field
                                .message
                                .scope
                                .file_and_mod(self.customize.clone()),
                        )
                        .default_value_typed(&self.customize, false)
                })
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn reconstruct_def(&self) -> String {
        let prefix = match (self.proto_field.field.proto().label(), self.syntax) {
            (field_descriptor_proto::Label::LABEL_REPEATED, _) => "repeated ",
            (_, Syntax::Proto3) => "",
            (field_descriptor_proto::Label::LABEL_OPTIONAL, _) => "optional ",
            (field_descriptor_proto::Label::LABEL_REQUIRED, _) => "required ",
        };
        format!(
            "{}{} {} = {}",
            prefix,
            field_type_protobuf_name(self.proto_field.field.proto()),
            self.proto_field.name(),
            self.proto_field.number()
        )
    }

    pub(crate) fn write_clear(&self, w: &mut CodeWriter) {
        match self.kind {
            FieldKind::Oneof(ref o) => {
                w.write_line(&format!(
                    "self.{} = ::std::option::Option::None;",
                    o.oneof_field_name
                ));
            }
            _ => {
                let clear_expr = self
                    .full_storage_type(
                        &self
                            .proto_field
                            .message
                            .scope
                            .file_and_mod(self.customize.clone()),
                    )
                    .clear(&self.self_field(), &self.customize);
                w.write_line(&format!("{};", clear_expr));
            }
        }
    }

    // output code that writes single element to stream
    pub(crate) fn write_write_element(
        &self,
        elem: &FieldElem,
        w: &mut CodeWriter,
        os: &str,
        v: &RustValueTyped,
    ) {
        assert!(!self.is_repeated_packed());

        elem.write_write_element(
            self.proto_field.number() as u32,
            v,
            &self.file_and_mod(),
            &self.customize,
            os,
            w,
        );
    }

    fn self_field(&self) -> String {
        format!("self.{}", self.rust_name)
    }

    fn self_field_is_some(&self) -> String {
        assert!(self.is_singular());
        format!("{}.is_some()", self.self_field())
    }

    fn self_field_is_none(&self) -> String {
        assert!(self.is_singular());
        format!("{}.is_none()", self.self_field())
    }

    // field data viewed as Option
    fn self_field_as_option(&self, elem: &FieldElem, option_kind: OptionKind) -> RustValueTyped {
        match self.full_storage_type(
            &self
                .proto_field
                .message
                .scope
                .file_and_mod(self.customize.clone()),
        ) {
            RustType::Option(ref e) if e.is_copy() => {
                return RustType::Option(e.clone()).value(self.self_field());
            }
            _ => {}
        };

        let as_option_type = option_kind.as_ref_type(
            elem.rust_storage_elem_type(
                &self
                    .proto_field
                    .message
                    .scope
                    .file_and_mod(self.customize.clone()),
            ),
        );

        as_option_type.value(format!("{}.as_ref()", self.self_field()))
    }

    pub(crate) fn write_struct_field(&self, w: &mut CodeWriter) {
        if self.proto_type == field_descriptor_proto::Type::TYPE_GROUP {
            w.comment(&format!("{}: <group>", &self.rust_name));
        } else {
            w.all_documentation(self.info, &self.path);

            write_protoc_insertion_point_for_field(w, &self.customize, &self.proto_field.field);
            w.field_decl_vis(
                Visibility::Public,
                &self.rust_name.to_string(),
                &self
                    .full_storage_type(
                        &self
                            .proto_field
                            .message
                            .scope
                            .file_and_mod(self.customize.clone()),
                    )
                    .to_code(&self.customize),
            );
        }
    }

    fn write_if_let_self_field_is_some<F>(&self, s: &SingularField, w: &mut CodeWriter, cb: F)
    where
        F: Fn(&RustValueTyped, &mut CodeWriter),
    {
        match s {
            SingularField {
                flag: SingularFieldFlag::WithFlag { option_kind, .. },
                ref elem,
            } => {
                let var = "v";
                let ref_prefix = match elem
                    .rust_storage_elem_type(
                        &self
                            .proto_field
                            .message
                            .scope
                            .file_and_mod(self.customize.clone()),
                    )
                    .is_copy()
                {
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
            } => match *elem {
                FieldElem::Primitive(field_descriptor_proto::Type::TYPE_STRING, ..)
                | FieldElem::Primitive(field_descriptor_proto::Type::TYPE_BYTES, ..) => {
                    w.if_stmt(format!("!{}.is_empty()", self.self_field()), |w| {
                        let v = RustValueTyped {
                            value: self.self_field(),
                            rust_type: self.full_storage_type(
                                &self
                                    .proto_field
                                    .message
                                    .scope
                                    .file_and_mod(self.customize.clone()),
                            ),
                        };
                        cb(&v, w);
                    });
                }
                _ => {
                    w.if_stmt(
                        format!(
                            "{} != {}",
                            self.self_field(),
                            self.full_storage_type(
                                &self
                                    .proto_field
                                    .message
                                    .scope
                                    .file_and_mod(self.customize.clone())
                            )
                            .default_value(&self.customize, false)
                        ),
                        |w| {
                            let v = RustValueTyped {
                                value: self.self_field(),
                                rust_type: self.full_storage_type(
                                    &self
                                        .proto_field
                                        .message
                                        .scope
                                        .file_and_mod(self.customize.clone()),
                                ),
                            };
                            cb(&v, w);
                        },
                    );
                }
            },
        }
    }

    pub(crate) fn write_if_self_field_is_none<F>(&self, w: &mut CodeWriter, cb: F)
    where
        F: Fn(&mut CodeWriter),
    {
        let self_field_is_none = self.self_field_is_none();
        w.if_stmt(self_field_is_none, cb)
    }

    // repeated or singular
    pub(crate) fn write_for_self_field<F>(&self, w: &mut CodeWriter, varn: &str, cb: F)
    where
        F: Fn(&mut CodeWriter, &RustType),
    {
        let file_and_mod = self
            .proto_field
            .message
            .scope
            .file_and_mod(self.customize.clone());

        match &self.kind {
            FieldKind::Oneof(oneof_field) => {
                let cond = format!(
                    "Some({}(ref {}))",
                    oneof_field.variant_path(&file_and_mod.relative_mod),
                    varn
                );
                w.if_let_stmt(
                    &cond,
                    &format!("self.{}", oneof_field.oneof_field_name),
                    |w| cb(w, &oneof_field.elem.rust_storage_elem_type(&file_and_mod)),
                )
            }
            _ => {
                let v_type = self.full_storage_iter_elem_type(&file_and_mod);
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
            &SingularField {
                flag: SingularFieldFlag::WithFlag { option_kind, .. },
                ..
            } => {
                self.write_self_field_assign(w, &option_kind.wrap_value(value, &self.customize));
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
        value: &RustValueTyped,
    ) {
        let SingularField { ref elem, ref flag } = s;
        let converted = value.into_type(
            elem.rust_storage_elem_type(
                &self
                    .proto_field
                    .message
                    .scope
                    .file_and_mod(self.customize.clone()),
            )
            .clone(),
            &self.customize,
        );
        let wrapped = match flag {
            SingularFieldFlag::WithoutFlag => converted.value,
            SingularFieldFlag::WithFlag { option_kind, .. } => {
                option_kind.wrap_value(&converted.value, &self.customize)
            }
        };
        self.write_self_field_assign(w, &wrapped);
    }

    fn write_self_field_assign_value(&self, w: &mut CodeWriter, value: &RustValueTyped) {
        match self.kind {
            FieldKind::Repeated(..) | FieldKind::Map(..) => {
                let converted = value.into_type(
                    self.full_storage_type(
                        &self
                            .proto_field
                            .message
                            .scope
                            .file_and_mod(self.customize.clone()),
                    ),
                    &self.customize,
                );
                self.write_self_field_assign(w, &converted.value);
            }
            FieldKind::Singular(ref s) => {
                self.write_self_field_assign_value_singular(w, s, value);
            }
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    fn write_self_field_assign_default(
        &self,
        field_kind: &SingularOrOneofField,
        w: &mut CodeWriter,
    ) {
        match field_kind {
            SingularOrOneofField::Oneof(oneof) => {
                w.write_line(format!(
                    "self.{} = ::std::option::Option::Some({}({}))",
                    oneof.oneof_field_name,
                    oneof.variant_path(&self.proto_field.message.scope.rust_path_to_file()),
                    // TODO: default from .proto is not needed here (?)
                    self.element_default_value_rust()
                        .into_type(
                            self.full_storage_iter_elem_type(
                                &self
                                    .proto_field
                                    .message
                                    .scope
                                    .file_and_mod(self.customize.clone())
                            ),
                            &self.customize
                        )
                        .value
                ));
            }
            SingularOrOneofField::Singular(singular) => {
                // Note it is different from C++ protobuf, where field is initialized
                // with default value
                match singular.flag {
                    SingularFieldFlag::WithFlag { option_kind, .. } => match option_kind {
                        OptionKind::MessageField => {
                            let self_field = self.self_field();
                            w.write_line(&format!("{}.set_default();", self_field));
                        }
                        _ => {
                            self.write_self_field_assign_some(
                                w,
                                singular,
                                &self
                                    .elem()
                                    .rust_storage_elem_type(
                                        &self
                                            .proto_field
                                            .message
                                            .scope
                                            .file_and_mod(self.customize.clone()),
                                    )
                                    .default_value_typed(&self.customize, false)
                                    .into_type(
                                        singular.elem.rust_storage_elem_type(
                                            &self
                                                .proto_field
                                                .message
                                                .scope
                                                .file_and_mod(self.customize.clone()),
                                        ),
                                        &self.customize,
                                    )
                                    .value,
                            );
                        }
                    },
                    SingularFieldFlag::WithoutFlag => unimplemented!(),
                }
            }
        }
    }

    fn self_field_vec_packed_size(&self) -> String {
        let fn_name = match self.proto_type {
            Type::TYPE_ENUM => "vec_packed_enum_or_unknown_size",
            Type::TYPE_SINT32 => "vec_packed_sint32_size",
            Type::TYPE_SINT64 => "vec_packed_sint64_size",
            Type::TYPE_INT32 => "vec_packed_int32_size",
            Type::TYPE_INT64 => "vec_packed_int64_size",
            Type::TYPE_UINT32 => "vec_packed_uint32_size",
            Type::TYPE_UINT64 => "vec_packed_uint64_size",
            Type::TYPE_BOOL => "vec_packed_bool_size",
            Type::TYPE_FIXED32 => "vec_packed_fixed32_size",
            Type::TYPE_FIXED64 => "vec_packed_fixed64_size",
            Type::TYPE_SFIXED32 => "vec_packed_sfixed32_size",
            Type::TYPE_SFIXED64 => "vec_packed_sfixed64_size",
            Type::TYPE_FLOAT => "vec_packed_float_size",
            Type::TYPE_DOUBLE => "vec_packed_double_size",
            t => unreachable!("{:?}", t),
        };
        format!(
            "{}::rt::{fn_name}({}, &{})",
            protobuf_crate_path(&self.customize),
            self.proto_field.number(),
            self.self_field()
        )
    }

    pub(crate) fn clear_field_func(&self) -> String {
        format!("clear_{}", self.rust_name)
    }

    fn write_merge_from_field_message_string_bytes_repeated(
        &self,
        r: &RepeatedField,
        w: &mut CodeWriter,
    ) {
        let read_fn = match &r.elem {
            FieldElem::Message(..) => "read_message",
            FieldElem::Primitive(Type::TYPE_STRING, PrimitiveTypeVariant::Default) => "read_string",
            FieldElem::Primitive(Type::TYPE_STRING, PrimitiveTypeVariant::TokioBytes) => {
                "read_tokio_chars"
            }
            FieldElem::Primitive(Type::TYPE_BYTES, PrimitiveTypeVariant::Default) => "read_bytes",
            FieldElem::Primitive(Type::TYPE_BYTES, PrimitiveTypeVariant::TokioBytes) => {
                "read_tokio_bytes"
            }
            _ => unreachable!("for field {}", self.proto_field.field),
        };
        w.write_line(&format!("self.{}.push(is.{}()?);", self.rust_name, read_fn,));
    }

    fn tag_with_wire_type(&self, wire_type: WireType) -> u32 {
        make_tag(self.proto_field.number() as u32, wire_type)
    }

    fn tag(&self) -> u32 {
        self.tag_with_wire_type(self.wire_type)
    }

    // Write `merge_from` part for this oneof field
    fn write_merge_from_oneof_case_block(&self, o: &OneofField, w: &mut CodeWriter) {
        w.case_block(&format!("{}", self.tag()), |w| {
            let typed = RustValueTyped {
                value: format!(
                    "{}?",
                    self.proto_type.read("is", o.elem.primitive_type_variant())
                ),
                rust_type: self.full_storage_iter_elem_type(
                    &self
                        .proto_field
                        .message
                        .scope
                        .file_and_mod(self.customize.clone()),
                ),
            };

            let maybe_boxed = if o.boxed {
                typed.boxed(&self.customize)
            } else {
                typed
            };

            w.write_line(&format!(
                "self.{} = ::std::option::Option::Some({}({}));",
                o.oneof_field_name,
                o.variant_path(&self.proto_field.message.scope.rust_path_to_file()),
                maybe_boxed.value
            ));
        })
    }

    // Write `merge_from` part for this map field
    fn write_merge_from_map_case_block(&self, map: &MapField, w: &mut CodeWriter) {
        let MapField { key, value, .. } = map;
        w.case_block(&format!("{}", self.tag()), |w| {
            w.write_line(&format!("let len = is.read_raw_varint32()?;",));
            w.write_line(&format!("let old_limit = is.push_limit(len as u64)?;"));
            w.write_line(&format!(
                "let mut key = ::std::default::Default::default();"
            ));
            w.write_line(&format!(
                "let mut value = ::std::default::Default::default();"
            ));
            w.while_block("let Some(tag) = is.read_raw_tag_or_eof()?", |w| {
                w.match_block("tag", |w| {
                    let key_tag = make_tag(1, WireType::for_type(key.proto_type()));
                    let value_tag = make_tag(2, WireType::for_type(value.proto_type()));
                    w.case_expr(
                        &format!("{key_tag}"),
                        &format!("key = {read}", read = key.read_one_liner()),
                    );
                    w.case_expr(
                        &format!("{value_tag}"),
                        &format!("value = {read}", read = value.read_one_liner()),
                    );
                    w.case_expr(
                        "_",
                        &format!(
                            "{protobuf_crate}::rt::skip_field_for_tag(tag, is)?",
                            protobuf_crate = protobuf_crate_path(&self.customize)
                        ),
                    );
                });
            });
            w.write_line(&format!("is.pop_limit(old_limit);"));
            w.write_line(&format!(
                "{field}.insert(key, value);",
                field = self.self_field()
            ));
        });
    }

    // Write `merge_from` part for this singular field
    fn write_merge_from_singular_case_block(&self, s: &SingularField, w: &mut CodeWriter) {
        w.case_block(&format!("{}", self.tag()), |w| match s.elem {
            FieldElem::Message(..) => {
                w.write_line(&format!(
                    "{}::rt::read_singular_message_into_field(is, &mut self.{})?;",
                    protobuf_crate_path(&self.customize),
                    self.rust_name,
                ));
            }
            _ => {
                let read_proc = s.elem.read_one_liner();
                self.write_self_field_assign_some(w, s, &read_proc);
            }
        })
    }

    // Write `merge_from` part for this repeated field
    fn write_merge_from_repeated_case_block(&self, w: &mut CodeWriter) {
        let field = match self.kind {
            FieldKind::Repeated(ref field) => field,
            _ => panic!(),
        };

        match field.elem {
            FieldElem::Message(..)
            | FieldElem::Primitive(field_descriptor_proto::Type::TYPE_STRING, ..)
            | FieldElem::Primitive(field_descriptor_proto::Type::TYPE_BYTES, ..) => {
                w.case_block(&format!("{}", self.tag()), |w| {
                    self.write_merge_from_field_message_string_bytes_repeated(field, w);
                })
            }
            FieldElem::Enum(..) => {
                w.case_block(
                    &format!("{}", self.tag_with_wire_type(WireType::Varint)),
                    |w| {
                        w.write_line(&format!(
                            "self.{}.push(is.read_enum_or_unknown()?);",
                            self.rust_name,
                        ));
                    },
                );
                w.case_block(
                    &format!("{}", self.tag_with_wire_type(WireType::LengthDelimited)),
                    |w| {
                        w.write_line(&format!(
                            "{}::rt::read_repeated_packed_enum_or_unknown_into(is, &mut self.{})?",
                            protobuf_crate_path(&self.customize),
                            self.rust_name,
                        ));
                    },
                );
            }
            _ => {
                assert_ne!(self.wire_type, WireType::LengthDelimited);
                w.case_block(
                    &format!("{}", self.tag_with_wire_type(WireType::LengthDelimited)),
                    |w| {
                        w.write_line(&format!(
                            "is.read_repeated_packed_{}_into(&mut self.{})?;",
                            self.proto_type.protobuf_name(),
                            self.rust_name
                        ));
                    },
                );
                w.case_block(&format!("{}", self.tag()), |w| {
                    w.write_line(&format!(
                        "self.{}.push(is.read_{}()?);",
                        self.rust_name,
                        self.proto_type.protobuf_name(),
                    ));
                });
            }
        }
    }

    /// Write `merge_from` part for this field
    pub(crate) fn write_merge_from_field_case_block(&self, w: &mut CodeWriter) {
        match &self.kind {
            FieldKind::Oneof(oneof) => self.write_merge_from_oneof_case_block(oneof, w),
            FieldKind::Map(map) => self.write_merge_from_map_case_block(map, w),
            FieldKind::Singular(ref s) => self.write_merge_from_singular_case_block(s, w),
            FieldKind::Repeated(..) => self.write_merge_from_repeated_case_block(w),
        }
    }

    pub(crate) fn write_element_size(
        &self,
        elem: &FieldElem,
        w: &mut CodeWriter,
        item_var: &RustValueTyped,
        sum_var: &str,
    ) {
        assert!(!self.is_repeated_packed());

        elem.write_element_size(
            self.proto_field.number() as u32,
            item_var,
            HowToGetMessageSize::Compute,
            sum_var,
            &self.customize,
            w,
        );
    }

    fn write_write_map_field(
        &self,
        key: &FieldElem,
        value: &FieldElem,
        os: &str,
        w: &mut CodeWriter,
    ) {
        self.for_each_map_entry(key, value, w, |k, v, w| {
            w.write_line("let mut entry_size = 0;");
            key.write_element_size(
                1,
                k,
                HowToGetMessageSize::GetCached,
                "entry_size",
                &self.customize,
                w,
            );
            value.write_element_size(
                2,
                v,
                HowToGetMessageSize::GetCached,
                "entry_size",
                &self.customize,
                w,
            );
            w.write_line(&format!(
                "{os}.write_raw_varint32({tag})?; // Tag.",
                tag = make_tag(self.proto_field.number() as u32, WireType::LengthDelimited),
            ));
            w.write_line(&format!("{os}.write_raw_varint32(entry_size as u32)?;",));
            key.write_write_element(1, k, &self.file_and_mod(), &self.customize, os, w);
            value.write_write_element(2, v, &self.file_and_mod(), &self.customize, os, w);
        });
    }

    pub(crate) fn write_message_write_field(&self, os: &str, w: &mut CodeWriter) {
        match &self.kind {
            FieldKind::Singular(s @ SingularField { elem, .. }) => {
                self.write_if_let_self_field_is_some(s, w, |v, w| {
                    self.write_write_element(&elem, w, os, &v);
                });
            }
            FieldKind::Repeated(RepeatedField {
                packed: false,
                elem,
                ..
            }) => {
                self.write_for_self_field(w, "v", |w, v_type| {
                    let v = RustValueTyped {
                        value: "v".to_owned(),
                        rust_type: v_type.clone(),
                    };
                    self.write_write_element(elem, w, "os", &v);
                });
            }
            FieldKind::Repeated(RepeatedField { packed: true, .. }) => {
                w.write_line(&format!(
                    "os.write_repeated_packed_{}({}, &{})?;",
                    self.os_write_fn_suffix_with_unknown_for_enum(),
                    self.proto_field.number(),
                    self.self_field()
                ));
            }
            FieldKind::Map(MapField { key, value, .. }) => {
                self.write_write_map_field(key, value, os, w)
            }
            FieldKind::Oneof(..) => unreachable!(),
        };
    }

    fn for_each_map_entry(
        &self,
        key: &FieldElem,
        value: &FieldElem,
        w: &mut CodeWriter,
        cb: impl FnOnce(&RustValueTyped, &RustValueTyped, &mut CodeWriter),
    ) {
        w.for_stmt(&format!("&{}", self.self_field()), "(k, v)", move |w| {
            let k = RustValueTyped {
                value: "k".to_owned(),
                rust_type: key.rust_storage_elem_type(&self.file_and_mod()).wrap_ref(),
            };
            let v = RustValueTyped {
                value: "v".to_owned(),
                rust_type: value
                    .rust_storage_elem_type(&self.file_and_mod())
                    .wrap_ref(),
            };
            cb(&k, &v, w)
        });
    }

    fn write_compute_map_field_size(
        &self,
        sum_var: &str,
        key: &FieldElem<'a>,
        value: &FieldElem<'a>,
        w: &mut CodeWriter,
    ) {
        self.for_each_map_entry(key, value, w, |k, v, w| {
                w.write_line("let mut entry_size = 0;");
                key.write_element_size(1, k, HowToGetMessageSize::Compute, "entry_size", &self.customize, w);
                value.write_element_size(2, v, HowToGetMessageSize::Compute, "entry_size", &self.customize, w);
                w.write_line(&format!("{sum_var} += {tag_size} + {protobuf_crate}::rt::compute_raw_varint64_size(entry_size) + entry_size",
                    tag_size = self.tag_size(),
                    protobuf_crate = protobuf_crate_path(&self.customize),
                ));
        });
    }

    pub(crate) fn write_message_compute_field_size(&self, sum_var: &str, w: &mut CodeWriter) {
        match &self.kind {
            FieldKind::Singular(s @ SingularField { elem, .. }) => {
                self.write_if_let_self_field_is_some(s, w, |v, w| {
                    self.write_element_size(&elem, w, v, sum_var)
                });
            }
            FieldKind::Repeated(RepeatedField {
                packed: false,
                elem,
                ..
            }) => {
                match elem.proto_type().encoded_size() {
                    Some(s) => {
                        let tag_size = self.tag_size();
                        let self_field = self.self_field();
                        w.write_line(&format!(
                            "{} += {} * {}.len() as u64;",
                            sum_var,
                            (s + tag_size) as isize,
                            self_field
                        ));
                    }
                    None => {
                        self.write_for_self_field(w, "value", |w, value_type| {
                            self.write_element_size(
                                elem,
                                w,
                                &RustValueTyped {
                                    value: "value".to_owned(),
                                    rust_type: value_type.clone(),
                                },
                                sum_var,
                            );
                        });
                    }
                };
            }
            FieldKind::Repeated(RepeatedField { packed: true, .. }) => {
                let size_expr = self.self_field_vec_packed_size();
                w.write_line(&format!("{} += {};", sum_var, size_expr));
            }
            FieldKind::Map(MapField { key, value, .. }) => {
                self.write_compute_map_field_size(sum_var, key, value, w)
            }
            FieldKind::Oneof(..) => unreachable!(),
        }
    }

    fn write_message_field_get_singular_message(&self, s: &SingularField, w: &mut CodeWriter) {
        match s.flag {
            SingularFieldFlag::WithoutFlag => unimplemented!(),
            SingularFieldFlag::WithFlag { option_kind, .. } => {
                let self_field = self.self_field();
                let ref field_type_name = self.elem().rust_storage_elem_type(
                    &self
                        .proto_field
                        .message
                        .scope
                        .file_and_mod(self.customize.clone()),
                );
                w.write_line(option_kind.unwrap_ref_or_else(
                    &format!("{}.as_ref()", self_field),
                    &format!(
                        "<{} as {}::Message>::default_instance()",
                        field_type_name.to_code(&self.customize),
                        protobuf_crate_path(&self.customize),
                    ),
                ));
            }
        }
    }

    fn write_message_field_get_singular_enum(
        &self,
        flag: SingularFieldFlag,
        _elem: &FieldElemEnum,
        w: &mut CodeWriter,
    ) {
        match flag {
            SingularFieldFlag::WithoutFlag => {
                w.write_line(&format!("self.{}.enum_value_or_default()", self.rust_name));
            }
            SingularFieldFlag::WithFlag { .. } => {
                w.match_expr(&self.self_field(), |w| {
                    let default_value = self.xxx_default_value_rust();
                    w.case_expr("Some(e)", &format!("e.enum_value_or({})", default_value));
                    w.case_expr("None", &format!("{}", default_value));
                });
            }
        }
    }

    fn write_message_field_get_singular(&self, singular: &SingularField, w: &mut CodeWriter) {
        let get_xxx_return_type = self.getter_return_type();

        match singular.elem {
            FieldElem::Message(..) => self.write_message_field_get_singular_message(singular, w),
            FieldElem::Enum(ref en) => {
                self.write_message_field_get_singular_enum(singular.flag, en, w)
            }
            _ => {
                let get_xxx_default_value_rust = self.xxx_default_value_rust();
                let self_field = self.self_field();
                match singular {
                    &SingularField {
                        ref elem,
                        flag: SingularFieldFlag::WithFlag { option_kind, .. },
                        ..
                    } => {
                        if get_xxx_return_type.is_ref().is_some() {
                            let as_option = self.self_field_as_option(elem, option_kind);
                            w.match_expr(&as_option.value, |w| {
                                let v_type = as_option.rust_type.elem_type();
                                let r_type = self.getter_return_type();
                                w.case_expr(
                                    "Some(v)",
                                    v_type.into_target(&r_type, "v", &self.customize),
                                );
                                let get_xxx_default_value_rust = self.xxx_default_value_rust();
                                w.case_expr("None", get_xxx_default_value_rust);
                            });
                        } else {
                            w.write_line(&format!(
                                "{}.unwrap_or({})",
                                self_field, get_xxx_default_value_rust
                            ));
                        }
                    }
                    &SingularField {
                        flag: SingularFieldFlag::WithoutFlag,
                        ..
                    } => {
                        w.write_line(
                            self.full_storage_type(
                                &self
                                    .proto_field
                                    .message
                                    .scope
                                    .file_and_mod(self.customize.clone()),
                            )
                            .into_target(
                                &get_xxx_return_type,
                                &self_field,
                                &self.customize,
                            ),
                        );
                    }
                }
            }
        }
    }

    fn write_message_field_get_oneof(&self, o: &OneofField, w: &mut CodeWriter) {
        let get_xxx_return_type = SingularOrOneofField::Oneof(o.clone()).getter_return_type(
            &self
                .proto_field
                .message
                .scope
                .file_and_mod(self.customize.clone()),
        );
        let OneofField { ref elem, .. } = o;
        w.match_expr(&format!("self.{}", o.oneof_field_name), |w| {
            let (refv, vtype) = if !elem.is_copy() {
                (
                    "ref v",
                    elem.rust_storage_elem_type(
                        &self
                            .proto_field
                            .message
                            .scope
                            .file_and_mod(self.customize.clone()),
                    )
                    .ref_type(),
                )
            } else {
                (
                    "v",
                    elem.rust_storage_elem_type(
                        &self
                            .proto_field
                            .message
                            .scope
                            .file_and_mod(self.customize.clone()),
                    ),
                )
            };
            w.case_expr(
                format!(
                    "::std::option::Option::Some({}({}))",
                    o.variant_path(&self.proto_field.message.scope.rust_path_to_file()),
                    refv
                ),
                vtype.into_target(&get_xxx_return_type, "v", &self.customize),
            );
            w.case_expr("_", self.xxx_default_value_rust());
        });
    }

    fn write_message_field_get(&self, w: &mut CodeWriter) {
        let get_xxx_return_type = self.getter_return_type();
        let fn_def = format!(
            "{}(&self) -> {}",
            self.rust_name,
            get_xxx_return_type.to_code(&self.customize)
        );

        w.pub_fn(&fn_def, |w| match self.kind {
            FieldKind::Oneof(ref o) => {
                self.write_message_field_get_oneof(o, w);
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

    fn has_name(&self) -> RustIdent {
        RustIdent::new(&format!("has_{}", self.rust_name.get()))
    }

    fn set_name(&self) -> RustIdent {
        RustIdent::new(&format!("set_{}", self.rust_name.get()))
    }

    fn mut_name(&self) -> RustIdent {
        RustIdent::new(&format!("mut_{}", self.rust_name.get()))
    }

    fn write_message_field_has(&self, w: &mut CodeWriter) {
        w.pub_fn(
            &format!("{}(&self) -> bool", self.has_name()),
            |w| match self.kind {
                FieldKind::Oneof(ref oneof) => {
                    w.match_expr(&format!("self.{}", oneof.oneof_field_name), |w| {
                        w.case_expr(
                            format!(
                                "::std::option::Option::Some({}(..))",
                                oneof.variant_path(
                                    &self.proto_field.message.scope.rust_path_to_file()
                                )
                            ),
                            "true",
                        );
                        w.case_expr("_", "false");
                    });
                }
                _ => {
                    let self_field_is_some = self.self_field_is_some();
                    w.write_line(self_field_is_some);
                }
            },
        );
    }

    fn write_message_field_set(&self, w: &mut CodeWriter) {
        let set_xxx_param_type = self.set_xxx_param_type(
            &self
                .proto_field
                .message
                .scope
                .file_and_mod(self.customize.clone()),
        );
        w.comment("Param is passed by value, moved");
        w.pub_fn(
            &format!(
                "{}(&mut self, v: {})",
                self.set_name(),
                set_xxx_param_type.to_code(&self.customize)
            ),
            |w| {
                let value_typed = RustValueTyped {
                    value: "v".to_owned(),
                    rust_type: set_xxx_param_type.clone(),
                };
                match self.kind {
                    FieldKind::Oneof(ref oneof) => {
                        let v = set_xxx_param_type.into_target(
                            &oneof.rust_type(
                                &self
                                    .proto_field
                                    .message
                                    .scope
                                    .file_and_mod(self.customize.clone()),
                            ),
                            "v",
                            &self.customize,
                        );
                        w.write_line(&format!(
                            "self.{} = ::std::option::Option::Some({}({}))",
                            oneof.oneof_field_name,
                            oneof.variant_path(&self.proto_field.message.scope.rust_path_to_file()),
                            v
                        ));
                    }
                    _ => {
                        self.write_self_field_assign_value(w, &value_typed);
                    }
                }
            },
        );
    }

    fn write_message_field_mut_singular_with_flag(
        &self,
        s: &SingularField,
        option_kind: OptionKind,
        w: &mut CodeWriter,
    ) {
        let self_field = self.self_field();
        match option_kind {
            OptionKind::MessageField => {
                w.write_line(&format!("{}.mut_or_insert_default()", self_field))
            }
            OptionKind::Option => {
                self.write_if_self_field_is_none(w, |w| {
                    self.write_self_field_assign_default(
                        &SingularOrOneofField::Singular(s.clone()),
                        w,
                    );
                });
                w.write_line(&format!("{}.as_mut().unwrap()", self_field));
            }
        }
    }

    fn write_message_field_mut_singular(&self, s: &SingularField, w: &mut CodeWriter) {
        match s {
            s @ SingularField {
                flag: SingularFieldFlag::WithFlag { option_kind, .. },
                ..
            } => self.write_message_field_mut_singular_with_flag(s, *option_kind, w),
            SingularField {
                flag: SingularFieldFlag::WithoutFlag,
                ..
            } => w.write_line(&format!("&mut {}", self.self_field())),
        }
    }

    fn write_message_field_mut(&self, w: &mut CodeWriter) {
        let mut_xxx_return_type = self.mut_xxx_return_type(
            &self
                .proto_field
                .message
                .scope
                .file_and_mod(self.customize.clone()),
        );
        w.comment("Mutable pointer to the field.");
        if self.is_singular() {
            w.comment("If field is not initialized, it is initialized with default value first.");
        }
        let fn_def = match mut_xxx_return_type {
            RustType::Ref(ref param) => format!(
                "{}(&mut self) -> &mut {}",
                self.mut_name(),
                param.to_code(&self.customize)
            ),
            _ => panic!(
                "not a ref: {}",
                mut_xxx_return_type.to_code(&self.customize)
            ),
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
                FieldKind::Oneof(ref o) => {
                    let self_field_oneof = format!("self.{}", o.oneof_field_name);

                    // if oneof does not contain current field
                    w.if_let_else_stmt(
                        &format!(
                            "::std::option::Option::Some({}(_))",
                            o.variant_path(&self.proto_field.message.scope.rust_path_to_file())
                        )[..],
                        &self_field_oneof[..],
                        |w| {
                            // initialize it with default value
                            w.write_line(&format!(
                                "{} = ::std::option::Option::Some({}({}));",
                                self_field_oneof,
                                o.variant_path(&self.proto_field.message.scope.rust_path_to_file()),
                                self.element_default_value_rust()
                                    .into_type(
                                        o.rust_type(
                                            &self
                                                .proto_field
                                                .message
                                                .scope
                                                .file_and_mod(self.customize.clone())
                                        ),
                                        &self.customize
                                    )
                                    .value
                            ));
                        },
                    );

                    // extract field
                    w.match_expr(self_field_oneof, |w| {
                        w.case_expr(
                            format!(
                                "::std::option::Option::Some({}(ref mut v))",
                                o.variant_path(&self.proto_field.message.scope.rust_path_to_file())
                            ),
                            "v",
                        );
                        w.case_expr("_", "panic!()");
                    });
                }
            }
        });
    }

    fn write_message_field_take_oneof(&self, o: &OneofField, w: &mut CodeWriter) {
        let take_xxx_return_type = self.take_xxx_return_type(
            &self
                .proto_field
                .message
                .scope
                .file_and_mod(self.customize.clone()),
        );

        // TODO: replace with if let
        w.write_line(&format!("if self.{}() {{", self.has_name()));
        w.indented(|w| {
            let self_field_oneof = format!("self.{}", o.oneof_field_name);
            w.match_expr(format!("{}.take()", self_field_oneof), |w| {
                let value_in_some = o
                    .rust_type(
                        &self
                            .proto_field
                            .message
                            .scope
                            .file_and_mod(self.customize.clone()),
                    )
                    .value("v".to_owned());
                let converted = value_in_some.into_type(
                    self.take_xxx_return_type(
                        &self
                            .proto_field
                            .message
                            .scope
                            .file_and_mod(self.customize.clone()),
                    ),
                    &self.customize,
                );
                w.case_expr(
                    format!(
                        "::std::option::Option::Some({}(v))",
                        o.variant_path(&self.proto_field.message.scope.rust_path_to_file())
                    ),
                    &converted.value,
                );
                w.case_expr("_", "panic!()");
            });
        });
        w.write_line("} else {");
        w.indented(|w| {
            w.write_line(
                self.elem()
                    .rust_storage_elem_type(
                        &self
                            .proto_field
                            .message
                            .scope
                            .file_and_mod(self.customize.clone()),
                    )
                    .default_value_typed(&self.customize, false)
                    .into_type(take_xxx_return_type.clone(), &self.customize)
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
                    w.write_line(
                        &option_kind.unwrap_or_else(
                            &format!("{}.take()", self.self_field()),
                            &elem
                                .rust_storage_elem_type(
                                    &self
                                        .proto_field
                                        .message
                                        .scope
                                        .file_and_mod(self.customize.clone()),
                                )
                                .default_value(&self.customize, false),
                        ),
                    );
                } else {
                    w.write_line(&format!(
                        "{}.take().unwrap_or({})",
                        self.self_field(),
                        self.element_default_value_rust().value
                    ));
                }
            }
            SingularField {
                flag: SingularFieldFlag::WithoutFlag,
                ..
            } => w.write_line(&format!(
                "::std::mem::replace(&mut {}, {})",
                self.self_field(),
                self.full_storage_type(
                    &self
                        .proto_field
                        .message
                        .scope
                        .file_and_mod(self.customize.clone())
                )
                .default_value(&self.customize, false)
            )),
        }
    }

    fn write_message_field_take(&self, w: &mut CodeWriter) {
        let take_xxx_return_type = self.take_xxx_return_type(
            &self
                .proto_field
                .message
                .scope
                .file_and_mod(self.customize.clone()),
        );
        w.comment("Take field");
        w.pub_fn(
            &format!(
                "take_{}(&mut self) -> {}",
                self.rust_name,
                take_xxx_return_type.to_code(&self.customize)
            ),
            |w| match self.kind {
                FieldKind::Singular(ref s) => self.write_message_field_take_singular(&s, w),
                FieldKind::Oneof(ref o) => self.write_message_field_take_oneof(o, w),
                FieldKind::Repeated(..) | FieldKind::Map(..) => {
                    w.write_line(&format!(
                        "::std::mem::replace(&mut self.{}, {})",
                        self.rust_name,
                        take_xxx_return_type.default_value(&self.customize, false)
                    ));
                }
            },
        );
    }

    pub(crate) fn write_message_single_field_accessors(&self, w: &mut CodeWriter) {
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

pub(crate) fn rust_field_name_for_protobuf_field_name(name: &str) -> RustIdent {
    RustIdent::new(name)
}

pub(crate) fn rust_variant_name_for_protobuf_oneof_field_name(name: &str) -> RustIdent {
    let name = camel_case(name);
    RustIdent::new(&name)
}
