use std::marker;

use stream::CodedInputStream;
use stream::CodedOutputStream;
use error::ProtobufResult;
use core::ProtobufEnum;
use core::Message;
use core::MessageStatic;
use wire_format::WireType;
use rt;
use reflect::ProtobufValue;

pub trait ProtobufType {
    type Value : ProtobufValue + Clone + 'static;

    fn wire_type() -> WireType;

    fn read(is: &mut CodedInputStream) -> ProtobufResult<Self::Value>;

    fn compute_size(value: &Self::Value) -> u32;

    #[inline]
    fn get_cached_size(value: &Self::Value) -> u32 {
        Self::compute_size(value)
    }

    #[inline]
    fn get_map_prefix_size(_: u32, _: &Self::Value) -> u32 {
        1
    }

    fn write_with_cached_size(field_number: u32, value: &Self::Value, os: &mut CodedOutputStream) -> ProtobufResult<()>;
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

    fn read(is: &mut CodedInputStream) -> ProtobufResult<f32> {
        is.read_float()
    }

    fn compute_size(_value: &f32) -> u32 {
        4
    }

    fn write_with_cached_size(field_number: u32, value: &f32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_float(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeDouble {
    type Value = f64;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed64
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<f64> {
        is.read_double()
    }

    fn compute_size(_value: &f64) -> u32 {
        8
    }

    fn write_with_cached_size(field_number: u32, value: &f64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_double(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeInt32 {
    type Value = i32;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i32> {
        is.read_int32()
    }

    fn compute_size(value: &i32) -> u32 {
        rt::compute_raw_varint32_size(*value as u32)
    }

    fn write_with_cached_size(field_number: u32, value: &i32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_int32(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeInt64 {
    type Value = i64;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i64> {
        is.read_int64()
    }

    fn compute_size(value: &i64) -> u32 {
        rt::compute_raw_varint64_size(*value as u64)
    }

    fn write_with_cached_size(field_number: u32, value: &i64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_int64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeUint32 {
    type Value = u32;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<u32> {
        is.read_uint32()
    }

    fn compute_size(value: &u32) -> u32 {
        rt::compute_raw_varint32_size(*value)
    }

    fn write_with_cached_size(field_number: u32, value: &u32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_uint32(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeUint64 {
    type Value = u64;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<u64> {
        is.read_uint64()
    }

    fn compute_size(value: &u64) -> u32 {
        rt::compute_raw_varint64_size(*value)
    }

    fn write_with_cached_size(field_number: u32, value: &u64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_uint64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeSint32 {
    type Value = i32;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i32> {
        is.read_sint32()
    }

    fn compute_size(value: &i32) -> u32 {
        rt::value_varint_zigzag_size_no_tag(*value)
    }

    fn write_with_cached_size(field_number: u32, value: &i32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_sint32(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeSint64 {
    type Value = i64;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i64> {
        is.read_sint64()
    }

    fn compute_size(value: &i64) -> u32 {
        rt::value_varint_zigzag_size_no_tag(*value)
    }

    fn write_with_cached_size(field_number: u32, value: &i64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_sint64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeFixed32 {
    type Value = u32;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed32
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<u32> {
        is.read_fixed32()
    }

    fn compute_size(_value: &u32) -> u32 {
        4
    }

    fn write_with_cached_size(field_number: u32, value: &u32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_fixed32(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeFixed64 {
    type Value = u64;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed64
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<u64> {
        is.read_fixed64()
    }

    fn compute_size(_value: &u64) -> u32 {
        8
    }

    fn write_with_cached_size(field_number: u32, value: &u64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_fixed64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeSfixed32 {
    type Value = i32;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed32
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i32> {
        is.read_sfixed32()
    }

    fn compute_size(_value: &i32) -> u32 {
        4
    }

    fn write_with_cached_size(field_number: u32, value: &i32, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_sfixed32(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeSfixed64 {
    type Value = i64;

    fn wire_type() -> WireType {
        WireType::WireTypeFixed64
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<i64> {
        is.read_sfixed64()
    }

    fn compute_size(_value: &i64) -> u32 {
        8
    }

    fn write_with_cached_size(field_number: u32, value: &i64, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_sfixed64(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeBool {
    type Value = bool;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<bool> {
        is.read_bool()
    }

    fn compute_size(_value: &bool) -> u32 {
        1
    }

    fn write_with_cached_size(field_number: u32, value: &bool, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_bool(field_number, *value)
    }
}

impl ProtobufType for ProtobufTypeString {
    type Value = String;

    fn wire_type() -> WireType {
        WireType::WireTypeLengthDelimited
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<String> {
        is.read_string()
    }

    fn compute_size(value: &String) -> u32 {
        rt::string_size_no_tag(&value)
    }

    fn write_with_cached_size(field_number: u32, value: &String, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_string(field_number, &value)
    }
}

impl ProtobufType for ProtobufTypeBytes {
    type Value = Vec<u8>;

    fn wire_type() -> WireType {
        WireType::WireTypeLengthDelimited
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<Vec<u8>> {
        is.read_bytes()
    }

    fn compute_size(value: &Vec<u8>) -> u32 {
        rt::bytes_size_no_tag(&value)
    }

    fn write_with_cached_size(field_number: u32, value: &Vec<u8>, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_bytes(field_number, &value)
    }
}

impl<E : ProtobufEnum + ProtobufValue> ProtobufType for ProtobufTypeEnum<E> {
    type Value = E;

    fn wire_type() -> WireType {
        WireType::WireTypeVarint
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<E> {
        is.read_enum()
    }

    fn compute_size(value: &E) -> u32 {
        rt::compute_raw_varint32_size(value.value() as u32) // TODO: wrap
    }

    fn write_with_cached_size(field_number: u32, value: &E, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        os.write_enum_obj(field_number, *value)
    }
}

impl<M : Message + MessageStatic + ProtobufValue> ProtobufType for ProtobufTypeMessage<M> {
    type Value = M;

    fn wire_type() -> WireType {
        WireType::WireTypeLengthDelimited
    }

    fn read(is: &mut CodedInputStream) -> ProtobufResult<M> {
        is.read_message()
    }

    fn compute_size(value: &M) -> u32 {
        value.compute_size()
    }

    fn get_cached_size(value: &M) -> u32 {
        value.get_cached_size()
    }

    fn get_map_prefix_size(field_number: u32, value: &M) -> u32 {
        rt::tag_size(field_number) +
            rt::compute_raw_varint32_size(value.compute_size())
    }

    fn write_with_cached_size(field_number: u32, value: &Self::Value, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        try!(os.write_tag(field_number, WireType::WireTypeLengthDelimited));
        try!(os.write_raw_varint32(value.get_cached_size()));
        try!(value.write_to_with_cached_sizes(os));
        Ok(())
    }
}
