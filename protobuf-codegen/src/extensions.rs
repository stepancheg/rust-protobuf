use super::code_writer::CodeWriter;
use super::rust_types_values::*;
use crate::customize::Customize;
use crate::field::rust_field_name_for_protobuf_field_name;
use crate::file_and_mod::FileAndMod;
use crate::inside::protobuf_crate_path;
use crate::protobuf_name::ProtobufAbsolutePath;
use crate::rust_name::RustIdentWithPath;
use crate::rust_name::RustRelativePath;
use crate::scope::RootScope;
use protobuf::descriptor::*;

struct ExtGen<'a> {
    file: &'a FileDescriptorProto,
    root_scope: &'a RootScope<'a>,
    field: &'a FieldDescriptorProto,
    customize: Customize,
}

impl<'a> ExtGen<'a> {
    fn extendee_rust_name(&self) -> RustIdentWithPath {
        type_name_to_rust_relative(
            &ProtobufAbsolutePath::from(self.field.get_extendee()),
            &FileAndMod {
                file: self.file.get_name().to_owned(),
                relative_mod: RustRelativePath::from("exts"),
                customize: self.customize.clone(),
            },
            self.root_scope,
        )
    }

    fn repeated(&self) -> bool {
        match self.field.get_label() {
            field_descriptor_proto::Label::LABEL_REPEATED => true,
            field_descriptor_proto::Label::LABEL_OPTIONAL => false,
            field_descriptor_proto::Label::LABEL_REQUIRED => {
                panic!("required ext field: {}", self.field.get_name())
            }
        }
    }

    fn return_type_gen(&self) -> ProtobufTypeGen {
        if self.field.has_type_name() {
            let rust_name_relative = type_name_to_rust_relative(
                &ProtobufAbsolutePath::from(self.field.get_type_name()),
                &FileAndMod {
                    file: self.file.get_name().to_owned(),
                    relative_mod: RustRelativePath::from("exts"),
                    customize: self.customize.clone(),
                },
                self.root_scope,
            );
            match self.field.get_field_type() {
                field_descriptor_proto::Type::TYPE_MESSAGE => {
                    ProtobufTypeGen::Message(rust_name_relative)
                }
                field_descriptor_proto::Type::TYPE_ENUM => {
                    ProtobufTypeGen::EnumOrUnknown(rust_name_relative)
                }
                t => panic!("unknown type: {:?}", t),
            }
        } else {
            ProtobufTypeGen::Primitive(self.field.get_field_type(), PrimitiveTypeVariant::Default)
        }
    }

    fn write(&self, w: &mut CodeWriter) {
        let suffix = if self.repeated() {
            "ExtFieldRepeated"
        } else {
            "ExtFieldOptional"
        };
        let field_type = format!("{}::ext::{}", protobuf_crate_path(&self.customize), suffix);
        w.pub_const(
            rust_field_name_for_protobuf_field_name(self.field.get_name()).get(),
            &format!(
                "{}<{}, {}>",
                field_type,
                self.extendee_rust_name(),
                self.return_type_gen().rust_type(&self.customize),
            ),
            &format!(
                "{} {{ field_number: {}, phantom: ::std::marker::PhantomData }}",
                field_type,
                self.field.get_number()
            ),
        );
    }
}

pub(crate) fn write_extensions(
    file: &FileDescriptorProto,
    root_scope: &RootScope,
    w: &mut CodeWriter,
    customize: &Customize,
) {
    if file.extension.is_empty() {
        return;
    }

    w.write_line("");
    w.write_line("/// Extension fields");
    w.pub_mod("exts", |w| {
        for field in &file.extension {
            if field.get_field_type() == field_descriptor_proto::Type::TYPE_GROUP {
                continue;
            }

            w.write_line("");
            ExtGen {
                file: file,
                root_scope: root_scope,
                field: field,
                customize: customize.clone(),
            }
            .write(w);
        }
    });
}
