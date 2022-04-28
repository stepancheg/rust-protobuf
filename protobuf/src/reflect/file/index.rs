use std::collections::HashMap;

use crate::descriptor::DescriptorProto;
use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::enums::path::EnumPath;
use crate::reflect::field::index::FieldIndex;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::file::fds::fds_extend_with_public;
use crate::reflect::message::path::MessagePath;
use crate::reflect::name::concat_paths;
use crate::reflect::service::index::ServiceIndex;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct MessageIndex {
    pub(crate) path: MessagePath,
    pub(crate) name_to_package: String,
    pub(crate) full_name: String,
    pub(crate) enclosing_message: Option<usize>,
    pub(crate) nested_messages: Vec<usize>,
    pub(crate) map_entry: bool,
    pub(crate) first_enum_index: usize,
    pub(crate) enum_count: usize,
    pub(crate) first_oneof_index: usize,
    pub(crate) oneof_count: usize,
    pub(crate) message_index: MessageFieldsIndex,
}

#[derive(Debug, Default)]
pub(crate) struct MessageFieldsIndex {
    pub(crate) fields: Vec<FieldIndex>,
    pub(crate) field_index_by_name: HashMap<String, usize>,
    pub(crate) field_index_by_name_or_json_name: HashMap<String, usize>,
    pub(crate) field_index_by_number: HashMap<u32, usize>,
    pub(crate) extensions: Vec<FieldIndex>,
}

#[derive(Debug)]
pub(crate) struct EnumIndex {
    pub(crate) enum_path: EnumPath,
    pub(crate) name_to_package: String,
    pub(crate) full_name: String,
    pub(crate) enclosing_message: Option<usize>,
    pub(crate) index_by_name: HashMap<String, usize>,
    pub(crate) index_by_number: HashMap<i32, usize>,
}

impl EnumIndex {
    pub(crate) fn new(
        enum_path: EnumPath,
        name_to_package: String,
        enclosing_message: Option<usize>,
        proto: &EnumDescriptorProto,
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
            enum_path,
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
}

#[derive(Debug)]
pub(crate) struct FileIndex {
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
    pub(crate) extensions: Vec<FieldIndex>,
}

impl FileIndex {
    pub(crate) fn index(
        file: &FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
    ) -> crate::Result<FileIndex> {
        let deps_with_public = fds_extend_with_public(dependencies.clone());

        let mut index = FileIndex {
            dependencies,
            messages: Vec::new(),
            message_by_name_to_package: HashMap::new(),
            enums: Vec::new(),
            top_level_messages: Vec::with_capacity(file.message_type.len()),
            enums_by_name_to_package: HashMap::new(),
            oneofs: Vec::new(),
            services: Vec::new(),
            extensions: Vec::new(),
        };

        // Top-level enums start with zero
        for (i, e) in file.enum_type.iter().enumerate() {
            index.enums.push(EnumIndex::new(
                EnumPath {
                    message_path: MessagePath::default(),
                    enum_index: i,
                },
                e.name().to_owned(),
                None,
                e,
                file,
            ));
        }

        for (i, message) in file.message_type.iter().enumerate() {
            let path = MessagePath(vec![i]);
            let message_index = index.index_message_and_inners(file, message, &path, None, "");
            index.top_level_messages.push(message_index);
        }

        index.build_message_by_name_to_package();
        index.build_enum_by_name_to_package();

        for service in &file.service {
            let service_index = ServiceIndex::index(
                service,
                &FileDescriptorBuilding {
                    current_file_descriptor: file,
                    current_file_index: &index,
                    deps_with_public: &deps_with_public,
                },
            )?;
            index.services.push(service_index);
        }

        index.build_message_index(file, &deps_with_public)?;

        index.extensions = file
            .extension
            .iter()
            .map(|ext| {
                FieldIndex::index(
                    ext,
                    &FileDescriptorBuilding {
                        current_file_descriptor: file,
                        current_file_index: &index,
                        deps_with_public: &deps_with_public,
                    },
                )
            })
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(index)
    }

    fn index_message_and_inners(
        &mut self,
        file: &FileDescriptorProto,
        message: &DescriptorProto,
        path: &MessagePath,
        parent: Option<usize>,
        parent_name_to_package: &str,
    ) -> usize {
        let name_to_package = concat_paths(parent_name_to_package, message.name());

        let message_index = self.messages.len();
        self.messages.push(MessageIndex {
            path: path.clone(),
            name_to_package: String::new(),
            full_name: String::new(),
            enclosing_message: parent,
            nested_messages: Vec::with_capacity(message.nested_type.len()),
            map_entry: message.options.get_or_default().map_entry(),
            first_enum_index: self.enums.len(),
            enum_count: message.enum_type.len(),
            first_oneof_index: self.oneofs.len(),
            oneof_count: message.oneof_decl.len(),
            message_index: MessageFieldsIndex::default(),
        });

        for (i, e) in message.enum_type.iter().enumerate() {
            self.enums.push(EnumIndex::new(
                EnumPath {
                    message_path: path.clone(),
                    enum_index: i,
                },
                concat_paths(&name_to_package, e.name()),
                Some(message_index),
                e,
                file,
            ));
        }

        for (i, _oneof) in message.oneof_decl.iter().enumerate() {
            self.oneofs.push(OneofIndex {
                containing_message: message_index,
                index_in_containing_message: i,
            });
        }

        for (i, nested) in message.nested_type.iter().enumerate() {
            let mut nested_path = path.clone();
            nested_path.push(i);
            let nested_index = self.index_message_and_inners(
                file,
                nested,
                &nested_path,
                Some(message_index),
                &name_to_package,
            );
            self.messages[message_index]
                .nested_messages
                .push(nested_index);
        }

        self.messages[message_index].full_name = concat_paths(file.package(), &name_to_package);
        self.messages[message_index].name_to_package = name_to_package;

        message_index
    }

    fn build_message_by_name_to_package(&mut self) {
        self.message_by_name_to_package = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| (m.name_to_package.to_owned(), i))
            .collect();
    }

    fn build_enum_by_name_to_package(&mut self) {
        self.enums_by_name_to_package = self
            .enums
            .iter()
            .enumerate()
            .map(|(i, e)| (e.name_to_package.to_owned(), i))
            .collect();
    }

    fn build_message_index(
        &mut self,
        file: &FileDescriptorProto,
        deps_with_public: &[FileDescriptor],
    ) -> crate::Result<()> {
        for i in 0..self.messages.len() {
            let message_proto = self.messages[i].path.eval(file).unwrap();
            let building = FileDescriptorBuilding {
                current_file_descriptor: file,
                current_file_index: self,
                deps_with_public,
            };
            let message_index = Self::index_message(message_proto, &building)?;
            self.messages[i].message_index = message_index;
        }
        Ok(())
    }

    fn index_message(
        proto: &DescriptorProto,
        building: &FileDescriptorBuilding,
    ) -> crate::Result<MessageFieldsIndex> {
        let mut index_by_name = HashMap::new();
        let mut index_by_name_or_json_name = HashMap::new();
        let mut index_by_number = HashMap::new();

        let fields: Vec<FieldIndex> = proto
            .field
            .iter()
            .map(|f| FieldIndex::index(f, building))
            .collect::<crate::Result<_>>()?;
        for (i, f) in proto.field.iter().enumerate() {
            let field_index = &fields[i];

            assert!(index_by_number.insert(f.number() as u32, i).is_none());
            assert!(index_by_name.insert(f.name().to_owned(), i).is_none());
            assert!(index_by_name_or_json_name
                .insert(f.name().to_owned(), i)
                .is_none());

            if field_index.json_name != f.name() {
                assert!(index_by_name_or_json_name
                    .insert(field_index.json_name.clone(), i)
                    .is_none());
            }
        }

        let extensions: Vec<FieldIndex> = proto
            .extension
            .iter()
            .map(|f| FieldIndex::index(f, building))
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(MessageFieldsIndex {
            fields,
            field_index_by_name: index_by_name,
            field_index_by_name_or_json_name: index_by_name_or_json_name,
            field_index_by_number: index_by_number,
            extensions,
        })
    }
}
