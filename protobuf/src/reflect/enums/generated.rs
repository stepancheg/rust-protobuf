//! Generated code support for enum descriptors.

use std::any::TypeId;

use crate::descriptor::FileDescriptorProto;
use crate::EnumFull;
use crate::EnumOrUnknown;

#[doc(hidden)]
pub struct GeneratedEnumDescriptorData {
    name_in_file: &'static str,
    type_id: TypeId,
    enum_or_unknown_type_id: TypeId,
    index_in_file: usize,
}

impl GeneratedEnumDescriptorData {
    #[doc(hidden)]
    pub fn new_2<E>(name_in_file: &'static str, index_in_file: usize) -> GeneratedEnumDescriptorData
    where
        E: EnumFull,
    {
        GeneratedEnumDescriptorData {
            index_in_file,
            name_in_file,
            type_id: TypeId::of::<E>(),
            enum_or_unknown_type_id: TypeId::of::<EnumOrUnknown<E>>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct GeneratedEnumDescriptor {
    /// Type id of `<E>`
    pub(crate) type_id: TypeId,
    /// Type id of `<ProtobufEnumOrUnknown<E>>`
    pub(crate) _enum_or_unknown_type_id: TypeId,
}

impl GeneratedEnumDescriptor {
    pub(crate) fn new(
        data: GeneratedEnumDescriptorData,
        expected_index: usize,
        file_descriptor_proto: &'static FileDescriptorProto,
    ) -> GeneratedEnumDescriptor {
        let GeneratedEnumDescriptorData {
            name_in_file,
            type_id,
            enum_or_unknown_type_id,
            index_in_file,
        } = data;

        assert!(expected_index == index_in_file);

        let _ = (name_in_file, file_descriptor_proto);

        GeneratedEnumDescriptor {
            type_id,
            _enum_or_unknown_type_id: enum_or_unknown_type_id,
        }
    }
}
