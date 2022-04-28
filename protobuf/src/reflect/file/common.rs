use crate::descriptor::FileDescriptorProto;
use crate::reflect::field::index::FieldIndex;
use crate::reflect::file::building::FileDescriptorBuilding;
use crate::reflect::file::fds::fds_extend_with_public;
use crate::reflect::file::index::FileIndex;
use crate::reflect::FileDescriptor;

#[derive(Debug)]
pub(crate) struct FileDescriptorCommon {
    pub(crate) index: FileIndex,
    pub(crate) extensions: Vec<FieldIndex>,
}

impl FileDescriptorCommon {
    pub(crate) fn new(
        dependencies: Vec<FileDescriptor>,
        current_file_descriptor: &FileDescriptorProto,
    ) -> crate::Result<FileDescriptorCommon> {
        let index = FileIndex::index(current_file_descriptor, dependencies)?;

        let deps_with_public = fds_extend_with_public(index.dependencies.clone());
        let building = FileDescriptorBuilding {
            current_file_descriptor,
            current_file_index: &index,
            deps_with_public: &deps_with_public,
        };

        let extensions = current_file_descriptor
            .extension
            .iter()
            .map(|ext| FieldIndex::index(ext, &building))
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(FileDescriptorCommon { index, extensions })
    }
}
