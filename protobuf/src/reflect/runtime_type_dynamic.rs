use std::marker;

use reflect::ProtobufValue;
use reflect::ReflectValueRef;
use reflect::runtime_types::RuntimeType;
use reflect::runtime_type_box::RuntimeTypeBox;

/// Dynamic version of `RuntimeType`
pub trait RuntimeTypeDynamic : Send + Sync + 'static {
    fn runtime_type_box(&self) -> RuntimeTypeBox;

    fn value_to_ref<'a>(&self, value: &'a ProtobufValue) -> ReflectValueRef<'a>;
}

pub(crate) struct RuntimeTypeDynamicImpl<T : RuntimeType>(pub marker::PhantomData<T>);

impl<T : RuntimeType> RuntimeTypeDynamic for RuntimeTypeDynamicImpl<T> {
    fn value_to_ref<'a>(&self, value: &'a ProtobufValue) -> ReflectValueRef<'a> {
        let value: &T::Value = value.as_any().downcast_ref().expect("wrong type");
        T::as_ref(value)
    }

    fn runtime_type_box(&self) -> RuntimeTypeBox {
        T::runtime_type_box()
    }
}
