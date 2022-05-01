use std::collections::HashMap;
use std::ops::Range;

use crate::descriptor::DescriptorProto;
use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::owning_ref::OwningRef;
use crate::reflect::error::ReflectError;
use crate::reflect::field::index::FieldIndex;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::file::fds::fds_extend_with_public;
use crate::reflect::message::is_initialized_is_always_true::compute_is_initialized_is_always_true;
use crate::reflect::name::concat_paths;
use crate::reflect::service::index::ServiceIndex;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct MessageIndex {
    pub(crate) proto: OwningRef<FileDescriptorProto, DescriptorProto>,
    pub(crate) name_to_package: String,
    pub(crate) full_name: String,
    pub(crate) enclosing_message: Option<usize>,
    pub(crate) nested_messages: Vec<usize>,
    pub(crate) nested_enums: Range<usize>,
    pub(crate) oneofs: Range<usize>,
    pub(crate) message_index: MessageFieldsIndex,
    pub(crate) is_initialized_is_always_true: bool,
}

#[derive(Debug, Default)]
pub(crate) struct MessageFieldsIndex {
    /// Index of the first field in global field index.
    pub(crate) first_field_index: usize,
    pub(crate) field_count: usize,
    /// Extensions follow fields in global field index.
    pub(crate) extension_count: usize,
    // Following fields map to the local field index.
    pub(crate) field_index_by_name: HashMap<String, usize>,
    pub(crate) field_index_by_name_or_json_name: HashMap<String, usize>,
    pub(crate) field_index_by_number: HashMap<u32, usize>,
}

impl MessageFieldsIndex {
    pub(crate) fn regular_field_range(&self) -> Range<usize> {
        self.first_field_index..(self.first_field_index + self.field_count)
    }

    pub(crate) fn extension_field_range(&self) -> Range<usize> {
        self.first_field_index + self.field_count
            ..self.first_field_index + self.field_count + self.extension_count
    }

    pub(crate) fn slice_fields<'a>(&self, file_fields: &'a [FieldIndex]) -> &'a [FieldIndex] {
        &file_fields[self.first_field_index..self.first_field_index + self.field_count]
    }
}

#[derive(Debug)]
pub(crate) struct EnumIndex {
    pub(crate) proto: OwningRef<FileDescriptorProto, EnumDescriptorProto>,
    pub(crate) name_to_package: String,
    pub(crate) full_name: String,
    pub(crate) enclosing_message: Option<usize>,
    pub(crate) index_by_name: HashMap<String, usize>,
    pub(crate) index_by_number: HashMap<i32, usize>,
}

impl EnumIndex {
    pub(crate) fn new(
        name_to_package: String,
        enclosing_message: Option<usize>,
        proto: OwningRef<FileDescriptorProto, EnumDescriptorProto>,
        file: &FileDescriptorProto,
    ) -> EnumIndex {
        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, v) in proto.value.iter().enumerate() {
            index_by_number.insert(v.number(), i);
            index_by_name.insert(v.name().to_owned(), i);
        }
        let full_name = concat_paths(file.package(), &name_to_package);
        EnumIndex {
            proto,
            full_name,
            name_to_package,
            enclosing_message,
            index_by_name,
            index_by_number,
        }
    }
}

#[derive(Debug)]
pub(crate) struct OneofIndex {
    pub(crate) containing_message: usize,
    pub(crate) index_in_containing_message: usize,
    /// Synthetic oneof for proto3 optional field.
    pub(crate) synthetic: bool,
    pub(crate) fields: Vec<usize>,
}

#[derive(Debug)]
pub(crate) struct FileDescriptorCommon {
    /// Direct dependencies of this file.
    pub(crate) dependencies: Vec<FileDescriptor>,
    /// All messages in this file.
    pub(crate) messages: Vec<MessageIndex>,
    pub(crate) message_by_name_to_package: HashMap<String, usize>,
    pub(crate) top_level_messages: Vec<usize>,
    pub(crate) enums: Vec<EnumIndex>,
    pub(crate) enums_by_name_to_package: HashMap<String, usize>,
    pub(crate) oneofs: Vec<OneofIndex>,
    pub(crate) services: Vec<ServiceIndex>,
    pub(crate) first_extension_field_index: usize,
    /// All fields followed by file-level extensions.
    pub(crate) fields: Vec<FieldIndex>,
}

impl FileDescriptorCommon {
    pub(crate) fn extension_field_range(&self) -> Range<usize> {
        self.first_extension_field_index..self.fields.len()
    }

    pub(crate) fn new(
        file: OwningRef<FileDescriptorProto, FileDescriptorProto>,
        dependencies: Vec<FileDescriptor>,
    ) -> crate::Result<FileDescriptorCommon> {
        let deps_with_public = fds_extend_with_public(dependencies.clone());

        let mut messages = Vec::new();
        let mut enums = Vec::new();
        let mut oneofs = Vec::new();
        let mut top_level_messages = Vec::new();

        // Top-level enums start with zero
        for e in file.flat_map_slice(|f| &f.enum_type) {
            enums.push(EnumIndex::new(e.name().to_owned(), None, e, file.owner()));
        }

        for message in file.flat_map_slice(|f| &f.message_type) {
            let message_index = Self::index_message_and_inners(
                file.owner(),
                message,
                None,
                "",
                &mut messages,
                &mut enums,
                &mut oneofs,
            );
            top_level_messages.push(message_index);
        }

        let message_by_name_to_package = Self::build_message_by_name_to_package(&messages);
        let enums_by_name_to_package = Self::build_enum_by_name_to_package(&enums);

        let mut services = Vec::new();

        for service in &file.service {
            let service_index = ServiceIndex::index(
                service,
                &FileDescriptorBuilding {
                    current_file_descriptor: file.owner(),
                    deps_with_public: &deps_with_public,
                    message_by_name_to_package: &message_by_name_to_package,
                    messages: &messages,
                    enums_by_name_to_package: &enums_by_name_to_package,
                },
            )?;
            services.push(service_index);
        }

        let mut fields = Vec::new();

        Self::build_message_index(
            file.owner(),
            &deps_with_public,
            &mut messages,
            &mut fields,
            &message_by_name_to_package,
            &enums_by_name_to_package,
        )?;

        let first_extension_field_index = fields.len();
        for ext in file.flat_map_slice(|f| &f.extension) {
            fields.push(FieldIndex::index(
                None,
                ext,
                &FileDescriptorBuilding {
                    current_file_descriptor: file.owner(),
                    deps_with_public: &deps_with_public,
                    message_by_name_to_package: &message_by_name_to_package,
                    messages: &messages,
                    enums_by_name_to_package: &enums_by_name_to_package,
                },
            )?);
        }

        compute_is_initialized_is_always_true(&mut messages, &fields, file.owner());

        Ok(FileDescriptorCommon {
            dependencies,
            messages,
            message_by_name_to_package,
            enums,
            top_level_messages,
            enums_by_name_to_package,
            oneofs,
            services,
            first_extension_field_index,
            fields,
        })
    }

    fn index_message_and_inners(
        file: &FileDescriptorProto,
        message: OwningRef<FileDescriptorProto, DescriptorProto>,
        parent: Option<usize>,
        parent_name_to_package: &str,
        messages: &mut Vec<MessageIndex>,
        enums: &mut Vec<EnumIndex>,
        oneofs: &mut Vec<OneofIndex>,
    ) -> usize {
        let name_to_package = concat_paths(parent_name_to_package, message.name());

        let message_index = messages.len();
        messages.push(MessageIndex {
            proto: message.clone(),
            full_name: concat_paths(file.package(), &name_to_package),
            name_to_package: name_to_package.clone(),
            enclosing_message: parent,
            nested_messages: Vec::with_capacity(message.nested_type.len()),
            nested_enums: enums.len()..enums.len() + message.enum_type.len(),
            oneofs: oneofs.len()..oneofs.len() + message.oneof_decl.len(),
            message_index: MessageFieldsIndex::default(),
            // Initialized later.
            is_initialized_is_always_true: false,
        });

        for e in message.flat_map_slice(|m| &m.enum_type) {
            enums.push(EnumIndex::new(
                concat_paths(&name_to_package, e.name()),
                Some(message_index),
                e,
                file,
            ));
        }

        for (i, _oneof) in message.oneof_decl.iter().enumerate() {
            let fields: Vec<_> = message
                .field
                .iter()
                .enumerate()
                .filter(|(_, f)| f.has_oneof_index() && f.oneof_index() == i as i32)
                .collect();
            let synthetic = fields.len() == 1 && fields[0].1.proto3_optional();
            oneofs.push(OneofIndex {
                containing_message: message_index,
                index_in_containing_message: i,
                synthetic,
                fields: fields.iter().map(|(i, _)| *i).collect(),
            });
        }

        for nested in message.flat_map_slice(|m| &m.nested_type) {
            let nested_index = Self::index_message_and_inners(
                file,
                nested,
                Some(message_index),
                &name_to_package,
                messages,
                enums,
                oneofs,
            );
            messages[message_index].nested_messages.push(nested_index);
        }

        message_index
    }

    fn build_message_by_name_to_package(messages: &[MessageIndex]) -> HashMap<String, usize> {
        messages
            .iter()
            .enumerate()
            .map(|(i, m)| (m.name_to_package.to_owned(), i))
            .collect()
    }

    fn build_enum_by_name_to_package(enums: &[EnumIndex]) -> HashMap<String, usize> {
        enums
            .iter()
            .enumerate()
            .map(|(i, e)| (e.name_to_package.to_owned(), i))
            .collect()
    }

    fn build_message_index(
        file: &FileDescriptorProto,
        deps_with_public: &[FileDescriptor],
        messages: &mut [MessageIndex],
        fields: &mut Vec<FieldIndex>,
        message_by_name_to_package: &HashMap<String, usize>,
        enums_by_name_to_package: &HashMap<String, usize>,
    ) -> crate::Result<()> {
        for i in 0..messages.len() {
            let message_proto = &messages[i].proto;
            let building = FileDescriptorBuilding {
                current_file_descriptor: file,
                deps_with_public,
                message_by_name_to_package,
                messages,
                enums_by_name_to_package,
            };
            let message_index = Self::index_message(i, message_proto, &building, fields)?;
            messages[i].message_index = message_index;
        }
        Ok(())
    }

    fn index_message(
        message_index: usize,
        proto: &OwningRef<FileDescriptorProto, DescriptorProto>,
        building: &FileDescriptorBuilding,
        fields: &mut Vec<FieldIndex>,
    ) -> crate::Result<MessageFieldsIndex> {
        let mut index_by_name = HashMap::new();
        let mut index_by_name_or_json_name = HashMap::new();
        let mut index_by_number = HashMap::new();

        let first_field_index = fields.len();

        for field in proto.flat_map_slice(|m| &m.field) {
            fields.push(FieldIndex::index(Some(message_index), field, building)?);
        }

        let field_count = proto.field.len();

        for (i, f) in proto.field.iter().enumerate() {
            let field_index = &fields[first_field_index + i];

            if index_by_number.insert(f.number() as u32, i).is_some() {
                return Err(ReflectError::NonUniqueFieldName(f.name().to_owned()).into());
            }
            if index_by_name.insert(f.name().to_owned(), i).is_some() {
                return Err(ReflectError::NonUniqueFieldName(f.name().to_owned()).into());
            }
            if index_by_name_or_json_name
                .insert(f.name().to_owned(), i)
                .is_some()
            {
                return Err(ReflectError::NonUniqueFieldName(f.name().to_owned()).into());
            }

            if field_index.json_name != f.name() {
                if index_by_name_or_json_name
                    .insert(field_index.json_name.clone(), i)
                    .is_some()
                {
                    return Err(ReflectError::NonUniqueFieldName(f.name().to_owned()).into());
                }
            }
        }

        for ext in proto.flat_map_slice(|m| &m.extension) {
            fields.push(FieldIndex::index(Some(message_index), ext, building)?);
        }

        let extension_count = proto.extension.len();

        Ok(MessageFieldsIndex {
            first_field_index,
            field_count,
            extension_count,
            field_index_by_name: index_by_name,
            field_index_by_name_or_json_name: index_by_name_or_json_name,
            field_index_by_number: index_by_number,
        })
    }
}
