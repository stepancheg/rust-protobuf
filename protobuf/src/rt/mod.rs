//! Functions and typrs used by generated protobuf code.
//!
//! Should rarely be used by programs written by hands.
use std::default::Default;

#[cfg(feature = "bytes")]
use ::bytes::Bytes;

pub use crate::cached_size::CachedSize;
#[cfg(feature = "bytes")]
use crate::chars::Chars;
use crate::coded_input_stream::CodedInputStream;
use crate::coded_output_stream::CodedOutputStream;
use crate::enums::Enum;
use crate::error::ProtobufError;
use crate::error::Result;
use crate::error::WireError;
pub use crate::lazy_v2::LazyV2;
use crate::message::*;
use crate::reflect::ProtobufValue;
use crate::unknown::UnknownFields;
use crate::varint::encoded_varint64_len;
use crate::wire_format::Tag;
pub use crate::wire_format::WireType;
use crate::zigzag::*;
use crate::EnumOrUnknown;
use crate::MessageField;

pub(crate) mod map;
pub use map::compute_map_size;
pub use map::read_map_into;
pub use map::write_map_with_cached_sizes;

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

/* Commented out due to https://github.com/mozilla/rust/issues/8075
impl<E:ProtobufEnum> ProtobufVarint for E {
    fn len_varint(&self) -> u32 {
        self.value().len_varint()
    }
}
*/

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
pub fn vec_packed_enum_data_size<E: Enum>(vec: &[E]) -> u64 {
    vec.iter()
        .map(|e| compute_raw_varint32_size(e.value() as u32))
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
pub fn vec_packed_enum_size<E: Enum>(field_number: u32, vec: &[E]) -> u64 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_enum_data_size(vec);
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
pub fn value_varint_zigzag_size_no_tag<T: ProtobufVarintZigzag>(value: T) -> u64 {
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
pub fn bytes_size_no_tag(bytes: &[u8]) -> u64 {
    compute_raw_varint64_size(bytes.len() as u64) + bytes.len() as u64
}

/// Size of encoded bytes field.
pub fn bytes_size(field_number: u32, bytes: &[u8]) -> u64 {
    tag_size(field_number) + bytes_size_no_tag(bytes)
}

/// Size of encoded string field.
pub fn string_size_no_tag(s: &str) -> u64 {
    bytes_size_no_tag(s.as_bytes())
}

/// Size of encoded string field.
pub fn string_size(field_number: u32, s: &str) -> u64 {
    tag_size(field_number) + string_size_no_tag(s)
}

/// Size of encoded unknown fields size.
pub fn unknown_fields_size(unknown_fields: &UnknownFields) -> u64 {
    let mut r = 0;
    for (number, values) in unknown_fields {
        r += (tag_size(number) + 4) * values.fixed32.len() as u64;
        r += (tag_size(number) + 8) * values.fixed64.len() as u64;

        r += tag_size(number) * values.varint.len() as u64;
        for varint in &values.varint {
            r += varint.len_varint();
        }

        r += tag_size(number) * values.length_delimited.len() as u64;
        for bytes in &values.length_delimited {
            r += bytes_size_no_tag(&bytes);
        }
    }
    r
}

/// Read repeated `int32` field into given vec.
pub fn read_repeated_int32_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<i32>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_int32_into(target),
        WireType::Varint => {
            target.push(is.read_int32()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `int64` field into given vec.
pub fn read_repeated_int64_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<i64>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_int64_into(target),
        WireType::Varint => {
            target.push(is.read_int64()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `uint32` field into given vec.
pub fn read_repeated_uint32_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<u32>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_uint32_into(target),
        WireType::Varint => {
            target.push(is.read_uint32()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `uint64` field into given vec.
pub fn read_repeated_uint64_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<u64>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_uint64_into(target),
        WireType::Varint => {
            target.push(is.read_uint64()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `sint32` field into given vec.
pub fn read_repeated_sint32_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<i32>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_sint32_into(target),
        WireType::Varint => {
            target.push(is.read_sint32()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `sint64` field into given vec.
pub fn read_repeated_sint64_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<i64>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_sint64_into(target),
        WireType::Varint => {
            target.push(is.read_sint64()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `fixed32` field into given vec.
pub fn read_repeated_fixed32_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<u32>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_fixed32_into(target),
        WireType::Fixed32 => {
            target.push(is.read_fixed32()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `fixed64` field into given vec.
pub fn read_repeated_fixed64_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<u64>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_fixed64_into(target),
        WireType::Fixed64 => {
            target.push(is.read_fixed64()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `sfixed32` field into given vec.
pub fn read_repeated_sfixed32_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<i32>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_sfixed32_into(target),
        WireType::Fixed32 => {
            target.push(is.read_sfixed32()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `sfixed64` field into given vec.
pub fn read_repeated_sfixed64_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<i64>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_sfixed64_into(target),
        WireType::Fixed64 => {
            target.push(is.read_sfixed64()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `double` field into given vec.
pub fn read_repeated_double_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<f64>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_double_into(target),
        WireType::Fixed64 => {
            target.push(is.read_double()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `float` field into given vec.
pub fn read_repeated_float_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<f32>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_float_into(target),
        WireType::Fixed32 => {
            target.push(is.read_float()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `bool` field into given vec.
pub fn read_repeated_bool_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<bool>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_bool_into(target),
        WireType::Varint => {
            target.push(is.read_bool()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `enum` field into given vec.
/// This function is no longer called from generated code, remove in 1.5.
pub fn read_repeated_enum_into<E: Enum + ProtobufValue>(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<E>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_repeated_packed_enum_into(target),
        WireType::Varint => {
            target.push(is.read_enum()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Helper function to read single enum value.
#[inline]
fn read_enum_with_unknown_fields_into<E: Enum, C>(
    is: &mut CodedInputStream,
    target: C,
    field_number: u32,
    unknown_fields: &mut UnknownFields,
) -> Result<()>
where
    C: FnOnce(E),
{
    let i = is.read_int32()?;
    match Enum::from_i32(i) {
        Some(e) => target(e),
        None => unknown_fields.add_varint(field_number, i as i64 as u64),
    }
    Ok(())
}

fn read_repeated_packed_enum_with_unknown_fields_into<E: Enum>(
    is: &mut CodedInputStream,
    target: &mut Vec<E>,
    field_number: u32,
    unknown_fields: &mut UnknownFields,
) -> Result<()> {
    let len = is.read_raw_varint64()?;
    let old_limit = is.push_limit(len)?;
    while !is.eof()? {
        read_enum_with_unknown_fields_into(is, |e| target.push(e), field_number, unknown_fields)?;
    }
    is.pop_limit(old_limit);
    Ok(())
}

fn read_repeated_packed_enum_or_unknown_into<E: Enum>(
    is: &mut CodedInputStream,
    target: &mut Vec<EnumOrUnknown<E>>,
) -> Result<()> {
    let len = is.read_raw_varint64()?;
    let old_limit = is.push_limit(len)?;
    while !is.eof()? {
        target.push(is.read_enum_or_unknown()?);
    }
    is.pop_limit(old_limit);
    Ok(())
}

/// Read repeated `enum` field into given vec,
/// and when value is unknown store it in unknown fields
/// which matches proto2 spec.
///
/// See explanation
/// [here](https://github.com/stepancheg/rust-protobuf/issues/233#issuecomment-375142710)
pub fn read_repeated_enum_with_unknown_fields_into<E: Enum>(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<E>,
    field_number: u32,
    unknown_fields: &mut UnknownFields,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => read_repeated_packed_enum_with_unknown_fields_into(
            is,
            target,
            field_number,
            unknown_fields,
        ),
        WireType::Varint => {
            read_enum_with_unknown_fields_into(is, |e| target.push(e), field_number, unknown_fields)
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `enum` field into given vec,
/// and when value is unknown store it in unknown fields
/// which matches proto2 spec.
///
/// See explanation
/// [here](https://github.com/stepancheg/rust-protobuf/issues/233#issuecomment-375142710)
pub fn read_repeated_enum_or_unknown_into<E: Enum>(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<EnumOrUnknown<E>>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => read_repeated_packed_enum_or_unknown_into(is, target),
        WireType::Varint => {
            target.push(is.read_enum_or_unknown()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `enum` field into given vec,
/// and when value is unknown store it in unknown fields
/// which matches proto2 spec.
///
/// See explanation
/// [here](https://github.com/stepancheg/rust-protobuf/issues/233#issuecomment-375142710)
pub fn read_proto3_enum_with_unknown_fields_into<E: Enum>(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut E,
    field_number: u32,
    unknown_fields: &mut UnknownFields,
) -> Result<()> {
    if wire_type != WireType::Varint {
        return Err(unexpected_wire_type(wire_type));
    }

    read_enum_with_unknown_fields_into(is, |e| *target = e, field_number, unknown_fields)
}

/// Read repeated `enum` field into given vec,
/// and when value is unknown store it in unknown fields
/// which matches proto2 spec.
///
/// See explanation
/// [here](https://github.com/stepancheg/rust-protobuf/issues/233#issuecomment-375142710)
pub fn read_proto2_enum_with_unknown_fields_into<E: Enum>(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Option<E>,
    field_number: u32,
    unknown_fields: &mut UnknownFields,
) -> Result<()> {
    if wire_type != WireType::Varint {
        return Err(unexpected_wire_type(wire_type));
    }

    read_enum_with_unknown_fields_into(is, |e| *target = Some(e), field_number, unknown_fields)
}

/// Read repeated `string` field into given vec.
pub fn read_repeated_string_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<String>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => {
            target.push(is.read_string()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `Chars` field into given vec.
#[cfg(feature = "bytes")]
pub fn read_repeated_tokio_string_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<Chars>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => {
            target.push(is.read_tokio_chars()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read singular `Chars` field.
#[cfg(feature = "bytes")]
pub fn read_singular_tokio_string_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Option<Chars>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => {
            *target = Some(is.read_tokio_chars()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read singular `string` field for proto3.
pub fn read_singular_proto3_string_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut String,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_string_into(target),
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read singular `Chars` field for proto3.
#[cfg(feature = "bytes")]
pub fn read_singular_proto3_tokio_string_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Chars,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => {
            *target = is.read_tokio_chars()?;
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `bytes` field into given vec.
pub fn read_repeated_bytes_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<Vec<u8>>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => {
            target.push(is.read_bytes()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `Bytes` field into given vec.
#[cfg(feature = "bytes")]
pub fn read_repeated_tokio_bytes_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<Bytes>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => {
            target.push(is.read_tokio_bytes()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read singular `Bytes` field.
#[cfg(feature = "bytes")]
pub fn read_singular_tokio_bytes_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Option<Bytes>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => {
            *target = Some(is.read_tokio_bytes()?);
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read singular `bytes` field for proto3.
pub fn read_singular_proto3_bytes_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<u8>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => is.read_bytes_into(target),
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read singular `Bytes` field for proto3.
#[cfg(feature = "bytes")]
pub fn read_singular_proto3_tokio_bytes_into(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Bytes,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => {
            *target = is.read_tokio_bytes()?;
            Ok(())
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read repeated `message` field.
pub fn read_repeated_message_into_vec<M: Message + Default>(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<M>,
) -> Result<()> {
    match wire_type {
        WireType::LengthDelimited => {
            is.incr_recursion()?;
            let res = match is.read_message() {
                Ok(m) => {
                    target.push(m);
                    Ok(())
                }
                Err(e) => Err(e),
            };
            is.decr_recursion();
            res
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

/// Read singular `message` field.
pub fn read_singular_message_into_field<M>(
    is: &mut CodedInputStream,
    target: &mut MessageField<M>,
) -> Result<()>
where
    M: Message + Default,
{
    is.incr_recursion()?;
    let mut m = M::new();
    let res = is.merge_message(&mut m);
    *target = MessageField::some(m);
    is.decr_recursion();
    res
}

fn skip_group(is: &mut CodedInputStream) -> Result<()> {
    loop {
        let (_, wire_type) = is.read_tag_unpack()?;
        if wire_type == WireType::EndGroup {
            return Ok(());
        }
        is.skip_field(wire_type)?;
    }
}

/// Handle unknown field in generated code.
/// Either store a value in unknown, or skip a group.
pub(crate) fn read_unknown_or_skip_group_with_tag_unpacked(
    field_number: u32,
    wire_type: WireType,
    is: &mut CodedInputStream,
    unknown_fields: &mut UnknownFields,
) -> crate::Result<()> {
    match wire_type {
        WireType::StartGroup => skip_group(is),
        _ => {
            let unknown = is.read_unknown(wire_type)?;
            unknown_fields.add_value(field_number, unknown);
            Ok(())
        }
    }
}

/// Handle unknown field in generated code.
/// Either store a value in unknown, or skip a group.
/// Return error if tag is incorrect.
pub fn read_unknown_or_skip_group(
    tag: u32,
    is: &mut CodedInputStream,
    unknown_fields: &mut UnknownFields,
) -> crate::Result<()> {
    let (field_humber, wire_type) = Tag::new(tag)?.unpack();
    read_unknown_or_skip_group_with_tag_unpacked(field_humber, wire_type, is, unknown_fields)
}

/// Create an error for unexpected wire type.
///
/// Function is used in generated code, so error types can be changed,
/// but this function remains unchanged.
pub fn unexpected_wire_type(wire_type: WireType) -> crate::Error {
    ProtobufError::WireError(WireError::UnexpectedWireType(wire_type)).into()
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
