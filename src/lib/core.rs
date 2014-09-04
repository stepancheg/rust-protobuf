// TODO: drop all fail!

use std::mem;
use std::raw;
use std::str::from_utf8;
use std::num::Bounded;
use std::fmt;
use std::default::Default;
use std::intrinsics::TypeId;
use std::io::EndOfFile;

use misc::VecWriter;
use misc::VecReader;
use zigzag::decode_zig_zag_32;
use zigzag::decode_zig_zag_64;
use zigzag::encode_zig_zag_32;
use zigzag::encode_zig_zag_64;
use unknown::UnknownValue;
use unknown::UnknownVarint;
use unknown::UnknownFixed64;
use unknown::UnknownFixed32;
use unknown::UnknownLengthDelimited;
use unknown::UnknownValueRef;
use unknown::UnknownVarintRef;
use unknown::UnknownFixed64Ref;
use unknown::UnknownFixed32Ref;
use unknown::UnknownLengthDelimitedRef;
use unknown::UnknownFields;
use clear::Clear;
use reflect::MessageDescriptor;
use reflect::EnumDescriptor;
use reflect::EnumValueDescriptor;

pub mod wire_format {
    pub static TAG_TYPE_BITS: u32 = 3;
    pub static TAG_TYPE_MASK: u32 = (1u << TAG_TYPE_BITS as uint) as u32 - 1;

    #[deriving(PartialEq, Eq, Clone, Show)]
    pub enum WireType {
        WireTypeVarint          = 0,
        WireTypeFixed64         = 1,
        WireTypeLengthDelimited = 2,
        WireTypeStartGroup      = 3,
        WireTypeEndGroup        = 4,
        WireTypeFixed32         = 5,
    }

    impl WireType {
        pub fn new(n: u32) -> WireType {
            match n {
                0 => WireTypeVarint,
                1 => WireTypeFixed64,
                2 => WireTypeLengthDelimited,
                3 => WireTypeStartGroup,
                4 => WireTypeEndGroup,
                5 => WireTypeFixed32,
                _ => fail!("unknown wire type: {}", n)
            }
        }
    }

    pub struct Tag(pub u32);

    impl Tag {
        pub fn value(self) -> u32 {
            match self {
                Tag(value) => value
            }
        }

        pub fn make(field_number: u32, wire_type: WireType) -> Tag {
            Tag((field_number << TAG_TYPE_BITS as uint) as u32 | (wire_type as u32))
        }

        pub fn unpack(self) -> (u32, WireType) {
            (self.field_number(), self.wire_type())
        }

        fn wire_type(self) -> WireType {
            WireType::new(self.value() & TAG_TYPE_MASK)
        }

        pub fn field_number(self) -> u32 {
            let r = (self.value() >> (TAG_TYPE_BITS as uint)) as u32;
            assert!(r > 0, "field number must be positive");
            r
        }
    }

    pub fn tag_unpack(tag: u32) -> (WireType, u32) {
        (Tag(tag).wire_type(), Tag(tag).field_number())
    }

}

pub struct CodedInputStream<'a> {
    buffer: Vec<u8>,
    buffer_size: u32,
    buffer_pos: u32,
    reader: Option<&'a mut Reader + 'a>,
    total_bytes_retired: u32,
    current_limit: u32,
    buffer_size_after_limit: u32,
}

impl<'a> CodedInputStream<'a> {
    pub fn new(reader: &'a mut Reader) -> CodedInputStream<'a> {
        let buffer_len = 4096;
        let mut buffer = Vec::with_capacity(buffer_len);
        unsafe { buffer.set_len(buffer_len); }
        CodedInputStream {
            buffer: buffer,
            buffer_size: 0,
            buffer_pos: 0,
            reader: Some(reader),
            total_bytes_retired: 0,
            current_limit: Bounded::max_value(),
            buffer_size_after_limit: 0,
        }
    }

    fn remaining_in_buffer(&self) -> u32 {
        self.buffer_size - self.buffer_pos
    }

    fn remaining_in_buffer_slice<'a>(&'a self) -> &'a [u8] {
        self.buffer.slice(self.buffer_pos as uint, self.buffer_size as uint)
    }

    fn pos(&self) -> u32 {
        self.total_bytes_retired + self.buffer_pos
    }

    fn bytes_until_limit(&self) -> u32 {
        self.current_limit - self.pos()
    }

    // Refill buffer if buffer is empty.
    // Fails if buffer is not empty.
    // Returns false on EOF, or if limit reached.
    // Otherwize returns true.
    fn refill_buffer(&mut self) -> bool {
        if self.buffer_pos < self.buffer_size {
            fail!("called when buffer is not empty");
        }
        if self.pos() == self.current_limit {
            return false;
        }
        if self.reader.is_none() {
            false
        } else {
            match self.reader {
                Some(ref mut reader) => {
                    self.total_bytes_retired += self.buffer_size;
                    self.buffer_pos = 0;
                    self.buffer_size = 0;

                    let r = reader.read(self.buffer.as_mut_slice());
                    self.buffer_size = match r {
                        Err(ref e) if e.kind == EndOfFile => return false,
                        Err(_) => fail!(),
                        Ok(x) => x as u32,
                    };
                    assert!(self.buffer_size > 0);
                },
                None => fail!(),
            }
            self.recompute_buffer_size_after_limit();
            true
        }
    }

    fn refill_buffer_really(&mut self) {
        if !self.refill_buffer() {
            fail!("at EOF");
        }
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

    pub fn push_limit(&mut self, limit: u32) -> u32 {
        let old_limit = self.current_limit;
        let new_limit = self.pos() + limit;
        if new_limit > old_limit {
            fail!("truncated message");
        }
        self.current_limit = new_limit;
        self.recompute_buffer_size_after_limit();
        old_limit
    }

    pub fn pop_limit(&mut self, old_limit: u32) {
        if self.bytes_until_limit() != 0 {
            fail!("must pop only at current limit")
        }
        self.current_limit = old_limit;
        self.recompute_buffer_size_after_limit();
    }

    pub fn eof(&mut self) -> bool {
        return self.buffer_pos == self.buffer_size && !self.refill_buffer()
    }

    pub fn read_raw_byte(&mut self) -> u8 {
        if self.buffer_pos == self.buffer_size {
            self.refill_buffer_really();
        }
        assert!(self.buffer_pos < self.buffer_size);
        let r = self.buffer[self.buffer_pos as uint];
        self.buffer_pos += 1;
        r
    }

    pub fn read_raw_varint64(&mut self) -> u64 {
        let mut r: u64 = 0;
        let mut i = 0;
        loop {
            let b = self.read_raw_byte();
            // Stop undefined behaviour
            if i <= 9 {
                r = r | (((b & 0x7f) as u64) << (i * 7));
                i += 1;
            }
            if b < 0x80 {
                return r;
            }
        }
    }

    pub fn read_raw_varint32(&mut self) -> u32 {
        self.read_raw_varint64() as u32
    }

    pub fn read_raw_little_endian32(&mut self) -> u32 {
        let mut bytes = [0u32, ..4];
        for i in range(0u, 4) {
            bytes[i] = self.read_raw_byte() as u32;
        }
        (bytes[0]      ) |
        (bytes[1] <<  8) |
        (bytes[2] << 16) |
        (bytes[3] << 24)
    }

    pub fn read_raw_little_endian64(&mut self) -> u64 {
        let mut bytes = [0u64, ..8];
        for i in range(0u, 8) {
            bytes[i] = self.read_raw_byte() as u64;
        }
        (bytes[0]      ) |
        (bytes[1] <<  8) |
        (bytes[2] << 16) |
        (bytes[3] << 24) |
        (bytes[4] << 32) |
        (bytes[5] << 40) |
        (bytes[6] << 48) |
        (bytes[7] << 56)
    }

    pub fn read_tag(&mut self) -> wire_format::Tag {
        wire_format::Tag(self.read_raw_varint32())
    }

    // Read tag, return it is pair (field number, wire type)
    pub fn read_tag_unpack(&mut self) -> (u32, wire_format::WireType) {
        self.read_tag().unpack()
    }

    pub fn read_double(&mut self) -> f64 {
        let bits = self.read_raw_little_endian64();
        unsafe {
            mem::transmute::<u64, f64>(bits)
        }
    }

    pub fn read_float(&mut self) -> f32 {
        let bits = self.read_raw_little_endian32();
        unsafe {
            mem::transmute::<u32, f32>(bits)
        }
    }

    pub fn read_int64(&mut self) -> i64 {
        self.read_raw_varint64() as i64
    }

    pub fn read_int32(&mut self) -> i32 {
        self.read_raw_varint32() as i32
    }

    pub fn read_uint64(&mut self) -> u64 {
        self.read_raw_varint64()
    }

    pub fn read_uint32(&mut self) -> u32 {
        self.read_raw_varint32()
    }

    pub fn read_sint64(&mut self) -> i64 {
        decode_zig_zag_64(self.read_uint64())
    }

    pub fn read_sint32(&mut self) -> i32 {
        decode_zig_zag_32(self.read_uint32())
    }

    pub fn read_fixed64(&mut self) -> u64 {
        self.read_raw_little_endian64()
    }

    pub fn read_fixed32(&mut self) -> u32 {
        self.read_raw_little_endian32()
    }

    pub fn read_sfixed64(&mut self) -> i64 {
        self.read_raw_little_endian64() as i64
    }

    pub fn read_sfixed32(&mut self) -> i32 {
        self.read_raw_little_endian32() as i32
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_raw_varint32() != 0
    }

    pub fn read_unknown(&mut self, wire_type: wire_format::WireType) -> UnknownValue {
        match wire_type {
            wire_format::WireTypeVarint => { UnknownVarint(self.read_raw_varint64()) },
            wire_format::WireTypeFixed64 => { UnknownFixed64(self.read_fixed64()) },
            wire_format::WireTypeFixed32 => { UnknownFixed32(self.read_fixed32()) } ,
            wire_format::WireTypeLengthDelimited => {
                let len = self.read_raw_varint32();
                UnknownLengthDelimited(self.read_raw_bytes(len))
            },
            _ => fail!("unknown wire type: {:i}", wire_type as int)
        }
    }

    pub fn skip_field(&mut self, wire_type: wire_format::WireType) {
        self.read_unknown(wire_type);
    }

    /// Read raw bytes into supplied vector. Vector must be empty.
    pub fn read_raw_bytes_into(&mut self, count: u32, target: &mut Vec<u8>) {
        assert!(target.is_empty());
        target.reserve_additional(count as uint);
        while target.len() < count as uint {
            let rem = count - target.len() as u32;
            if rem <= self.remaining_in_buffer() {
                target.push_all(self.buffer.slice(self.buffer_pos as uint, (self.buffer_pos + rem) as uint));
                self.buffer_pos += rem;
            } else {
                target.push_all(self.remaining_in_buffer_slice());
                self.buffer_pos = self.buffer_size;
                self.refill_buffer_really();
            }
        }
    }

    /// Read exact number of bytes
    pub fn read_raw_bytes(&mut self, count: u32) -> Vec<u8> {
        let mut r = Vec::new();
        self.read_raw_bytes_into(count, &mut r);
        r
    }

    pub fn skip_raw_bytes(&mut self, count: u32) {
        // TODO: make it more efficient
        self.read_raw_bytes(count);
    }

    pub fn read_bytes(&mut self) -> Vec<u8> {
        let mut r = Vec::new();
        self.read_bytes_into(&mut r);
        r
    }

    pub fn read_bytes_into(&mut self, target: &mut Vec<u8>) {
        let len = self.read_raw_varint32();
        self.read_raw_bytes_into(len, target);
    }

    pub fn read_string(&mut self) -> String {
        let mut r = String::new();
        self.read_string_into(&mut r);
        r
    }

    pub fn read_string_into(&mut self, target: &mut String) {
        // assert string is empty, otherwize UTF-8 validation is too expensive
        assert!(target.is_empty());
        // take target's buffer
        let mut vec = mem::replace(target, String::new()).into_bytes();
        self.read_bytes_into(&mut vec);
        // crash if bytes are not valid UTF-8
        mem::replace(target, String::from_utf8(vec).unwrap());
    }

    pub fn merge_message<M : Message>(&mut self, message: &mut M) {
        let len = self.read_raw_varint32();
        let old_limit = self.push_limit(len);
        message.merge_from(self);
        self.pop_limit(old_limit);
    }

    pub fn read_message<M : Message>(&mut self) -> M {
        let mut r: M = Message::new();
        self.merge_message(&mut r);
        r.check_initialized();
        r
    }
}

trait WithCodedOutputStream {
    fn with_coded_output_stream<T>(self, cb: |&mut CodedOutputStream| -> T) -> T;
}

impl<'a> WithCodedOutputStream for &'a mut Writer + 'a {
    fn with_coded_output_stream<T>(self, cb: |&mut CodedOutputStream| -> T) -> T {
        let mut os = CodedOutputStream::new(self);
        let r = cb(&mut os);
        os.flush();
        r
    }
}

fn with_coded_output_stream_to_bytes(cb: |&mut CodedOutputStream|) -> Vec<u8> {
    let mut w = VecWriter::new();
    (&mut w as &mut Writer).with_coded_output_stream(|os| {
        cb(os)
    });
    w.vec
}

trait WithCodedInputStream {
    fn with_coded_input_stream<T>(self, cb: |&mut CodedInputStream| -> T) -> T;
}

impl<'a> WithCodedInputStream for &'a mut Reader + 'a {
    fn with_coded_input_stream<T>(self, cb: |&mut CodedInputStream| -> T) -> T {
        let mut is = CodedInputStream::new(self);
        let r = cb(&mut is);
        // reading from Reader requires all data to be read,
        // because CodedInputStream caches data, and otherwize
        // buffer would be discarded
        assert!(is.eof());
        r
    }
}

impl<'a> WithCodedInputStream for &'a [u8] {
    fn with_coded_input_stream<T>(self, cb: |&mut CodedInputStream| -> T) -> T {
        let mut reader = VecReader::new(Vec::from_slice(self));
        (&mut reader as &mut Reader).with_coded_input_stream(|is| {
            cb(is)
        })
    }
}


pub struct CodedOutputStream<'a> {
    buffer: Vec<u8>,
    // within buffer
    position: u32,
    writer: Option<&'a mut Writer + 'a>,
    sizes: Vec<u32>, // used by Message::write_to
}

impl<'a> CodedOutputStream<'a> {
    pub fn new(writer: &'a mut Writer) -> CodedOutputStream<'a> {
        let buffer_len = 4096;
        let mut buffer = Vec::with_capacity(buffer_len);
        unsafe { buffer.set_len(buffer_len); }
        CodedOutputStream {
            buffer: buffer,
            position: 0,
            writer: Some(writer),
            sizes: Vec::new(),
        }
    }

    fn refresh_buffer(&mut self) {
        match self.writer {
            Some(ref mut writer) => {
                writer.write(self.buffer.slice(0, self.position as uint)).unwrap();
            },
            None => fail!()
        };
        self.position = 0;
    }

    pub fn flush(&mut self) {
        if self.writer.is_some() {
            self.refresh_buffer();
        }
    }

    pub fn write_raw_byte(&mut self, byte: u8) {
        if self.position as uint == self.buffer.len() {
            self.refresh_buffer()
        }
        self.buffer.as_mut_slice()[self.position as uint] = byte;
        self.position += 1;
    }

    pub fn write_raw_bytes(&mut self, bytes: &[u8]) {
        self.refresh_buffer();
        match self.writer {
            Some(ref mut writer) => writer.write(bytes).unwrap(),
            None => fail!()
        };
    }

    pub fn write_tag(&mut self, field_number: u32, wire_type: wire_format::WireType) {
        self.write_raw_varint32(wire_format::Tag::make(field_number, wire_type).value());
    }

    pub fn write_raw_varint32(&mut self, value: u32) {
        self.write_raw_varint64(value as u64);
    }

    pub fn write_raw_varint64(&mut self, value: u64) {
        let mut temp = value;
        loop {
            if (temp & !0x7Fu64) == 0 {
                self.write_raw_byte(temp as u8);
                break;
            } else {
                self.write_raw_byte(((temp & 0x7F) | 0x80) as u8);
                temp >>= 7;
            }
        }
    }

    pub fn write_raw_little_endian32(&mut self, value: u32) {
        self.write_raw_byte(((value      ) & 0xFF) as u8);
        self.write_raw_byte(((value >>  8) & 0xFF) as u8);
        self.write_raw_byte(((value >> 16) & 0xFF) as u8);
        self.write_raw_byte(((value >> 24) & 0xFF) as u8);
    }

    pub fn write_raw_little_endian64(&mut self, value: u64) {
        self.write_raw_byte(((value      ) & 0xFF) as u8);
        self.write_raw_byte(((value >>  8) & 0xFF) as u8);
        self.write_raw_byte(((value >> 16) & 0xFF) as u8);
        self.write_raw_byte(((value >> 24) & 0xFF) as u8);
        self.write_raw_byte(((value >> 32) & 0xFF) as u8);
        self.write_raw_byte(((value >> 40) & 0xFF) as u8);
        self.write_raw_byte(((value >> 48) & 0xFF) as u8);
        self.write_raw_byte(((value >> 56) & 0xFF) as u8);
    }

    pub fn write_float_no_tag(&mut self, value: f32) {
        let bits = unsafe {
            mem::transmute::<f32, u32>(value)
        };
        self.write_raw_little_endian32(bits);
    }

    pub fn write_double_no_tag(&mut self, value: f64) {
        let bits = unsafe {
            mem::transmute::<f64, u64>(value)
        };
        self.write_raw_little_endian64(bits);
    }

    pub fn write_float(&mut self, field_number: u32, value: f32) {
        self.write_tag(field_number, wire_format::WireTypeFixed32);
        self.write_float_no_tag(value);
    }

    pub fn write_double(&mut self, field_number: u32, value: f64) {
        self.write_tag(field_number, wire_format::WireTypeFixed64);
        self.write_double_no_tag(value);
    }

    pub fn write_uint64_no_tag(&mut self, value: u64) {
        self.write_raw_varint64(value);
    }

    pub fn write_uint32_no_tag(&mut self, value: u32) {
        self.write_raw_varint32(value);
    }

    pub fn write_int64_no_tag(&mut self, value: i64) {
        self.write_raw_varint64(value as u64);
    }

    pub fn write_int32_no_tag(&mut self, value: i32) {
        self.write_raw_varint64(value as u64);
    }

    pub fn write_sint64_no_tag(&mut self, value: i64) {
        self.write_uint64_no_tag(encode_zig_zag_64(value));
    }

    pub fn write_sint32_no_tag(&mut self, value: i32) {
        self.write_uint32_no_tag(encode_zig_zag_32(value));
    }

    pub fn write_fixed64_no_tag(&mut self, value: u64) {
        self.write_raw_little_endian64(value);
    }

    pub fn write_fixed32_no_tag(&mut self, value: u32) {
        self.write_raw_little_endian32(value);
    }

    pub fn write_sfixed64_no_tag(&mut self, value: i64) {
        self.write_raw_little_endian64(value as u64);
    }

    pub fn write_sfixed32_no_tag(&mut self, value: i32) {
        self.write_raw_little_endian32(value as u32);
    }

    pub fn write_bool_no_tag(&mut self, value: bool) {
        self.write_raw_varint32(if value { 1 } else { 0 });
    }

    pub fn write_enum_no_tag(&mut self, value: i32) {
        self.write_int32_no_tag(value);
    }

    pub fn write_unknown_no_tag(&mut self, unknown: UnknownValueRef) {
        match unknown {
            UnknownFixed64Ref(fixed64) => self.write_raw_little_endian64(fixed64),
            UnknownFixed32Ref(fixed32) => self.write_raw_little_endian32(fixed32),
            UnknownVarintRef(varint) => self.write_raw_varint64(varint),
            UnknownLengthDelimitedRef(bytes) => self.write_bytes_no_tag(bytes),
        }
    }

    pub fn write_uint64(&mut self, field_number: u32, value: u64) {
        self.write_tag(field_number, wire_format::WireTypeVarint);
        self.write_uint64_no_tag(value);
    }

    pub fn write_uint32(&mut self, field_number: u32, value: u32) {
        self.write_tag(field_number, wire_format::WireTypeVarint);
        self.write_uint32_no_tag(value);
    }

    pub fn write_int64(&mut self, field_number: u32, value: i64) {
        self.write_tag(field_number, wire_format::WireTypeVarint);
        self.write_int64_no_tag(value);
    }

    pub fn write_int32(&mut self, field_number: u32, value: i32) {
        self.write_tag(field_number, wire_format::WireTypeVarint);
        self.write_int32_no_tag(value);
    }

    pub fn write_sint64(&mut self, field_number: u32, value: i64) {
        self.write_tag(field_number, wire_format::WireTypeVarint);
        self.write_sint64_no_tag(value);
    }

    pub fn write_sint32(&mut self, field_number: u32, value: i32) {
        self.write_tag(field_number, wire_format::WireTypeVarint);
        self.write_sint32_no_tag(value);
    }

    pub fn write_fixed64(&mut self, field_number: u32, value: u64) {
        self.write_tag(field_number, wire_format::WireTypeFixed64);
        self.write_fixed64_no_tag(value);
    }

    pub fn write_fixed32(&mut self, field_number: u32, value: u32) {
        self.write_tag(field_number, wire_format::WireTypeFixed32);
        self.write_fixed32_no_tag(value);
    }

    pub fn write_sfixed64(&mut self, field_number: u32, value: i64) {
        self.write_tag(field_number, wire_format::WireTypeFixed64);
        self.write_sfixed64_no_tag(value);
    }

    pub fn write_sfixed32(&mut self, field_number: u32, value: i32) {
        self.write_tag(field_number, wire_format::WireTypeFixed32);
        self.write_sfixed32_no_tag(value);
    }

    pub fn write_bool(&mut self, field_number: u32, value: bool) {
        self.write_tag(field_number, wire_format::WireTypeVarint);
        self.write_bool_no_tag(value);
    }

    pub fn write_enum(&mut self, field_number: u32, value: i32) {
        self.write_tag(field_number, wire_format::WireTypeVarint);
        self.write_enum_no_tag(value);
    }

    pub fn write_unknown(&mut self, field_number: u32, value: UnknownValueRef) {
        self.write_tag(field_number, value.wire_type());
        self.write_unknown_no_tag(value);
    }

    pub fn write_unknown_fields(&mut self, fields: &UnknownFields) {
        for (number, values) in fields.iter() {
            for value in values.iter() {
                self.write_unknown(number, value);
            }
        }
    }

    pub fn write_bytes_no_tag(&mut self, bytes: &[u8]) {
        self.write_raw_varint32(bytes.len() as u32);
        self.write_raw_bytes(bytes);
    }

    pub fn write_string_no_tag(&mut self, s: &str) {
        self.write_bytes_no_tag(s.as_bytes());
    }

    pub fn write_message_no_tag<M : Message>(&mut self, msg: &M) {
        msg.write_length_delimited_to(self);
    }

    pub fn write_bytes(&mut self, field_number: u32, bytes: &[u8]) {
        self.write_tag(field_number, wire_format::WireTypeLengthDelimited);
        self.write_bytes_no_tag(bytes);
    }

    pub fn write_string(&mut self, field_number: u32, s: &str) {
        self.write_tag(field_number, wire_format::WireTypeLengthDelimited);
        self.write_string_no_tag(s);
    }

    pub fn write_message<M : Message>(&mut self, field_number: u32, msg: &M) {
        self.write_tag(field_number, wire_format::WireTypeLengthDelimited);
        self.write_message_no_tag(msg);
    }
}


pub trait Message : PartialEq + Clone + Default + fmt::Show + Clear {
    fn new() -> Self;
    // all required fields set
    fn is_initialized(&self) -> bool;
    fn merge_from(&mut self, is: &mut CodedInputStream);
    fn write_to_with_computed_sizes(&self, os: &mut CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint);
    fn compute_sizes(&self, sizes: &mut Vec<u32>) -> u32;

    fn write_to(&self, os: &mut CodedOutputStream) {
        self.check_initialized();
        let mut sizes = mem::replace(&mut os.sizes, Vec::new());
        assert!(sizes.is_empty());
        self.compute_sizes(&mut sizes);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes.as_slice(), &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
        // TODO: assert we've written same number of bytes as computed
        sizes.truncate(0);
        mem::replace(&mut os.sizes, sizes);
    }

    fn write_length_delimited_to(&self, os: &mut CodedOutputStream) {
        let mut sizes = mem::replace(&mut os.sizes, Vec::new());
        assert!(sizes.is_empty());
        let size = self.compute_sizes(&mut sizes);
        os.write_raw_varint32(size);
        let mut sizes_pos = 1; // first element is self
        self.write_to_with_computed_sizes(os, sizes.as_slice(), &mut sizes_pos);
        assert_eq!(sizes_pos, sizes.len());
        // TODO: assert we've written same number of bytes as computed
        sizes.truncate(0);
        mem::replace(&mut os.sizes, sizes);
    }

    fn serialized_size(&self) -> u32 {
        let mut sizes = Vec::new();
        self.compute_sizes(&mut sizes)
    }

    fn check_initialized(&self) {
        // TODO: report which fields are not initialized
        assert!(self.is_initialized());
    }

    fn write_to_writer(&self, w: &mut Writer) {
        w.with_coded_output_stream(|os| {
            self.write_to(os);
        })
    }

    fn write_to_bytes(&self) -> Vec<u8> {
        with_coded_output_stream_to_bytes(|os| {
            self.write_to(os)
        })
    }

    fn write_length_delimited_to_writer(&self, w: &mut Writer) {
        w.with_coded_output_stream(|os| {
            self.write_length_delimited_to(os);
        })
    }

    fn write_length_delimited_to_bytes(&self) -> Vec<u8> {
        with_coded_output_stream_to_bytes(|os| {
            self.write_length_delimited_to(os);
        })
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s UnknownFields;
    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut UnknownFields;

    fn descriptor(&self) -> &'static MessageDescriptor {
        Message::descriptor_static(None::<Self>)
    }

    // http://stackoverflow.com/q/20342436/15018
    fn descriptor_static(_: Option<Self>) -> &'static MessageDescriptor {
        fail!();
    }

    fn type_id(&self) -> TypeId {
        fail!();
    }

    // Rust does not allow implementation of trait for trait:
    // impl<M : Message> fmt::Show for M {
    // ...
    // }
    fn fmt_impl(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ::text_format::fmt(self, f)
    }
}

pub fn message_is<M : 'static + Message>(m: &Message) -> bool {
    TypeId::of::<M>() == m.type_id()
}

pub fn message_down_cast<'a, M : 'static + Message>(m: &'a Message) -> &'a M {
    assert!(message_is::<M>(m));
    unsafe {
        // TODO: really weird
        let r: raw::TraitObject = mem::transmute(m);
        mem::transmute(r.data)
    }
}


pub trait ProtobufEnum : Eq {
    fn value(&self) -> i32;

    fn descriptor(&self) -> &'static EnumValueDescriptor {
        self.enum_descriptor().value_by_number(self.value())
    }

    fn enum_descriptor(&self) -> &'static EnumDescriptor {
        ProtobufEnum::enum_descriptor_static(None::<Self>)
    }

    // http://stackoverflow.com/q/20342436/15018
    fn enum_descriptor_static(_: Option<Self>) -> &'static EnumDescriptor {
        fail!();
    }
}

pub fn parse_from<M : Message>(is: &mut CodedInputStream) -> M {
    let mut r: M = Message::new();
    r.merge_from(is);
    r.check_initialized();
    r
}

pub fn parse_from_reader<M : Message>(reader: &mut Reader) -> M {
    reader.with_coded_input_stream(|is| {
        parse_from::<M>(is)
    })
}

pub fn parse_from_bytes<M : Message>(bytes: &[u8]) -> M {
    bytes.with_coded_input_stream(|is| {
        parse_from::<M>(is)
    })
}

pub fn parse_length_delimited_from<M : Message>(is: &mut CodedInputStream) -> M {
    is.read_message::<M>()
}

pub fn parse_length_delimited_from_reader<M : Message>(r: &mut Reader) -> M {
    // TODO: wrong: we may read length first, and then read exact number of bytes needed
    r.with_coded_input_stream(|is| {
        is.read_message::<M>()
    })
}

pub fn parse_length_delimited_from_bytes<M : Message>(bytes: &[u8]) -> M {
    bytes.with_coded_input_stream(|is| {
        is.read_message::<M>()
    })
}



#[cfg(test)]
mod test {

    use std::io::MemReader;
    use hex::encode_hex;
    use hex::decode_hex;
    use misc::VecWriter;
    use core::wire_format;
    use super::CodedInputStream;
    use super::CodedOutputStream;

    fn test_read(hex: &str, callback: |&mut CodedInputStream|) {
        let d = decode_hex(hex);
        let len = d.len();
        let mut reader = MemReader::new(Vec::from_slice(d.as_slice()));
        let mut is = CodedInputStream::new(&mut reader as &mut Reader);
        assert_eq!(0, is.pos());
        callback(&mut is);
        assert!(is.eof());
        assert_eq!(len as u32, is.pos());
    }

    #[test]
    fn test_input_stream_read_raw_byte() {
        test_read("17", |is| {
            assert_eq!(23, is.read_raw_byte());
        });
    }

    #[test]
    fn test_input_stream_read_varint() {
        test_read("07", |reader| {
            assert_eq!(7, reader.read_raw_varint32());
        });
        test_read("07", |reader| {
            assert_eq!(7, reader.read_raw_varint64());
        });
        test_read("96 01", |reader| {
            assert_eq!(150, reader.read_raw_varint32());
        });
        test_read("96 01", |reader| {
            assert_eq!(150, reader.read_raw_varint64());
        });
    }

    #[test]
    fn test_output_input_stream_read_float() {
        test_read("95 73 13 61", |is| {
            assert_eq!(17e19, is.read_float());
        });
    }

    #[test]
    fn test_input_stream_read_double() {
        test_read("40 d5 ab 68 b3 07 3d 46", |is| {
            assert_eq!(23e29, is.read_double());
        });
    }

    #[test]
    fn test_input_stream_skip_raw_bytes() {
        test_read("", |reader| {
            reader.skip_raw_bytes(0);
        });
        test_read("aa bb", |reader| {
            reader.skip_raw_bytes(2);
        });
        test_read("aa bb cc dd ee ff", |reader| {
            reader.skip_raw_bytes(6);
        });
    }

    #[test]
    fn test_input_stream_limits() {
        test_read("aa bb cc", |is| {
            let old_limit = is.push_limit(1);
            assert_eq!(1, is.bytes_until_limit());
            assert_eq!([0xaa].as_slice(), is.read_raw_bytes(1).as_slice());
            is.pop_limit(old_limit);
            assert_eq!([0xbb, 0xcc].as_slice(), is.read_raw_bytes(2).as_slice());
        });
    }

    fn test_write(expected: &str, gen: |&mut CodedOutputStream|) {
        let mut writer = VecWriter::new();
        {
            let mut os = CodedOutputStream::new(&mut writer as &mut Writer);
            gen(&mut os);
            os.flush();
        }
        let r = writer.vec;
        assert_eq!(encode_hex(decode_hex(expected).as_slice()), encode_hex(r.as_slice()));
    }

    #[test]
    fn test_output_stream_write_raw_byte() {
        test_write("a1", |os| {
            os.write_raw_byte(0xa1);
        });
    }

    #[test]
    fn test_output_stream_write_tag() {
        test_write("08", |os| {
            os.write_tag(1, wire_format::WireTypeVarint);
        });
    }

    #[test]
    fn test_output_stream_write_raw_bytes() {
        test_write("00 ab", |os| {
            os.write_raw_bytes([0x00, 0xab]);
        });
    }

    #[test]
    fn test_output_stream_write_raw_varint32() {
        test_write("96 01", |os| {
            os.write_raw_varint32(150);
        });
    }

    #[test]
    fn test_output_stream_write_raw_varint64() {
        test_write("96 01", |os| {
            os.write_raw_varint64(150);
        });
    }

    #[test]
    fn test_output_stream_write_raw_little_endian32() {
        test_write("f1 e2 d3 c4", |os| {
            os.write_raw_little_endian32(0xc4d3e2f1);
        });
    }

    #[test]
    fn test_output_stream_write_float_no_tag() {
        test_write("95 73 13 61", |os| {
            os.write_float_no_tag(17e19);
        });
    }

    #[test]
    fn test_output_stream_write_double_no_tag() {
        test_write("40 d5 ab 68 b3 07 3d 46", |os| {
            os.write_double_no_tag(23e29);
        });
    }

    #[test]
    fn test_output_stream_write_raw_little_endian64() {
        test_write("f1 e2 d3 c4 b5 a6 07 f8", |os| {
            os.write_raw_little_endian64(0xf807a6b5c4d3e2f1);
        });
    }
}
