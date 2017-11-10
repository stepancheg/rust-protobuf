use std::mem;
use std::io;
use std::io::{BufRead, Read};
use std::io::Write;
use std::slice;

#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "bytes")]
use chars::Chars;

use varint;
use misc::remaining_capacity_as_slice_mut;
use misc::remove_lifetime_mut;
use core::Message;
use core::MessageStatic;
use core::ProtobufEnum;
use unknown::UnknownFields;
use unknown::UnknownValue;
use unknown::UnknownValueRef;
use zigzag::decode_zig_zag_32;
use zigzag::decode_zig_zag_64;
use zigzag::encode_zig_zag_32;
use zigzag::encode_zig_zag_64;
use error::ProtobufResult;
use error::ProtobufError;
use error::WireError;
use buf_read_iter::BufReadIter;

// Equal to the default buffer size of `BufWriter`, so when
// `CodedOutputStream` wraps `BufWriter`, it often skips double buffering.
const OUTPUT_STREAM_BUFFER_SIZE: usize = 8 * 1024;

// Default recursion level limit. 100 is the default value of C++'s implementation.
const DEFAULT_RECURSION_LIMIT: u32 = 100;


pub mod wire_format {
    // TODO: temporary
    pub use self::WireType::*;

    pub const TAG_TYPE_BITS: u32 = 3;
    pub const TAG_TYPE_MASK: u32 = (1u32 << TAG_TYPE_BITS as usize) - 1;
    // max possible tag number
    pub const FIELD_NUMBER_MAX: u32 = 0x1fffffff;

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub enum WireType {
        WireTypeVarint = 0,
        WireTypeFixed64 = 1,
        WireTypeLengthDelimited = 2,
        WireTypeStartGroup = 3,
        WireTypeEndGroup = 4,
        WireTypeFixed32 = 5,
    }

    impl Copy for WireType {}

    impl WireType {
        pub fn new(n: u32) -> Option<WireType> {
            match n {
                0 => Some(WireTypeVarint),
                1 => Some(WireTypeFixed64),
                2 => Some(WireTypeLengthDelimited),
                3 => Some(WireTypeStartGroup),
                4 => Some(WireTypeEndGroup),
                5 => Some(WireTypeFixed32),
                _ => None,
            }
        }
    }

    #[derive(Clone)]
    pub struct Tag {
        field_number: u32,
        wire_type: WireType,
    }

    impl Copy for Tag {}

    impl Tag {
        pub fn value(self) -> u32 {
            (self.field_number << TAG_TYPE_BITS) | (self.wire_type as u32)
        }

        // TODO: should return Result instead of Option
        pub fn new(value: u32) -> Option<Tag> {
            let wire_type = WireType::new(value & TAG_TYPE_MASK);
            if wire_type.is_none() {
                return None;
            }
            let field_number = value >> TAG_TYPE_BITS;
            if field_number == 0 {
                return None;
            }
            Some(Tag {
                field_number: field_number,
                wire_type: wire_type.unwrap(),
            })
        }

        pub fn make(field_number: u32, wire_type: WireType) -> Tag {
            assert!(field_number > 0 && field_number <= FIELD_NUMBER_MAX);
            Tag {
                field_number: field_number,
                wire_type: wire_type,
            }
        }

        pub fn unpack(self) -> (u32, WireType) {
            (self.field_number(), self.wire_type())
        }

        fn wire_type(self) -> WireType {
            self.wire_type
        }

        pub fn field_number(self) -> u32 {
            self.field_number
        }
    }

}

pub struct CodedInputStream<'a> {
    source: BufReadIter<'a>,
    recursion_level: u32,
    recursion_limit: u32,
}

impl<'a> CodedInputStream<'a> {
    pub fn new(read: &'a mut Read) -> CodedInputStream<'a> {
        CodedInputStream::from_buf_read_iter(BufReadIter::from_read(read))
    }

    pub fn from_buffered_reader(buf_read: &'a mut BufRead) -> CodedInputStream<'a> {
        CodedInputStream::from_buf_read_iter(BufReadIter::from_buf_read(buf_read))
    }

    pub fn from_bytes(bytes: &'a [u8]) -> CodedInputStream<'a> {
        CodedInputStream::from_buf_read_iter(BufReadIter::from_byte_slice(bytes))
    }

    #[cfg(feature = "bytes")]
    pub fn from_carllerche_bytes(bytes: &'a Bytes) -> CodedInputStream<'a> {
        CodedInputStream::from_buf_read_iter(BufReadIter::from_bytes(bytes))
    }

    fn from_buf_read_iter(source: BufReadIter<'a>) -> CodedInputStream<'a> {
        CodedInputStream {
            source: source,
            recursion_level: 0,
            recursion_limit: DEFAULT_RECURSION_LIMIT,
        }
    }

    /// Set the recursion limit.
    pub fn set_recursion_limit(&mut self, limit: u32) {
        self.recursion_limit = limit;
    }

    #[inline]
    pub(crate) fn incr_recursion(&mut self) -> ProtobufResult<()> {
        if self.recursion_level >= self.recursion_limit {
            return Err(ProtobufError::WireError(WireError::OverRecursionLimit));
        }
        self.recursion_level += 1;
        Ok(())
    }

    #[inline]
    pub(crate) fn decr_recursion(&mut self) {
        self.recursion_level -= 1;
    }

    pub fn pos(&self) -> u64 {
        self.source.pos()
    }

    pub fn bytes_until_limit(&self) -> u64 {
        self.source.bytes_until_limit()
    }

    pub fn read(&mut self, buf: &mut [u8]) -> ProtobufResult<()> {
        self.source.read_exact(buf)?;
        Ok(())
    }

    #[cfg(feature = "bytes")]
    fn read_raw_callerche_bytes(&mut self, count: usize) -> ProtobufResult<Bytes> {
        self.source.read_exact_bytes(count)
    }

    #[inline(always)]
    pub fn read_raw_byte(&mut self) -> ProtobufResult<u8> {
        self.source.read_byte()
    }

    pub fn push_limit(&mut self, limit: u64) -> ProtobufResult<u64> {
        self.source.push_limit(limit)
    }

    pub fn pop_limit(&mut self, old_limit: u64) {
        self.source.pop_limit(old_limit);
    }

    #[inline(always)]
    pub fn eof(&mut self) -> ProtobufResult<bool> {
        self.source.eof()
    }

    pub fn check_eof(&mut self) -> ProtobufResult<()> {
        let eof = self.eof()?;
        if !eof {
            return Err(ProtobufError::WireError(WireError::UnexpectedEof));
        }
        Ok(())
    }

    fn read_raw_varint64_slow(&mut self) -> ProtobufResult<u64> {
        let mut r: u64 = 0;
        let mut i = 0;
        loop {
            if i == 10 {
                return Err(ProtobufError::WireError(WireError::IncorrectVarint));
            }
            let b = self.read_raw_byte()?;
            // TODO: may overflow if i == 9
            r = r | (((b & 0x7f) as u64) << (i * 7));
            i += 1;
            if b < 0x80 {
                return Ok(r);
            }
        }
    }

    #[inline(always)]
    pub fn read_raw_varint64(&mut self) -> ProtobufResult<u64> {
        'slow: loop {
            let ret;
            let consume;

            loop {
                let rem = self.source.remaining_in_buf();

                if rem.len() >= 1 {
                    // most varints are in practice fit in 1 byte
                    if rem[0] < 0x80 {
                        ret = rem[0] as u64;
                        consume = 1;
                    } else {
                        // handle case of two bytes too
                        if rem.len() >= 2 && rem[1] < 0x80 {
                            ret = (rem[0] & 0x7f) as u64 | (rem[1] as u64) << 7;
                            consume = 2;
                        } else if rem.len() >= 10 {
                            // Read from array when buf at at least 10 bytes,
                            // max len for varint.
                            let mut r: u64 = 0;
                            let mut i: usize = 0;
                            {
                                let rem = rem;
                                loop {
                                    if i == 10 {
                                        return Err(
                                            ProtobufError::WireError(WireError::IncorrectVarint),
                                        );
                                    }

                                    let b = if true {
                                        // skip range check
                                        unsafe { *rem.get_unchecked(i) }
                                    } else {
                                        rem[i]
                                    };

                                    // TODO: may overflow if i == 9
                                    r = r | (((b & 0x7f) as u64) << (i * 7));
                                    i += 1;
                                    if b < 0x80 {
                                        break;
                                    }
                                }
                            }
                            consume = i;
                            ret = r;
                        } else {
                            break 'slow;
                        }
                    }
                } else {
                    break 'slow;
                }
                break;
            }

            self.source.consume(consume);
            return Ok(ret);
        }

        self.read_raw_varint64_slow()
    }

    #[inline(always)]
    pub fn read_raw_varint32(&mut self) -> ProtobufResult<u32> {
        self.read_raw_varint64().map(|v| v as u32)
    }


    pub fn read_raw_little_endian32(&mut self) -> ProtobufResult<u32> {
        let mut r = 0u32;
        let bytes: &mut [u8] = unsafe {
            let p: *mut u8 = mem::transmute(&mut r);
            slice::from_raw_parts_mut(p, mem::size_of::<u32>())
        };
        self.read(bytes)?;
        Ok(r.to_le())
    }

    pub fn read_raw_little_endian64(&mut self) -> ProtobufResult<u64> {
        let mut r = 0u64;
        let bytes: &mut [u8] = unsafe {
            let p: *mut u8 = mem::transmute(&mut r);
            slice::from_raw_parts_mut(p, mem::size_of::<u64>())
        };
        self.read(bytes)?;
        Ok(r.to_le())
    }

    #[inline]
    pub fn read_tag(&mut self) -> ProtobufResult<wire_format::Tag> {
        let v = self.read_raw_varint32()?;
        match wire_format::Tag::new(v) {
            Some(tag) => Ok(tag),
            None => Err(ProtobufError::WireError(WireError::IncorrectTag(v))),
        }
    }

    // Read tag, return it is pair (field number, wire type)
    #[inline]
    pub fn read_tag_unpack(&mut self) -> ProtobufResult<(u32, wire_format::WireType)> {
        self.read_tag().map(|t| t.unpack())
    }

    pub fn read_double(&mut self) -> ProtobufResult<f64> {
        let bits = self.read_raw_little_endian64()?;
        unsafe { Ok(mem::transmute::<u64, f64>(bits)) }
    }

    pub fn read_float(&mut self) -> ProtobufResult<f32> {
        let bits = self.read_raw_little_endian32()?;
        unsafe { Ok(mem::transmute::<u32, f32>(bits)) }
    }

    pub fn read_int64(&mut self) -> ProtobufResult<i64> {
        self.read_raw_varint64().map(|v| v as i64)
    }

    pub fn read_int32(&mut self) -> ProtobufResult<i32> {
        self.read_raw_varint32().map(|v| v as i32)
    }

    pub fn read_uint64(&mut self) -> ProtobufResult<u64> {
        self.read_raw_varint64()
    }

    pub fn read_uint32(&mut self) -> ProtobufResult<u32> {
        self.read_raw_varint32()
    }

    pub fn read_sint64(&mut self) -> ProtobufResult<i64> {
        self.read_uint64().map(decode_zig_zag_64)
    }

    pub fn read_sint32(&mut self) -> ProtobufResult<i32> {
        self.read_uint32().map(decode_zig_zag_32)
    }

    pub fn read_fixed64(&mut self) -> ProtobufResult<u64> {
        self.read_raw_little_endian64()
    }

    pub fn read_fixed32(&mut self) -> ProtobufResult<u32> {
        self.read_raw_little_endian32()
    }

    pub fn read_sfixed64(&mut self) -> ProtobufResult<i64> {
        self.read_raw_little_endian64().map(|v| v as i64)
    }

    pub fn read_sfixed32(&mut self) -> ProtobufResult<i32> {
        self.read_raw_little_endian32().map(|v| v as i32)
    }

    pub fn read_bool(&mut self) -> ProtobufResult<bool> {
        self.read_raw_varint32().map(|v| v != 0)
    }

    pub fn read_enum<E : ProtobufEnum>(&mut self) -> ProtobufResult<E> {
        let i = self.read_int32()?;
        match ProtobufEnum::from_i32(i) {
            Some(e) => Ok(e),
            None => Err(ProtobufError::WireError(WireError::InvalidEnumValue(i))),
        }
    }

    pub fn read_repeated_packed_double_into(
        &mut self,
        target: &mut Vec<f64>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;

        target.reserve((len / 4) as usize);

        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_double()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_float_into(&mut self, target: &mut Vec<f32>) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;

        target.reserve((len / 4) as usize);

        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_float()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_int64_into(&mut self, target: &mut Vec<i64>) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len as u64)?;
        while !self.eof()? {
            target.push(self.read_int64()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_int32_into(&mut self, target: &mut Vec<i32>) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_int32()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_uint64_into(
        &mut self,
        target: &mut Vec<u64>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_uint64()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_uint32_into(
        &mut self,
        target: &mut Vec<u32>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_uint32()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_sint64_into(
        &mut self,
        target: &mut Vec<i64>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_sint64()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_sint32_into(
        &mut self,
        target: &mut Vec<i32>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_sint32()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_fixed64_into(
        &mut self,
        target: &mut Vec<u64>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;

        target.reserve((len / 8) as usize);

        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_fixed64()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_fixed32_into(
        &mut self,
        target: &mut Vec<u32>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;

        target.reserve((len / 4) as usize);

        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_fixed32()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_sfixed64_into(
        &mut self,
        target: &mut Vec<i64>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;

        target.reserve((len / 8) as usize);

        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_sfixed64()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_sfixed32_into(
        &mut self,
        target: &mut Vec<i32>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;

        target.reserve((len / 4) as usize);

        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_sfixed32()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_bool_into(&mut self, target: &mut Vec<bool>) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;

        // regular bool value is 1-byte size
        target.reserve(len as usize);

        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_bool()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_enum_into<E : ProtobufEnum>(
        &mut self,
        target: &mut Vec<E>,
    ) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        while !self.eof()? {
            target.push(self.read_enum()?);
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_unknown(
        &mut self,
        wire_type: wire_format::WireType,
    ) -> ProtobufResult<UnknownValue> {
        match wire_type {
            wire_format::WireTypeVarint => {
                self.read_raw_varint64().map(|v| UnknownValue::Varint(v))
            }
            wire_format::WireTypeFixed64 => self.read_fixed64().map(|v| UnknownValue::Fixed64(v)),
            wire_format::WireTypeFixed32 => self.read_fixed32().map(|v| UnknownValue::Fixed32(v)),
            wire_format::WireTypeLengthDelimited => {
                let len = self.read_raw_varint32()?;
                self.read_raw_bytes(len)
                    .map(|v| UnknownValue::LengthDelimited(v))
            }
            _ => Err(ProtobufError::WireError(
                WireError::UnexpectedWireType(wire_type),
            )),
        }
    }

    pub fn skip_field(&mut self, wire_type: wire_format::WireType) -> ProtobufResult<()> {
        self.read_unknown(wire_type).map(|_| ())
    }

    /// Read raw bytes into the supplied vector.  The vector will be resized as needed and
    /// overwritten.
    pub fn read_raw_bytes_into(&mut self, count: u32, target: &mut Vec<u8>) -> ProtobufResult<()> {
        unsafe {
            target.set_len(0);
        }
        target.reserve(count as usize);
        unsafe {
            target.set_len(count as usize);
        }
        self.read(target)?;
        Ok(())
    }

    /// Read exact number of bytes
    pub fn read_raw_bytes(&mut self, count: u32) -> ProtobufResult<Vec<u8>> {
        let mut r = Vec::new();
        self.read_raw_bytes_into(count, &mut r)?;
        Ok(r)
    }

    pub fn skip_raw_bytes(&mut self, count: u32) -> ProtobufResult<()> {
        // TODO: make it more efficient
        self.read_raw_bytes(count).map(|_| ())
    }

    pub fn read_bytes(&mut self) -> ProtobufResult<Vec<u8>> {
        let mut r = Vec::new();
        self.read_bytes_into(&mut r)?;
        Ok(r)
    }

    #[cfg(feature = "bytes")]
    pub fn read_carllerche_bytes(&mut self) -> ProtobufResult<Bytes> {
        let len = self.read_raw_varint32()?;
        self.read_raw_callerche_bytes(len as usize)
    }

    #[cfg(feature = "bytes")]
    pub fn read_carllerche_chars(&mut self) -> ProtobufResult<Chars> {
        let bytes = self.read_carllerche_bytes()?;
        Ok(Chars::from_bytes(bytes)?)
    }

    pub fn read_bytes_into(&mut self, target: &mut Vec<u8>) -> ProtobufResult<()> {
        let len = self.read_raw_varint32()?;
        self.read_raw_bytes_into(len, target)?;
        Ok(())
    }

    pub fn read_string(&mut self) -> ProtobufResult<String> {
        let mut r = String::new();
        self.read_string_into(&mut r)?;
        Ok(r)
    }

    pub fn read_string_into(&mut self, target: &mut String) -> ProtobufResult<()> {
        // assert string is empty, otherwize UTF-8 validation is too expensive
        assert!(target.is_empty());
        // take target's buffer
        let mut vec = mem::replace(target, String::new()).into_bytes();
        self.read_bytes_into(&mut vec)?;

        let s = match String::from_utf8(vec) {
            Ok(t) => t,
            Err(_) => return Err(ProtobufError::WireError(WireError::Utf8Error)),
        };
        mem::replace(target, s);
        Ok(())
    }

    pub fn merge_message<M : Message>(&mut self, message: &mut M) -> ProtobufResult<()> {
        let len = self.read_raw_varint64()?;
        let old_limit = self.push_limit(len)?;
        message.merge_from(self)?;
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_message<M : Message + MessageStatic>(&mut self) -> ProtobufResult<M> {
        let mut r: M = MessageStatic::new();
        self.merge_message(&mut r)?;
        r.check_initialized()?;
        Ok(r)
    }
}

impl<'a> Read for CodedInputStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf).map_err(Into::into)
    }
}

impl<'a> BufRead for CodedInputStream<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf().map_err(Into::into)
    }

    fn consume(&mut self, amt: usize) {
        self.source.consume(amt)
    }
}

pub trait WithCodedOutputStream {
    fn with_coded_output_stream<T, F>(self, cb: F) -> ProtobufResult<T>
    where
        F : FnOnce(&mut CodedOutputStream) -> ProtobufResult<T>;
}

impl<'a> WithCodedOutputStream for &'a mut (Write + 'a) {
    fn with_coded_output_stream<T, F>(self, cb: F) -> ProtobufResult<T>
    where
        F : FnOnce(&mut CodedOutputStream) -> ProtobufResult<T>,
    {
        let mut os = CodedOutputStream::new(self);
        let r = cb(&mut os)?;
        os.flush()?;
        Ok(r)
    }
}

impl<'a> WithCodedOutputStream for &'a mut Vec<u8> {
    fn with_coded_output_stream<T, F>(mut self, cb: F) -> ProtobufResult<T>
    where
        F : FnOnce(&mut CodedOutputStream) -> ProtobufResult<T>,
    {
        let mut os = CodedOutputStream::vec(&mut self);
        let r = cb(&mut os)?;
        os.flush()?;
        Ok(r)
    }
}

pub fn with_coded_output_stream_to_bytes<F>(cb: F) -> ProtobufResult<Vec<u8>>
where
    F : FnOnce(&mut CodedOutputStream) -> ProtobufResult<()>,
{
    let mut v = Vec::new();
    v.with_coded_output_stream(cb)?;
    Ok(v)
}

pub trait WithCodedInputStream {
    fn with_coded_input_stream<T, F>(self, cb: F) -> ProtobufResult<T>
    where
        F : FnOnce(&mut CodedInputStream) -> ProtobufResult<T>;
}

impl<'a> WithCodedInputStream for &'a mut (Read + 'a) {
    fn with_coded_input_stream<T, F>(self, cb: F) -> ProtobufResult<T>
    where
        F : FnOnce(&mut CodedInputStream) -> ProtobufResult<T>,
    {
        let mut is = CodedInputStream::new(self);
        let r = cb(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }
}

impl<'a> WithCodedInputStream for &'a mut (BufRead + 'a) {
    fn with_coded_input_stream<T, F>(self, cb: F) -> ProtobufResult<T>
    where
        F : FnOnce(&mut CodedInputStream) -> ProtobufResult<T>,
    {
        let mut is = CodedInputStream::from_buffered_reader(self);
        let r = cb(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }
}

impl<'a> WithCodedInputStream for &'a [u8] {
    fn with_coded_input_stream<T, F>(self, cb: F) -> ProtobufResult<T>
    where
        F : FnOnce(&mut CodedInputStream) -> ProtobufResult<T>,
    {
        let mut is = CodedInputStream::from_bytes(self);
        let r = cb(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }
}

#[cfg(feature = "bytes")]
impl<'a> WithCodedInputStream for &'a Bytes {
    fn with_coded_input_stream<T, F>(self, cb: F) -> ProtobufResult<T>
    where
        F : FnOnce(&mut CodedInputStream) -> ProtobufResult<T>,
    {
        let mut is = CodedInputStream::from_carllerche_bytes(self);
        let r = cb(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }
}


enum OutputTarget<'a> {
    Write(&'a mut Write, Vec<u8>),
    Vec(&'a mut Vec<u8>),
    Bytes,
}


pub struct CodedOutputStream<'a> {
    target: OutputTarget<'a>,
    // alias to buf from target
    buffer: &'a mut [u8],
    // within buffer
    position: usize,
}

impl<'a> CodedOutputStream<'a> {
    pub fn new(writer: &'a mut Write) -> CodedOutputStream<'a> {
        let buffer_len = OUTPUT_STREAM_BUFFER_SIZE;

        let mut buffer_storage = Vec::with_capacity(buffer_len);
        unsafe {
            buffer_storage.set_len(buffer_len);
        }

        let buffer = unsafe { remove_lifetime_mut(&mut buffer_storage as &mut [u8]) };

        CodedOutputStream {
            target: OutputTarget::Write(writer, buffer_storage),
            buffer: buffer,
            position: 0,
        }
    }

    /// `CodedOutputStream` which writes directly to bytes.
    ///
    /// Attempt to write more than bytes capacity results in error.
    pub fn bytes(bytes: &'a mut [u8]) -> CodedOutputStream<'a> {
        CodedOutputStream {
            target: OutputTarget::Bytes,
            buffer: bytes,
            position: 0,
        }
    }

    /// `CodedOutputStream` which writes directly to `Vec<u8>`.
    ///
    /// Caller should call `flush` at the end to guarantee vec contains
    /// all written data.
    pub fn vec(vec: &'a mut Vec<u8>) -> CodedOutputStream<'a> {
        CodedOutputStream {
            target: OutputTarget::Vec(vec),
            buffer: &mut [],
            position: 0,
        }
    }

    pub fn check_eof(&self) {
        match self.target {
            OutputTarget::Bytes => {
                assert_eq!(self.buffer.len() as u64, self.position as u64);
            }
            OutputTarget::Write(..) |
            OutputTarget::Vec(..) => {
                panic!("must not be called with Writer or Vec");
            }
        }
    }

    fn refresh_buffer(&mut self) -> ProtobufResult<()> {
        match self.target {
            OutputTarget::Write(ref mut write, _) => {
                write.write_all(&self.buffer[0..self.position as usize])?;
                self.position = 0;
            }
            OutputTarget::Vec(ref mut vec) => unsafe {
                let vec_len = vec.len();
                assert!(vec_len + self.position <= vec.capacity());
                vec.set_len(vec_len + self.position);
                vec.reserve(1);
                self.buffer = remove_lifetime_mut(remaining_capacity_as_slice_mut(vec));
                self.position = 0;
            },
            OutputTarget::Bytes => {
                panic!("refresh_buffer must not be called on CodedOutputStream create from slice");
            }
        }
        Ok(())
    }

    pub fn flush(&mut self) -> ProtobufResult<()> {
        match self.target {
            OutputTarget::Bytes => Ok(()),
            OutputTarget::Write(..) |
            OutputTarget::Vec(..) => {
                // TODO: must not reserve additional in Vec
                self.refresh_buffer()
            }
        }
    }

    pub fn write_raw_byte(&mut self, byte: u8) -> ProtobufResult<()> {
        if self.position as usize == self.buffer.len() {
            self.refresh_buffer()?;
        }
        self.buffer[self.position as usize] = byte;
        self.position += 1;
        Ok(())
    }

    pub fn write_raw_bytes(&mut self, bytes: &[u8]) -> ProtobufResult<()> {
        if bytes.len() <= self.buffer.len() - self.position {
            let bottom = self.position as usize;
            let top = bottom + (bytes.len() as usize);
            self.buffer[bottom..top].copy_from_slice(bytes);
            self.position += bytes.len();
            return Ok(());
        }

        self.refresh_buffer()?;

        assert!(self.position == 0);

        if self.position + bytes.len() < self.buffer.len() {
            &mut self.buffer[self.position..self.position + bytes.len()].copy_from_slice(bytes);
            self.position += bytes.len();
            return Ok(());
        }

        match self.target {
            OutputTarget::Bytes => {
                unreachable!();
            }
            OutputTarget::Write(ref mut write, _) => {
                write.write_all(bytes)?;
            }
            OutputTarget::Vec(ref mut vec) => {
                vec.extend(bytes);
                unsafe {
                    self.buffer = remove_lifetime_mut(remaining_capacity_as_slice_mut(vec));
                }
            }
        }
        Ok(())
    }

    pub fn write_tag(
        &mut self,
        field_number: u32,
        wire_type: wire_format::WireType,
    ) -> ProtobufResult<()> {
        self.write_raw_varint32(wire_format::Tag::make(field_number, wire_type).value())
    }

    pub fn write_raw_varint32(&mut self, value: u32) -> ProtobufResult<()> {
        if self.buffer.len() - self.position >= 5 {
            // fast path
            let len = varint::encode_varint32(value, &mut self.buffer[self.position..]);
            self.position += len;
            Ok(())
        } else {
            // slow path
            let mut buf = &mut [0u8; 5];
            let len = varint::encode_varint32(value, buf);
            self.write_raw_bytes(&buf[..len])
        }
    }

    pub fn write_raw_varint64(&mut self, value: u64) -> ProtobufResult<()> {
        if self.buffer.len() - self.position >= 10 {
            // fast path
            let len = varint::encode_varint64(value, &mut self.buffer[self.position..]);
            self.position += len;
            Ok(())
        } else {
            // slow path
            let mut buf = &mut [0u8; 10];
            let len = varint::encode_varint64(value, buf);
            self.write_raw_bytes(&buf[..len])
        }
    }

    pub fn write_raw_little_endian32(&mut self, value: u32) -> ProtobufResult<()> {
        let bytes = unsafe { mem::transmute::<_, [u8; 4]>(value.to_le()) };
        self.write_raw_bytes(&bytes)
    }

    pub fn write_raw_little_endian64(&mut self, value: u64) -> ProtobufResult<()> {
        let bytes = unsafe { mem::transmute::<_, [u8; 8]>(value.to_le()) };
        self.write_raw_bytes(&bytes)
    }

    pub fn write_float_no_tag(&mut self, value: f32) -> ProtobufResult<()> {
        let bits = unsafe { mem::transmute::<f32, u32>(value) };
        self.write_raw_little_endian32(bits)
    }

    pub fn write_double_no_tag(&mut self, value: f64) -> ProtobufResult<()> {
        let bits = unsafe { mem::transmute::<f64, u64>(value) };
        self.write_raw_little_endian64(bits)
    }

    pub fn write_float(&mut self, field_number: u32, value: f32) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeFixed32)?;
        self.write_float_no_tag(value)?;
        Ok(())
    }

    pub fn write_double(&mut self, field_number: u32, value: f64) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeFixed64)?;
        self.write_double_no_tag(value)?;
        Ok(())
    }

    pub fn write_uint64_no_tag(&mut self, value: u64) -> ProtobufResult<()> {
        self.write_raw_varint64(value)
    }

    pub fn write_uint32_no_tag(&mut self, value: u32) -> ProtobufResult<()> {
        self.write_raw_varint32(value)
    }

    pub fn write_int64_no_tag(&mut self, value: i64) -> ProtobufResult<()> {
        self.write_raw_varint64(value as u64)
    }

    pub fn write_int32_no_tag(&mut self, value: i32) -> ProtobufResult<()> {
        self.write_raw_varint64(value as u64)
    }

    pub fn write_sint64_no_tag(&mut self, value: i64) -> ProtobufResult<()> {
        self.write_uint64_no_tag(encode_zig_zag_64(value))
    }

    pub fn write_sint32_no_tag(&mut self, value: i32) -> ProtobufResult<()> {
        self.write_uint32_no_tag(encode_zig_zag_32(value))
    }

    pub fn write_fixed64_no_tag(&mut self, value: u64) -> ProtobufResult<()> {
        self.write_raw_little_endian64(value)
    }

    pub fn write_fixed32_no_tag(&mut self, value: u32) -> ProtobufResult<()> {
        self.write_raw_little_endian32(value)
    }

    pub fn write_sfixed64_no_tag(&mut self, value: i64) -> ProtobufResult<()> {
        self.write_raw_little_endian64(value as u64)
    }

    pub fn write_sfixed32_no_tag(&mut self, value: i32) -> ProtobufResult<()> {
        self.write_raw_little_endian32(value as u32)
    }

    pub fn write_bool_no_tag(&mut self, value: bool) -> ProtobufResult<()> {
        self.write_raw_varint32(if value { 1 } else { 0 })
    }

    pub fn write_enum_no_tag(&mut self, value: i32) -> ProtobufResult<()> {
        self.write_int32_no_tag(value)
    }

    pub fn write_enum_obj_no_tag<E>(&mut self, value: E) -> ProtobufResult<()>
    where
        E : ProtobufEnum,
    {
        self.write_enum_no_tag(value.value())
    }

    pub fn write_unknown_no_tag(&mut self, unknown: UnknownValueRef) -> ProtobufResult<()> {
        match unknown {
            UnknownValueRef::Fixed64(fixed64) => self.write_raw_little_endian64(fixed64),
            UnknownValueRef::Fixed32(fixed32) => self.write_raw_little_endian32(fixed32),
            UnknownValueRef::Varint(varint) => self.write_raw_varint64(varint),
            UnknownValueRef::LengthDelimited(bytes) => self.write_bytes_no_tag(bytes),
        }
    }

    pub fn write_uint64(&mut self, field_number: u32, value: u64) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeVarint)?;
        self.write_uint64_no_tag(value)?;
        Ok(())
    }

    pub fn write_uint32(&mut self, field_number: u32, value: u32) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeVarint)?;
        self.write_uint32_no_tag(value)?;
        Ok(())
    }

    pub fn write_int64(&mut self, field_number: u32, value: i64) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeVarint)?;
        self.write_int64_no_tag(value)?;
        Ok(())
    }

    pub fn write_int32(&mut self, field_number: u32, value: i32) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeVarint)?;
        self.write_int32_no_tag(value)?;
        Ok(())
    }

    pub fn write_sint64(&mut self, field_number: u32, value: i64) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeVarint)?;
        self.write_sint64_no_tag(value)?;
        Ok(())
    }

    pub fn write_sint32(&mut self, field_number: u32, value: i32) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeVarint)?;
        self.write_sint32_no_tag(value)?;
        Ok(())
    }

    pub fn write_fixed64(&mut self, field_number: u32, value: u64) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeFixed64)?;
        self.write_fixed64_no_tag(value)?;
        Ok(())
    }

    pub fn write_fixed32(&mut self, field_number: u32, value: u32) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeFixed32)?;
        self.write_fixed32_no_tag(value)?;
        Ok(())
    }

    pub fn write_sfixed64(&mut self, field_number: u32, value: i64) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeFixed64)?;
        self.write_sfixed64_no_tag(value)?;
        Ok(())
    }

    pub fn write_sfixed32(&mut self, field_number: u32, value: i32) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeFixed32)?;
        self.write_sfixed32_no_tag(value)?;
        Ok(())
    }

    pub fn write_bool(&mut self, field_number: u32, value: bool) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeVarint)?;
        self.write_bool_no_tag(value)?;
        Ok(())
    }

    pub fn write_enum(&mut self, field_number: u32, value: i32) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeVarint)?;
        self.write_enum_no_tag(value)?;
        Ok(())
    }

    pub fn write_enum_obj<E>(&mut self, field_number: u32, value: E) -> ProtobufResult<()>
    where
        E : ProtobufEnum,
    {
        self.write_enum(field_number, value.value())
    }

    pub fn write_unknown(
        &mut self,
        field_number: u32,
        value: UnknownValueRef,
    ) -> ProtobufResult<()> {
        self.write_tag(field_number, value.wire_type())?;
        self.write_unknown_no_tag(value)?;
        Ok(())
    }

    pub fn write_unknown_fields(&mut self, fields: &UnknownFields) -> ProtobufResult<()> {
        for (number, values) in fields {
            for value in values {
                self.write_unknown(number, value)?;
            }
        }
        Ok(())
    }

    pub fn write_bytes_no_tag(&mut self, bytes: &[u8]) -> ProtobufResult<()> {
        self.write_raw_varint32(bytes.len() as u32)?;
        self.write_raw_bytes(bytes)?;
        Ok(())
    }

    pub fn write_string_no_tag(&mut self, s: &str) -> ProtobufResult<()> {
        self.write_bytes_no_tag(s.as_bytes())
    }

    pub fn write_message_no_tag<M : Message>(&mut self, msg: &M) -> ProtobufResult<()> {
        msg.write_length_delimited_to(self)
    }

    pub fn write_bytes(&mut self, field_number: u32, bytes: &[u8]) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeLengthDelimited)?;
        self.write_bytes_no_tag(bytes)?;
        Ok(())
    }

    pub fn write_string(&mut self, field_number: u32, s: &str) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeLengthDelimited)?;
        self.write_string_no_tag(s)?;
        Ok(())
    }

    pub fn write_message<M : Message>(&mut self, field_number: u32, msg: &M) -> ProtobufResult<()> {
        self.write_tag(field_number, wire_format::WireTypeLengthDelimited)?;
        self.write_message_no_tag(msg)?;
        Ok(())
    }
}

impl<'a> Write for CodedOutputStream<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write_raw_bytes(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        CodedOutputStream::flush(self).map_err(Into::into)
    }
}


#[cfg(test)]
mod test {

    use std::io;
    use std::io::Read;
    use std::io::BufRead;
    use std::io::Write;
    use std::iter::repeat;
    use std::fmt::Debug;

    use hex::encode_hex;
    use hex::decode_hex;
    use error::ProtobufResult;
    use error::ProtobufError;

    use super::wire_format;
    use super::CodedInputStream;
    use super::CodedOutputStream;

    fn test_read_partial<F>(hex: &str, mut callback: F)
    where
        F : FnMut(&mut CodedInputStream),
    {
        let d = decode_hex(hex);
        let mut reader = io::Cursor::new(d);
        let mut is = CodedInputStream::from_buffered_reader(&mut reader as &mut BufRead);
        assert_eq!(0, is.pos());
        callback(&mut is);
    }

    fn test_read<F>(hex: &str, mut callback: F)
    where
        F : FnMut(&mut CodedInputStream),
    {
        let len = decode_hex(hex).len();
        test_read_partial(hex, |reader| {
            callback(reader);
            assert!(reader.eof().expect("eof"));
            assert_eq!(len as u64, reader.pos());
        });
    }

    fn test_read_v<F, V>(hex: &str, v: V, mut callback: F)
    where
        F : FnMut(&mut CodedInputStream) -> ProtobufResult<V>,
        V : PartialEq + Debug,
    {
        test_read(hex, |reader| {
            assert_eq!(v, callback(reader).unwrap());
        });
    }

    #[test]
    fn test_input_stream_read_raw_byte() {
        test_read("17", |is| {
            assert_eq!(23, is.read_raw_byte().unwrap());
        });
    }

    #[test]
    fn test_input_stream_read_raw_varint() {
        test_read_v("07", 7, |reader| reader.read_raw_varint32());
        test_read_v("07", 7, |reader| reader.read_raw_varint64());

        test_read_v("96 01", 150, |reader| reader.read_raw_varint32());
        test_read_v("96 01", 150, |reader| reader.read_raw_varint64());

        test_read_v(
            "ff ff ff ff ff ff ff ff ff 01",
            0xffffffffffffffff,
            |reader| reader.read_raw_varint64(),
        );

        test_read_v("ff ff ff ff 0f", 0xffffffff, |reader| {
            reader.read_raw_varint32()
        });
        test_read_v("ff ff ff ff 0f", 0xffffffff, |reader| {
            reader.read_raw_varint64()
        });
    }

    #[test]
    fn test_input_stream_read_raw_vaint_malformed() {
        // varint cannot have length > 10
        test_read_partial("ff ff ff ff ff ff ff ff ff ff 01", |reader| {
            let result = reader.read_raw_varint64();
            match result {
                // TODO: make an enum variant
                Err(ProtobufError::WireError(..)) => (),
                _ => panic!(),
            }
        });
        test_read_partial("ff ff ff ff ff ff ff ff ff ff 01", |reader| {
            let result = reader.read_raw_varint32();
            match result {
                // TODO: make an enum variant
                Err(ProtobufError::WireError(..)) => (),
                _ => panic!(),
            }
        });
    }

    #[test]
    fn test_input_stream_read_raw_varint_unexpected_eof() {
        test_read_partial("96 97", |reader| {
            let result = reader.read_raw_varint32();
            match result {
                Err(ProtobufError::WireError(..)) => (),
                _ => panic!(),
            }
        });
    }

    #[test]
    fn test_input_stream_read_raw_varint_pos() {
        test_read_partial("95 01 98", |reader| {
            assert_eq!(149, reader.read_raw_varint32().unwrap());
            assert_eq!(2, reader.pos());
        });
    }

    #[test]
    fn test_input_stream_read_int32() {
        test_read_v("02", 2, |reader| reader.read_int32());
    }

    #[test]
    fn test_input_stream_read_float() {
        test_read_v("95 73 13 61", 17e19, |is| is.read_float());
    }

    #[test]
    fn test_input_stream_read_double() {
        test_read_v("40 d5 ab 68 b3 07 3d 46", 23e29, |is| is.read_double());
    }

    #[test]
    fn test_input_stream_skip_raw_bytes() {
        test_read("", |reader| { reader.skip_raw_bytes(0).unwrap(); });
        test_read("aa bb", |reader| { reader.skip_raw_bytes(2).unwrap(); });
        test_read("aa bb cc dd ee ff", |reader| {
            reader.skip_raw_bytes(6).unwrap();
        });
    }

    #[test]
    fn test_input_stream_read_raw_bytes() {
        test_read("", |reader| {
            assert_eq!(
                Vec::from(&b""[..]),
                reader.read_raw_bytes(0).expect("read_raw_bytes")
            );
        })
    }

    #[test]
    fn test_input_stream_limits() {
        test_read("aa bb cc", |is| {
            let old_limit = is.push_limit(1).unwrap();
            assert_eq!(1, is.bytes_until_limit());
            let r1 = is.read_raw_bytes(1).unwrap();
            assert_eq!(&[0xaa as u8], &r1[..]);
            is.pop_limit(old_limit);
            let r2 = is.read_raw_bytes(2).unwrap();
            assert_eq!(&[0xbb as u8, 0xcc], &r2[..]);
        });
    }

    #[test]
    fn test_input_stream_io_read() {
        test_read("aa bb cc", |is| {
            let mut buf = [0; 3];
            assert_eq!(Read::read(is, &mut buf).expect("io::Read"), 3);
            assert_eq!(buf, [0xaa, 0xbb, 0xcc]);
        });
    }

    #[test]
    fn test_input_stream_io_bufread() {
        test_read("aa bb cc", |is| {
            assert_eq!(
                BufRead::fill_buf(is).expect("io::BufRead::fill_buf"),
                &[0xaa, 0xbb, 0xcc]
            );
            BufRead::consume(is, 3);
        });
    }

    fn test_write<F>(expected: &str, mut gen: F)
    where
        F : FnMut(&mut CodedOutputStream) -> ProtobufResult<()>,
    {
        let expected_bytes = decode_hex(expected);

        // write to Write
        {
            let mut v = Vec::new();
            {
                let mut os = CodedOutputStream::new(&mut v as &mut Write);
                gen(&mut os).unwrap();
                os.flush().unwrap();
            }
            assert_eq!(encode_hex(&expected_bytes), encode_hex(&v));
        }

        // write to &[u8]
        {
            let mut r = Vec::with_capacity(expected_bytes.len());
            r.resize(expected_bytes.len(), 0);
            {
                let mut os = CodedOutputStream::bytes(&mut r);
                gen(&mut os).unwrap();
                os.check_eof();
            }
            assert_eq!(encode_hex(&expected_bytes), encode_hex(&r));
        }

        // write to Vec<u8>
        {
            let mut r = Vec::new();
            r.extend(&[11, 22, 33, 44, 55, 66, 77]);
            {
                let mut os = CodedOutputStream::vec(&mut r);
                gen(&mut os).unwrap();
                os.flush().unwrap();
            }

            r.drain(..7);
            assert_eq!(encode_hex(&expected_bytes), encode_hex(&r));
        }
    }

    #[test]
    fn test_output_stream_write_raw_byte() {
        test_write("a1", |os| os.write_raw_byte(0xa1));
    }

    #[test]
    fn test_output_stream_write_tag() {
        test_write("08", |os| os.write_tag(1, wire_format::WireTypeVarint));
    }

    #[test]
    fn test_output_stream_write_raw_bytes() {
        test_write("00 ab", |os| os.write_raw_bytes(&[0x00, 0xab]));

        let expected = repeat("01 02 03 04")
            .take(2048)
            .collect::<Vec<_>>()
            .join(" ");
        test_write(&expected, |os| {
            for _ in 0..2048 {
                os.write_raw_bytes(&[0x01, 0x02, 0x03, 0x04])?;
            }

            Ok(())
        });
    }

    #[test]
    fn test_output_stream_write_raw_varint32() {
        test_write("96 01", |os| os.write_raw_varint32(150));
        test_write("ff ff ff ff 0f", |os| os.write_raw_varint32(0xffffffff));
    }

    #[test]
    fn test_output_stream_write_raw_varint64() {
        test_write("96 01", |os| os.write_raw_varint64(150));
        test_write("ff ff ff ff ff ff ff ff ff 01", |os| {
            os.write_raw_varint64(0xffffffffffffffff)
        });
    }

    #[test]
    fn test_output_stream_write_int32_no_tag() {
        test_write("ff ff ff ff ff ff ff ff ff 01", |os| {
            os.write_int32_no_tag(-1)
        });
    }

    #[test]
    fn test_output_stream_write_int64_no_tag() {
        test_write("ff ff ff ff ff ff ff ff ff 01", |os| {
            os.write_int64_no_tag(-1)
        });
    }

    #[test]
    fn test_output_stream_write_raw_little_endian32() {
        test_write("f1 e2 d3 c4", |os| os.write_raw_little_endian32(0xc4d3e2f1));
    }

    #[test]
    fn test_output_stream_write_float_no_tag() {
        test_write("95 73 13 61", |os| os.write_float_no_tag(17e19));
    }

    #[test]
    fn test_output_stream_write_double_no_tag() {
        test_write(
            "40 d5 ab 68 b3 07 3d 46",
            |os| os.write_double_no_tag(23e29),
        );
    }

    #[test]
    fn test_output_stream_write_raw_little_endian64() {
        test_write("f1 e2 d3 c4 b5 a6 07 f8", |os| {
            os.write_raw_little_endian64(0xf807a6b5c4d3e2f1)
        });
    }

    #[test]
    fn test_output_stream_io_write() {
        let expected = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];

        // write to Write
        {
            let mut v = Vec::new();
            {
                let mut os = CodedOutputStream::new(&mut v as &mut Write);
                Write::write(&mut os, &expected).expect("io::Write::write");
                Write::flush(&mut os).expect("io::Write::flush");
            }
            assert_eq!(expected, *v);
        }

        // write to &[u8]
        {
            let mut v = Vec::with_capacity(expected.len());
            v.resize(expected.len(), 0);
            {
                let mut os = CodedOutputStream::bytes(&mut v);
                Write::write(&mut os, &expected).expect("io::Write::write");
                Write::flush(&mut os).expect("io::Write::flush");
                os.check_eof();
            }
            assert_eq!(expected, *v);
        }

        // write to Vec<u8>
        {
            let mut v = Vec::new();
            {
                let mut os = CodedOutputStream::vec(&mut v);
                Write::write(&mut os, &expected).expect("io::Write::write");
                Write::flush(&mut os).expect("io::Write::flush");
            }
            assert_eq!(expected, *v);
        }
    }
}
