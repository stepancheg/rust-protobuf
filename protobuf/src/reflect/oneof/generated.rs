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
    /// Synthetic oneof for proto3 optional field.
    pub(crate) fn new_synthetic() -> GeneratedOneofDescriptor {
        GeneratedOneofDescriptor {}
    }

    pub(crate) fn new(_data: &GeneratedOneofDescriptorData) -> GeneratedOneofDescriptor {
        GeneratedOneofDescriptor {}
    }
}
