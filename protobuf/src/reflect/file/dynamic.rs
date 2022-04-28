use std::sync::Arc;

use crate::descriptor::FileDescriptorProto;
use crate::reflect::file::common::FileDescriptorCommon;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct DynamicFileDescriptor {
    pub(crate) proto: Arc<FileDescriptorProto>,
    pub(crate) common: FileDescriptorCommon,
}

impl DynamicFileDescriptor {
    pub fn new(
        proto: FileDescriptorProto,
        dependencies: Vec<FileDescriptor>,
    ) -> crate::Result<DynamicFileDescriptor> {
        let proto = Arc::new(proto);

        let common = FileDescriptorCommon::new(dependencies, &proto)?;

        Ok(DynamicFileDescriptor { proto, common })
    }
}
