use std::collections::HashMap;

use protobuf_parse::ProtobufRelPath;

use crate::gen::scope::FileScope;
use crate::gen::scope::WithScope;

pub(crate) struct FileIndex {
    pub(crate) messsage_to_index: HashMap<ProtobufRelPath, u32>,
    pub(crate) enum_to_index: HashMap<ProtobufRelPath, u32>,
}

impl FileIndex {
    pub(crate) fn index(file_scope: &FileScope) -> FileIndex {
        FileIndex {
            messsage_to_index: file_scope
                .find_messages()
                .into_iter()
                .map(|m| m.protobuf_name_to_package())
                .enumerate()
                .map(|(i, n)| (n, i as u32))
                .collect(),
            enum_to_index: file_scope
                .find_enums()
                .into_iter()
                .map(|m| m.protobuf_name_to_package())
                .enumerate()
                .map(|(i, n)| (n, i as u32))
                .collect(),
        }
    }
}
