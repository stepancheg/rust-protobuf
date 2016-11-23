use unknown::UnknownFields;
use std::cell::Cell;

#[derive(Clone, Default)]
pub struct SpecialFields {
    unknown_fields: UnknownFields,
    cached_size: Cell<u32>,
}

impl ::std::cmp::PartialEq for SpecialFields {
    fn eq(&self, other: &SpecialFields) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl SpecialFields {

    pub fn get_unknown_fields(&self) -> &UnknownFields {
        &self.unknown_fields
    }

    pub fn mut_unknown_fields(&mut self) -> &mut UnknownFields {
        &mut self.unknown_fields
    }

    pub fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    pub fn set_cached_size(&self, mut size: u32) -> u32 {
        size += ::protobuf::rt::unknown_fields_size(&self.unknown_fields);
        self.cached_size.set(size);
        size
    }
}
