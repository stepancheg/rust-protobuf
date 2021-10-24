use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::message::path::MessagePath;
use crate::reflect::name::concat_paths;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct FileIndexMessageEntry {
    pub path: MessagePath,
    pub name_to_package: String,
    pub full_name: String,
    pub _parent: Option<usize>,
    pub nested_messages: Vec<usize>,
    pub _nested_enums: Vec<usize>,
    pub map_entry: bool,
    pub first_enum_index: usize,
}

#[derive(Debug)]
pub(crate) struct FileIndexEnumEntry {
    pub _message_path: MessagePath,
    pub _enum_index: usize,
    pub name_to_package: String,
}

#[derive(Debug)]
pub(crate) struct FileIndex {
    pub(crate) messages: Vec<FileIndexMessageEntry>,
    pub(crate) message_by_name_to_package: HashMap<String, usize>,
    pub(crate) top_level_messages: Vec<usize>,
    pub(crate) enums: Vec<FileIndexEnumEntry>,
    pub(crate) enums_by_name_to_package: HashMap<String, usize>,
}

impl FileIndex {
    pub fn index(file: &FileDescriptorProto) -> FileIndex {
        let mut index = FileIndex {
            messages: Vec::new(),
            message_by_name_to_package: HashMap::new(),
            enums: Vec::new(),
            top_level_messages: Vec::with_capacity(file.message_type.len()),
            enums_by_name_to_package: HashMap::new(),
        };

        // Top-level enums start with zero
        for (_, e) in file.enum_type.iter().enumerate() {
            index.enums.push(FileIndexEnumEntry {
                _message_path: MessagePath(Vec::new()),
                _enum_index: index.enums.len(),
                name_to_package: e.get_name().to_owned(),
            });
        }

        for (i, message) in file.message_type.iter().enumerate() {
            let path = MessagePath(vec![i]);
            let message_index = index.index_message_and_inners(file, message, &path, None, "");
            index.top_level_messages.push(message_index);
        }

        index.build_message_by_name_to_package();
        index.build_enum_by_name_to_package();

        index
    }

    fn index_message_and_inners(
        &mut self,
        file: &FileDescriptorProto,
        message: &DescriptorProto,
        path: &MessagePath,
        parent: Option<usize>,
        parent_name_to_package: &str,
    ) -> usize {
        let name_to_package = concat_paths(parent_name_to_package, message.get_name());

        let message_index = self.messages.len();
        self.messages.push(FileIndexMessageEntry {
            path: path.clone(),
            name_to_package: String::new(),
            full_name: String::new(),
            _parent: parent,
            nested_messages: Vec::with_capacity(message.nested_type.len()),
            _nested_enums: Vec::with_capacity(message.enum_type.len()), // TODO
            map_entry: message.options.get_or_default().get_map_entry(),
            first_enum_index: self.enums.len(),
        });

        for (_, e) in message.enum_type.iter().enumerate() {
            self.enums.push(FileIndexEnumEntry {
                _message_path: path.clone(),
                _enum_index: self.enums.len(),
                name_to_package: concat_paths(&name_to_package, e.get_name()),
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

        self.messages[message_index].full_name = concat_paths(file.get_package(), &name_to_package);
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
}
