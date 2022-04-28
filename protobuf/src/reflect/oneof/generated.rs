use crate::descriptor::FileDescriptorProto;
use crate::OneofFull;

#[doc(hidden)]
pub struct GeneratedOneofDescriptorData {}

impl GeneratedOneofDescriptorData {
    #[doc(hidden)]
    pub fn new<O>(_name_in_message: &'static str) -> GeneratedOneofDescriptorData
    where
        O: OneofFull,
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
