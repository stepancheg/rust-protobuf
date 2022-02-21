//! Implementations of `ProtobufType` for all types.

#![doc(hidden)]

use std::marker;

#[cfg(feature = "bytes")]
use ::bytes::Bytes;

#[cfg(feature = "bytes")]
use crate::chars::Chars;
use crate::coded_input_stream::CodedInputStream;
use crate::coded_output_stream::CodedOutputStream;
use crate::enums::Enum;
use crate::error::Result;
pub use crate::reflect::type_dynamic::ProtobufTypeDynamic;
use crate::reflect::type_dynamic::ProtobufTypeDynamicImpl;
use crate::reflect::ProtobufValue;
use crate::rt;
use crate::unknown::UnknownValues;
use crate::wire_format::WireType;
use crate::zigzag::decode_zig_zag_32;
use crate::zigzag::decode_zig_zag_64;
use crate::EnumOrUnknown;
use crate::Message;

/// Encapsulate type-specific serialization and conversion logic
pub trait ProtobufType: Send + Sync + Clone + Sized + 'static {
    /// Rust type for this protobuf type.
    type ProtobufValue: Default;

    /// Dynamic version of this
    fn dynamic() -> &'static dyn ProtobufTypeDynamic
    where
        Self::ProtobufValue: ProtobufValue,
    {
        &ProtobufTypeDynamicImpl::<Self>(marker::PhantomData)
    }

    /// Wire type for encoding objects of this type
    const WIRE_TYPE: WireType;

    /// Read a value from `CodedInputStream`
    fn read(is: &mut CodedInputStream) -> Result<Self::ProtobufValue>;

    /// Take a value from `UnknownValues`
    fn get_from_unknown(_unknown_values: &UnknownValues) -> Option<Self::ProtobufValue>;

    /// Compute serialized size of a value
    fn compute_size(value: &Self::ProtobufValue) -> u64;

    /// Compute size adding length prefix if wire type is length delimited
    /// (i. e. string, bytes, message)
    fn compute_size_with_length_delimiter(value: &Self::ProtobufValue) -> u64 {
        let size = Self::compute_size(value);
        if Self::WIRE_TYPE == WireType::LengthDelimited {
            rt::compute_raw_varint64_size(size) + size
        } else {
            size
        }
    }

    /// Get previously computed size
    #[inline]
    fn get_cached_size(value: &Self::ProtobufValue) -> u32 {
        Self::compute_size(value) as u32
    }

    /// Get previously cached size with length prefix
    #[inline]
    fn get_cached_size_with_length_delimiter(value: &Self::ProtobufValue) -> u32 {
        let size = Self::get_cached_size(value);
        if Self::WIRE_TYPE == WireType::LengthDelimited {
            rt::compute_raw_varint32_size(size) as u32 + size
        } else {
            size
        }
    }

    /// Write a value with previously cached size
    fn write_with_cached_size(
        field_number: u32,
        value: &Self::ProtobufValue,
        os: &mut CodedOutputStream,
    ) -> Result<()>;
}

/// All fixed size types
pub trait ProtobufTypeFixed: ProtobufType {
    /// Encoded size of value in bytes of this type.
    ///
    /// E. g. it is `4` for `fixed32`
    const ENCODED_SIZE: u32;
}

/// `float`
#[derive(Copy, Clone)]
pub struct ProtobufTypeFloat;
/// `double`
#[derive(Copy, Clone)]
pub struct ProtobufTypeDouble;
/// `int32`
#[derive(Copy, Clone)]
pub struct ProtobufTypeInt32;
/// `int64`
#[derive(Copy, Clone)]
pub struct ProtobufTypeInt64;
/// `uint32`
#[derive(Copy, Clone)]
pub struct ProtobufTypeUint32;
/// `uint64`
#[derive(Copy, Clone)]
pub struct ProtobufTypeUint64;
/// `sint32`
#[derive(Copy, Clone)]
pub struct ProtobufTypeSint32;
/// `sint64`
#[derive(Copy, Clone)]
pub struct ProtobufTypeSint64;
/// `fixed32`
#[derive(Copy, Clone)]
pub struct ProtobufTypeFixed32;
/// `fixed64`
#[derive(Copy, Clone)]
pub struct ProtobufTypeFixed64;
/// `sfixed32`
#[derive(Copy, Clone)]
pub struct ProtobufTypeSfixed32;
/// `sfixed64`
#[derive(Copy, Clone)]
pub struct ProtobufTypeSfixed64;
/// `bool`
#[derive(Copy, Clone)]
pub struct ProtobufTypeBool;
/// `string`
#[derive(Copy, Clone)]
pub struct ProtobufTypeString;
/// `bytes`
#[derive(Copy, Clone)]
pub struct ProtobufTypeBytes;

/// `bytes` as [`Bytes`](bytes::Bytes)
#[cfg(feature = "bytes")]
#[derive(Copy, Clone)]
pub struct ProtobufTypeTokioBytes;
/// `string` as [`Chars`](crate::Chars)
#[cfg(feature = "bytes")]
#[derive(Copy, Clone)]
pub struct ProtobufTypeTokioChars;

/// `enum` as `ProtobufEnumOrUnknown`
#[derive(Copy, Clone)]
pub struct ProtobufTypeEnumOrUnknown<E: Enum>(marker::PhantomData<E>);
/// `message`
#[derive(Copy, Clone)]
pub struct ProtobufTypeMessage<M: Message>(marker::PhantomData<M>);

impl ProtobufType for ProtobufTypeFloat {
    type ProtobufValue = f32;

    const WIRE_TYPE: WireType = WireType::Fixed32;

    fn read(is: &mut CodedInputStream) -> Result<f32> {
        is.read_float()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<f32> {
        unknown_values
            .fixed32
            .iter()
            .rev()
            .next()
            .map(|&bits| f32::from_bits(bits))
    }

    fn compute_size(_value: &f32) -> u64 {
        Self::ENCODED_SIZE as u64
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &f32,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_float(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeFloat {
    const ENCODED_SIZE: u32 = 4;
}

impl ProtobufType for ProtobufTypeDouble {
    type ProtobufValue = f64;

    const WIRE_TYPE: WireType = WireType::Fixed64;

    fn read(is: &mut CodedInputStream) -> Result<f64> {
        is.read_double()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<f64> {
        unknown_values
            .fixed64
            .iter()
            .rev()
            .next()
            .map(|&bits| f64::from_bits(bits))
    }

    fn compute_size(_value: &f64) -> u64 {
        Self::ENCODED_SIZE as u64
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &f64,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_double(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeDouble {
    const ENCODED_SIZE: u32 = 8;
}

impl ProtobufType for ProtobufTypeInt32 {
    type ProtobufValue = i32;

    const WIRE_TYPE: WireType = WireType::Varint;

    fn read(is: &mut CodedInputStream) -> Result<i32> {
        is.read_int32()
    }

    fn compute_size(value: &i32) -> u64 {
        // See also: https://github.com/protocolbuffers/protobuf/blob/bd00671b924310c0353a730bf8fa77c44e0a9c72/src/google/protobuf/io/coded_stream.h#L1300-L1306
        if *value < 0 {
            return 10;
        }
        rt::compute_raw_varint32_size(*value as u32)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i32,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_int32(field_number, *value)
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i32> {
        unknown_values.varint.iter().rev().next().map(|&v| v as i32)
    }
}

impl ProtobufType for ProtobufTypeInt64 {
    type ProtobufValue = i64;

    const WIRE_TYPE: WireType = WireType::Varint;

    fn read(is: &mut CodedInputStream) -> Result<i64> {
        is.read_int64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i64> {
        unknown_values.varint.iter().rev().next().map(|&v| v as i64)
    }

    fn compute_size(value: &i64) -> u64 {
        rt::compute_raw_varint64_size(*value as u64)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i64,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_int64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeUint32 {
    type ProtobufValue = u32;

    const WIRE_TYPE: WireType = WireType::Varint;

    fn read(is: &mut CodedInputStream) -> Result<u32> {
        is.read_uint32()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<u32> {
        unknown_values.varint.iter().rev().next().map(|&v| v as u32)
    }

    fn compute_size(value: &u32) -> u64 {
        rt::compute_raw_varint32_size(*value)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &u32,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_uint32(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeUint64 {
    type ProtobufValue = u64;

    const WIRE_TYPE: WireType = WireType::Varint;

    fn read(is: &mut CodedInputStream) -> Result<u64> {
        is.read_uint64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<u64> {
        unknown_values.varint.iter().cloned().rev().next()
    }

    fn compute_size(value: &u64) -> u64 {
        rt::compute_raw_varint64_size(*value)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &u64,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_uint64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeSint32 {
    type ProtobufValue = i32;

    const WIRE_TYPE: WireType = WireType::Varint;

    fn read(is: &mut CodedInputStream) -> Result<i32> {
        is.read_sint32()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i32> {
        ProtobufTypeUint32::get_from_unknown(unknown_values).map(decode_zig_zag_32)
    }

    fn compute_size(value: &i32) -> u64 {
        rt::value_varint_zigzag_size_no_tag(*value)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i32,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_sint32(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeSint64 {
    type ProtobufValue = i64;

    const WIRE_TYPE: WireType = WireType::Varint;

    fn read(is: &mut CodedInputStream) -> Result<i64> {
        is.read_sint64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i64> {
        ProtobufTypeUint64::get_from_unknown(unknown_values).map(decode_zig_zag_64)
    }

    fn compute_size(value: &i64) -> u64 {
        rt::value_varint_zigzag_size_no_tag(*value)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i64,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_sint64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeFixed32 {
    type ProtobufValue = u32;

    const WIRE_TYPE: WireType = WireType::Fixed32;

    fn read(is: &mut CodedInputStream) -> Result<u32> {
        is.read_fixed32()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<u32> {
        unknown_values.fixed32.iter().cloned().rev().next()
    }

    fn compute_size(_value: &u32) -> u64 {
        Self::ENCODED_SIZE as u64
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &u32,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_fixed32(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeFixed32 {
    const ENCODED_SIZE: u32 = 4;
}

impl ProtobufType for ProtobufTypeFixed64 {
    type ProtobufValue = u64;

    const WIRE_TYPE: WireType = WireType::Fixed64;

    fn read(is: &mut CodedInputStream) -> Result<u64> {
        is.read_fixed64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<u64> {
        unknown_values.fixed64.iter().cloned().rev().next()
    }

    fn compute_size(_value: &u64) -> u64 {
        Self::ENCODED_SIZE as u64
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &u64,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_fixed64(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeFixed64 {
    const ENCODED_SIZE: u32 = 8;
}

impl ProtobufType for ProtobufTypeSfixed32 {
    type ProtobufValue = i32;

    const WIRE_TYPE: WireType = WireType::Fixed32;

    fn read(is: &mut CodedInputStream) -> Result<i32> {
        is.read_sfixed32()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i32> {
        ProtobufTypeFixed32::get_from_unknown(unknown_values).map(|u| u as i32)
    }

    fn compute_size(_value: &i32) -> u64 {
        Self::ENCODED_SIZE as u64
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i32,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_sfixed32(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeSfixed32 {
    const ENCODED_SIZE: u32 = 4;
}

impl ProtobufType for ProtobufTypeSfixed64 {
    type ProtobufValue = i64;

    const WIRE_TYPE: WireType = WireType::Fixed64;

    fn read(is: &mut CodedInputStream) -> Result<i64> {
        is.read_sfixed64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i64> {
        ProtobufTypeFixed64::get_from_unknown(unknown_values).map(|u| u as i64)
    }

    fn compute_size(_value: &i64) -> u64 {
        8
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i64,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_sfixed64(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeSfixed64 {
    const ENCODED_SIZE: u32 = 8;
}

impl ProtobufType for ProtobufTypeBool {
    type ProtobufValue = bool;

    const WIRE_TYPE: WireType = WireType::Varint;

    fn read(is: &mut CodedInputStream) -> Result<bool> {
        is.read_bool()
    }

    fn get_from_unknown(unknown: &UnknownValues) -> Option<bool> {
        unknown.varint.iter().rev().next().map(|&v| v != 0)
    }

    fn compute_size(_value: &bool) -> u64 {
        1
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &bool,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_bool(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeString {
    type ProtobufValue = String;

    const WIRE_TYPE: WireType = WireType::LengthDelimited;

    fn read(is: &mut CodedInputStream) -> Result<String> {
        is.read_string()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<String> {
        // TODO: should not panic
        ProtobufTypeBytes::get_from_unknown(unknown_values)
            .map(|b| String::from_utf8(b).expect("not a valid string"))
    }

    fn compute_size(value: &String) -> u64 {
        value.len() as u64
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &String,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_string(field_number, &value)
    }
}

impl ProtobufType for ProtobufTypeBytes {
    type ProtobufValue = Vec<u8>;

    const WIRE_TYPE: WireType = WireType::LengthDelimited;

    fn read(is: &mut CodedInputStream) -> Result<Vec<u8>> {
        is.read_bytes()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<Vec<u8>> {
        unknown_values.length_delimited.iter().cloned().rev().next()
    }

    fn compute_size(value: &Vec<u8>) -> u64 {
        value.len() as u64
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &Vec<u8>,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_bytes(field_number, &value)
    }
}

#[cfg(feature = "bytes")]
impl ProtobufType for ProtobufTypeTokioBytes {
    type ProtobufValue = bytes::Bytes;

    const WIRE_TYPE: WireType = ProtobufTypeBytes::WIRE_TYPE;

    fn read(is: &mut CodedInputStream) -> Result<Bytes> {
        is.read_tokio_bytes()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<Bytes> {
        ProtobufTypeBytes::get_from_unknown(unknown_values).map(Bytes::from)
    }

    fn compute_size(value: &Bytes) -> u64 {
        value.len() as u64
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &Bytes,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_bytes(field_number, &value)
    }
}

#[cfg(feature = "bytes")]
impl ProtobufType for ProtobufTypeTokioChars {
    type ProtobufValue = Chars;

    const WIRE_TYPE: WireType = ProtobufTypeBytes::WIRE_TYPE;

    fn read(is: &mut CodedInputStream) -> Result<Chars> {
        is.read_tokio_chars()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<Chars> {
        ProtobufTypeString::get_from_unknown(unknown_values).map(Chars::from)
    }

    fn compute_size(value: &Chars) -> u64 {
        value.len() as u64
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &Chars,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_string(field_number, &value)
    }
}

impl<E: Enum> ProtobufType for ProtobufTypeEnumOrUnknown<E> {
    type ProtobufValue = EnumOrUnknown<E>;

    const WIRE_TYPE: WireType = WireType::Varint;

    fn read(is: &mut CodedInputStream) -> Result<EnumOrUnknown<E>> {
        is.read_enum_or_unknown()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<EnumOrUnknown<E>> {
        ProtobufTypeInt32::get_from_unknown(unknown_values).map(|i| EnumOrUnknown::from_i32(i))
    }

    fn compute_size(value: &EnumOrUnknown<E>) -> u64 {
        rt::compute_raw_varint32_size(value.value() as u32) // TODO: wrap
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &EnumOrUnknown<E>,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_enum_or_unknown(field_number, *value)
    }
}

impl<M: Message + Clone + Default> ProtobufType for ProtobufTypeMessage<M> {
    type ProtobufValue = M;

    const WIRE_TYPE: WireType = WireType::LengthDelimited;

    fn read(is: &mut CodedInputStream) -> Result<M> {
        is.read_message()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<M> {
        // TODO: do not panic
        unknown_values
            .length_delimited
            .iter()
            .rev()
            .next()
            .map(|bytes| M::parse_from_bytes(bytes).expect("cannot parse message"))
    }

    fn compute_size(value: &M) -> u64 {
        value.compute_size()
    }

    fn get_cached_size(value: &M) -> u32 {
        value.cached_size()
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &Self::ProtobufValue,
        os: &mut CodedOutputStream,
    ) -> Result<()> {
        os.write_tag(field_number, WireType::LengthDelimited)?;
        os.write_raw_varint32(value.cached_size())?;
        value.write_to_with_cached_sizes(os)?;
        Ok(())
    }
}
