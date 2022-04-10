use std::collections::HashMap;

use protobuf::reflect::EnumDescriptor;

use crate::gen::scope::FileScope;

pub(crate) struct FileIndex {
    pub(crate) enum_to_index: HashMap<EnumDescriptor, u32>,
}

impl FileIndex {
    pub(crate) fn index(file_scope: &FileScope) -> FileIndex {
        FileIndex {
            enum_to_index: file_scope
                .find_enums()
                .into_iter()
                .enumerate()
                .map(|(i, n)| (n.en, i as u32))
                .collect(),
        }
    }
}
