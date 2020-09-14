use crate::descriptor::field_descriptor_proto;
use crate::descriptor::DescriptorProto;
use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FieldDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::field::index::ForwardRuntimeFieldType;
use crate::reflect::field::index::ForwardRuntimeTypeBox;
use crate::reflect::file::index::FileIndex;
use crate::reflect::find_message_or_enum::find_message_or_enum;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::name::protobuf_name_starts_with_package;
use crate::reflect::runtime_type_box::RuntimeTypeBox;
use crate::reflect::FileDescriptor;
use std::iter;

pub(crate) struct FileDescriptorBuilding<'a> {
    pub current_file_descriptor: &'a FileDescriptorProto,
    pub current_file_index: &'a FileIndex,
    pub deps_with_public: &'a [FileDescriptor],
}

impl<'a> FileDescriptorBuilding<'a> {
    fn all_descriptors(&self) -> impl Iterator<Item = &'a FileDescriptorProto> {
        iter::once(self.current_file_descriptor)
            .chain(self.deps_with_public.iter().map(|d| d.proto()))
    }

    pub fn find_enum(&self, full_name: &str) -> &'a EnumDescriptorProto {
        assert!(full_name.starts_with("."));

        for file in self.all_descriptors() {
            if let Some(name_to_package) =
                protobuf_name_starts_with_package(full_name, file.get_package())
            {
                if let Some((_, me)) = find_message_or_enum(file, name_to_package) {
                    match me {
                        MessageOrEnum::Enum(e) => return e,
                        MessageOrEnum::Message(_) => panic!("not an enum: {}", full_name),
                    }
                }
            }
        }

        panic!(
            "enum not found: {}, in files: {}",
            full_name,
            self.all_files_str()
        );
    }

    fn all_files_str(&self) -> String {
        self.all_descriptors()
            .map(|d| d.get_name())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn resolve_field_type(&self, field: &FieldDescriptorProto) -> ForwardRuntimeFieldType {
        match field.get_label() {
            field_descriptor_proto::Label::LABEL_OPTIONAL
            | field_descriptor_proto::Label::LABEL_REQUIRED => {
                ForwardRuntimeFieldType::Singular(self.resolve_field_element_type(field))
            }
            field_descriptor_proto::Label::LABEL_REPEATED => {
                let element = self.resolve_field_element_type(field);
                let type_proto = match &element {
                    ForwardRuntimeTypeBox::CurrentFileMessage(m) => Some(
                        self.current_file_index.messages[*m]
                            .path
                            .eval(self.current_file_descriptor)
                            .unwrap(),
                    ),
                    ForwardRuntimeTypeBox::RuntimeTypeBox(RuntimeTypeBox::Message(m)) => {
                        Some(m.get_proto())
                    }
                    _ => None,
                };
                match type_proto {
                    Some(m) if m.options.get_or_default().get_map_entry() => self.map_field(m),
                    _ => ForwardRuntimeFieldType::Repeated(element),
                }
            }
        }
    }

    fn resolve_field_element_type(&self, field: &FieldDescriptorProto) -> ForwardRuntimeTypeBox {
        match field.get_field_type() {
            field_descriptor_proto::Type::TYPE_MESSAGE
            | field_descriptor_proto::Type::TYPE_GROUP => {
                if let Some(name_to_package) = protobuf_name_starts_with_package(
                    field.get_type_name(),
                    self.current_file_descriptor.get_package(),
                ) {
                    if let Some(index) = self
                        .current_file_index
                        .message_by_name_to_package
                        .get(name_to_package)
                    {
                        return ForwardRuntimeTypeBox::CurrentFileMessage(*index);
                    }
                }
                for dep in self.deps_with_public {
                    if let Some(m) = dep.message_by_full_name(field.get_type_name()) {
                        return ForwardRuntimeTypeBox::RuntimeTypeBox(RuntimeTypeBox::Message(m));
                    }
                }
                panic!(
                    "message not found: {}; files: {}",
                    field.get_type_name(),
                    self.all_files_str()
                );
            }
            field_descriptor_proto::Type::TYPE_ENUM => {
                if let Some(name_to_package) = protobuf_name_starts_with_package(
                    field.get_type_name(),
                    self.current_file_descriptor.get_package(),
                ) {
                    if let Some(index) = self
                        .current_file_index
                        .enums_by_name_to_package
                        .get(name_to_package)
                    {
                        return ForwardRuntimeTypeBox::CurrentFileEnum(*index);
                    }
                }
                for dep in self.deps_with_public {
                    if let Some(m) = dep.enum_by_full_name(field.get_type_name()) {
                        return ForwardRuntimeTypeBox::RuntimeTypeBox(RuntimeTypeBox::Enum(m));
                    }
                }
                panic!(
                    "enum not found: {}; files: {}",
                    field.get_type_name(),
                    self.all_files_str()
                );
            }
            t => ForwardRuntimeTypeBox::RuntimeTypeBox(RuntimeTypeBox::from_proto_type(t)),
        }
    }

    fn map_field(&self, type_proto: &DescriptorProto) -> ForwardRuntimeFieldType {
        assert!(type_proto.get_name().ends_with("Entry"));

        assert_eq!(0, type_proto.extension.len());
        assert_eq!(0, type_proto.extension_range.len());
        assert_eq!(0, type_proto.nested_type.len());
        assert_eq!(0, type_proto.enum_type.len());

        assert_eq!(2, type_proto.field.len());
        let key = &type_proto.field[0];
        let value = &type_proto.field[1];

        assert_eq!("key", key.get_name());
        assert_eq!("value", value.get_name());

        assert_eq!(1, key.get_number());
        assert_eq!(2, value.get_number());

        assert_eq!(
            field_descriptor_proto::Label::LABEL_OPTIONAL,
            key.get_label()
        );
        assert_eq!(
            field_descriptor_proto::Label::LABEL_OPTIONAL,
            value.get_label()
        );

        // It is OK to resolve using current descriptor because map field
        // should always point to the same file.
        let key = self.resolve_field_element_type(key);
        let value = self.resolve_field_element_type(value);
        ForwardRuntimeFieldType::Map(key, value)
    }
}
