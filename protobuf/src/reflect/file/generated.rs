use std::collections::HashMap;

use crate::descriptor::FileDescriptorProto;
use crate::reflect::enums::generated::GeneratedEnumDescriptor;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::file::common::FileDescriptorCommon;
use crate::reflect::file::fds::fds_extend_with_public;
use crate::reflect::file::index::FileIndex;
use crate::reflect::message::generated::GeneratedMessageDescriptor;
use crate::reflect::FileDescriptor;
use crate::reflect::GeneratedEnumDescriptorData;
use crate::reflect::GeneratedMessageDescriptorData;

/// Reflection for objects defined in `.proto` file (messages, enums, etc).
#[doc(hidden)]
#[derive(Debug)]
pub struct GeneratedFileDescriptor {
    pub(crate) proto: &'static FileDescriptorProto,
    pub(crate) messages: Vec<GeneratedMessageDescriptor>,
    pub(crate) enums: Vec<GeneratedEnumDescriptor>,
    pub(crate) common: FileDescriptorCommon,
}

impl GeneratedFileDescriptor {
    /// This function is to be called from generated code.
    pub fn new_generated(
        file_descriptor_proto: &'static FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
        messages: Vec<GeneratedMessageDescriptorData>,
        enums: Vec<GeneratedEnumDescriptorData>,
    ) -> GeneratedFileDescriptor {
        let index = FileIndex::index(file_descriptor_proto, &dependencies);

        let mut messages: HashMap<&str, GeneratedMessageDescriptorData> = messages
            .into_iter()
            .map(|m| (m.protobuf_name_to_package, m))
            .collect();

        let file_descriptor_building = FileDescriptorBuilding {
            current_file_index: &index,
            current_file_descriptor: file_descriptor_proto,
            deps_with_public: &fds_extend_with_public(dependencies.clone()),
        };

        let messages = index
            .messages
            .iter()
            .map(|message_index| {
                if message_index.map_entry {
                    GeneratedMessageDescriptor::new_map_entry()
                } else {
                    let message = messages
                        .remove(message_index.name_to_package.as_str())
                        .unwrap();
                    GeneratedMessageDescriptor::new(
                        message,
                        file_descriptor_proto,
                        &index,
                        &file_descriptor_building,
                    )
                    .unwrap()
                }
            })
            .collect();

        let enums = enums
            .into_iter()
            .enumerate()
            .map(|(i, e)| GeneratedEnumDescriptor::new(e, i, file_descriptor_proto))
            .collect();

        let common = FileDescriptorCommon::new(index, dependencies, file_descriptor_proto).unwrap();

        GeneratedFileDescriptor {
            proto: file_descriptor_proto,
            messages,
            enums,
            common,
        }
    }
}
