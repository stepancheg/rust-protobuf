use std::marker;

use reflect::ProtobufValue;
use reflect::ReflectValueRef;
use reflect::runtime_types::RuntimeType;
use reflect::runtime_type_box::RuntimeTypeBox;
use reflect::MessageDescriptor;
use reflect::EnumDescriptor;

/// Dynamic version of `RuntimeType`
pub trait RuntimeTypeDynamic : Send + Sync + 'static {
    fn to_box(&self) -> RuntimeTypeBox;

    fn value_to_ref<'a>(&self, value: &'a ProtobufValue) -> ReflectValueRef<'a>;

    fn default_value_ref(&self) -> ReflectValueRef;

    fn enum_descriptor(&self) -> &'static EnumDescriptor;

    fn message_descriptor(&self) -> &'static MessageDescriptor;
}

pub(crate) struct RuntimeTypeDynamicImpl<T : RuntimeType>(pub marker::PhantomData<T>);

impl<T : RuntimeType> RuntimeTypeDynamic for RuntimeTypeDynamicImpl<T> {
    fn to_box(&self) -> RuntimeTypeBox {
        T::runtime_type_box()
    }

    fn value_to_ref<'a>(&self, value: &'a ProtobufValue) -> ReflectValueRef<'a> {
        let value: &T::Value = value.as_any().downcast_ref().expect("wrong type");
        T::as_ref(value)
    }

    fn default_value_ref(&self) -> ReflectValueRef {
        T::default_value_ref()
    }

    fn enum_descriptor(&self) -> &'static EnumDescriptor {
        T::enum_descriptor()
    }
    fn message_descriptor(&self) -> &'static MessageDescriptor {
        T::message_descriptor()
    }
}
