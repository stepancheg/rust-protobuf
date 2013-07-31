use std::io;
use std::vec;

struct VecWriter {
    vec: @mut ~[u8],
}

impl VecWriter {
    pub fn new() -> @VecWriter {
        @VecWriter {
            vec: @mut ~[],
        } // as @Writer
    }
}

impl Writer for VecWriter {
    fn write(&self, v: &[u8]) {
        (*(self.vec)).push_all(v);
    }

    fn seek(&self, _: int, _: io::SeekStyle) {
        fail!();
    }

    fn tell(&self) -> uint {
        fail!();
    }

    fn flush(&self) -> int {
        fail!();
    }

    fn get_type(&self) -> io::WriterType {
        fail!();
    }
}

struct VecReader {
    vec: @~[u8],
    pos: @mut uint,
}

impl VecReader {
    pub fn new(bytes: @~[u8]) -> @VecReader {
        @VecReader {
            vec: bytes,
            pos: @mut 0,
        }
    }

    fn remaining(&self) -> uint {
        (*self.vec).len() - *self.pos
    }
}

impl Reader for VecReader {
    fn read(&self, bytes: &mut [u8], len: uint) -> uint {
        assert!(bytes.len() >= len);
        let n = if len < self.remaining() { len } else { self.remaining() };
        vec::bytes::copy_memory(bytes, (*self.vec).slice(*self.pos, (*self.vec).len()), n);
        *self.pos += n;
        n
    }

    fn read_byte(&self) -> int {
        let mut bytes = [0u8, 1];
        let c = self.read(bytes, 1);
        match c {
            0 => -1 as int,
            1 => bytes[0] as int,
            _ => fail!()
        }
    }

    fn eof(&self) -> bool {
        fail!();
    }

    fn seek(&self, _: int, _: io::SeekStyle) {
        fail!();
    }

    fn tell(&self) -> uint {
        fail!();
    }
}



struct CountWriter {
    count: @mut uint,
}

impl CountWriter {
    pub fn new() -> @CountWriter {
        @CountWriter {
            count: @mut 0,
        }
    }
}

impl Writer for CountWriter {
    fn write(&self, v: &[u8]) {
        *self.count += v.len();
    }

    fn seek(&self, _: int, _: io::SeekStyle) {
        fail!();
    }

    fn tell(&self) -> uint {
        fail!();
    }

    fn flush(&self) -> int {
        fail!();
    }

    fn get_type(&self) -> io::WriterType {
        fail!();
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec_writer() {
        let w = VecWriter::new();
        fn foo(writer: @Writer) {
            writer.write_str("hi");
        }
        foo(w as @Writer);
        assert_eq!(~['h' as u8, 'i' as u8], w.vec.to_owned());
    }

    fn test_count_writer() {
        let w = CountWriter::new();
        (w as @Writer).write_str("hi");
        (w as @Writer).write_str("there");
        assert_eq!(7, *w.count);
    }
}
