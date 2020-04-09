use std::collections::hash_map::DefaultHasher;
use std::default::Default;
use std::hash::Hash;
use std::hash::Hasher;
use std::slice;

use crate::clear::Clear;

use crate::wire_format;
use crate::zigzag::encode_zig_zag_32;
use crate::zigzag::encode_zig_zag_64;

/// Unknown value.
///
/// See [`UnknownFields`](crate::UnknownFields) for the explanations.
#[derive(Debug)]
pub enum UnknownValue {
    /// 32-bit unknown (e. g. `fixed32` or `float`)
    Fixed32(u32),
    /// 64-bit unknown (e. g. `fixed64` or `double`)
    Fixed64(u64),
    /// Varint unknown (e. g. `int32` or `bool`)
    Varint(u64),
    /// Length-delimited unknown (e. g. `message` or `string`)
    LengthDelimited(Vec<u8>),
}

impl UnknownValue {
    /// Wire type for this unknown
    pub fn wire_type(&self) -> wire_format::WireType {
        self.get_ref().wire_type()
    }

    /// As ref
    pub fn get_ref<'s>(&'s self) -> UnknownValueRef<'s> {
        match *self {
            UnknownValue::Fixed32(fixed32) => UnknownValueRef::Fixed32(fixed32),
            UnknownValue::Fixed64(fixed64) => UnknownValueRef::Fixed64(fixed64),
            UnknownValue::Varint(varint) => UnknownValueRef::Varint(varint),
            UnknownValue::LengthDelimited(ref bytes) => UnknownValueRef::LengthDelimited(&bytes),
        }
    }

    /// Construct unknown value from `sint32` value.
    pub fn sint32(i: i32) -> UnknownValue {
        UnknownValue::Varint(encode_zig_zag_32(i) as u64)
    }

    /// Construct unknown value from `sint64` value.
    pub fn sint64(i: i64) -> UnknownValue {
        UnknownValue::Varint(encode_zig_zag_64(i))
    }
}

/// Reference to unknown value.
///
/// See [`UnknownFields`](crate::UnknownFields) for explanations.
pub enum UnknownValueRef<'o> {
    /// 32-bit unknown
    Fixed32(u32),
    /// 64-bit unknown
    Fixed64(u64),
    /// Varint unknown
    Varint(u64),
    /// Length-delimited unknown
    LengthDelimited(&'o [u8]),
}

impl<'o> UnknownValueRef<'o> {
    /// Wire-type to serialize this unknown
    pub fn wire_type(&self) -> wire_format::WireType {
        match *self {
            UnknownValueRef::Fixed32(_) => wire_format::WireTypeFixed32,
            UnknownValueRef::Fixed64(_) => wire_format::WireTypeFixed64,
            UnknownValueRef::Varint(_) => wire_format::WireTypeVarint,
            UnknownValueRef::LengthDelimited(_) => wire_format::WireTypeLengthDelimited,
        }
    }
}

/// Field unknown values.
///
/// See [`UnknownFields`](crate::UnknownFields) for explanations.
#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub struct UnknownValues {
    /// 32-bit unknowns
    pub fixed32: Vec<u32>,
    /// 64-bit unknowns
    pub fixed64: Vec<u64>,
    /// Varint unknowns
    pub varint: Vec<u64>,
    /// Length-delimited unknowns
    pub length_delimited: Vec<Vec<u8>>,
}

impl UnknownValues {
    /// Add unknown value
    pub fn add_value(&mut self, value: UnknownValue) {
        match value {
            UnknownValue::Fixed64(fixed64) => self.fixed64.push(fixed64),
            UnknownValue::Fixed32(fixed32) => self.fixed32.push(fixed32),
            UnknownValue::Varint(varint) => self.varint.push(varint),
            UnknownValue::LengthDelimited(length_delimited) => {
                self.length_delimited.push(length_delimited)
            }
        };
    }

    /// Iterate over unknown values
    pub fn iter<'s>(&'s self) -> UnknownValuesIter<'s> {
        UnknownValuesIter {
            fixed32: self.fixed32.iter(),
            fixed64: self.fixed64.iter(),
            varint: self.varint.iter(),
            length_delimited: self.length_delimited.iter(),
        }
    }
}

impl<'a> IntoIterator for &'a UnknownValues {
    type Item = UnknownValueRef<'a>;
    type IntoIter = UnknownValuesIter<'a>;

    fn into_iter(self) -> UnknownValuesIter<'a> {
        self.iter()
    }
}

/// Iterator over unknown values
pub struct UnknownValuesIter<'o> {
    fixed32: slice::Iter<'o, u32>,
    fixed64: slice::Iter<'o, u64>,
    varint: slice::Iter<'o, u64>,
    length_delimited: slice::Iter<'o, Vec<u8>>,
}

impl<'o> Iterator for UnknownValuesIter<'o> {
    type Item = UnknownValueRef<'o>;

    fn next(&mut self) -> Option<UnknownValueRef<'o>> {
        let fixed32 = self.fixed32.next();
        if fixed32.is_some() {
            return Some(UnknownValueRef::Fixed32(*fixed32.unwrap()));
        }
        let fixed64 = self.fixed64.next();
        if fixed64.is_some() {
            return Some(UnknownValueRef::Fixed64(*fixed64.unwrap()));
        }
        let varint = self.varint.next();
        if varint.is_some() {
            return Some(UnknownValueRef::Varint(*varint.unwrap()));
        }
        let length_delimited = self.length_delimited.next();
        if length_delimited.is_some() {
            return Some(UnknownValueRef::LengthDelimited(&length_delimited.unwrap()));
        }
        None
    }
}

/// Hold "unknown" fields in parsed message.
///
/// Field may be unknown if it they are added in newer version of `.proto`.
/// Unknown fields are stored in `UnknownFields` structure, so
/// protobuf message could process messages without losing data.
///
/// For example, in this operation: load from DB, modify, store to DB,
/// even when working with older `.proto` file, new fields won't be lost.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct UnknownFields {
    /// The map.
    fields: Vec<(u32, UnknownValues)>,
}

/// Very simple hash implementation of `Hash` for `UnknownFields`.
/// Since map is unordered, we cannot put entry hashes into hasher,
/// instead we summing hashes of entries.
impl Hash for UnknownFields {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash: u64 = 0;
        for (k, v) in &self.fields {
            let mut entry_hasher = DefaultHasher::new();
            Hash::hash(&(k, v), &mut entry_hasher);
            hash = hash.wrapping_add(entry_hasher.finish());
        }
        Hash::hash(&self.fields.len(), state);
        Hash::hash(&hash, state);
    }
}

impl UnknownFields {
    /// Empty unknown fields
    pub fn new() -> UnknownFields {
        Default::default()
    }

    fn find_field<'a>(&'a mut self, number: u32) -> &'a mut UnknownValues {
        for i in 0..self.fields.len() {
            if self.fields[i].0 == number {
                return &mut self.fields[i].1;
            }
        }
        self.fields.push((number, Default::default()));
        &mut self.fields.last_mut().unwrap().1
    }

    /// Add unknown fixed 32-bit
    pub fn add_fixed32(&mut self, number: u32, fixed32: u32) {
        self.find_field(number).fixed32.push(fixed32);
    }

    /// Add unknown fixed 64-bit
    pub fn add_fixed64(&mut self, number: u32, fixed64: u64) {
        self.find_field(number).fixed64.push(fixed64);
    }

    /// Add unknown varint
    pub fn add_varint(&mut self, number: u32, varint: u64) {
        self.find_field(number).varint.push(varint);
    }

    /// Add unknown length delimited
    pub fn add_length_delimited(&mut self, number: u32, length_delimited: Vec<u8>) {
        self.find_field(number)
            .length_delimited
            .push(length_delimited);
    }

    /// Add unknown value
    pub fn add_value(&mut self, number: u32, value: UnknownValue) {
        self.find_field(number).add_value(value);
    }

    /// Iterate over all unknowns
    pub fn iter<'s>(&'s self) -> UnknownFieldsIter<'s> {
        UnknownFieldsIter {
            entries: self.fields.iter(),
        }
    }

    /// Find unknown field by number
    pub fn get(&self, field_number: u32) -> Option<&UnknownValues> {
        self.fields
            .iter()
            .find(|(n, _v)| *n == field_number)
            .map(|(_n, v)| v)
    }
}

impl Clear for UnknownFields {
    fn clear(&mut self) {
        self.fields.clear();
    }
}

impl<'a> IntoIterator for &'a UnknownFields {
    type Item = (u32, &'a UnknownValues);
    type IntoIter = UnknownFieldsIter<'a>;

    fn into_iter(self) -> UnknownFieldsIter<'a> {
        self.iter()
    }
}

/// Iterator over [`UnknownFields`](crate::UnknownFields)
pub struct UnknownFieldsIter<'s> {
    entries: slice::Iter<'s, (u32, UnknownValues)>,
}

impl<'s> Iterator for UnknownFieldsIter<'s> {
    type Item = (u32, &'s UnknownValues);

    fn next(&mut self) -> Option<(u32, &'s UnknownValues)> {
        self.entries
            .next()
            .map(|(number, values)| (*number, values))
    }
}

#[cfg(test)]
mod test {
    use super::UnknownFields;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;

    #[test]
    fn unknown_fields_hash() {
        let mut unknown_fields_1 = UnknownFields::new();
        let mut unknown_fields_2 = UnknownFields::new();

        // Check field order is not important

        unknown_fields_1.add_fixed32(10, 222);
        unknown_fields_1.add_fixed32(10, 223);
        unknown_fields_1.add_fixed64(14, 224);

        unknown_fields_2.add_fixed32(10, 222);
        unknown_fields_2.add_fixed64(14, 224);
        unknown_fields_2.add_fixed32(10, 223);

        fn hash(unknown_fields: &UnknownFields) -> u64 {
            let mut hasher = DefaultHasher::new();
            Hash::hash(unknown_fields, &mut hasher);
            hasher.finish()
        }

        assert_eq!(hash(&unknown_fields_1), hash(&unknown_fields_2));
    }
}
