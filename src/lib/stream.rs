use std;
use std::mem;
use std::io;
use std::io::Read;
use std::io::Write;

use maybe_owned_slice::MaybeOwnedSlice;
use core::Message;
use core::MessageStatic;
use core::ProtobufEnum;
use misc::VecWriter;
use unknown::UnknownFields;
use unknown::UnknownValue;
use unknown::UnknownValueRef;
use zigzag::decode_zig_zag_32;
use zigzag::decode_zig_zag_64;
use zigzag::encode_zig_zag_32;
use zigzag::encode_zig_zag_64;
use error::ProtobufResult;
use error::ProtobufError;

pub mod wire_format {
    // TODO: temporary
    pub use self::WireType::*;

    pub const TAG_TYPE_BITS: u32 = 3;
    pub const TAG_TYPE_MASK: u32 = (1u32 << TAG_TYPE_BITS as usize) - 1;
    // max possible tag number
    pub const FIELD_NUMBER_MAX: u32 = 0x1fffffff;

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub enum WireType {
        WireTypeVarint          = 0,
        WireTypeFixed64         = 1,
        WireTypeLengthDelimited = 2,
        WireTypeStartGroup      = 3,
        WireTypeEndGroup        = 4,
        WireTypeFixed32         = 5,
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
    buffer: MaybeOwnedSlice<'a, u8>,
    buffer_size: u32,
    buffer_pos: u32,
    reader: Option<&'a mut (Read + 'a)>,
    total_bytes_retired: u32,
    current_limit: u32,
    buffer_size_after_limit: u32,
}

impl<'a> CodedInputStream<'a> {
    pub fn new(reader: &'a mut Read) -> CodedInputStream<'a> {
        let buffer_len = 4096;
        let mut buffer = Vec::with_capacity(buffer_len);
        unsafe { buffer.set_len(buffer_len); }
        CodedInputStream {
            buffer: MaybeOwnedSlice::from_vec(buffer),
            buffer_size: 0,
            buffer_pos: 0,
            reader: Some(reader),
            total_bytes_retired: 0,
            current_limit: std::u32::MAX,
            buffer_size_after_limit: 0,
        }
    }

    pub fn from_bytes(bytes: &'a [u8]) -> CodedInputStream<'a> {
        let len = bytes.len() as u32;
        CodedInputStream {
            buffer: MaybeOwnedSlice::from_slice(bytes),
            buffer_size: len,
            buffer_pos: 0,
            reader: None,
            total_bytes_retired: 0,
            current_limit: len,
            buffer_size_after_limit: 0,
        }
    }

    fn remaining_in_buffer(&self) -> u32 {
        self.buffer_size - self.buffer_pos
    }

    fn remaining_in_buffer_slice<'b>(&'b self) -> &'b [u8] {
        self.buffer.slice(self.buffer_pos as usize, self.buffer_size as usize)
    }

    pub fn pos(&self) -> u32 {
        self.total_bytes_retired + self.buffer_pos
    }

    fn bytes_until_limit(&self) -> u32 {
        self.current_limit - self.pos()
    }

    // Refill buffer if buffer is empty.
    // Fails if buffer is not empty.
    // Retuns Err if IO error occurred.
    // Returns Ok(false) on EOF, or if limit reached.
    // Otherwize returns Ok(true).
    fn refill_buffer(&mut self) -> ProtobufResult<bool> {
        if self.buffer_pos < self.buffer_size {
            panic!("called when buffer is not empty");
        }
        if self.pos() == self.current_limit {
            return Ok(false);
        }
        if self.reader.is_none() {
            Ok(false)
        } else {
            match self.reader {
                Some(ref mut reader) => {
                    self.total_bytes_retired += self.buffer_size;
                    self.buffer_pos = 0;
                    self.buffer_size = 0;

                    let r = reader.read(self.buffer.as_mut_slice());
                    self.buffer_size = match r {
                        Err(e) => return Err(ProtobufError::IoError(e)),
                        Ok(x) if x == 0 => return Ok(false),
                        Ok(x) => x as u32,
                    };
                    assert!(self.buffer_size > 0);
                },
                None => panic!(),
            }
            self.recompute_buffer_size_after_limit();
            Ok(true)
        }
    }

    fn refill_buffer_really(&mut self) -> ProtobufResult<()> {
        if !try!(self.refill_buffer()) {
            return Err(ProtobufError::IoError(io::Error::new(
                io::ErrorKind::Other, "unexpected EOF")));
        }
        Ok(())
    }

    fn recompute_buffer_size_after_limit(&mut self) {
        self.buffer_size += self.buffer_size_after_limit;
        let buffer_end = self.total_bytes_retired + self.buffer_size;
        if buffer_end > self.current_limit {
            // limit is in current buffer
            self.buffer_size_after_limit = buffer_end - self.current_limit;
            self.buffer_size -= self.buffer_size_after_limit;
        } else {
            self.buffer_size_after_limit = 0;
        }
    }

    pub fn push_limit(&mut self, limit: u32) -> ProtobufResult<u32> {
        let old_limit = self.current_limit;
        let new_limit = self.pos() + limit;
        if new_limit > old_limit {
            return Err(ProtobufError::WireError(format!("truncated message")));
        }
        self.current_limit = new_limit;
        self.recompute_buffer_size_after_limit();
        Ok(old_limit)
    }

    pub fn pop_limit(&mut self, old_limit: u32) {
        if self.bytes_until_limit() != 0 {
            panic!("must pop only at current limit")
        }
        self.current_limit = old_limit;
        self.recompute_buffer_size_after_limit();
    }

    pub fn eof(&mut self) -> ProtobufResult<bool> {
        return Ok(self.buffer_pos == self.buffer_size && !try!(self.refill_buffer()))
    }

    pub fn check_eof(&mut self) -> ProtobufResult<()> {
        let eof = try!(self.eof());
        if !eof {
            return Err(ProtobufError::WireError(format!("expecting EOF")));
        }
        Ok(())
    }

    pub fn read_raw_byte(&mut self) -> ProtobufResult<u8> {
        if self.buffer_pos == self.buffer_size {
            try!(self.refill_buffer_really());
        }
        assert!(self.buffer_pos < self.buffer_size);
        let r = self.buffer[self.buffer_pos as usize];
        self.buffer_pos += 1;
        Ok(r)
    }

    pub fn read_raw_varint64(&mut self) -> ProtobufResult<u64> {
        let mut r: u64 = 0;
        let mut i = 0;
        loop {
            let b = try!(self.read_raw_byte());
            // Stop undefined behaviour
            if i <= 9 {
                r = r | (((b & 0x7f) as u64) << (i * 7));
                i += 1;
            }
            if b < 0x80 {
                return Ok(r);
            }
        }
    }

    pub fn read_raw_varint32(&mut self) -> ProtobufResult<u32> {
        self.read_raw_varint64().map(|v| v as u32)
    }

    pub fn read_raw_little_endian32(&mut self) -> ProtobufResult<u32> {
        let mut bytes = [0u32; 4];
        for i in 0..4 {
            bytes[i] = try!(self.read_raw_byte()) as u32;
        }
        Ok(
            (bytes[0]      ) |
            (bytes[1] <<  8) |
            (bytes[2] << 16) |
            (bytes[3] << 24)
        )
    }

    pub fn read_raw_little_endian64(&mut self) -> ProtobufResult<u64> {
        let mut bytes = [0u64; 8];
        for i in 0..8 {
            bytes[i] = try!(self.read_raw_byte()) as u64;
        }
        Ok(
            (bytes[0]      ) |
            (bytes[1] <<  8) |
            (bytes[2] << 16) |
            (bytes[3] << 24) |
            (bytes[4] << 32) |
            (bytes[5] << 40) |
            (bytes[6] << 48) |
            (bytes[7] << 56)
        )
    }

    pub fn read_tag(&mut self) -> ProtobufResult<wire_format::Tag> {
        let v = try!(self.read_raw_varint32());
        match wire_format::Tag::new(v) {
            Some(tag) => Ok(tag),
            None => Err(ProtobufError::WireError(format!("unknown tag: {}", v))),
        }
    }

    // Read tag, return it is pair (field number, wire type)
    pub fn read_tag_unpack(&mut self) -> ProtobufResult<(u32, wire_format::WireType)> {
        self.read_tag().map(|t| t.unpack())
    }

    pub fn read_double(&mut self) -> ProtobufResult<f64> {
        let bits = try!(self.read_raw_little_endian64());
        unsafe {
            Ok(mem::transmute::<u64, f64>(bits))
        }
    }

    pub fn read_float(&mut self) -> ProtobufResult<f32> {
        let bits = try!(self.read_raw_little_endian32());
        unsafe {
            Ok(mem::transmute::<u32, f32>(bits))
        }
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
        let i = try!(self.read_int32());
        match ProtobufEnum::from_i32(i) {
            Some(e) => Ok(e),
            None => Err(ProtobufError::WireError(
                format!("invalid value for enum: {}", i))),
        }
    }

    pub fn read_repeated_packed_double_into(&mut self, target: &mut Vec<f64>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());

        target.reserve((len / 4) as usize);

        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_double()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_float_into(&mut self, target: &mut Vec<f32>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());

        target.reserve((len / 4) as usize);

        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_float()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_int64_into(&mut self, target: &mut Vec<i64>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());
        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_int64()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_int32_into(&mut self, target: &mut Vec<i32>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());
        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_int32()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_uint64_into(&mut self, target: &mut Vec<u64>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());
        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_uint64()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_uint32_into(&mut self, target: &mut Vec<u32>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());
        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_uint32()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_sint64_into(&mut self, target: &mut Vec<i64>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());
        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_sint64()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_sint32_into(&mut self, target: &mut Vec<i32>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());
        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_sint32()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_fixed64_into(&mut self, target: &mut Vec<u64>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());

        target.reserve((len / 8) as usize);

        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_fixed64()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_fixed32_into(&mut self, target: &mut Vec<u32>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());

        target.reserve((len / 4) as usize);

        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_fixed32()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_sfixed64_into(&mut self, target: &mut Vec<i64>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());

        target.reserve((len / 8) as usize);

        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_sfixed64()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_sfixed32_into(&mut self, target: &mut Vec<i32>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());

        target.reserve((len / 4) as usize);

        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_sfixed32()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_bool_into(&mut self, target: &mut Vec<bool>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());

        // regular bool value is 1-byte size
        target.reserve(len as usize);

        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_bool()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_repeated_packed_enum_into<E : ProtobufEnum>(&mut self, target: &mut Vec<E>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());
        let old_limit = try!(self.push_limit(len));
        while !try!(self.eof()) {
            target.push(try!(self.read_enum()));
        }
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_unknown(&mut self, wire_type: wire_format::WireType) -> ProtobufResult<UnknownValue> {
        match wire_type {
            wire_format::WireTypeVarint => { self.read_raw_varint64().map(|v| UnknownValue::Varint(v)) },
            wire_format::WireTypeFixed64 => { self.read_fixed64().map(|v| UnknownValue::Fixed64(v)) },
            wire_format::WireTypeFixed32 => { self.read_fixed32().map(|v| UnknownValue::Fixed32(v)) } ,
            wire_format::WireTypeLengthDelimited => {
                let len = try!(self.read_raw_varint32());
                self.read_raw_bytes(len).map(|v| UnknownValue::LengthDelimited(v))
            },
            _ => Err(ProtobufError::WireError(format!("unknown wire type: {}", wire_type as isize)))
        }
    }

    pub fn skip_field(&mut self, wire_type: wire_format::WireType) -> ProtobufResult<()> {
        self.read_unknown(wire_type).map(|_| ())
    }

    /// Read raw bytes into supplied vector. Vector must be empty.
    pub fn read_raw_bytes_into(&mut self, count: u32, target: &mut Vec<u8>) -> ProtobufResult<()> {
        assert!(target.is_empty());
        target.reserve(count as usize);
        while target.len() < count as usize {
            let rem = count - target.len() as u32;
            if rem <= self.remaining_in_buffer() {
                target.extend(self.buffer.slice(self.buffer_pos as usize, (self.buffer_pos + rem) as usize).iter().map(|x| *x));
                self.buffer_pos += rem;
            } else {
                target.extend(self.remaining_in_buffer_slice().iter().map(|x| *x));
                self.buffer_pos = self.buffer_size;
                try!(self.refill_buffer_really());
            }
        }
        Ok(())
    }

    /// Read exact number of bytes
    pub fn read_raw_bytes(&mut self, count: u32) -> ProtobufResult<Vec<u8>> {
        let mut r = Vec::new();
        try!(self.read_raw_bytes_into(count, &mut r));
        Ok(r)
    }

    pub fn skip_raw_bytes(&mut self, count: u32) -> ProtobufResult<()> {
        // TODO: make it more efficient
        self.read_raw_bytes(count).map(|_| ())
    }

    pub fn read_bytes(&mut self) -> ProtobufResult<Vec<u8>> {
        let mut r = Vec::new();
        try!(self.read_bytes_into(&mut r));
        Ok(r)
    }

    pub fn read_bytes_into(&mut self, target: &mut Vec<u8>) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());
        try!(self.read_raw_bytes_into(len, target));
        Ok(())
    }

    pub fn read_string(&mut self) -> ProtobufResult<String> {
        let mut r = String::new();
        try!(self.read_string_into(&mut r));
        Ok(r)
    }

    pub fn read_string_into(&mut self, target: &mut String) -> ProtobufResult<()> {
        // assert string is empty, otherwize UTF-8 validation is too expensive
        assert!(target.is_empty());
        // take target's buffer
        let mut vec = mem::replace(target, String::new()).into_bytes();
        try!(self.read_bytes_into(&mut vec));

        let s = match String::from_utf8(vec) {
            Ok(t) => t,
            Err(_) => return Err(ProtobufError::WireError(format!("invalid UTF-8 string on wire"))),
        };
        mem::replace(target, s);
        Ok(())
    }

    pub fn merge_message<M : Message>(&mut self, message: &mut M) -> ProtobufResult<()> {
        let len = try!(self.read_raw_varint32());
        let old_limit = try!(self.push_limit(len));
        try!(message.merge_from(self));
        self.pop_limit(old_limit);
        Ok(())
    }

    pub fn read_message<M : Message + MessageStatic>(&mut self) -> ProtobufResult<M> {
        let mut r: M = MessageStatic::new();
        try!(self.merge_message(&mut r));
        r.check_initialized();
        Ok(r)
    }
}

pub trait WithCodedOutputStream {
    fn with_coded_output_stream<T, F>(self, cb: F)
            -> ProtobufResult<T>
        where F : FnOnce(&mut CodedOutputStream) -> ProtobufResult<T>;
}

impl<'a> WithCodedOutputStream for &'a mut (Write + 'a) {
    fn with_coded_output_stream<T, F>(self, cb: F)
            -> ProtobufResult<T>
        where F : FnOnce(&mut CodedOutputStream) -> ProtobufResult<T>
    {
        let mut os = CodedOutputStream::new(self);
        let r = try!(cb(&mut os));
        try!(os.flush());
        Ok(r)
    }
}

impl<'a> WithCodedOutputStream for &'a mut Vec<u8> {
    fn with_coded_output_stream<T, F>(self, cb: F)
            -> ProtobufResult<T>
        where F : FnOnce(&mut CodedOutputStream) -> ProtobufResult<T>
    {
        let mut w = VecWriter::new(self);
        (&mut w as &mut Write).with_coded_output_stream(cb)
    }
}

pub fn with_coded_output_stream_to_bytes<F>(cb: F)
        -> ProtobufResult<Vec<u8>>
    where F : FnOnce(&mut CodedOutputStream) -> ProtobufResult<()>
{
    let mut v = Vec::new();
    try!(v.with_coded_output_stream(cb));
    Ok(v)
}

pub trait WithCodedInputStream {
    fn with_coded_input_stream<T, F>(self, cb: F) -> ProtobufResult<T>
        where F : FnOnce(&mut CodedInputStream) -> ProtobufResult<T>;
}

impl<'a> WithCodedInputStream for &'a mut (Read + 'a) {
    fn with_coded_input_stream<T, F>(self, cb: F) -> ProtobufResult<T>
        where F : FnOnce(&mut CodedInputStream) -> ProtobufResult<T>
    {
        let mut is = CodedInputStream::new(self);
        let r = try!(cb(&mut is));
        // reading from Reader requires all data to be read,
        // because CodedInputStream caches data, and otherwize
        // buffer would be discarded
        try!(is.check_eof());
        Ok(r)
    }
}

impl<'a> WithCodedInputStream for &'a [u8] {
    fn with_coded_input_stream<T, F>(self, cb: F) -> ProtobufResult<T>
        where F : FnOnce(&mut CodedInputStream) -> ProtobufResult<T>
    {
        let mut is = CodedInputStream::from_bytes(self);
        let r = try!(cb(&mut is));
        try!(is.check_eof());
        Ok(r)
    }
}


pub struct CodedOutputStream<'a> {
    buffer: Vec<u8>,
    // within buffer
    position: u32,
    writer: Option<&'a mut (Write + 'a)>,
}

impl<'a> CodedOutputStream<'a> {
    pub fn new(writer: &'a mut Write) -> CodedOutputStream<'a> {
        let buffer_len = 4096;
        let mut buffer = Vec::with_capacity(buffer_len);
        unsafe { buffer.set_len(buffer_len); }
        CodedOutputStream {
            buffer: buffer,
            position: 0,
            writer: Some(writer),
        }
    }

    fn refresh_buffer(&mut self) -> ProtobufResult<()> {
        match self.writer {
            Some(ref mut writer) => {
                try!(writer.write_all(&self.buffer[0..self.position as usize])
                    .map_err(|e| ProtobufError::IoError(e)));
            },
            None => panic!()
        };
        self.position = 0;
        Ok(())
    }

    pub fn flush(&mut self) -> ProtobufResult<()> {
        if self.writer.is_some() {
            try!(self.refresh_buffer());
        }
        Ok(())
    }

    pub fn write_raw_byte(&mut self, byte: u8) -> ProtobufResult<()> {
        if self.position as usize == self.buffer.len() {
            try!(self.refresh_buffer());
        }
        self.buffer[self.position as usize] = byte;
        self.position += 1;
        Ok(())
    }

    pub fn write_raw_bytes(&mut self, bytes: &[u8]) -> ProtobufResult<()> {
        try!(self.refresh_buffer());
        // TODO: write into buffer if enough capacity
        match self.writer {
            Some(ref mut writer) => try!(writer.write_all(bytes).map_err(|e| ProtobufError::IoError(e))),
            None => panic!()
        };
        Ok(())
    }

    pub fn write_tag(&mut self, field_number: u32, wire_type: wire_format::WireType) -> ProtobufResult<()> {
        self.write_raw_varint32(wire_format::Tag::make(field_number, wire_type).value())
    }

    pub fn write_raw_varint32(&mut self, value: u32) -> ProtobufResult<()> {
        self.write_raw_varint64(value as u64)
    }

    pub fn write_raw_varint64(&mut self, value: u64) -> ProtobufResult<()> {
        let mut temp = value;
        loop {
            if (temp & !0x7Fu64) == 0 {
                try!(self.write_raw_byte(temp as u8));
                break;
            } else {
                try!(self.write_raw_byte(((temp & 0x7F) | 0x80) as u8));
                temp >>= 7;
            }
        }
        Ok(())
    }

    pub fn write_raw_little_endian32(&mut self, value: u32) -> ProtobufResult<()> {
        try!(self.write_raw_byte(((value      ) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >>  8) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >> 16) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >> 24) & 0xFF) as u8));
        Ok(())
    }

    pub fn write_raw_little_endian64(&mut self, value: u64) -> ProtobufResult<()> {
        try!(self.write_raw_byte(((value      ) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >>  8) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >> 16) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >> 24) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >> 32) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >> 40) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >> 48) & 0xFF) as u8));
        try!(self.write_raw_byte(((value >> 56) & 0xFF) as u8));
        Ok(())
    }

    pub fn write_float_no_tag(&mut self, value: f32) -> ProtobufResult<()> {
        let bits = unsafe {
            mem::transmute::<f32, u32>(value)
        };
        self.write_raw_little_endian32(bits)
    }

    pub fn write_double_no_tag(&mut self, value: f64) -> ProtobufResult<()> {
        let bits = unsafe {
            mem::transmute::<f64, u64>(value)
        };
        self.write_raw_little_endian64(bits)
    }

    pub fn write_float(&mut self, field_number: u32, value: f32) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeFixed32));
        try!(self.write_float_no_tag(value));
        Ok(())
    }

    pub fn write_double(&mut self, field_number: u32, value: f64) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeFixed64));
        try!(self.write_double_no_tag(value));
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

    pub fn write_unknown_no_tag(&mut self, unknown: UnknownValueRef) -> ProtobufResult<()> {
        match unknown {
            UnknownValueRef::Fixed64(fixed64) => self.write_raw_little_endian64(fixed64),
            UnknownValueRef::Fixed32(fixed32) => self.write_raw_little_endian32(fixed32),
            UnknownValueRef::Varint(varint) => self.write_raw_varint64(varint),
            UnknownValueRef::LengthDelimited(bytes) => self.write_bytes_no_tag(bytes),
        }
    }

    pub fn write_uint64(&mut self, field_number: u32, value: u64) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeVarint));
        try!(self.write_uint64_no_tag(value));
        Ok(())
    }

    pub fn write_uint32(&mut self, field_number: u32, value: u32) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeVarint));
        try!(self.write_uint32_no_tag(value));
        Ok(())
    }

    pub fn write_int64(&mut self, field_number: u32, value: i64) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeVarint));
        try!(self.write_int64_no_tag(value));
        Ok(())
    }

    pub fn write_int32(&mut self, field_number: u32, value: i32) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeVarint));
        try!(self.write_int32_no_tag(value));
        Ok(())
    }

    pub fn write_sint64(&mut self, field_number: u32, value: i64) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeVarint));
        try!(self.write_sint64_no_tag(value));
        Ok(())
    }

    pub fn write_sint32(&mut self, field_number: u32, value: i32) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeVarint));
        try!(self.write_sint32_no_tag(value));
        Ok(())
    }

    pub fn write_fixed64(&mut self, field_number: u32, value: u64) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeFixed64));
        try!(self.write_fixed64_no_tag(value));
        Ok(())
    }

    pub fn write_fixed32(&mut self, field_number: u32, value: u32) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeFixed32));
        try!(self.write_fixed32_no_tag(value));
        Ok(())
    }

    pub fn write_sfixed64(&mut self, field_number: u32, value: i64) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeFixed64));
        try!(self.write_sfixed64_no_tag(value));
        Ok(())
    }

    pub fn write_sfixed32(&mut self, field_number: u32, value: i32) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeFixed32));
        try!(self.write_sfixed32_no_tag(value));
        Ok(())
    }

    pub fn write_bool(&mut self, field_number: u32, value: bool) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeVarint));
        try!(self.write_bool_no_tag(value));
        Ok(())
    }

    pub fn write_enum(&mut self, field_number: u32, value: i32) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeVarint));
        try!(self.write_enum_no_tag(value));
        Ok(())
    }

    pub fn write_unknown(&mut self, field_number: u32, value: UnknownValueRef) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, value.wire_type()));
        try!(self.write_unknown_no_tag(value));
        Ok(())
    }

    pub fn write_unknown_fields(&mut self, fields: &UnknownFields) -> ProtobufResult<()> {
        for (number, values) in fields.iter() {
            for value in values.iter() {
                try!(self.write_unknown(number, value));
            }
        }
        Ok(())
    }

    pub fn write_bytes_no_tag(&mut self, bytes: &[u8]) -> ProtobufResult<()> {
        try!(self.write_raw_varint32(bytes.len() as u32));
        try!(self.write_raw_bytes(bytes));
        Ok(())
    }

    pub fn write_string_no_tag(&mut self, s: &str) -> ProtobufResult<()> {
        self.write_bytes_no_tag(s.as_bytes())
    }

    pub fn write_message_no_tag<M : Message>(&mut self, msg: &M) -> ProtobufResult<()> {
        msg.write_length_delimited_to(self)
    }

    pub fn write_bytes(&mut self, field_number: u32, bytes: &[u8]) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeLengthDelimited));
        try!(self.write_bytes_no_tag(bytes));
        Ok(())
    }

    pub fn write_string(&mut self, field_number: u32, s: &str) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeLengthDelimited));
        try!(self.write_string_no_tag(s));
        Ok(())
    }

    pub fn write_message<M : Message>(&mut self, field_number: u32, msg: &M) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, wire_format::WireTypeLengthDelimited));
        try!(self.write_message_no_tag(msg));
        Ok(())
    }
}


#[cfg(test)]
mod test {

    use std::io;
    use std::io::Read;
    use std::io::Write;

    use hex::encode_hex;
    use hex::decode_hex;
    use misc::VecWriter;
    use error::ProtobufResult;

    use super::wire_format;
    use super::CodedInputStream;
    use super::CodedOutputStream;

    fn test_read<F>(hex: &str, mut callback: F)
        where F : FnMut(&mut CodedInputStream)
    {
        let d = decode_hex(hex);
        let len = d.len();
        let mut reader = io::Cursor::new(d);
        let mut is = CodedInputStream::new(&mut reader as &mut Read);
        assert_eq!(0, is.pos());
        callback(&mut is);
        assert!(is.eof().unwrap());
        assert_eq!(len as u32, is.pos());
    }

    #[test]
    fn test_input_stream_read_raw_byte() {
        test_read("17", |is| {
            assert_eq!(23, is.read_raw_byte().unwrap());
        });
    }

    #[test]
    fn test_input_stream_read_varint() {
        test_read("07", |reader| {
            assert_eq!(7, reader.read_raw_varint32().unwrap());
        });
        test_read("07", |reader| {
            assert_eq!(7, reader.read_raw_varint64().unwrap());
        });
        test_read("96 01", |reader| {
            assert_eq!(150, reader.read_raw_varint32().unwrap());
        });
        test_read("96 01", |reader| {
            assert_eq!(150, reader.read_raw_varint64().unwrap());
        });
    }

    #[test]
    fn test_output_input_stream_read_float() {
        test_read("95 73 13 61", |is| {
            assert_eq!(17e19, is.read_float().unwrap());
        });
    }

    #[test]
    fn test_input_stream_read_double() {
        test_read("40 d5 ab 68 b3 07 3d 46", |is| {
            assert_eq!(23e29, is.read_double().unwrap());
        });
    }

    #[test]
    fn test_input_stream_skip_raw_bytes() {
        test_read("", |reader| {
            reader.skip_raw_bytes(0).unwrap();
        });
        test_read("aa bb", |reader| {
            reader.skip_raw_bytes(2).unwrap();
        });
        test_read("aa bb cc dd ee ff", |reader| {
            reader.skip_raw_bytes(6).unwrap();
        });
    }

    #[test]
    fn test_input_stream_limits() {
        test_read("aa bb cc", |is| {
            let old_limit = is.push_limit(1).unwrap();
            assert_eq!(1, is.bytes_until_limit());
            assert_eq!(&[0xaa as u8], &is.read_raw_bytes(1).unwrap() as &[u8]);
            is.pop_limit(old_limit);
            assert_eq!(&[0xbb as u8, 0xcc], &is.read_raw_bytes(2).unwrap() as &[u8]);
        });
    }

    fn test_write<F>(expected: &str, mut gen: F)
        where F : FnMut(&mut CodedOutputStream) -> ProtobufResult<()>
    {
        let mut v = Vec::new();
        {
            let mut writer = VecWriter::new(&mut v);
            let mut os = CodedOutputStream::new(&mut writer as &mut Write);
            gen(&mut os).unwrap();
            os.flush().unwrap();
        }
        assert_eq!(encode_hex(&decode_hex(expected)), encode_hex(&v));
    }

    #[test]
    fn test_output_stream_write_raw_byte() {
        test_write("a1", |os| {
            os.write_raw_byte(0xa1)
        });
    }

    #[test]
    fn test_output_stream_write_tag() {
        test_write("08", |os| {
            os.write_tag(1, wire_format::WireTypeVarint)
        });
    }

    #[test]
    fn test_output_stream_write_raw_bytes() {
        test_write("00 ab", |os| {
            os.write_raw_bytes(&[0x00, 0xab])
        });
    }

    #[test]
    fn test_output_stream_write_raw_varint32() {
        test_write("96 01", |os| {
            os.write_raw_varint32(150)
        });
    }

    #[test]
    fn test_output_stream_write_raw_varint64() {
        test_write("96 01", |os| {
            os.write_raw_varint64(150)
        });
    }

    #[test]
    fn test_output_stream_write_raw_little_endian32() {
        test_write("f1 e2 d3 c4", |os| {
            os.write_raw_little_endian32(0xc4d3e2f1)
        });
    }

    #[test]
    fn test_output_stream_write_float_no_tag() {
        test_write("95 73 13 61", |os| {
            os.write_float_no_tag(17e19)
        });
    }

    #[test]
    fn test_output_stream_write_double_no_tag() {
        test_write("40 d5 ab 68 b3 07 3d 46", |os| {
            os.write_double_no_tag(23e29)
        });
    }

    #[test]
    fn test_output_stream_write_raw_little_endian64() {
        test_write("f1 e2 d3 c4 b5 a6 07 f8", |os| {
            os.write_raw_little_endian64(0xf807a6b5c4d3e2f1)
        });
    }
}
