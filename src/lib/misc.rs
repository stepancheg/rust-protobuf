use std::io::Writer;
use std::io::Reader;
use std::io;
use std::slice;
use std::result::Ok;
use std::result::Err;

pub struct VecWriter {
    pub vec: Vec<u8>,
}

impl VecWriter {
    pub fn new() -> VecWriter {
        VecWriter {
            vec: Vec::new(),
        }
    }
}

impl Writer for VecWriter {
    fn write(&mut self, v: &[u8]) -> io::IoResult<()> {
        self.vec.push_all(v);
        Ok(())
    }
}

pub struct VecReader {
    vec: Vec<u8>,
    pos: uint,
}

impl VecReader {
    pub fn new(bytes: Vec<u8>) -> VecReader {
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
    fn read(&mut self, bytes: &mut [u8]) -> io::IoResult<uint> {
        if self.remaining() == 0 {
            Err(io::standard_error(io::EndOfFile))
        } else {
            let n = if bytes.len() < self.remaining() { bytes.len() } else { self.remaining() };
            slice::bytes::copy_memory(bytes, self.vec.slice(self.pos, self.pos + n));
            self.pos += n;
            Ok(n)
        }
    }
}



#[cfg(test)]
mod test {

    use std::io::Writer;
    use misc::VecWriter;

    #[test]
    fn test_vec_writer() {
        let mut w = VecWriter::new();
        fn foo(writer: &mut Writer) {
            writer.write(b"hi").unwrap();
        }
        foo(&mut w as &mut Writer);
        assert_eq!(b"hi".to_vec(), w.vec);
    }
}
