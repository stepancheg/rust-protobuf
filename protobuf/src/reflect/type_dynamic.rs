//! Reflection internals.

use std::marker;

use crate::reflect::types::ProtobufType;
use crate::reflect::ProtobufValue;
use crate::reflect::RuntimeTypeBox;
use crate::wire_format::WireType;

/// Dynamic version of [`ProtobufType`](crate::reflect::types::ProtobufType).
///
/// This is used internally.
pub trait ProtobufTypeDynamic: Send + Sync + 'static {
    /// Wire type for this type.
    fn wire_type(&self) -> WireType;

    /// Get runtime type for this protobuf type.
    fn runtime_type(&self) -> RuntimeTypeBox;
}

pub(crate) struct ProtobufTypeDynamicImpl<T: ProtobufType>(pub marker::PhantomData<T>);

impl<T: ProtobufType> ProtobufTypeDynamic for ProtobufTypeDynamicImpl<T> {
    fn wire_type(&self) -> WireType {
        T::WIRE_TYPE
    }

    fn runtime_type(&self) -> RuntimeTypeBox {
        <T::ProtobufValue as ProtobufValue>::runtime_type_box()
    }
}
