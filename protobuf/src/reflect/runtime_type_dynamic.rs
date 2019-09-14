use std::any::Any;
use std::any::TypeId;
use std::marker;

use crate::reflect::runtime_type_box::RuntimeTypeBox;
use crate::reflect::runtime_types::RuntimeType;
use crate::reflect::EnumDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueRef;

/// Dynamic version of `RuntimeType`.
///
/// This is used internally in reflection implementation.
pub trait RuntimeTypeDynamic: Send + Sync + 'static {
    /// Convert to "enum" version
    fn to_box(&self) -> RuntimeTypeBox;

    /// Convert a value reference to `ReflectValueRef` object.
    ///
    /// # Panics
    ///
    /// If value type does not match this type object
    fn value_to_ref<'a>(&self, value: &'a dyn ProtobufValue) -> ReflectValueRef<'a>;

    /// Default value for type
    fn default_value_ref(&self) -> ReflectValueRef;

    /// `EnumDescriptor` for this type.
    ///
    /// # Panics
    ///
    /// If this type is not enum.
    fn enum_descriptor(&self) -> &'static EnumDescriptor;

    /// `MessageDescriptor` for this type.
    ///
    /// # Panics
    ///
    /// If this type is not message.
    fn message_descriptor(&self) -> &'static MessageDescriptor;
}

pub(crate) struct RuntimeTypeDynamicImpl<T: RuntimeType>(pub marker::PhantomData<T>);

impl<T: RuntimeType> RuntimeTypeDynamic for RuntimeTypeDynamicImpl<T> {
    fn to_box(&self) -> RuntimeTypeBox {
        T::runtime_type_box()
    }

    fn value_to_ref<'a>(&self, value: &'a dyn ProtobufValue) -> ReflectValueRef<'a> {
        if Any::type_id(value) == TypeId::of::<T::Value>() {
            unsafe { T::as_ref(&*(value as *const dyn ProtobufValue as *const T::Value)) }
        } else {
            panic!("wrong type")
        }
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
