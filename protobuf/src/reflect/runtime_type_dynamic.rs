use std::marker;

use crate::reflect::runtime_type_box::RuntimeTypeBox;
use crate::reflect::runtime_types::RuntimeType;

/// Dynamic version of `RuntimeType`.
///
/// This is used internally in reflection implementation.
pub trait RuntimeTypeDynamic: Send + Sync + 'static {
    /// Convert to "enum" version
    fn to_box(&self) -> RuntimeTypeBox;
}

pub(crate) struct RuntimeTypeDynamicImpl<T: RuntimeType>(pub marker::PhantomData<T>);

impl<T: RuntimeType> RuntimeTypeDynamic for RuntimeTypeDynamicImpl<T> {
    fn to_box(&self) -> RuntimeTypeBox {
        T::runtime_type_box()
    }
}
