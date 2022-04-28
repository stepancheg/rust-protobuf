use crate::descriptor::FileDescriptorProto;
use crate::reflect::file::index::FileIndex;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct FileDescriptorCommon {
    pub(crate) index: FileIndex,
}

impl FileDescriptorCommon {
    pub(crate) fn new(
        dependencies: Vec<FileDescriptor>,
        current_file_descriptor: &FileDescriptorProto,
    ) -> crate::Result<FileDescriptorCommon> {
        let index = FileIndex::index(current_file_descriptor, dependencies)?;

        Ok(FileDescriptorCommon { index })
    }
}
