use collections::HashMap;
use collections::hashmap;
use std::default::Default;
use std::slice;
use core::wire_format;

pub enum UnknownValue {
    UnknownFixed32(u32),
    UnknownFixed64(u64),
    UnknownVarint(u64),
    UnknownLengthDelimited(~[u8]),
}

impl UnknownValue {
    pub fn wire_type(&self) -> wire_format::WireType {
        self.get_ref().wire_type()
    }

    pub fn get_ref<'s>(&'s self) -> UnknownValueRef<'s> {
        match *self {
            UnknownFixed32(fixed32) => UnknownFixed32Ref(fixed32),
            UnknownFixed64(fixed64) => UnknownFixed64Ref(fixed64),
            UnknownVarint(varint) => UnknownVarintRef(varint),
            UnknownLengthDelimited(ref bytes) => UnknownLengthDelimitedRef(bytes.as_slice()),
        }
    }
}

pub enum UnknownValueRef<'o> {
    UnknownFixed32Ref(u32),
    UnknownFixed64Ref(u64),
    UnknownVarintRef(u64),
    UnknownLengthDelimitedRef(&'o [u8]),
}

impl<'o> UnknownValueRef<'o> {
    pub fn wire_type(&self) -> wire_format::WireType {
        match *self {
            UnknownFixed32Ref(_) => wire_format::WireTypeFixed32,
            UnknownFixed64Ref(_) => wire_format::WireTypeFixed64,
            UnknownVarintRef(_) => wire_format::WireTypeVarint,
            UnknownLengthDelimitedRef(_) => wire_format::WireTypeLengthDelimited,
        }
    }
}

#[deriving(Clone,Eq,Default)]
pub struct UnknownValues {
    pub fixed32: Vec<u32>,
    pub fixed64: Vec<u64>,
    pub varint: Vec<u64>,
    pub length_delimited: Vec<~[u8]>,
}

impl UnknownValues {
    pub fn add_value(&mut self, value: UnknownValue) {
        match value {
            UnknownFixed64(fixed64) => self.fixed64.push(fixed64),
            UnknownFixed32(fixed32) => self.fixed32.push(fixed32),
            UnknownVarint(varint) => self.varint.push(varint),
            UnknownLengthDelimited(length_delimited) => self.length_delimited.push(length_delimited),
        };
    }

    pub fn iter<'s>(&'s self) -> UnknownValuesIter<'s> {
        UnknownValuesIter {
            fixed32: self.fixed32.iter(),
            fixed64: self.fixed64.iter(),
            varint: self.varint.iter(),
            length_delimited: self.length_delimited.iter(),
        }
    }
}

pub struct UnknownValuesIter<'o> {
    fixed32: slice::Items<'o, u32>,
    fixed64: slice::Items<'o, u64>,
    varint: slice::Items<'o, u64>,
    length_delimited: slice::Items<'o, ~[u8]>,
}

impl<'o> Iterator<UnknownValueRef<'o>> for UnknownValuesIter<'o> {
    fn next(&mut self) -> Option<UnknownValueRef<'o>> {
        let fixed32 = self.fixed32.next();
        if fixed32.is_some() {
            return Some(UnknownFixed32Ref(*fixed32.unwrap()));
        }
        let fixed64 = self.fixed64.next();
        if fixed64.is_some() {
            return Some(UnknownFixed64Ref(*fixed64.unwrap()));
        }
        let varint = self.varint.next();
        if varint.is_some() {
            return Some(UnknownVarintRef(*varint.unwrap()));
        }
        let length_delimited = self.length_delimited.next();
        if length_delimited.is_some() {
            return Some(UnknownLengthDelimitedRef(length_delimited.get_ref().as_slice()))
        }
        None
    }
}

#[deriving(Clone,Eq,Default)]
pub struct UnknownFields {
    // option is needed, so it could be placed in static field
    pub fields: Option<HashMap<u32, UnknownValues>>,
}

impl UnknownFields {
    pub fn default_instance() -> &'static UnknownFields {
        static instance: UnknownFields = UnknownFields {
            fields: None,
        };
        &'static instance
    }

    fn init_map(&mut self) {
        if self.fields.is_none() {
            self.fields = Some(Default::default());
        }
    }

    fn find_field<'a>(&'a mut self, number: u32) -> &'a mut UnknownValues {
        self.init_map();

        self.fields.get_mut_ref()
            .find_or_insert_with(number, |_| Default::default())
    }

    pub fn add_fixed32(&mut self, number: u32, fixed32: u32) {
        self.find_field(number).fixed32.push(fixed32);
    }

    pub fn add_fixed64(&mut self, number: u32, fixed64: u64) {
        self.find_field(number).fixed64.push(fixed64);
    }

    pub fn add_varint(&mut self, number: u32, varint: u64) {
        self.find_field(number).varint.push(varint);
    }

    pub fn add_length_delimited(&mut self, number: u32, length_delimited: ~[u8]) {
        self.find_field(number).length_delimited.push(length_delimited);
    }

    pub fn add_value(&mut self, number: u32, value: UnknownValue) {
        self.find_field(number).add_value(value);
    }

    pub fn iter<'s>(&'s self) -> UnknownFieldIter<'s> {
        UnknownFieldIter {
            entries: self.fields.as_ref().map(|m| m.iter())
        }
    }
}

pub struct UnknownFieldIter<'s> {
    entries: Option<hashmap::Entries<'s, u32, UnknownValues>>,
}

impl<'s> Iterator<(u32, &'s UnknownValues)> for UnknownFieldIter<'s> {
    fn next(&mut self) -> Option<(u32, &'s UnknownValues)> {
        if self.entries.is_none() {
            None
        } else {
            self.entries.get_mut_ref().next().map(|(&number, values)| (number, values))
        }
    }
}
