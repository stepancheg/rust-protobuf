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
    pub parent: Option<usize>,
    pub nested_messages: Vec<usize>,
    pub nested_enums: Vec<usize>,
    pub map_entry: bool,
}

#[derive(Debug)]
struct FileIndexEnumEntry {
    message_path: MessagePath,
    enum_index: usize,
}

#[derive(Debug)]
pub(crate) struct FileIndex {
    pub(crate) messages: Vec<FileIndexMessageEntry>,
    pub(crate) message_by_name_to_package: HashMap<String, usize>,
    enums: Vec<FileIndexEnumEntry>,
}

impl FileIndex {
    pub fn index(file: &FileDescriptorProto) -> FileIndex {
        let mut index = FileIndex {
            messages: Vec::new(),
            message_by_name_to_package: HashMap::new(),
            enums: Vec::new(), // TODO
        };

        for (i, message) in file.message_type.iter().enumerate() {
            let path = MessagePath(vec![i]);
            index.index_message_and_inners(file, message, &path, None, "");
        }

        index.build_message_by_name_to_package();

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
            parent,
            nested_messages: Vec::with_capacity(message.nested_type.len()),
            nested_enums: Vec::with_capacity(message.enum_type.len()), // TODO
            map_entry: message.options.get_or_default().get_map_entry(),
        });

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
        let map = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| (m.name_to_package.to_owned(), i))
            .collect();
        self.message_by_name_to_package = map;
    }
}
