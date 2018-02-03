use std::marker::PhantomData;

use core::Message;
use types::ProtobufType;

/// Optional ext field
pub struct ExtFieldOptional<M : Message, T : ProtobufType> {
    pub field_number: u32,
    pub phantom: PhantomData<(M, T)>,
}

/// Repeated ext field
pub struct ExtFieldRepeated<M : Message, T : ProtobufType> {
    pub field_number: u32,
    pub phantom: PhantomData<(M, T)>,
}

impl<M : Message, T : ProtobufType> ExtFieldOptional<M, T> {
    pub fn get(&self, m: &M) -> Option<T::Value> {
        m.get_unknown_fields()
            .get(self.field_number)
            .and_then(T::get_from_unknown)
    }
}

impl<M : Message, T : ProtobufType> ExtFieldRepeated<M, T> {
    pub fn get(&self, _m: &M) -> Vec<T::Value> {
        unimplemented!()
    }
}
