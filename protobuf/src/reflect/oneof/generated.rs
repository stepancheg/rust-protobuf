use crate::descriptor::FileDescriptorProto;
use crate::OneofFull;

#[doc(hidden)]
pub struct GeneratedOneofDescriptorData {}

impl GeneratedOneofDescriptorData {
    #[doc(hidden)]
    pub fn new_2<E>(
        _name_in_file: &'static str,
        _index_in_file: usize,
    ) -> GeneratedOneofDescriptorData
    where
        E: OneofFull,
    {
        GeneratedOneofDescriptorData {}
    }
}

#[derive(Debug)]
pub(crate) struct GeneratedOneofDescriptor {}

impl GeneratedOneofDescriptor {
    pub(crate) fn _new(
        data: GeneratedOneofDescriptorData,
        expected_index: usize,
        file_descriptor_proto: &'static FileDescriptorProto,
    ) -> GeneratedOneofDescriptor {
        let _ = (data, expected_index, file_descriptor_proto);
        GeneratedOneofDescriptor {}
    }
}
