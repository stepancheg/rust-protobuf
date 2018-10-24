use std::marker;

use reflect::runtime_type_dynamic::RuntimeTypeDynamic;
use reflect::runtime_types::RuntimeType;
use reflect::types::ProtobufType;
use wire_format::WireType;

/// Dynamic version of `RuntimeType`
pub trait ProtobufTypeDynamic: Send + Sync + 'static {
    fn wire_type(&self) -> WireType;

    fn runtime_type(&self) -> &RuntimeTypeDynamic;
}

pub(crate) struct ProtobufTypeDynamicImpl<T: ProtobufType>(pub marker::PhantomData<T>);

impl<T: ProtobufType> ProtobufTypeDynamic for ProtobufTypeDynamicImpl<T> {
    fn wire_type(&self) -> WireType {
        T::wire_type()
    }

    fn runtime_type(&self) -> &RuntimeTypeDynamic {
        <T::RuntimeType as RuntimeType>::dynamic()
    }
}
