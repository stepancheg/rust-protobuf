//! Utilities to support "extension" fields.
//!
//! Extensions are [described in the official protobuf documentation][exts].
//!
//! [exts]: https://developers.google.com/protocol-buffers/docs/proto#extensions

use std::marker::PhantomData;

use crate::core::Message;
use crate::reflect::runtime_types::RuntimeType;
use crate::reflect::types::ProtobufType;

/// Optional ext field
///
/// This is initialized from generated code, do not instantiate directly.
pub struct ExtFieldOptional<M: Message, T: ProtobufType> {
    /// Extension field number
    #[doc(hidden)]
    pub field_number: u32,
    /// Marker
    #[doc(hidden)]
    pub phantom: PhantomData<(M, T)>,
}

/// Repeated ext field
///
/// This is initialized from generated code, do not instantiate directly.
pub struct ExtFieldRepeated<M: Message, T: ProtobufType> {
    /// Extension field number
    #[doc(hidden)]
    pub field_number: u32,
    /// Extension field number
    #[doc(hidden)]
    pub phantom: PhantomData<(M, T)>,
}

impl<M: Message, T: ProtobufType> ExtFieldOptional<M, T> {
    /// Get a copy of value from a message.
    ///
    /// Extension data is stored in [`UnknownFields`](crate::UnknownFields).
    pub fn get(&self, m: &M) -> Option<<T::RuntimeType as RuntimeType>::Value> {
        m.get_unknown_fields()
            .get(self.field_number)
            .and_then(T::get_from_unknown)
    }
}

impl<M: Message, T: ProtobufType> ExtFieldRepeated<M, T> {
    /// Get a copy of value from a message (**not implemented**).
    pub fn get(&self, _m: &M) -> Vec<<T::RuntimeType as RuntimeType>::Value> {
        // TODO
        unimplemented!()
    }
}
