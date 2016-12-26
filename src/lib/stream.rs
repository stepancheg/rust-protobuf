use std;
use std::mem;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::io::Write;
use std::ptr;
use std::slice;

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

// If an input stream is constructed with a `Read`, we create a
// `BufReader` with an internal buffer of this size.
const INPUT_STREAM_BUFFER_SIZE: usize = 4096;

// Equal to the default buffer size of `BufWriter`, so when
// `CodedOutputStream` wraps `BufWriter`, it often skips double buffering.
const OUTPUT_STREAM_BUFFER_SIZE: usize = 8 * 1024;


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

enum InputSource<'a> {
    BufRead(&'a mut BufRead),
    Read(BufReader<&'a mut Read>),
    Cursor(io::Cursor<&'a [u8]>),
}

impl<'a> InputSource<'a> {
    pub fn read(&mut self, target: &mut [u8]) -> ProtobufResult<()> {
        let mut target = target;
        let count = target.len();
        let mut bread = 0;
        while bread != count {
            let res = match self {
                &mut InputSource::BufRead(ref mut br) => br.read(&mut target[bread..]),
                &mut InputSource::Read(ref mut br) => br.read(&mut target[bread..]),
                &mut InputSource::Cursor(ref mut c) => c.read(&mut target[bread..]),
            };
            match try!(res) {
                0 => {
                    return Err(ProtobufError::IoError(io::Error::new(
                        io::ErrorKind::Other, "unexpected EOF")));
                },
                x => {
                    bread += x;
                },
            }
        }
        Ok(())
    }

    pub fn eof(&mut self) -> ProtobufResult<bool> {
        let res = match self {
            &mut InputSource::BufRead(ref mut br) => br.fill_buf(),
            &mut InputSource::Read(ref mut br) => br.fill_buf(),
            &mut InputSource::Cursor(ref mut c) => c.fill_buf(),
        };
        Ok(try!(res).len() == 0)
    }
}

const NO_LIMIT: u32 = std::u32::MAX;

pub struct CodedInputStream<'a> {
    source: InputSource<'a>,
    current_limit: u32,
    pos: u32,
}

impl<'a> CodedInputStream<'a> {
    pub fn new(reader: &'a mut Read) -> CodedInputStream<'a> {
        CodedInputStream {
            source: InputSource::Read(BufReader::with_capacity(
                INPUT_STREAM_BUFFER_SIZE, reader)),
            current_limit: NO_LIMIT,
            pos: 0,
        }
    }

    pub fn from_buffered_reader(buffered_reader: &'a mut BufRead) -> CodedInputStream<'a> {
        CodedInputStream {
            source: InputSource::BufRead(buffered_reader),
            current_limit: NO_LIMIT,
            pos: 0,
        }
    }

    pub fn from_bytes(bytes: &'a [u8]) -> CodedInputStream<'a> {
        let len = bytes.len();
        assert!(len < NO_LIMIT as usize);
        CodedInputStream {
            source: InputSource::Cursor(io::Cursor::new(bytes)),
            current_limit: len as u32,
            pos: 0,
        }
    }

    pub fn pos(&self) -> u32 { self.pos }

    pub fn bytes_until_limit(&self) -> u32 {
        assert!(self.current_limit != NO_LIMIT);
        self.current_limit - self.pos
    }

    pub fn read(&mut self, buf: &mut[u8]) -> ProtobufResult<()> {
        assert!(buf.len() < NO_LIMIT as usize);
        let new_pos = match self.pos.checked_add(buf.len() as u32) {
            None | Some(NO_LIMIT) =>
                return Err(ProtobufError::WireError(format!("u32 overflow"))),
            Some(new_pos) => new_pos,
        };
        try!(self.source.read(buf));
        self.pos = new_pos;
        Ok(())
    }

    pub fn read_raw_byte(&mut self) -> ProtobufResult<u8> {
        let mut r = 0u8;
        let bytes: &mut [u8] = unsafe {
            let p: *mut u8 = mem::transmute(&mut r);
            slice::from_raw_parts_mut(p, mem::size_of::<u8>())
        };
        try!(self.read(bytes));
        Ok(r)
    }

    pub fn push_limit(&mut self, limit: u32) -> ProtobufResult<u32> {
        let old_limit = self.current_limit;
        let new_limit = match self.pos.checked_add(limit) {
            None | Some(NO_LIMIT) => return Err(ProtobufError::WireError(format!("corrupted stream"))),
            Some(new_limit) => new_limit,
        };
        if old_limit != NO_LIMIT && new_limit > old_limit {
            return Err(ProtobufError::WireError(format!("truncated message")));
        }
        self.current_limit = new_limit;
        Ok(old_limit)
    }

    pub fn pop_limit(&mut self, old_limit: u32) {
        self.current_limit = old_limit;
    }

    pub fn eof(&mut self) -> ProtobufResult<bool> {
        assert!(self.pos <= self.current_limit);
        if self.current_limit == NO_LIMIT {
            self.source.eof()
        } else {
            Ok(self.pos == self.current_limit)
        }
    }

    pub fn check_eof(&mut self) -> ProtobufResult<()> {
        let eof = try!(self.eof());
        if !eof {
            return Err(ProtobufError::WireError(format!("expecting EOF")));
        }
        Ok(())
    }

    pub fn read_raw_varint64(&mut self) -> ProtobufResult<u64> {
        let mut r: u64 = 0;
        let mut i = 0;
        loop {
            if i == 10 {
                return Err(ProtobufError::WireError(format!("invalid varint")));
            }
            let b = try!(self.read_raw_byte());
            // TODO: may overflow if i == 9
            r = r | (((b & 0x7f) as u64) << (i * 7));
            i += 1;
            if b < 0x80 {
                return Ok(r);
            }
        }
    }

    pub fn read_raw_varint32(&mut self) -> ProtobufResult<u32> {
        self.read_raw_varint64().map(|v| v as u32)
    }


    pub fn read_raw_little_endian32(&mut self) -> ProtobufResult<u32> {
        let mut r = 0u32;
        let bytes: &mut [u8] = unsafe {
            let p: *mut u8 = mem::transmute(&mut r);
            slice::from_raw_parts_mut(p, mem::size_of::<u32>())
        };
        try!(self.read(bytes));
        Ok(r.to_le())
    }

    pub fn read_raw_little_endian64(&mut self) -> ProtobufResult<u64> {
        let mut r = 0u64;
        let bytes: &mut [u8] = unsafe {
            let p: *mut u8 = mem::transmute(&mut r);
            slice::from_raw_parts_mut(p, mem::size_of::<u64>())
        };
        try!(self.read(bytes));
        Ok(r.to_le())
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

    /// Read raw bytes into the supplied vector.  The vector will be resized as needed and
    /// overwritten.
    pub fn read_raw_bytes_into(&mut self, count: u32, target: &mut Vec<u8>) -> ProtobufResult<()> {
        unsafe { target.set_len(0); }
        target.reserve(count as usize);
        unsafe { target.set_len(count as usize); }
        try!(self.read(target));
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
        try!(r.check_initialized());
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
    fn with_coded_output_stream<T, F>(mut self, cb: F)
            -> ProtobufResult<T>
        where F : FnOnce(&mut CodedOutputStream) -> ProtobufResult<T>
    {
        (&mut self as &mut Write).with_coded_output_stream(cb)
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
        try!(is.check_eof());
        Ok(r)
    }
}

impl<'a> WithCodedInputStream for &'a mut (BufRead + 'a) {
    fn with_coded_input_stream<T, F>(self, cb: F) -> ProtobufResult<T>
        where F : FnOnce(&mut CodedInputStream) -> ProtobufResult<T>
    {
        let mut is = CodedInputStream::from_buffered_reader(self);
        let r = try!(cb(&mut is));
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
    buffer: Box<[u8]>,
    // within buffer
    position: u32,
    writer: Option<&'a mut (Write + 'a)>,
}

impl<'a> CodedOutputStream<'a> {
    pub fn new(writer: &'a mut Write) -> CodedOutputStream<'a> {
        let buffer_len = OUTPUT_STREAM_BUFFER_SIZE;
        let mut buffer = Vec::with_capacity(buffer_len);
        unsafe { buffer.set_len(buffer_len); }
        CodedOutputStream {
            buffer: buffer.into_boxed_slice(),
            position: 0,
            writer: Some(writer),
        }
    }

    fn refresh_buffer(&mut self) -> ProtobufResult<()> {
        match self.writer {
            Some(ref mut writer) => {
                try!(writer.write_all(&self.buffer[0..self.position as usize]));
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
        if bytes.len() <= self.buffer.len() - self.position as usize {
            // TODO: use `copy_from_slice` as soon as rust 1.9 released
            unsafe {
                let dest = &mut self.buffer[self.position as usize..];
                ptr::copy_nonoverlapping(bytes.as_ptr(), dest.as_mut_ptr(), bytes.len());
                self.position += bytes.len() as u32;
                return Ok(());
            }
        }

        try!(self.refresh_buffer());
        match self.writer {
            Some(ref mut writer) => try!(writer.write_all(bytes)),
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

    pub fn write_enum_obj_no_tag<E>(&mut self, value: E) -> ProtobufResult<()>
        where E : ProtobufEnum
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

    pub fn write_enum_obj<E>(&mut self, field_number: u32, value: E) -> ProtobufResult<()>
        where E : ProtobufEnum
    {
        self.write_enum(field_number, value.value())
    }

    pub fn write_unknown(&mut self, field_number: u32, value: UnknownValueRef) -> ProtobufResult<()> {
        try!(self.write_tag(field_number, value.wire_type()));
        try!(self.write_unknown_no_tag(value));
        Ok(())
    }

    pub fn write_unknown_fields(&mut self, fields: &UnknownFields) -> ProtobufResult<()> {
        for (number, values) in fields {
            for value in values {
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
    use std::io::{BufRead};
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
        where F : FnMut(&mut CodedInputStream)
    {
        let d = decode_hex(hex);
        let mut reader = io::Cursor::new(d);
        let mut is = CodedInputStream::from_buffered_reader(&mut reader as &mut BufRead);
        assert_eq!(0, is.pos());
        callback(&mut is);
    }

    fn test_read<F>(hex: &str, mut callback: F)
        where F : FnMut(&mut CodedInputStream)
    {
        let len = decode_hex(hex).len();
        test_read_partial(hex, |reader| {
            callback(reader);
            assert!(reader.eof().unwrap());
            assert_eq!(len as u32, reader.pos());
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

        test_read_v("ff ff ff ff ff ff ff ff ff 01", 0xffffffffffffffff,
            |reader| reader.read_raw_varint64());

        test_read_v("ff ff ff ff 0f", 0xffffffff, |reader| reader.read_raw_varint32());
        test_read_v("ff ff ff ff 0f", 0xffffffff, |reader| reader.read_raw_varint64());
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
                // TODO: make unexpected EOF an enum variant
                Err(ProtobufError::IoError(..)) => (),
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
            let r1 = is.read_raw_bytes(1).unwrap();
            assert_eq!(&[0xaa as u8], &r1[..]);
            is.pop_limit(old_limit);
            let r2 = is.read_raw_bytes(2).unwrap();
            assert_eq!(&[0xbb as u8, 0xcc], &r2[..]);
        });
    }

    fn test_write<F>(expected: &str, mut gen: F)
        where F : FnMut(&mut CodedOutputStream) -> ProtobufResult<()>
    {
        let mut v = Vec::new();
        {
            let mut os = CodedOutputStream::new(&mut v as &mut Write);
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

        let expected = repeat("01 02 03 04").take(2048).collect::<Vec<_>>().join(" ");
        test_write(&expected, |os| {
            for _ in 0..2048 {
                try!(os.write_raw_bytes(&[0x01, 0x02, 0x03, 0x04]));
            }

            Ok(())
        });
    }

    #[test]
    fn test_output_stream_write_raw_varint32() {
        test_write("96 01", |os| {
            os.write_raw_varint32(150)
        });
        test_write("ff ff ff ff 0f", |os| {
            os.write_raw_varint32(0xffffffff)
        });
    }

    #[test]
    fn test_output_stream_write_raw_varint64() {
        test_write("96 01", |os| {
            os.write_raw_varint64(150)
        });
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
