//! Generated code support for enum descriptors.

use crate::ProtobufEnum;
use std::marker;

pub(crate) trait GetEnumDescriptor: Send + Sync + 'static {
    unsafe fn copy_to(&self, value: i32, dest: *mut ());
}

pub(crate) struct GetDescriptorImpl<E: ProtobufEnum>(marker::PhantomData<E>);

impl<E: ProtobufEnum> GetEnumDescriptor for GetDescriptorImpl<E> {
    unsafe fn copy_to(&self, value: i32, dest: *mut ()) {
        let e = E::from_i32(value).expect("unknown value");
        (&e as *const E).copy_to(dest as *mut E, 1);
    }
}

#[doc(hidden)]
pub struct GeneratedEnumDescriptorData {
    #[cfg(not(rustc_nightly))]
    get_descriptor: &'static dyn GetEnumDescriptor,
}

impl GeneratedEnumDescriptorData {
    #[doc(hidden)]
    pub fn new<E>(name_in_file: &'static str) -> GeneratedEnumDescriptorData
    where
        E: ProtobufEnum,
    {
        GeneratedEnumDescriptorData {
            #[cfg(not(rustc_nightly))]
            get_descriptor: &GetDescriptorImpl(marker::PhantomData::<E>),
        }
    }
}
