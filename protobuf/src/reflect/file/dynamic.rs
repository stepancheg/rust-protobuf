use std::sync::Arc;

use crate::descriptor::FileDescriptorProto;
use crate::reflect::file::common::FileDescriptorCommon;
use crate::reflect::file::index::FileIndex;
use crate::reflect::oneof::dynamic::DynamicOneofDescriptor;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct DynamicFileDescriptor {
    pub(crate) proto: Arc<FileDescriptorProto>,
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

        let oneofs = index
            .oneofs
            .iter()
            .map(|_| DynamicOneofDescriptor {})
            .collect();

        let common = FileDescriptorCommon::new(index, dependencies, &proto)?;

        Ok(DynamicFileDescriptor {
            oneofs,
            proto,
            common,
        })
    }
}
