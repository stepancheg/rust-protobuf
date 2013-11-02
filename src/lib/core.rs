// TODO: drop all fail!

use std::u32;
use std::cast;
use std::str::from_utf8_owned;
use std::rt::io::*;

use misc::*;
use zigzag::*;

pub mod wire_format {
    pub static TAG_TYPE_BITS: u32 = 3;
    pub static TAG_TYPE_MASK: u32 = (1 << TAG_TYPE_BITS) - 1;

    #[deriving(Eq, Clone)]
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
                _ => fail!("unknown wire type")
            }
        }
    }

    pub struct Tag(u32);

    impl Tag {
        fn value(self) -> u32 {
            match self {
                Tag(value) => value
            }
        }

        pub fn make(field_number: u32, wire_type: WireType) -> Tag {
            Tag((field_number << TAG_TYPE_BITS) | (wire_type as u32))
        }

        pub fn unpack(self) -> (u32, WireType) {
            (self.field_number(), self.wire_type())
        }

        fn wire_type(self) -> WireType {
            WireType::new(self.value() & TAG_TYPE_MASK)
        }

        pub fn field_number(self) -> u32 {
            let r = self.value() >> TAG_TYPE_BITS;
            assert!(r > 0, "field number must be positive");
            r
        }
    }

    pub fn tag_unpack(tag: u32) -> (WireType, u32) {
        (Tag(tag).wire_type(), Tag(tag).field_number())
    }

}

pub struct CodedInputStream {
    buffer: ~[u8],
    buffer_size: u32,
    buffer_pos: u32,
    reader: Option<@Reader>,
    total_bytes_retired: u32,
    current_limit: u32,
    buffer_size_after_limit: u32,
}

impl CodedInputStream {
    pub fn new(reader: @Reader) -> CodedInputStream {
        CodedInputStream {
            // TODO: buffer of size 1 is used, because
            // impl Reader for FILE* (that is io::stdin()) does not not stop
            // reading until buffer is full of EOF reached
            // This makes reading from pipe practically impossible.
            buffer: ~[0, ..1],
            buffer_size: 0,
            buffer_pos: 0,
            reader: Some(reader),
            total_bytes_retired: 0,
            current_limit: u32::max_value,
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
        match self.reader {
            Some(reader) => {
                self.total_bytes_retired += self.buffer_size;
                self.buffer_pos = 0;
                let mut_reader: @mut Reader = unsafe { cast::transmute(reader) };
                self.buffer_size = do io_error::cond.trap(|e| {
                    if e.kind != EndOfFile {
                        io_error::cond.raise(e);
                    };
                }).inside {
                    mut_reader.read(self.buffer).unwrap_or(0) as u32
                };
                if self.buffer_size == 0 {
                    return false;
                }
                self.recompute_buffer_size_after_limit();
                true
            },
            None => false,
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
        let r = self.buffer[self.buffer_pos];
        self.buffer_pos += 1;
        r
    }

    pub fn read_raw_varint64(&mut self) -> u64 {
        let mut bytes: ~[u8] = ~[];
        loop {
            let b = self.read_raw_byte();
            bytes.push(b & 0x7F);
            if b < 0x80 {
                break;
            }
        }
        let mut r = 0u64;
        for i in range(0, bytes.len()) {
            r = (r << 7) | bytes[bytes.len() - i - 1] as u64;
        }
        r
    }

    pub fn read_raw_varint32(&mut self) -> u32 {
        self.read_raw_varint64() as u32
    }

    pub fn read_raw_little_endian32(&mut self) -> u32 {
        let mut bytes = [0u32, ..4];
        for i in range(0, 4) {
            bytes[i] = self.read_raw_byte() as u32;
        }
        (bytes[0]      ) |
        (bytes[1] <<  8) |
        (bytes[2] << 16) |
        (bytes[3] << 24)
    }

    pub fn read_raw_little_endian64(&mut self) -> u64 {
        let mut bytes = [0u64, ..8];
        for i in range(0, 8) {
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
            cast::transmute::<u64, f64>(bits)
        }
    }

    pub fn read_float(&mut self) -> f32 {
        let bits = self.read_raw_little_endian32();
        unsafe {
            cast::transmute::<u32, f32>(bits)
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

    pub fn skip_field(&mut self, wire_type: wire_format::WireType) {
        match wire_type {
            wire_format::WireTypeVarint => { self.read_raw_varint64(); },
            wire_format::WireTypeFixed64 => { self.read_fixed64(); },
            wire_format::WireTypeFixed32 => { self.read_fixed32(); } ,
            wire_format::WireTypeLengthDelimited => {
                let len = self.read_raw_varint32();
                self.skip_raw_bytes(len);
            },
            _ => fail!("unknown wire type: {:i}", wire_type as int)
        }
    }

    pub fn read_raw_bytes(&mut self, count: u32) -> ~[u8] {
        let mut r: ~[u8] = ~[];
        r.reserve(count as uint);
        while r.len() < count as uint {
            let rem = count - r.len() as u32;
            if rem <= self.remaining_in_buffer() {
                r.push_all(self.buffer.slice(self.buffer_pos as uint, (self.buffer_pos + rem) as uint));
                self.buffer_pos += rem;
            } else {
                r.push_all(self.remaining_in_buffer_slice());
                self.buffer_pos = self.buffer_size;
                self.refill_buffer_really();
            }
        }
        r
    }

    pub fn skip_raw_bytes(&mut self, count: u32) {
        self.read_raw_bytes(count);
    }

    pub fn read_bytes(&mut self) -> ~[u8] {
        let len = self.read_raw_varint32();
        self.read_raw_bytes(len)
    }

    pub fn read_string(&mut self) -> ~str {
        from_utf8_owned(self.read_bytes())
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
    fn with_coded_output_stream<T>(&self, cb: &fn(&mut CodedOutputStream) -> T) -> T;
}

impl WithCodedOutputStream for @Writer {
    fn with_coded_output_stream<T>(&self, cb: &fn(&mut CodedOutputStream) -> T) -> T {
        let mut os = CodedOutputStream::new(*self);
        let r = cb(&mut os);
        os.flush();
        r
    }
}

fn with_coded_output_stream_to_bytes(cb: &fn(&mut CodedOutputStream)) -> ~[u8] {
    let w = VecWriter::new();
    do (w as @Writer).with_coded_output_stream |os| {
        cb(os)
    }
    (*w.vec).to_owned()
}

trait WithCodedInputStream {
    fn with_coded_input_stream<T>(&self, cb: &fn(&mut CodedInputStream) -> T) -> T;
}

impl WithCodedInputStream for @Reader {
    fn with_coded_input_stream<T>(&self, cb: &fn(&mut CodedInputStream) -> T) -> T {
        let mut is = CodedInputStream::new(*self);
        let r = cb(&mut is);
        // reading from @Reader requires all data to be read,
        // because CodedInputStream caches data, and otherwize
        // buffer would be discarded
        assert!(is.eof());
        r
    }
}

impl<'self> WithCodedInputStream for &'self [u8] {
    fn with_coded_input_stream<T>(&self, cb: &fn(&mut CodedInputStream) -> T) -> T {
        let reader = VecReader::new(@self.to_owned());
        do (reader as @Reader).with_coded_input_stream |is| {
            cb(is)
        }
    }
}


pub struct CodedOutputStream {
    buffer: ~[u8],
    position: u32,
    writer: Option<@Writer>,
}

impl CodedOutputStream {
    pub fn new(writer: @Writer) -> CodedOutputStream {
        CodedOutputStream {
            buffer: ~[0, ..4096],
            position: 0,
            writer: Some(writer),
        }
    }

    fn refresh_buffer(&mut self) {
        let mut_writer: @mut Writer = unsafe { cast::transmute(self.writer.unwrap()) };
        mut_writer.write(self.buffer.slice(0, self.position as uint));
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
        self.buffer[self.position] = byte;
        self.position += 1;
    }

    pub fn write_raw_bytes(&mut self, bytes: &[u8]) {
        self.refresh_buffer();
        let mut_writer: @mut Writer = unsafe { cast::transmute(self.writer.unwrap()) };
        mut_writer.write(bytes);
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
            cast::transmute::<f32, u32>(value)
        };
        self.write_raw_little_endian32(bits);
    }

    pub fn write_double_no_tag(&mut self, value: f64) {
        let bits = unsafe {
            cast::transmute::<f64, u64>(value)
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
        self.write_raw_varint32(value as u32);
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


pub trait Message : Eq {
    fn new() -> Self;
    fn clear(&mut self);
    // all required fields set
    fn is_initialized(&self) -> bool;
    fn merge_from(&mut self, is: &mut CodedInputStream);
    fn write_to(&self, os: &mut CodedOutputStream);
    fn compute_sizes(&self, sizes: &mut ~[u32]) -> u32;
}

pub trait ProtobufEnum : Eq {
    fn value(&self) -> i32;
}

pub trait MessageUtil {
    // broken in 0.7
    //fn parse_from_**(is: &mut Xxx) -> Self;
    fn write_to_writer(&self, w: @Writer);
    fn write_to_bytes(&self) -> ~[u8];
    fn write_length_delimited_to(&self, os: &mut CodedOutputStream);
    fn write_length_delimited_to_writer(&self, w: @Writer);
    fn write_length_delimited_to_bytes(&self) -> ~[u8];
    fn serialized_size(&self) -> u32;
    fn check_initialized(&self);
}

pub fn parse_from<M : Message>(is: &mut CodedInputStream) -> M {
    let mut r: M = Message::new();
    r.merge_from(is);
    r.check_initialized();
    r
}

pub fn parse_from_reader<M : Message>(reader: @Reader) -> M {
    do reader.with_coded_input_stream |is| {
        parse_from::<M>(is)
    }
}

pub fn parse_from_bytes<M : Message>(bytes: &[u8]) -> M {
    do bytes.with_coded_input_stream |is| {
        parse_from::<M>(is)
    }
}

pub fn parse_length_delimited_from<M : Message>(is: &mut CodedInputStream) -> M {
    is.read_message::<M>()
}

pub fn parse_length_delimited_from_reader<M : Message>(r: @Reader) -> M {
    // TODO: wrong: we may read length first, and then read exact number of bytes needed
    do r.with_coded_input_stream |is| {
        is.read_message::<M>()
    }
}

pub fn parse_length_delimited_from_bytes<M : Message>(bytes: &[u8]) -> M {
    do bytes.with_coded_input_stream |is| {
        is.read_message::<M>()
    }
}



impl<M : Message> MessageUtil for M {
    fn serialized_size(&self) -> u32 {
        let mut sizes = ~[];
        self.compute_sizes(&mut sizes)
    }

    fn check_initialized(&self) {
        // TODO: report which fields are not initialized
        assert!(self.is_initialized());
    }

    fn write_to_writer(&self, w: @Writer) {
        do w.with_coded_output_stream |os| {
            self.write_to(os);
        }
    }

    fn write_to_bytes(&self) -> ~[u8] {
        do with_coded_output_stream_to_bytes |os| {
            self.write_to(os)
        }
    }

    fn write_length_delimited_to(&self, os: &mut CodedOutputStream) {
        os.write_raw_varint32(self.serialized_size());
        self.write_to(os);
    }

    fn write_length_delimited_to_writer(&self, w: @Writer) {
        do w.with_coded_output_stream |os| {
            self.write_length_delimited_to(os);
        }
    }

    fn write_length_delimited_to_bytes(&self) -> ~[u8] {
        do with_coded_output_stream_to_bytes |os| {
            self.write_length_delimited_to(os);
        }
    }

}

#[cfg(test)]
mod test {

    use super::*;
    use std::rt::io::*;
    use std::rt::io::mem::*;
    use misc::*;
    use hex::*;

    fn test_read(hex: &str, callback: &fn(&mut CodedInputStream)) {
        let d = decode_hex(hex);
        let len = d.len();
        let reader = @MemReader::new(d) as @Reader;
        let mut is = CodedInputStream::new(reader);
        assert_eq!(0, is.pos());
        callback(&mut is);
        assert!(is.eof());
        assert_eq!(len as u32, is.pos());
    }

    #[test]
    fn test_input_stream_read_raw_byte() {
        do test_read("17") |is| {
            assert_eq!(23, is.read_raw_byte());
        }
    }

    #[test]
    fn test_input_stream_read_varint() {
        do test_read("07") |reader| {
            assert_eq!(7, reader.read_raw_varint32());
        }
        do test_read("07") |reader| {
            assert_eq!(7, reader.read_raw_varint64());
        }
        do test_read("96 01") |reader| {
            assert_eq!(150, reader.read_raw_varint32());
        }
        do test_read("96 01") |reader| {
            assert_eq!(150, reader.read_raw_varint64());
        }
    }

    #[test]
    fn test_output_input_stream_read_float() {
        do test_read("95 73 13 61") |is| {
            assert_eq!(17e19, is.read_float());
        }
    }

    #[test]
    fn test_input_stream_read_double() {
        do test_read("40 d5 ab 68 b3 07 3d 46") |is| {
            assert_eq!(23e29, is.read_double());
        }
    }

    #[test]
    fn test_input_stream_skip_raw_bytes() {
        do test_read("") |reader| {
            reader.skip_raw_bytes(0);
        }
        do test_read("aa bb") |reader| {
            reader.skip_raw_bytes(2);
        }
        do test_read("aa bb cc dd ee ff") |reader| {
            reader.skip_raw_bytes(6);
        }
    }

    #[test]
    fn test_input_stream_limits() {
        do test_read("aa bb cc") |is| {
            let old_limit = is.push_limit(1);
            assert_eq!(1, is.bytes_until_limit());
            assert_eq!(~[0xaa], is.read_raw_bytes(1));
            is.pop_limit(old_limit);
            assert_eq!(~[0xbb, 0xcc], is.read_raw_bytes(2));
        }
    }

    fn test_write(expected: &str, gen: &fn(&mut CodedOutputStream)) {
        let writer = VecWriter::new();
        let mut os = CodedOutputStream::new(writer as @Writer);
        gen(&mut os);
        os.flush();
        let r = writer.vec.to_owned();
        assert_eq!(encode_hex(decode_hex(expected)), encode_hex(r));
    }

    #[test]
    fn test_output_stream_write_raw_byte() {
        do test_write("a1") |os| {
            os.write_raw_byte(0xa1);
        }
    }

    #[test]
    fn test_output_stream_write_tag() {
        do test_write("08") |os| {
            os.write_tag(1, wire_format::WireTypeVarint);
        }
    }

    #[test]
    fn test_output_stream_write_raw_bytes() {
        do test_write("00 ab") |os| {
            os.write_raw_bytes([0x00, 0xab]);
        }
    }

    #[test]
    fn test_output_stream_write_raw_varint32() {
        do test_write("96 01") |os| {
            os.write_raw_varint32(150);
        }
    }

    #[test]
    fn test_output_stream_write_raw_varint64() {
        do test_write("96 01") |os| {
            os.write_raw_varint64(150);
        }
    }

    #[test]
    fn test_output_stream_write_raw_little_endian32() {
        do test_write("f1 e2 d3 c4") |os| {
            os.write_raw_little_endian32(0xc4d3e2f1);
        }
    }

    #[test]
    fn test_output_stream_write_float_no_tag() {
        do test_write("95 73 13 61") |os| {
            os.write_float_no_tag(17e19);
        }
    }

    #[test]
    fn test_output_stream_write_double_no_tag() {
        do test_write("40 d5 ab 68 b3 07 3d 46") |os| {
            os.write_double_no_tag(23e29);
        }
    }

    #[test]
    fn test_output_stream_write_raw_little_endian64() {
        do test_write("f1 e2 d3 c4 b5 a6 07 f8") |os| {
            os.write_raw_little_endian64(0xf807a6b5c4d3e2f1);
        }
    }
}
