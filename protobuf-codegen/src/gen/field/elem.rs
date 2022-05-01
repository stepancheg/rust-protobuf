use protobuf::descriptor::field_descriptor_proto::Type;
use protobuf::reflect::RuntimeFieldType;
use protobuf::rt::tag_size;
use protobuf_parse::ProtobufAbsPath;

use crate::gen::code_writer::CodeWriter;
use crate::gen::field::type_ext::TypeExt;
use crate::gen::file_and_mod::FileAndMod;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::message::RustTypeMessage;
use crate::gen::rust::ident_with_path::RustIdentWithPath;
use crate::gen::rust_types_values::message_or_enum_to_rust_relative;
use crate::gen::rust_types_values::PrimitiveTypeVariant;
use crate::gen::rust_types_values::RustType;
use crate::gen::rust_types_values::RustValueTyped;
use crate::gen::scope::EnumValueWithContext;
use crate::gen::scope::FieldWithContext;
use crate::gen::scope::MessageOrEnumWithScope;
use crate::gen::scope::MessageWithScope;
use crate::gen::scope::RootScope;
use crate::Customize;

#[derive(Clone, Debug)]
pub(crate) struct FieldElemEnum<'a> {
    /// Enum default value variant, either from proto or from enum definition
    default_value: EnumValueWithContext<'a>,
}

impl<'a> FieldElemEnum<'a> {
    fn rust_name_relative(&self, reference: &FileAndMod) -> RustIdentWithPath {
        message_or_enum_to_rust_relative(&self.default_value.en, reference)
    }

    pub(crate) fn enum_rust_type(&self, reference: &FileAndMod) -> RustType {
        RustType::Enum(
            self.rust_name_relative(reference),
            self.default_value.rust_name(),
            self.default_value.proto.proto().number(),
        )
    }

    fn enum_or_unknown_rust_type(&self, reference: &FileAndMod) -> RustType {
        RustType::EnumOrUnknown(
            self.rust_name_relative(reference),
            self.default_value.rust_name(),
            self.default_value.proto.proto().number(),
        )
    }

    pub(crate) fn default_value_rust_expr(&self, reference: &FileAndMod) -> RustIdentWithPath {
        self.rust_name_relative(reference)
            .to_path()
            .with_ident(self.default_value.rust_name())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FieldElemMessage<'a> {
    pub message: MessageWithScope<'a>,
}

impl<'a> FieldElemMessage<'a> {
    pub(crate) fn rust_name_relative(&self, reference: &FileAndMod) -> RustTypeMessage {
        RustTypeMessage(message_or_enum_to_rust_relative(&self.message, reference))
    }

    fn rust_type(&self, reference: &FileAndMod) -> RustType {
        RustType::Message(self.rust_name_relative(reference))
    }
}

#[derive(Clone, Debug)]
pub(crate) enum FieldElem<'a> {
    Primitive(Type, PrimitiveTypeVariant),
    Message(FieldElemMessage<'a>),
    Enum(FieldElemEnum<'a>),
    Group,
}

pub(crate) enum HowToGetMessageSize {
    Compute,
    GetCached,
}

impl<'a> FieldElem<'a> {
    pub(crate) fn proto_type(&self) -> Type {
        match *self {
            FieldElem::Primitive(t, ..) => t,
            FieldElem::Group => Type::TYPE_GROUP,
            FieldElem::Message(..) => Type::TYPE_MESSAGE,
            FieldElem::Enum(..) => Type::TYPE_ENUM,
        }
    }

    pub(crate) fn is_copy(&self) -> bool {
        self.proto_type().is_copy()
    }

    pub(crate) fn rust_storage_elem_type(&self, reference: &FileAndMod) -> RustType {
        match *self {
            FieldElem::Primitive(t, PrimitiveTypeVariant::Default) => t.rust_type(),
            FieldElem::Primitive(Type::TYPE_STRING, PrimitiveTypeVariant::TokioBytes) => {
                RustType::Chars
            }
            FieldElem::Primitive(Type::TYPE_BYTES, PrimitiveTypeVariant::TokioBytes) => {
                RustType::Bytes
            }
            FieldElem::Primitive(.., PrimitiveTypeVariant::TokioBytes) => unreachable!(),
            FieldElem::Group => RustType::Group,
            FieldElem::Message(ref m) => m.rust_type(reference),
            FieldElem::Enum(ref en) => en.enum_or_unknown_rust_type(reference),
        }
    }

    // Type of set_xxx function parameter type for singular fields
    pub(crate) fn rust_set_xxx_param_type(&self, reference: &FileAndMod) -> RustType {
        if let FieldElem::Enum(ref en) = *self {
            en.enum_rust_type(reference)
        } else {
            self.rust_storage_elem_type(reference)
        }
    }

    pub(crate) fn primitive_type_variant(&self) -> PrimitiveTypeVariant {
        match self {
            &FieldElem::Primitive(_, v) => v,
            _ => PrimitiveTypeVariant::Default,
        }
    }

    pub(crate) fn singular_field_size(
        &self,
        field_number: u32,
        var: &RustValueTyped,
        customize: &Customize,
    ) -> String {
        let tag_size = tag_size(field_number);
        match self.proto_type().encoded_size() {
            Some(data_size) => format!("{tag_size} + {data_size}"),
            None => match self.proto_type() {
                Type::TYPE_MESSAGE => panic!("not a single-liner"),
                // We are not inlining `bytes_size` here,
                // assuming the compiler is smart enough to do it for us.
                // https://rust.godbolt.org/z/GrKa5zxq6
                Type::TYPE_BYTES => format!(
                    "{}::rt::bytes_size({}, &{})",
                    protobuf_crate_path(customize),
                    field_number,
                    var.value
                ),
                Type::TYPE_STRING => format!(
                    "{}::rt::string_size({}, &{})",
                    protobuf_crate_path(customize),
                    field_number,
                    var.value
                ),
                Type::TYPE_ENUM => {
                    format!(
                        "{}::rt::int32_size({}, {}.value())",
                        protobuf_crate_path(customize),
                        field_number,
                        var.value,
                    )
                }
                _ => {
                    let param_type = match &var.rust_type {
                        RustType::Ref(t) => (**t).clone(),
                        t => t.clone(),
                    };
                    let f = match self.proto_type() {
                        Type::TYPE_SINT32 => "sint32_size",
                        Type::TYPE_SINT64 => "sint64_size",
                        Type::TYPE_INT32 => "int32_size",
                        Type::TYPE_INT64 => "int64_size",
                        Type::TYPE_UINT32 => "uint32_size",
                        Type::TYPE_UINT64 => "uint64_size",
                        t => unreachable!("unexpected type: {:?}", t),
                    };
                    format!(
                        "{}::rt::{f}({}, {})",
                        protobuf_crate_path(customize),
                        field_number,
                        var.into_type(param_type, customize).value
                    )
                }
            },
        }
    }

    pub(crate) fn write_element_size(
        &self,
        field_number: u32,
        item_var: &RustValueTyped,
        how_to_get_message_size: HowToGetMessageSize,
        sum_var: &str,
        customize: &Customize,
        w: &mut CodeWriter,
    ) {
        let tag_size = tag_size(field_number);
        match self.proto_type() {
            Type::TYPE_MESSAGE => {
                match how_to_get_message_size {
                    HowToGetMessageSize::Compute => {
                        w.write_line(&format!("let len = {}.compute_size();", item_var.value))
                    }
                    HowToGetMessageSize::GetCached => w.write_line(&format!(
                        "let len = {}.cached_size() as u64;",
                        item_var.value
                    )),
                }
                w.write_line(&format!(
                    "{sum_var} += {tag_size} + {}::rt::compute_raw_varint64_size(len) + len;",
                    protobuf_crate_path(customize),
                ));
            }
            _ => {
                w.write_line(&format!(
                    "{sum_var} += {};",
                    self.singular_field_size(field_number, item_var, customize)
                ));
            }
        }
    }

    pub(crate) fn write_write_element(
        &self,
        field_number: u32,
        v: &RustValueTyped,
        file_and_mod: &FileAndMod,
        customize: &Customize,
        os: &str,
        w: &mut CodeWriter,
    ) {
        match self.proto_type() {
            Type::TYPE_MESSAGE => {
                let param_type = RustType::Ref(Box::new(self.rust_storage_elem_type(file_and_mod)));

                w.write_line(&format!(
                    "{}::rt::write_message_field_with_cached_size({}, {}, {})?;",
                    protobuf_crate_path(customize),
                    field_number,
                    v.into_type(param_type, customize).value,
                    os
                ));
            }
            _ => {
                let param_type = self.proto_type().os_write_fn_param_type();
                let os_write_fn_suffix = self.proto_type().protobuf_name();
                w.write_line(&format!(
                    "{}.write_{}({}, {})?;",
                    os,
                    os_write_fn_suffix,
                    field_number,
                    v.into_type(param_type, customize).value
                ));
            }
        }
    }

    pub(crate) fn read_one_liner(&self) -> String {
        format!(
            "{}?",
            self.proto_type().read("is", self.primitive_type_variant())
        )
    }
}

pub(crate) fn field_elem<'a>(
    field: &FieldWithContext,
    root_scope: &'a RootScope<'a>,
    customize: &Customize,
) -> FieldElem<'a> {
    if let RuntimeFieldType::Map(..) = field.field.runtime_field_type() {
        unreachable!();
    }

    if field.field.proto().type_() == Type::TYPE_GROUP {
        FieldElem::Group
    } else if field.field.proto().has_type_name() {
        let message_or_enum = root_scope
            .find_message_or_enum(&ProtobufAbsPath::from(field.field.proto().type_name()));
        match (field.field.proto().type_(), message_or_enum) {
            (Type::TYPE_MESSAGE, MessageOrEnumWithScope::Message(message)) => {
                FieldElem::Message(FieldElemMessage {
                    message: message.clone(),
                })
            }
            (Type::TYPE_ENUM, MessageOrEnumWithScope::Enum(enum_with_scope)) => {
                let default_value = if field.field.proto().has_default_value() {
                    enum_with_scope.value_by_name(field.field.proto().default_value())
                } else {
                    enum_with_scope.values()[0].clone()
                };
                FieldElem::Enum(FieldElemEnum { default_value })
            }
            _ => panic!("unknown named type: {:?}", field.field.proto().type_()),
        }
    } else if field.field.proto().has_type() {
        let tokio_for_bytes = customize.tokio_bytes.unwrap_or(false);
        let tokio_for_string = customize.tokio_bytes_for_string.unwrap_or(false);

        let elem = match field.field.proto().type_() {
            Type::TYPE_STRING if tokio_for_string => {
                FieldElem::Primitive(Type::TYPE_STRING, PrimitiveTypeVariant::TokioBytes)
            }
            Type::TYPE_BYTES if tokio_for_bytes => {
                FieldElem::Primitive(Type::TYPE_BYTES, PrimitiveTypeVariant::TokioBytes)
            }
            t => FieldElem::Primitive(t, PrimitiveTypeVariant::Default),
        };

        elem
    } else {
        panic!(
            "neither type_name, nor field_type specified for field: {}",
            field.field.name()
        );
    }
}
