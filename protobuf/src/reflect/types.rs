use std::fmt;
use std::marker;

#[cfg(feature = "bytes")]
use crate::bytes::Bytes;
#[cfg(feature = "bytes")]
use crate::chars::Chars;

use crate::core::Message;
use crate::enums::ProtobufEnum;
use crate::error::ProtobufResult;
use crate::reflect::runtime_types::{RuntimeType, RuntimeTypeEnumOrUnknown};
use crate::reflect::runtime_types::RuntimeTypeBool;
#[cfg(feature = "bytes")]
use crate::reflect::runtime_types::RuntimeTypeCarllercheBytes;
#[cfg(feature = "bytes")]
use crate::reflect::runtime_types::RuntimeTypeCarllercheChars;
use crate::reflect::runtime_types::RuntimeTypeEnum;
use crate::reflect::runtime_types::RuntimeTypeF32;
use crate::reflect::runtime_types::RuntimeTypeF64;
use crate::reflect::runtime_types::RuntimeTypeI32;
use crate::reflect::runtime_types::RuntimeTypeI64;
use crate::reflect::runtime_types::RuntimeTypeMessage;
use crate::reflect::runtime_types::RuntimeTypeString;
use crate::reflect::runtime_types::RuntimeTypeU32;
use crate::reflect::runtime_types::RuntimeTypeU64;
use crate::reflect::runtime_types::RuntimeTypeUnreachable;
use crate::reflect::runtime_types::RuntimeTypeVecU8;
use crate::reflect::type_dynamic::ProtobufTypeDynamic;
use crate::reflect::type_dynamic::ProtobufTypeDynamicImpl;
use crate::reflect::ProtobufValue;
use crate::rt;
use crate::enums::ProtobufEnumOrUnknown;
use crate::stream::CodedInputStream;
use crate::stream::CodedOutputStream;
use crate::unknown::UnknownValues;
use crate::wire_format::WireType;
use crate::zigzag::decode_zig_zag_32;
use crate::zigzag::decode_zig_zag_64;
use crate::core::parse_from_bytes;

pub trait ProtobufType: Send + Sync + Clone + 'static {
    type RuntimeType: RuntimeType;

    fn dynamic() -> &'static ProtobufTypeDynamic
    where
        Self: Sized,
    {
        &ProtobufTypeDynamicImpl::<Self>(marker::PhantomData)
    }

    fn wire_type() -> WireType;

    fn read(is: &mut CodedInputStream)
        -> ProtobufResult<<Self::RuntimeType as RuntimeType>::Value>;

    fn get_from_unknown(
        _unknown_values: &UnknownValues,
    ) -> Option<<Self::RuntimeType as RuntimeType>::Value>;

    fn compute_size(value: &<Self::RuntimeType as RuntimeType>::Value) -> u32;

    /// Compute size adding length prefix if wire type is length delimited
    /// (i. e. string, bytes, message)
    fn compute_size_with_length_delimiter(
        value: &<Self::RuntimeType as RuntimeType>::Value,
    ) -> u32 {
        let size = Self::compute_size(value);
        if Self::wire_type() == WireType::WireTypeLengthDelimited {
            rt::compute_raw_varint32_size(size) + size
        } else {
            size
        }
    }

    /// Get previously computed size
    #[inline]
    fn get_cached_size(value: &<Self::RuntimeType as RuntimeType>::Value) -> u32 {
        Self::compute_size(value)
    }

    /// Get previously cached size with length prefix
    #[inline]
    fn get_cached_size_with_length_delimiter(
        value: &<Self::RuntimeType as RuntimeType>::Value,
    ) -> u32 {
        let size = Self::get_cached_size(value);
        if Self::wire_type() == WireType::WireTypeLengthDelimited {
            rt::compute_raw_varint32_size(size) + size
        } else {
            size
        }
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &<Self::RuntimeType as RuntimeType>::Value,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()>;
}

/// All fixed size types
pub trait ProtobufTypeFixed: ProtobufType {
    fn encoded_size() -> u32;
}

#[derive(Copy, Clone)]
pub struct ProtobufTypeFloat;
#[derive(Copy, Clone)]
pub struct ProtobufTypeDouble;
#[derive(Copy, Clone)]
pub struct ProtobufTypeInt32;
#[derive(Copy, Clone)]
pub struct ProtobufTypeInt64;
#[derive(Copy, Clone)]
pub struct ProtobufTypeUint32;
#[derive(Copy, Clone)]
pub struct ProtobufTypeUint64;
#[derive(Copy, Clone)]
pub struct ProtobufTypeSint32;
#[derive(Copy, Clone)]
pub struct ProtobufTypeSint64;
#[derive(Copy, Clone)]
pub struct ProtobufTypeFixed32;
#[derive(Copy, Clone)]
pub struct ProtobufTypeFixed64;
#[derive(Copy, Clone)]
pub struct ProtobufTypeSfixed32;
#[derive(Copy, Clone)]
pub struct ProtobufTypeSfixed64;
#[derive(Copy, Clone)]
pub struct ProtobufTypeBool;
#[derive(Copy, Clone)]
pub struct ProtobufTypeString;
#[derive(Copy, Clone)]
pub struct ProtobufTypeBytes;
#[derive(Copy, Clone)]
pub struct ProtobufTypeChars;

#[cfg(feature = "bytes")]
#[derive(Copy, Clone)]
pub struct ProtobufTypeCarllercheBytes;
#[cfg(feature = "bytes")]
#[derive(Copy, Clone)]
pub struct ProtobufTypeCarllercheChars;

#[derive(Copy, Clone)]
pub struct ProtobufTypeEnum<E: ProtobufEnum>(marker::PhantomData<E>);
#[derive(Copy, Clone)]
pub struct ProtobufTypeEnumOrUnknown<E: ProtobufEnum>(marker::PhantomData<E>);
#[derive(Copy, Clone)]
pub struct ProtobufTypeMessage<M: Message>(marker::PhantomData<M>);

#[derive(Copy, Clone)]
pub struct ProtobufTypeUnreachable;

impl ProtobufType for ProtobufTypeFloat {
    type RuntimeType = RuntimeTypeF32;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed32
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<f32> {
        is.read_float()
    }

    fn compute_size(_value: &f32) -> u32 {
        Self::encoded_size()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<f32> {
        unknown_values.fixed32.iter().rev().next()
            .map(|&bits| f32::from_bits(bits) )
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &f32,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_float(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeFloat {
    fn encoded_size() -> u32 {
        4
    }
}

impl ProtobufType for ProtobufTypeDouble {
    type RuntimeType = RuntimeTypeF64;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed64
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<f64> {
        is.read_double()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<f64> {
        unknown_values.fixed64.iter().rev().next()
            .map(|&bits| f64::from_bits(bits))
    }

    fn compute_size(_value: &f64) -> u32 {
        Self::encoded_size()
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &f64,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_double(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeDouble {
    fn encoded_size() -> u32 {
        8
    }
}

impl ProtobufType for ProtobufTypeInt32 {
    type RuntimeType = RuntimeTypeI32;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i32> {
        is.read_int32()
    }

    fn compute_size(value: &i32) -> u32 {
        // See also: https://github.com/protocolbuffers/protobuf/blob/bd00671b924310c0353a730bf8fa77c44e0a9c72/src/google/protobuf/io/coded_stream.h#L1300-L1306
        if *value < 0 {
            return 10
        }
        rt::compute_raw_varint32_size(*value as u32)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i32,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_int32(field_number, *value)
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i32> {
        unknown_values.varint.iter().rev().next()
            .map(|&v| v as i32)
    }
}

impl ProtobufType for ProtobufTypeInt64 {
    type RuntimeType = RuntimeTypeI64;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i64> {
        is.read_int64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i64> {
        unknown_values.varint.iter().rev().next()
            .map(|&v| v as i64)
    }

    fn compute_size(value: &i64) -> u32 {
        rt::compute_raw_varint64_size(*value as u64)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i64,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_int64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeUint32 {
    type RuntimeType = RuntimeTypeU32;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<u32> {
        is.read_uint32()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<u32> {
        unknown_values.varint.iter().rev().next()
            .map(|&v| v as u32)
    }

    fn compute_size(value: &u32) -> u32 {
        rt::compute_raw_varint32_size(*value)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &u32,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_uint32(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeUint64 {
    type RuntimeType = RuntimeTypeU64;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<u64> {
        is.read_uint64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<u64> {
        unknown_values.varint.iter().cloned().rev().next()
    }

    fn compute_size(value: &u64) -> u32 {
        rt::compute_raw_varint64_size(*value)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &u64,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_uint64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeSint32 {
    type RuntimeType = RuntimeTypeI32;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i32> {
        is.read_sint32()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i32> {
        ProtobufTypeUint32::get_from_unknown(unknown_values)
            .map(decode_zig_zag_32)
    }

    fn compute_size(value: &i32) -> u32 {
        rt::value_varint_zigzag_size_no_tag(*value)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i32,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_sint32(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeSint64 {
    type RuntimeType = RuntimeTypeI64;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i64> {
        is.read_sint64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i64> {
        ProtobufTypeUint64::get_from_unknown(unknown_values)
            .map(decode_zig_zag_64)
    }

    fn compute_size(value: &i64) -> u32 {
        rt::value_varint_zigzag_size_no_tag(*value)
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i64,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_sint64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeFixed32 {
    type RuntimeType = RuntimeTypeU32;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed32
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<u32> {
        is.read_fixed32()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<u32> {
        unknown_values.fixed32.iter().cloned().rev().next()
    }

    fn compute_size(_value: &u32) -> u32 {
        Self::encoded_size()
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &u32,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_fixed32(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeFixed32 {
    fn encoded_size() -> u32 {
        4
    }
}

impl ProtobufType for ProtobufTypeFixed64 {
    type RuntimeType = RuntimeTypeU64;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed64
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<u64> {
        is.read_fixed64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<u64> {
        unknown_values.fixed64.iter().cloned().rev().next()
    }

    fn compute_size(_value: &u64) -> u32 {
        Self::encoded_size()
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &u64,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_fixed64(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeFixed64 {
    fn encoded_size() -> u32 {
        8
    }
}

impl ProtobufType for ProtobufTypeSfixed32 {
    type RuntimeType = RuntimeTypeI32;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed32
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i32> {
        is.read_sfixed32()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i32> {
        ProtobufTypeFixed32::get_from_unknown(unknown_values).map(|u| u as i32)
    }

    fn compute_size(_value: &i32) -> u32 {
        Self::encoded_size()
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i32,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_sfixed32(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeSfixed32 {
    fn encoded_size() -> u32 {
        4
    }
}

impl ProtobufType for ProtobufTypeSfixed64 {
    type RuntimeType = RuntimeTypeI64;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed64
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i64> {
        is.read_sfixed64()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<i64> {
        ProtobufTypeFixed64::get_from_unknown(unknown_values).map(|u| u as i64)
    }

    fn compute_size(_value: &i64) -> u32 {
        8
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &i64,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_sfixed64(field_number, *value)
    }
}

impl ProtobufTypeFixed for ProtobufTypeSfixed64 {
    fn encoded_size() -> u32 {
        8
    }
}

impl ProtobufType for ProtobufTypeBool {
    type RuntimeType = RuntimeTypeBool;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<bool> {
        is.read_bool()
    }

    fn get_from_unknown(unknown: &UnknownValues) -> Option<bool> {
        unknown.varint.iter().rev().next().map(|&v| v != 0)
    }

    fn compute_size(_value: &bool) -> u32 {
        1
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &bool,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_bool(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeString {
    type RuntimeType = RuntimeTypeString;

    fn wire_type() -> WireType {
        WireType::WireTypeLengthDelimited
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<String> {
        is.read_string()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<String> {
        // TODO: should not panic
        ProtobufTypeBytes::get_from_unknown(unknown_values)
            .map(|b| String::from_utf8(b).expect("not a valid string"))
    }

    fn compute_size(value: &String) -> u32 {
        value.len() as u32
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &String,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_string(field_number, &value)
    }
}

impl ProtobufType for ProtobufTypeBytes {
    type RuntimeType = RuntimeTypeVecU8;

    fn wire_type() -> WireType {
        WireType::WireTypeLengthDelimited
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<Vec<u8>> {
        is.read_bytes()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<Vec<u8>> {
        unknown_values.length_delimited.iter().cloned().rev().next()
    }

    fn compute_size(value: &Vec<u8>) -> u32 {
        value.len() as u32
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &Vec<u8>,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_bytes(field_number, &value)
    }
}

#[cfg(feature = "bytes")]
impl ProtobufType for ProtobufTypeCarllercheBytes {
    type RuntimeType = RuntimeTypeCarllercheBytes;

    fn wire_type() -> WireType {
        ProtobufTypeBytes::wire_type()
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<Bytes> {
        is.read_carllerche_bytes()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<Bytes> {
        ProtobufTypeBytes::get_from_unknown(unknown_values)
            .map(Bytes::from)
    }

    fn compute_size(value: &Bytes) -> u32 {
        value.len() as u32
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &Bytes,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_bytes(field_number, &value)
    }
}

#[cfg(feature = "bytes")]
impl ProtobufType for ProtobufTypeCarllercheChars {
    type RuntimeType = RuntimeTypeCarllercheChars;

    fn wire_type() -> WireType {
        ProtobufTypeBytes::wire_type()
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<Chars> {
        is.read_carllerche_chars()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<Chars> {
        ProtobufTypeString::get_from_unknown(unknown_values)
            .map(Chars::from)
    }

    fn compute_size(value: &Chars) -> u32 {
        value.len() as u32
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &Chars,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_string(field_number, &value)
    }
}

impl<E: ProtobufEnum + ProtobufValue + fmt::Debug> ProtobufType for ProtobufTypeEnum<E> {
    type RuntimeType = RuntimeTypeEnum<E>;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<E> {
        is.read_enum()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<E> {
        // TODO: do not panic
        ProtobufTypeInt32::get_from_unknown(unknown_values)
            .map(|i| E::from_i32(i).expect("not a valid enum value"))
    }

    fn compute_size(value: &E) -> u32 {
        rt::compute_raw_varint32_size(value.value() as u32) // TODO: wrap
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &E,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_enum_obj(field_number, *value)
    }
}

impl<E: ProtobufEnum + ProtobufValue + fmt::Debug> ProtobufType for ProtobufTypeEnumOrUnknown<E> {
    type RuntimeType = RuntimeTypeEnumOrUnknown<E>;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<ProtobufEnumOrUnknown<E>> {
        is.read_enum_or_unknown()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<ProtobufEnumOrUnknown<E>> {
        ProtobufTypeInt32::get_from_unknown(unknown_values)
            .map(|i| ProtobufEnumOrUnknown::from_i32(i))
    }

    fn compute_size(value: &ProtobufEnumOrUnknown<E>) -> u32 {
        rt::compute_raw_varint32_size(value.value() as u32) // TODO: wrap
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &ProtobufEnumOrUnknown<E>,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_enum_or_unknown(field_number, *value)
    }
}

impl<M: Message + Clone + ProtobufValue + Default> ProtobufType for ProtobufTypeMessage<M> {
    type RuntimeType = RuntimeTypeMessage<M>;

    fn wire_type() -> WireType {
        WireType::WireTypeLengthDelimited
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<M> {
        is.read_message()
    }

    fn get_from_unknown(unknown_values: &UnknownValues) -> Option<M> {
        // TODO: do not panic
        unknown_values.length_delimited.iter().rev().next()
            .map(|bytes| parse_from_bytes(bytes).expect("cannot parse message"))
    }

    fn compute_size(value: &M) -> u32 {
        value.compute_size()
    }

    fn get_cached_size(value: &M) -> u32 {
        value.get_cached_size()
    }

    fn write_with_cached_size(
        field_number: u32,
        value: &<Self::RuntimeType as RuntimeType>::Value,
        os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        os.write_tag(field_number, WireType::WireTypeLengthDelimited)?;
        os.write_raw_varint32(value.get_cached_size())?;
        value.write_to_with_cached_sizes(os)?;
        Ok(())
    }
}

impl ProtobufType for ProtobufTypeUnreachable {
    type RuntimeType = RuntimeTypeUnreachable;

    fn wire_type() -> WireType {
        unreachable!()
    }

    fn read(_is: &mut CodedInputStream) -> ProtobufResult<u32> {
        unreachable!()
    }

    fn get_from_unknown(_unknown_values: &UnknownValues) -> Option<u32> {
        unreachable!()
    }

    fn compute_size(_value: &u32) -> u32 {
        unreachable!()
    }

    fn write_with_cached_size(
        _field_number: u32,
        _value: &u32,
        _os: &mut CodedOutputStream,
    ) -> ProtobufResult<()> {
        unreachable!()
    }
}
