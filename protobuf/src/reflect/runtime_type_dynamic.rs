use std::marker;

use reflect::ProtobufValue;
use reflect::ReflectValueRef;
use reflect::runtime_types::RuntimeType;

/// Dynamic version of `RuntimeType`
pub trait RuntimeTypeDynamic : Send + Sync + 'static {
    fn value_to_ref<'a>(&self, value: &'a ProtobufValue) -> ReflectValueRef<'a>;
}

pub struct RuntimeTypeDynamicImpl<T : RuntimeType>(pub marker::PhantomData<T>);

impl<T : RuntimeType> RuntimeTypeDynamic for RuntimeTypeDynamicImpl<T> {
    fn value_to_ref<'a>(&self, value: &'a ProtobufValue) -> ReflectValueRef<'a> {
        let value: &T::Value = value.as_any().downcast_ref().expect("wrong type");
        T::as_ref(value)
    }
}
