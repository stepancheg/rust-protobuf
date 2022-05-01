use protobuf::descriptor::*;
use protobuf::reflect::FileDescriptor;
use protobuf_parse::ProtobufAbsPath;

use crate::customize::ctx::CustomizeElemCtx;
use crate::customize::Customize;
use crate::gen::code_writer::CodeWriter;
use crate::gen::field::rust_field_name_for_protobuf_field_name;
use crate::gen::file_and_mod::FileAndMod;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::message::RustTypeMessage;
use crate::gen::rust::ident_with_path::RustIdentWithPath;
use crate::gen::rust::rel_path::RustRelativePath;
use crate::gen::rust_types_values::*;
use crate::gen::scope::RootScope;

struct ExtGen<'a> {
    file: &'a FileDescriptor,
    root_scope: &'a RootScope<'a>,
    field: &'a FieldDescriptorProto,
    customize: Customize,
}

impl<'a> ExtGen<'a> {
    fn extendee_rust_name(&self) -> RustIdentWithPath {
        type_name_to_rust_relative(
            &ProtobufAbsPath::from(self.field.extendee()),
            &FileAndMod {
                file: self.file.proto().name().to_owned(),
                relative_mod: RustRelativePath::from("exts"),
                customize: self.customize.clone(),
            },
            self.root_scope,
        )
    }

    fn repeated(&self) -> bool {
        match self.field.label() {
            field_descriptor_proto::Label::LABEL_REPEATED => true,
            field_descriptor_proto::Label::LABEL_OPTIONAL => false,
            field_descriptor_proto::Label::LABEL_REQUIRED => {
                panic!("required ext field: {}", self.field.name())
            }
        }
    }

    fn return_type_gen(&self) -> ProtobufTypeGen {
        if self.field.has_type_name() {
            let rust_name_relative = type_name_to_rust_relative(
                &ProtobufAbsPath::from(self.field.type_name()),
                &FileAndMod {
                    file: self.file.proto().name().to_owned(),
                    relative_mod: RustRelativePath::from("exts"),
                    customize: self.customize.clone(),
                },
                self.root_scope,
            );
            match self.field.type_() {
                field_descriptor_proto::Type::TYPE_MESSAGE => {
                    ProtobufTypeGen::Message(RustTypeMessage(rust_name_relative))
                }
                field_descriptor_proto::Type::TYPE_ENUM => {
                    ProtobufTypeGen::EnumOrUnknown(rust_name_relative)
                }
                t => panic!("unknown type: {:?}", t),
            }
        } else {
            ProtobufTypeGen::Primitive(self.field.type_(), PrimitiveTypeVariant::Default)
        }
    }

    fn write(&self, w: &mut CodeWriter) {
        let suffix = if self.repeated() {
            "ExtFieldRepeated"
        } else {
            "ExtFieldOptional"
        };
        let field_type = format!(
            "{protobuf_crate}::ext::{suffix}",
            protobuf_crate = protobuf_crate_path(&self.customize)
        );
        w.pub_const(
            &rust_field_name_for_protobuf_field_name(self.field.name()).to_string(),
            &format!(
                "{field_type}<{extendee}, {rust_type}>",
                extendee=self.extendee_rust_name(),
                rust_type=self.return_type_gen().protobuf_value(&self.customize),
            ),
            &format!(
                "{field_type}::new({field_number}, {protobuf_crate}::descriptor::field_descriptor_proto::Type::{t:?})",
                field_number=self.field.number(),
                protobuf_crate = protobuf_crate_path(&self.customize),
                t=self.field.type_(),
            ),
        );
    }
}

pub(crate) fn write_extensions(
    file: &FileDescriptor,
    root_scope: &RootScope,
    w: &mut CodeWriter,
    customize: &CustomizeElemCtx,
) {
    if file.proto().extension.is_empty() {
        return;
    }

    if customize.for_elem.lite_runtime.unwrap_or(false) {
        w.write_line("");
        w.comment("Extension generation with lite runtime is not supported");
        return;
    }

    w.write_line("");
    w.write_line("/// Extension fields");
    w.pub_mod("exts", |w| {
        for field in &file.proto().extension {
            if field.type_() == field_descriptor_proto::Type::TYPE_GROUP {
                continue;
            }

            w.write_line("");
            ExtGen {
                file,
                root_scope,
                field,
                customize: customize.for_elem.clone(),
            }
            .write(w);
        }
    });
}
