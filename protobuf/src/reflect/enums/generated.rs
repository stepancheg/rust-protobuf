//! Generated code support for enum descriptors.

use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::find_message_or_enum::find_message_or_enum;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::name::compute_full_name;
use crate::reflect::EnumDescriptor;
use crate::reflect::ProtobufValue;
use crate::ProtobufEnum;
use crate::ProtobufEnumOrUnknown;
use std::any::TypeId;
use std::collections::HashMap;
use std::marker;

pub(crate) trait GetEnumDescriptor: Send + Sync + 'static {
    #[cfg(not(rustc_nightly))]
    unsafe fn copy_to(&self, value: i32, dest: *mut ());
}

pub(crate) struct GetEnumDescriptorImpl<E: ProtobufEnum>(marker::PhantomData<E>);

impl<E: ProtobufEnum> GetEnumDescriptor for GetEnumDescriptorImpl<E> {
    #[cfg(not(rustc_nightly))]
    unsafe fn copy_to(&self, value: i32, dest: *mut ()) {
        let e = E::from_i32(value).expect("unknown value");
        (&e as *const E).copy_to(dest as *mut E, 1);
    }
}

#[doc(hidden)]
pub struct GeneratedEnumDescriptorData {
    get_descriptor: &'static dyn GetEnumDescriptor,
    values: Vec<&'static dyn ProtobufValue>,
    name_in_file: &'static str,
    type_id: TypeId,
    enum_or_unknown_type_id: TypeId,
    index_in_file: usize,
}

impl GeneratedEnumDescriptorData {
    #[doc(hidden)]
    pub fn new_2<E>(name_in_file: &'static str, index_in_file: usize) -> GeneratedEnumDescriptorData
    where
        E: ProtobufEnum,
    {
        let values = E::values()
            .iter()
            .map(|e| e as &dyn ProtobufValue)
            .collect();

        GeneratedEnumDescriptorData {
            index_in_file,
            get_descriptor: &GetEnumDescriptorImpl(marker::PhantomData::<E>),
            values,
            name_in_file,
            type_id: TypeId::of::<E>(),
            enum_or_unknown_type_id: TypeId::of::<ProtobufEnumOrUnknown<E>>(),
        }
    }
}

pub(crate) struct GeneratedEnumDescriptor {
    pub(crate) full_name: String,
    pub(crate) proto: &'static EnumDescriptorProto,
    /// Type id of `<E>`
    pub(crate) type_id: TypeId,
    /// Type id of `<ProtobufEnumOrUnknown<E>>`
    pub(crate) enum_or_unknown_type_id: TypeId,

    pub(crate) index_by_name: HashMap<String, usize>,
    pub(crate) index_by_number: HashMap<i32, usize>,

    #[allow(dead_code)]
    pub(crate) get_descriptor: &'static dyn GetEnumDescriptor,
}

impl GeneratedEnumDescriptor {
    pub fn new(
        data: GeneratedEnumDescriptorData,
        expected_index: usize,
        file_descriptor_proto: &'static FileDescriptorProto,
    ) -> GeneratedEnumDescriptor {
        let GeneratedEnumDescriptorData {
            values,
            name_in_file,
            type_id,
            enum_or_unknown_type_id,
            get_descriptor,
            index_in_file,
        } = data;

        assert!(expected_index == index_in_file);

        let (path_to_package, proto) =
            match find_message_or_enum(file_descriptor_proto, name_in_file) {
                (path_to_package, MessageOrEnum::Enum(e)) => (path_to_package, e),
                (_, MessageOrEnum::Message(_)) => panic!("not an enum"),
            };

        let (index_by_name, index_by_number) = EnumDescriptor::make_indices(proto);

        let proto_values = &proto.value;
        assert_eq!(proto_values.len(), values.len());

        GeneratedEnumDescriptor {
            full_name: compute_full_name(
                file_descriptor_proto.get_package(),
                &path_to_package,
                proto.get_name(),
            ),
            proto,
            type_id,
            enum_or_unknown_type_id,
            index_by_name,
            index_by_number,
            get_descriptor,
        }
    }
}
