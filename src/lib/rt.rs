// Functions used by generated protobuf code.
// Should not be used by programs written by hands.

use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;
use std::io::Write;

use core::*;
use zigzag::*;
use stream::wire_format;
use stream::wire_format::WireType;
use stream::wire_format::WireTypeFixed32;
use stream::wire_format::WireTypeFixed64;
use stream::wire_format::WireTypeLengthDelimited;
use stream::wire_format::WireTypeVarint;
use error::ProtobufError;
use error::ProtobufResult;
use singular::SingularField;
use singular::SingularPtrField;
use repeated::RepeatedField;
use stream::CodedInputStream;
use stream::CodedOutputStream;
use types::*;

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
    vec.iter().map(|v| v.len_varint()).fold(0, |a, i| a + i)
}

// Size of serialized data, excluding length and tag
pub fn vec_packed_varint_zigzag_data_size<T : ProtobufVarintZigzag>(vec: &[T]) -> u32 {
    vec.iter().map(|v| v.len_varint_zigzag()).fold(0, |a, i| a + i)
}

pub fn vec_packed_enum_data_size<E : ProtobufEnum>(vec: &[E]) -> u32 {
    vec.iter().map(|e| compute_raw_varint32_size(e.value() as u32)).fold(0, |a, i| a + i)
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
    wire_format::Tag::make(field_number, WireTypeFixed64).value().len_varint()
}

pub fn value_size_no_tag<T : ProtobufVarint>(value: T, wt: WireType) -> u32 {
    match wt {
        WireTypeFixed64 => 8,
        WireTypeFixed32 => 4,
        WireTypeVarint => value.len_varint(),
        _ => panic!()
    }
}

pub fn value_size<T : ProtobufVarint>(field_number: u32, value: T, wt: WireType) -> u32 {
    tag_size(field_number) + value_size_no_tag(value, wt)
}

/// Length of value when encoded with zigzag encoding
pub fn value_varint_zigzag_size_no_tag<T : ProtobufVarintZigzag>(value: T) -> u32 {
    value.len_varint_zigzag()
}

/// Length of value when encoding with zigzag encoding with tag
pub fn value_varint_zigzag_size<T : ProtobufVarintZigzag>(field_number: u32, value: T) -> u32 {
    tag_size(field_number) + value_varint_zigzag_size_no_tag(value)
}

fn enum_size_no_tag<E : ProtobufEnum>(value: E) -> u32 {
    value.value().len_varint()
}

pub fn enum_size<E : ProtobufEnum>(field_number: u32, value: E) -> u32 {
    tag_size(field_number) + enum_size_no_tag(value)
}

// TODO: not used in generated code
pub fn bytes_size_no_tag(bytes: &[u8]) -> u32 {
    compute_raw_varint64_size(bytes.len() as u64) + bytes.len() as u32
}

pub fn bytes_size(field_number: u32, bytes: &[u8]) -> u32 {
    tag_size(field_number) + bytes_size_no_tag(bytes)
}

// TODO: not used in generated code
pub fn string_size_no_tag(s: &str) -> u32 {
    bytes_size_no_tag(s.as_bytes())
}

pub fn string_size(field_number: u32, s: &str) -> u32 {
    tag_size(field_number) + string_size_no_tag(s)
}

pub fn unknown_fields_size(unknown_fields: &UnknownFields) -> u32 {
    let mut r = 0;
    for (number, values) in unknown_fields {
        r += (tag_size(number) + 4) * values.fixed32.len() as u32;
        r += (tag_size(number) + 8) * values.fixed64.len() as u32;

        r += tag_size(number) * values.varint.len() as u32;
        for varint in &values.varint {
            r += varint.len_varint();
        }

        r += tag_size(number) * values.length_delimited.len() as u32;
        for bytes in &values.length_delimited {
            r += bytes_size_no_tag(&bytes);
        }
    }
    r
}


pub fn read_repeated_int32_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<i32>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_int32_into(target),
        WireTypeVarint => { target.push(is.read_int32()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_int64_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<i64>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_int64_into(target),
        WireTypeVarint => { target.push(is.read_int64()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_uint32_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<u32>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_uint32_into(target),
        WireTypeVarint => { target.push(is.read_uint32()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_uint64_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<u64>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_uint64_into(target),
        WireTypeVarint => { target.push(is.read_uint64()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_sint32_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<i32>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_sint32_into(target),
        WireTypeVarint => { target.push(is.read_sint32()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_sint64_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<i64>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_sint64_into(target),
        WireTypeVarint => { target.push(is.read_sint64()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_fixed32_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<u32>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_fixed32_into(target),
        WireTypeFixed32 => { target.push(is.read_fixed32()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_fixed64_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<u64>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_fixed64_into(target),
        WireTypeFixed64 => { target.push(is.read_fixed64()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_sfixed32_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<i32>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_sfixed32_into(target),
        WireTypeFixed32 => { target.push(is.read_sfixed32()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_sfixed64_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<i64>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_sfixed64_into(target),
        WireTypeFixed64 => { target.push(is.read_sfixed64()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_double_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<f64>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_double_into(target),
        WireTypeFixed64 => { target.push(is.read_double()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_float_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<f32>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_float_into(target),
        WireTypeFixed32 => { target.push(is.read_float()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_bool_into(wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<bool>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_bool_into(target),
        WireTypeVarint => { target.push(is.read_bool()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_enum_into<E : ProtobufEnum>(
    wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<E>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => is.read_repeated_packed_enum_into(target),
        WireTypeVarint => { target.push(is.read_enum()?); Ok(()) },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_string_into(
    wire_type: WireType, is: &mut CodedInputStream, target: &mut RepeatedField<String>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => {
            let tmp = target.push_default();
            is.read_string_into(tmp)
        },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_singular_string_into(
    wire_type: WireType, is: &mut CodedInputStream, target: &mut SingularField<String>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => {
            let tmp = target.set_default();
            is.read_string_into(tmp)
        },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_singular_proto3_string_into(
    wire_type: WireType, is: &mut CodedInputStream, target: &mut String)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => {
            is.read_string_into(target)
        }
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_bytes_into(
    wire_type: WireType, is: &mut CodedInputStream, target: &mut RepeatedField<Vec<u8>>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => {
            let tmp = target.push_default();
            is.read_bytes_into(tmp)
        },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_singular_bytes_into(
    wire_type: WireType, is: &mut CodedInputStream, target: &mut SingularField<Vec<u8>>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => {
            let tmp = target.set_default();
            is.read_bytes_into(tmp)
        },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_singular_proto3_bytes_into(
    wire_type: WireType, is: &mut CodedInputStream, target: &mut Vec<u8>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => {
            is.read_bytes_into(target)
        },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_repeated_message_into<M : Message + Default>(
    wire_type: WireType, is: &mut CodedInputStream, target: &mut RepeatedField<M>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => {
            let tmp = target.push_default();
            is.merge_message(tmp)
        },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

pub fn read_singular_message_into<M : Message + Default>(
    wire_type: WireType, is: &mut CodedInputStream, target: &mut SingularPtrField<M>)
        -> ProtobufResult<()>
{
    match wire_type {
        WireTypeLengthDelimited => {
            let tmp = target.set_default();
            is.merge_message(tmp)
        },
        _ => Err(unexpected_wire_type(wire_type)),
    }
}

fn skip_group(is: &mut CodedInputStream) -> ProtobufResult<()> {
    loop {
        let (_, wire_type) = is.read_tag_unpack()?;
        if wire_type == wire_format::WireTypeEndGroup {
            return Ok(());
        }
        is.skip_field(wire_type)?;
    }
}

// Handle unknown field in generated code.
// Either store a value in unknown, or skip a group.
pub fn read_unknown_or_skip_group(
    field_number: u32, wire_type: WireType,
    is: &mut CodedInputStream,
    unknown_fields: &mut UnknownFields,
)
    -> ProtobufResult<()>
{
    match wire_type {
        wire_format::WireTypeStartGroup => skip_group(is),
        _ => {
            let unknown = is.read_unknown(wire_type)?;
            unknown_fields.add_value(field_number, unknown);
            Ok(())
        }
    }
}


/// Create an error for unexpected wire type.
///
/// Function is used in generated code, so error types can be changed,
/// but this function remains unchanged.
pub fn unexpected_wire_type(wire_type: WireType) -> ProtobufError {
    ProtobufError::WireError(format!("unexpected wire type {:?}", wire_type))
}



pub fn compute_map_size<K, V>(field_number: u32, map: &HashMap<K::Value, V::Value>) -> u32
    where
        K : ProtobufType, V : ProtobufType,
        K::Value : Eq + Hash,
{
    let mut sum = 0;
    for (k, v) in map {
        let key_tag_size = 1;
        let value_tag_size = 1;

        let key_len = K::compute_size_with_length_delimiter(k);
        let value_len = V::compute_size_with_length_delimiter(v);

        let entry_len = key_tag_size + key_len
            + value_tag_size + value_len;
        sum += tag_size(field_number) + compute_raw_varint32_size(entry_len) + entry_len;
    }
    sum
}

pub fn write_map_with_cached_sizes<K, V, W>(
    field_number: u32,
    map: &HashMap<K::Value, V::Value>,
    os: &mut W)
        -> ProtobufResult<()>
    where
        K : ProtobufType, V : ProtobufType,
        K::Value : Eq + Hash,
        W: Write,
{
    for (k, v) in map {

        let key_tag_size = 1;
        let value_tag_size = 1;

        let key_len = K::get_cached_size_with_length_delimiter(k);
        let value_len = V::get_cached_size_with_length_delimiter(v);

        let entry_len = key_tag_size + key_len
            + value_tag_size + value_len;

        os.write_tag(field_number, WireType::WireTypeLengthDelimited)?;
        os.write_raw_varint32(entry_len)?;
        K::write_with_cached_size(1, k, os)?;
        V::write_with_cached_size(2, v, os)?;
    }
    Ok(())
}

pub fn read_map_into<K, V>(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut HashMap<K::Value, V::Value>)
        -> ProtobufResult<()>
    where
        K : ProtobufType, V : ProtobufType,
        K::Value : Eq + Hash,
{
    if wire_type != WireType::WireTypeLengthDelimited {
        return Err(unexpected_wire_type(wire_type));
    }

    let mut key = None;
    let mut value = None;

    let len = is.read_raw_varint32()?;
    let old_limit = is.push_limit(len as u64)?;
    while !is.eof()? {
        let (field_number, wire_type) = is.read_tag_unpack()?;
        match field_number {
            1 => {
                if wire_type != K::wire_type() {
                    return Err(unexpected_wire_type(wire_type));
                }
                key = Some(K::read(is)?);
            },
            2 => {
                if wire_type != V::wire_type() {
                    return Err(unexpected_wire_type(wire_type));
                }
                value = Some(V::read(is)?);
            },
            _ => is.skip_field(wire_type)?,
        }
    }
    is.pop_limit(old_limit);

    match (key, value) {
        (None, _) => return Err(ProtobufError::WireError(format!("map key is not set"))),
        (_, None) => return Err(ProtobufError::WireError(format!("map value is not set"))),
        (Some(key), Some(value)) => {
            target.insert(key, value);
        }
    }

    Ok(())
}
