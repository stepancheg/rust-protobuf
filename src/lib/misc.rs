use std::io::Writer;
use std::io::Reader;
use std::vec;

pub struct VecWriter {
    vec: ~[u8],
}

impl VecWriter {
    pub fn new() -> VecWriter {
        VecWriter {
            vec: ~[],
        }
    }
}

impl Writer for VecWriter {
    fn write(&mut self, v: &[u8]) {
        self.vec.push_all(v);
    }

    fn flush(&mut self) {
        fail!();
    }
}

pub struct VecReader {
    vec: ~[u8],
    pos: uint,
}

impl VecReader {
    pub fn new(bytes: ~[u8]) -> VecReader {
        VecReader {
            vec: bytes,
            pos: 0,
        }
    }

    fn remaining(&self) -> uint {
        self.vec.len() - self.pos
    }
}

impl Reader for VecReader {
    fn read(&mut self, bytes: &mut [u8]) -> Option<uint> {
        let n = if bytes.len() < self.remaining() { bytes.len() } else { self.remaining() };
        vec::bytes::copy_memory(bytes, self.vec.slice(self.pos, self.pos + n));
        self.pos += n;
        Some(n)
    }
}



#[cfg(test)]
mod test {
    use super::*;

    use std::io::Writer;

    #[test]
    fn test_vec_writer() {
        let mut w = VecWriter::new();
        fn foo(writer: &mut Writer) {
            writer.write("hi".as_bytes());
        }
        foo(&mut w as &mut Writer);
        assert_eq!(~['h' as u8, 'i' as u8], w.vec.to_owned());
    }
}
