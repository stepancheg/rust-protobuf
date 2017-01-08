use std::marker;

use stream::{CodedInputStream, CodedOutputStream, InputSource};
use error::ProtobufResult;
use core::{ProtobufEnum, Message, MessageStatic, CodedMessage};
use wire_format::WireType;
use rt;
use reflect::ProtobufValue;

pub trait ProtobufType {
    type Value : ProtobufValue + Clone + 'static;

    fn wire_type() -> WireType;

    fn compute_size(value: &Self::Value) -> u32;

    /// Compute size adding length prefix if wire type is length delimited
    /// (i. e. string, bytes, message)
    fn compute_size_with_length_delimiter(value: &Self::Value) -> u32 {
        let size = Self::compute_size(value);
        if Self::wire_type() == WireType::WireTypeLengthDelimited {
            rt::compute_raw_varint32_size(size) + size
        } else {
            size
        }
    }

    /// Get previously computed size
    #[inline]
    fn get_cached_size(value: &Self::Value) -> u32 {
        Self::compute_size(value)
    }

    /// Get previously cached size with length prefix
    #[inline]
    fn get_cached_size_with_length_delimiter(value: &Self::Value) -> u32 {
        let size = Self::get_cached_size(value);
        if Self::wire_type() == WireType::WireTypeLengthDelimited {
            rt::compute_raw_varint32_size(size) + size
        } else {
            size
        }
    }

    fn write_with_cached_size(field_number: u32, value: &Self::Value, os: &mut CodedOutputStream) -> ProtobufResult<()>;
}

pub trait CodedProtobufType: ProtobufType {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<Self::Value>;
}


pub struct ProtobufTypeFloat;
pub struct ProtobufTypeDouble;
pub struct ProtobufTypeInt32;
pub struct ProtobufTypeInt64;
pub struct ProtobufTypeUint32;
pub struct ProtobufTypeUint64;
pub struct ProtobufTypeSint32;
pub struct ProtobufTypeSint64;
pub struct ProtobufTypeFixed32;
pub struct ProtobufTypeFixed64;
pub struct ProtobufTypeSfixed32;
pub struct ProtobufTypeSfixed64;
pub struct ProtobufTypeBool;
pub struct ProtobufTypeString;
pub struct ProtobufTypeBytes;
pub struct ProtobufTypeEnum<E : ProtobufEnum>(marker::PhantomData<E>);
pub struct ProtobufTypeMessage<M : Message>(marker::PhantomData<M>);

impl ProtobufType for ProtobufTypeFloat {
    type Value = f32;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed32
    }

    fn compute_size(_value: &f32) -> u32 {
        4
    }

    fn write_with_cached_size(field_number: u32, value: &f32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_float(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeFloat {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<f32> {
        is.read_float()
    }
}

impl ProtobufType for ProtobufTypeDouble {
    type Value = f64;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed64
    }

    fn compute_size(_value: &f64) -> u32 {
        8
    }

    fn write_with_cached_size(field_number: u32, value: &f64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_double(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeDouble {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<f64> {
        is.read_double()
    }
}

impl ProtobufType for ProtobufTypeInt32 {
    type Value = i32;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn compute_size(value: &i32) -> u32 {
        rt::compute_raw_varint32_size(*value as u32)
    }

    fn write_with_cached_size(field_number: u32, value: &i32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_int32(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeInt32 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<i32> {
        is.read_int32()
    }
}

impl ProtobufType for ProtobufTypeInt64 {
    type Value = i64;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn compute_size(value: &i64) -> u32 {
        rt::compute_raw_varint64_size(*value as u64)
    }

    fn write_with_cached_size(field_number: u32, value: &i64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_int64(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeInt64 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<i64> {
        is.read_int64()
    }
}

impl ProtobufType for ProtobufTypeUint32 {
    type Value = u32;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn compute_size(value: &u32) -> u32 {
        rt::compute_raw_varint32_size(*value)
    }

    fn write_with_cached_size(field_number: u32, value: &u32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_uint32(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeUint32 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<u32> {
        is.read_uint32()
    }
}

impl ProtobufType for ProtobufTypeUint64 {
    type Value = u64;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn compute_size(value: &u64) -> u32 {
        rt::compute_raw_varint64_size(*value)
    }

    fn write_with_cached_size(field_number: u32, value: &u64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_uint64(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeUint64 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<u64> {
        is.read_uint64()
    }
}

impl ProtobufType for ProtobufTypeSint32 {
    type Value = i32;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn compute_size(value: &i32) -> u32 {
        rt::value_varint_zigzag_size_no_tag(*value)
    }

    fn write_with_cached_size(field_number: u32, value: &i32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_sint32(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeSint32 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<i32> {
        is.read_sint32()
    }
}

impl ProtobufType for ProtobufTypeSint64 {
    type Value = i64;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn compute_size(value: &i64) -> u32 {
        rt::value_varint_zigzag_size_no_tag(*value)
    }

    fn write_with_cached_size(field_number: u32, value: &i64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_sint64(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeSint64 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<i64> {
        is.read_sint64()
    }
}

impl ProtobufType for ProtobufTypeFixed32 {
    type Value = u32;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed32
    }

    fn compute_size(_value: &u32) -> u32 {
        4
    }

    fn write_with_cached_size(field_number: u32, value: &u32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_fixed32(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeFixed32 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<u32> {
        is.read_fixed32()
    }
}

impl ProtobufType for ProtobufTypeFixed64 {
    type Value = u64;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed64
    }

    fn compute_size(_value: &u64) -> u32 {
        8
    }

    fn write_with_cached_size(field_number: u32, value: &u64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_fixed64(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeFixed64 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<u64> {
        is.read_fixed64()
    }
}

impl ProtobufType for ProtobufTypeSfixed32 {
    type Value = i32;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed32
    }

    fn compute_size(_value: &i32) -> u32 {
        4
    }

    fn write_with_cached_size(field_number: u32, value: &i32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_sfixed32(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeSfixed32 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<i32> {
        is.read_sfixed32()
    }
}

impl ProtobufType for ProtobufTypeSfixed64 {
    type Value = i64;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed64
    }

    fn compute_size(_value: &i64) -> u32 {
        8
    }

    fn write_with_cached_size(field_number: u32, value: &i64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_sfixed64(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeSfixed64 {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<i64> {
        is.read_sfixed64()
    }
}

impl ProtobufType for ProtobufTypeBool {
    type Value = bool;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn compute_size(_value: &bool) -> u32 {
        1
    }

    fn write_with_cached_size(field_number: u32, value: &bool, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_bool(field_number, *value)
    }
}

impl CodedProtobufType for ProtobufTypeBool {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<bool> {
        is.read_bool()
    }
}

impl ProtobufType for ProtobufTypeString {
    type Value = String;

    fn wire_type() -> WireType {
        WireType::WireTypeLengthDelimited
    }

    fn compute_size(value: &String) -> u32 {
        value.len() as u32
    }

    fn write_with_cached_size(field_number: u32, value: &String, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_string(field_number, &value)
    }
}

impl CodedProtobufType for ProtobufTypeString {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<String> {
        is.read_string()
    }
}

impl ProtobufType for ProtobufTypeBytes {
    type Value = Vec<u8>;

    fn wire_type() -> WireType {
        WireType::WireTypeLengthDelimited
    }

    fn compute_size(value: &Vec<u8>) -> u32 {
        value.len() as u32
    }

    fn write_with_cached_size(field_number: u32, value: &Vec<u8>, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_bytes(field_number, &value)
    }
}

impl CodedProtobufType for ProtobufTypeBytes {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<Vec<u8>> {
        is.read_bytes()
    }
}

impl<E : ProtobufEnum + ProtobufValue> ProtobufType for ProtobufTypeEnum<E> {
    type Value = E;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn compute_size(value: &E) -> u32 {
        rt::compute_raw_varint32_size(value.value() as u32) // TODO: wrap
    }

    fn write_with_cached_size(field_number: u32, value: &E, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_enum_obj(field_number, *value)
    }
}

impl<E: ProtobufEnum + ProtobufValue> CodedProtobufType for ProtobufTypeEnum<E> {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<E> {
        is.read_enum()
    }
}

impl<M : Message + MessageStatic + ProtobufValue> ProtobufType for ProtobufTypeMessage<M> {
    type Value = M;

    fn wire_type() -> WireType {
        WireType::WireTypeLengthDelimited
    }

    fn compute_size(value: &M) -> u32 {
        value.compute_size()
    }

    fn get_cached_size(value: &M) -> u32 {
        value.get_cached_size()
    }

    fn write_with_cached_size(field_number: u32, value: &Self::Value, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        try!(os.write_tag(field_number, WireType::WireTypeLengthDelimited));
        try!(os.write_raw_varint32(value.get_cached_size()));
        try!(value.write_to_with_cached_sizes(os));
        Ok(())
    }
}

impl<M : CodedMessage + MessageStatic + ProtobufValue> CodedProtobufType for ProtobufTypeMessage<M> {
    fn read<I: InputSource>(is: &mut CodedInputStream<I>) -> ProtobufResult<M> {
        is.read_message()
    }
}
