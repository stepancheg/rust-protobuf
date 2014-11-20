// Functions used by generated protobuf code.
// Should not be used by programs written by hands.

use std::iter::AdditiveIterator;

use core::*;
use zigzag::*;
use stream::wire_format;

use unknown::UnknownFields;


pub fn compute_raw_varint64_size(value: u64) -> u32 {
    if (value & (0xffffffffffffffffu64 <<  7)) == 0 { return 1; }
    if (value & (0xffffffffffffffffu64 << 14)) == 0 { return 2; }
    if (value & (0xffffffffffffffffu64 << 21)) == 0 { return 3; }
    if (value & (0xffffffffffffffffu64 << 28)) == 0 { return 4; }
    if (value & (0xffffffffffffffffu64 << 35)) == 0 { return 5; }
    if (value & (0xffffffffffffffffu64 << 42)) == 0 { return 6; }
    if (value & (0xffffffffffffffffu64 << 49)) == 0 { return 7; }
    if (value & (0xffffffffffffffffu64 << 56)) == 0 { return 8; }
    if (value & (0xffffffffffffffffu64 << 63)) == 0 { return 9; }
    10
}

pub fn compute_raw_varint32_size(value: u32) -> u32 {
    compute_raw_varint64_size(value as u64)
}

pub trait ProtobufVarint {
    // size of self when written as varint
    fn len_varint(&self) -> u32;
}

pub trait ProtobufVarintZigzag {
    fn len_varint_zigzag(&self) -> u32;
}

impl ProtobufVarint for u64 {
    fn len_varint(&self) -> u32 {
        compute_raw_varint64_size(*self)
    }
}

impl ProtobufVarint for u32 {
    fn len_varint(&self) -> u32 {
        (*self as u64).len_varint()
    }
}

impl ProtobufVarint for i64 {
    fn len_varint(&self) -> u32 {
        // same as length of u64
        (*self as u64).len_varint()
    }
}

impl ProtobufVarintZigzag for i64 {
    fn len_varint_zigzag(&self) -> u32 {
        compute_raw_varint64_size(encode_zig_zag_64(*self))
    }
}

impl ProtobufVarint for i32 {
    fn len_varint(&self) -> u32 {
        // sign-extend and then compute
        (*self as i64).len_varint()
    }
}

impl ProtobufVarintZigzag for i32 {
    fn len_varint_zigzag(&self) -> u32 {
        compute_raw_varint32_size(encode_zig_zag_32(*self))
    }
}

impl ProtobufVarint for bool {
    fn len_varint(&self) -> u32 {
        1
    }
}

/* Commented out due to https://github.com/mozilla/rust/issues/8075
impl<E:ProtobufEnum> ProtobufVarint for E {
    fn len_varint(&self) -> u32 {
        self.value().len_varint()
    }
}
*/

// Size of serialized data, excluding length and tag
pub fn vec_packed_varint_data_size<T : ProtobufVarint>(vec: &[T]) -> u32 {
    vec.iter().map(|v| v.len_varint()).sum()
}

// Size of serialized data, excluding length and tag
pub fn vec_packed_varint_zigzag_data_size<T : ProtobufVarintZigzag>(vec: &[T]) -> u32 {
    vec.iter().map(|v| v.len_varint_zigzag()).sum()
}

pub fn vec_packed_enum_data_size<E : ProtobufEnum>(vec: &[E]) -> u32 {
    vec.iter().map(|e| compute_raw_varint32_size(e.value() as u32)).sum()
}

// Size of serialized data with length prefix and tag
pub fn vec_packed_varint_size<T : ProtobufVarint>(field_number: u32, vec: &[T]) -> u32 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_varint_data_size(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

// Size of serialized data with length prefix and tag
pub fn vec_packed_varint_zigzag_size<T : ProtobufVarintZigzag>(field_number: u32, vec: &[T]) -> u32 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_varint_zigzag_data_size(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

pub fn vec_packed_enum_size<E : ProtobufEnum>(field_number: u32, vec: &[E]) -> u32 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_enum_data_size(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

// Size of tag does not depend on wire type
pub fn tag_size(field_number: u32) -> u32 {
    wire_format::Tag::make(field_number, wire_format::WireTypeFixed64).value().len_varint()
}

pub fn value_size_no_tag<T : ProtobufVarint>(value: T, wt: wire_format::WireType) -> u32 {
    match wt {
        wire_format::WireTypeFixed64 => 8,
        wire_format::WireTypeFixed32 => 4,
        wire_format::WireTypeVarint => value.len_varint(),
        _ => panic!()
    }
}

pub fn value_size<T : ProtobufVarint>(field_number: u32, value: T, wt: wire_format::WireType) -> u32 {
    tag_size(field_number) + value_size_no_tag(value, wt)
}

fn enum_size_no_tag<E : ProtobufEnum>(value: E) -> u32 {
    value.value().len_varint()
}

pub fn enum_size<E : ProtobufEnum>(field_number: u32, value: E) -> u32 {
    tag_size(field_number) + enum_size_no_tag(value)
}

fn bytes_size_no_tag(bytes: &[u8]) -> u32 {
    compute_raw_varint64_size(bytes.len() as u64) + bytes.len() as u32
}

pub fn bytes_size(field_number: u32, bytes: &[u8]) -> u32 {
    tag_size(field_number) + bytes_size_no_tag(bytes)
}

fn string_size_no_tag(s: &str) -> u32 {
    bytes_size_no_tag(s.as_bytes())
}

pub fn string_size(field_number: u32, s: &str) -> u32 {
    tag_size(field_number) + string_size_no_tag(s)
}

pub fn unknown_fields_size(unknown_fields: &UnknownFields) -> u32 {
    let mut r = 0;
    for (number, values) in unknown_fields.iter() {
        r += (tag_size(number) + 4) * values.fixed32.len() as u32;
        r += (tag_size(number) + 8) * values.fixed64.len() as u32;

        r += tag_size(number) * values.varint.len() as u32;
        for varint in values.varint.iter() {
            r += varint.len_varint();
        }

        r += tag_size(number) * values.length_delimited.len() as u32;
        for bytes in values.length_delimited.iter() {
            r += bytes_size_no_tag(bytes.as_slice());
        }
    }
    r
}

