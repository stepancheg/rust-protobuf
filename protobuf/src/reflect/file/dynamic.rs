use std::sync::Arc;

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::enums::dynamic::DynamicEnumDescriptor;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::file::common::FileDescriptorCommon;
use crate::reflect::file::fds::fds_extend_with_public;
use crate::reflect::file::index::FileIndex;
use crate::reflect::message::dynamic::DynamicMessageDescriptor;
use crate::reflect::message::path::MessagePath;
use crate::reflect::oneof::dynamic::DynamicOneofDescriptor;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct DynamicFileDescriptor {
    pub(crate) proto: Arc<FileDescriptorProto>,
    pub(crate) messages: Vec<DynamicMessageDescriptor>,
    pub(crate) enums: Vec<DynamicEnumDescriptor>,
    pub(crate) oneofs: Vec<DynamicOneofDescriptor>,
    pub(crate) common: FileDescriptorCommon,
}

impl DynamicFileDescriptor {
    pub fn new(
        proto: FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
    ) -> crate::Result<DynamicFileDescriptor> {
        let proto = Arc::new(proto);

        let index = FileIndex::index(&*proto, &dependencies)?;

        let file_descriptor_building = FileDescriptorBuilding {
            current_file_index: &index,
            current_file_descriptor: &proto,
            deps_with_public: &fds_extend_with_public(dependencies.clone()),
        };

        let messages = index
            .messages
            .iter()
            .map(|message_index_entry| {
                DynamicMessageDescriptor::new(
                    &*proto,
                    &message_index_entry.path,
                    &file_descriptor_building,
                )
            })
            .collect::<crate::Result<Vec<_>>>()?;

        let oneofs = index
            .oneofs
            .iter()
            .map(|_| DynamicOneofDescriptor {})
            .collect();

        let common = FileDescriptorCommon::new(index, dependencies, &proto)?;

        Ok(DynamicFileDescriptor {
            messages,
            enums: Self::enums(&proto),
            oneofs,
            proto,
            common,
        })
    }

    fn enums(proto: &Arc<FileDescriptorProto>) -> Vec<DynamicEnumDescriptor> {
        let mut r = Vec::new();
        let mut path = MessagePath(Vec::new());
        for (i, _e) in proto.enum_type.iter().enumerate() {
            r.push(DynamicEnumDescriptor::new(proto.clone(), &path, i));
        }
        for (i, m) in proto.message_type.iter().enumerate() {
            path.0.push(i);
            Self::enums_from(proto, m, &mut r, &mut path);
            path.0.pop().unwrap();
        }
        assert!(path.is_empty());
        r
    }

    fn enums_from(
        proto: &Arc<FileDescriptorProto>,
        scope: &DescriptorProto,
        r: &mut Vec<DynamicEnumDescriptor>,
        path: &mut MessagePath,
    ) {
        for (i, _e) in scope.enum_type.iter().enumerate() {
            r.push(DynamicEnumDescriptor::new(proto.clone(), &path, i));
        }
        for (i, m) in scope.nested_type.iter().enumerate() {
            path.push(i);
            Self::enums_from(proto, m, r, path);
            path.pop().unwrap();
        }
    }
}
