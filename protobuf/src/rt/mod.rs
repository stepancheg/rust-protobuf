//! # Functions and types used by generated protobuf code
//!
//! These are not considered to be public API of rust-protobuf,
//! so they can be changed any time (provided compatibility with
//! previously generated code is preserved).
use std::default::Default;

pub use crate::cached_size::CachedSize;
use crate::coded_input_stream::CodedInputStream;
use crate::coded_output_stream::CodedOutputStream;
use crate::enums::Enum;
use crate::error::Result;
pub use crate::lazy::Lazy;
use crate::varint::encoded_varint64_len;
pub use crate::wire_format::WireType;
use crate::zigzag::*;
use crate::EnumOrUnknown;
use crate::Message;
use crate::MessageField;

pub(crate) mod map;
pub(crate) mod repeated;
pub(crate) mod unsorted;
pub use map::compute_map_size;
pub use map::read_map_into;
pub use map::write_map_with_cached_sizes;
pub use repeated::read_repeated_packed_enum_or_unknown_into;
pub use unsorted::read_unknown_or_skip_group;
pub use unsorted::unknown_fields_size;

/// Given `u64` value compute varint encoded length.
pub fn compute_raw_varint64_size(value: u64) -> u64 {
    encoded_varint64_len(value) as u64
}

/// Given `u32` value compute varint encoded length.
pub fn compute_raw_varint32_size(value: u32) -> u64 {
    compute_raw_varint64_size(value as u64)
}

/// Given `usize` value compute varint encoded length.
pub fn compute_raw_varint_usize_size(value: usize) -> u64 {
    compute_raw_varint64_size(value as u64)
}

/// Fixed size integers.
pub trait ProtobufFixed {
    /// Size of this fixed type in bytes.
    const LEN: u32;
}

/// Helper trait implemented by integer types which could be encoded as varint.
pub trait ProtobufVarint {
    /// Size of self when encoded as varint.
    fn len_varint(&self) -> u64;
}

/// Helper trait implemented by integer types which could be encoded as zigzag varint.
pub trait ProtobufVarintZigzag {
    /// Size of self when encoded as zigzag varint.
    fn len_varint_zigzag(&self) -> u64;
}

impl ProtobufVarint for u64 {
    fn len_varint(&self) -> u64 {
        compute_raw_varint64_size(*self)
    }
}

impl ProtobufVarint for u32 {
    fn len_varint(&self) -> u64 {
        (*self as u64).len_varint()
    }
}

impl ProtobufVarint for i64 {
    fn len_varint(&self) -> u64 {
        // same as length of u64
        (*self as u64).len_varint()
    }
}

impl ProtobufVarintZigzag for i64 {
    fn len_varint_zigzag(&self) -> u64 {
        compute_raw_varint64_size(encode_zig_zag_64(*self))
    }
}

impl ProtobufVarint for i32 {
    fn len_varint(&self) -> u64 {
        // sign-extend and then compute
        (*self as i64).len_varint()
    }
}

impl ProtobufVarintZigzag for i32 {
    fn len_varint_zigzag(&self) -> u64 {
        compute_raw_varint32_size(encode_zig_zag_32(*self))
    }
}

impl ProtobufVarint for bool {
    fn len_varint(&self) -> u64 {
        1
    }
}

impl ProtobufFixed for u32 {
    const LEN: u32 = 4;
}

impl ProtobufFixed for i32 {
    const LEN: u32 = 4;
}

impl ProtobufFixed for u64 {
    const LEN: u32 = 8;
}

impl ProtobufFixed for i64 {
    const LEN: u32 = 8;
}

impl ProtobufFixed for f32 {
    const LEN: u32 = 4;
}

impl ProtobufFixed for f64 {
    const LEN: u32 = 8;
}

/// Technically `bool` is not fixed, but it can be considered as fixed
/// for the purpose of encoding.
impl ProtobufFixed for bool {
    const LEN: u32 = 1;
}

/// Size of serialized repeated packed field, excluding length and tag.
pub fn vec_packed_varint_data_size<T: ProtobufVarint>(vec: &[T]) -> u64 {
    vec.iter()
        .map(|v| v.len_varint() as u64)
        .fold(0, |a, i| a + i)
}

/// Size of serialized repeated packed field, excluding length and tag.
pub fn vec_packed_varint_zigzag_data_size<T: ProtobufVarintZigzag>(vec: &[T]) -> u64 {
    vec.iter()
        .map(|v| v.len_varint_zigzag())
        .fold(0, |a, i| a + i)
}

/// Size of serialized repeated packed enum field, excluding length and tag.
pub fn vec_packed_enum_or_unknown_data_size<E: Enum>(vec: &[EnumOrUnknown<E>]) -> u64 {
    vec.iter()
        .map(|e| compute_raw_varint32_size(e.value() as u32))
        .fold(0, |a, i| a + i)
}

/// Size of serialized data with length prefix and tag
pub fn vec_packed_varint_size<T: ProtobufVarint>(field_number: u32, vec: &[T]) -> u64 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_varint_data_size(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

/// Size of serialized data with length prefix and tag
pub fn vec_packed_varint_zigzag_size<T: ProtobufVarintZigzag>(field_number: u32, vec: &[T]) -> u64 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_varint_zigzag_data_size(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

/// Size of serialized data with length prefix and tag
pub fn vec_packed_enum_or_unknown_size<E: Enum>(
    field_number: u32,
    vec: &[EnumOrUnknown<E>],
) -> u64 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_enum_or_unknown_data_size(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

/// Compute data size of fixed encoding of repeated field data.
pub(crate) fn vec_packed_fixed_data_size<V: ProtobufFixed>(vec: &[V]) -> u64 {
    (vec.len() as u64) * (V::LEN as u64)
}

/// Compute field size (data plus header) of fixed encoding of repeated field.
pub fn vec_packed_fixed_size<V: ProtobufFixed>(field_number: u32, vec: &[V]) -> u64 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_fixed_data_size::<V>(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

/// Compute tag size. Size of tag does not depend on wire type.
pub fn tag_size(field_number: u32) -> u64 {
    encoded_varint64_len((field_number as u64) << 3) as u64
}

fn value_size_no_tag<T: ProtobufVarint>(value: T, wt: WireType) -> u64 {
    match wt {
        WireType::Fixed64 => 8,
        WireType::Fixed32 => 4,
        WireType::Varint => value.len_varint(),
        _ => panic!(),
    }
}

/// Integer value size when encoded as specified wire type.
pub fn value_size<T: ProtobufVarint>(field_number: u32, value: T, wt: WireType) -> u64 {
    tag_size(field_number) + value_size_no_tag(value, wt)
}

/// Integer value size when encoded as specified wire type.
pub(crate) fn value_varint_zigzag_size_no_tag<T: ProtobufVarintZigzag>(value: T) -> u64 {
    value.len_varint_zigzag()
}

/// Length of value when encoding with zigzag encoding with tag
pub fn value_varint_zigzag_size<T: ProtobufVarintZigzag>(field_number: u32, value: T) -> u64 {
    tag_size(field_number) + value_varint_zigzag_size_no_tag(value)
}

fn enum_or_unknown_size_no_tag<E: Enum>(value: EnumOrUnknown<E>) -> u64 {
    value.value().len_varint() as u64
}

/// Size of encoded enum field value.
pub fn enum_or_unknown_size<E: Enum>(field_number: u32, value: EnumOrUnknown<E>) -> u64 {
    tag_size(field_number) + enum_or_unknown_size_no_tag(value)
}

/// Size of encoded bytes field.
pub(crate) fn bytes_size_no_tag(bytes: &[u8]) -> u64 {
    compute_raw_varint64_size(bytes.len() as u64) + bytes.len() as u64
}

/// Size of encoded bytes field.
pub fn bytes_size(field_number: u32, bytes: &[u8]) -> u64 {
    tag_size(field_number) + bytes_size_no_tag(bytes)
}

/// Size of encoded string field.
pub(crate) fn string_size_no_tag(s: &str) -> u64 {
    bytes_size_no_tag(s.as_bytes())
}

/// Size of encoded string field.
pub fn string_size(field_number: u32, s: &str) -> u64 {
    tag_size(field_number) + string_size_no_tag(s)
}

/// Read singular `message` field.
pub fn read_singular_message_into_field<M>(
    is: &mut CodedInputStream,
    target: &mut MessageField<M>,
) -> Result<()>
where
    M: Message + Default,
{
    let mut m = M::new();
    is.merge_message(&mut m)?;
    *target = MessageField::some(m);
    Ok(())
}

/// Write message with field number and length to the stream.
pub fn write_message_field_with_cached_size<M>(
    field_number: u32,
    message: &M,
    os: &mut CodedOutputStream,
) -> Result<()>
where
    M: Message,
{
    os.write_tag(field_number, WireType::LengthDelimited)?;
    os.write_raw_varint32(message.cached_size())?;
    message.write_to_with_cached_sizes(os)
}
