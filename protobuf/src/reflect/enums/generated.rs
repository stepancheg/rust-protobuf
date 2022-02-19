//! Generated code support for enum descriptors.

use std::any::TypeId;
use std::fmt;
use std::marker;

use crate::descriptor::EnumDescriptorProto;
use crate::descriptor::FileDescriptorProto;
use crate::reflect::enums::index::EnumIndex;
use crate::reflect::find_message_or_enum::find_message_or_enum;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::name::compute_full_name;
use crate::Enum;
use crate::EnumOrUnknown;

pub(crate) trait GetEnumDescriptor: Send + Sync + 'static {
    #[cfg(not(rustc_nightly))]
    unsafe fn copy_to(&self, value: i32, dest: *mut ());
}

impl<'a> fmt::Debug for &'a dyn GetEnumDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GetEnumDescriptor").finish()
    }
}

pub(crate) struct GetEnumDescriptorImpl<E: Enum>(marker::PhantomData<E>);

impl<E: Enum> GetEnumDescriptor for GetEnumDescriptorImpl<E> {
    #[cfg(not(rustc_nightly))]
    unsafe fn copy_to(&self, value: i32, dest: *mut ()) {
        let e = E::from_i32(value).expect("unknown value");
        (&e as *const E).copy_to(dest as *mut E, 1);
    }
}

#[doc(hidden)]
pub struct GeneratedEnumDescriptorData {
    get_descriptor: &'static dyn GetEnumDescriptor,
    name_in_file: &'static str,
    type_id: TypeId,
    enum_or_unknown_type_id: TypeId,
    index_in_file: usize,
}

impl GeneratedEnumDescriptorData {
    #[doc(hidden)]
    pub fn new_2<E>(name_in_file: &'static str, index_in_file: usize) -> GeneratedEnumDescriptorData
    where
        E: Enum,
    {
        GeneratedEnumDescriptorData {
            index_in_file,
            get_descriptor: &GetEnumDescriptorImpl(marker::PhantomData::<E>),
            name_in_file,
            type_id: TypeId::of::<E>(),
            enum_or_unknown_type_id: TypeId::of::<EnumOrUnknown<E>>(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct GeneratedEnumDescriptor {
    pub(crate) full_name: String,
    pub(crate) proto: &'static EnumDescriptorProto,
    /// Type id of `<E>`
    pub(crate) type_id: TypeId,
    /// Type id of `<ProtobufEnumOrUnknown<E>>`
    pub(crate) _enum_or_unknown_type_id: TypeId,

    pub indices: EnumIndex<&'static str>,

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
            name_in_file,
            type_id,
            enum_or_unknown_type_id,
            get_descriptor,
            index_in_file,
        } = data;

        assert!(expected_index == index_in_file);

        let (path_to_package, proto) =
            match find_message_or_enum(file_descriptor_proto, name_in_file).unwrap() {
                (path_to_package, MessageOrEnum::Enum(e)) => (path_to_package, e),
                (_, MessageOrEnum::Message(_)) => panic!("not an enum"),
            };

        let indices = EnumIndex::<&'static str>::index::<&'static str>(proto);

        GeneratedEnumDescriptor {
            full_name: compute_full_name(
                file_descriptor_proto.package(),
                &path_to_package,
                proto.name(),
            ),
            proto,
            type_id,
            _enum_or_unknown_type_id: enum_or_unknown_type_id,
            indices,
            get_descriptor,
        }
    }
}
