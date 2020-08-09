use std::marker;

use crate::reflect::runtime_type_box::RuntimeTypeBox;
use crate::reflect::runtime_types::RuntimeType;
use crate::reflect::ReflectValueRef;

/// Dynamic version of `RuntimeType`.
///
/// This is used internally in reflection implementation.
pub trait RuntimeTypeDynamic: Send + Sync + 'static {
    /// Convert to "enum" version
    fn to_box(&self) -> RuntimeTypeBox;

    /// Default value for type
    fn default_value_ref(&self) -> ReflectValueRef;
}

pub(crate) struct RuntimeTypeDynamicImpl<T: RuntimeType>(pub marker::PhantomData<T>);

impl<T: RuntimeType> RuntimeTypeDynamic for RuntimeTypeDynamicImpl<T> {
    fn to_box(&self) -> RuntimeTypeBox {
        T::runtime_type_box()
    }

    fn default_value_ref(&self) -> ReflectValueRef {
        T::default_value_ref()
    }
}
