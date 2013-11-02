use std::rt::io::Writer;
use std::rt::io::Reader;
use std::vec;

struct VecWriter {
    vec: @mut ~[u8],
}

impl VecWriter {
    pub fn new() -> @mut VecWriter {
        @mut VecWriter {
            vec: @mut ~[],
        } // as @Writer
    }
}

impl Writer for VecWriter {
    fn write(&mut self, v: &[u8]) {
        (*(self.vec)).push_all(v);
    }

    fn flush(&mut self) {
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
    fn read(&mut self, bytes: &mut [u8]) -> Option<uint> {
        let n = if bytes.len() < self.remaining() { bytes.len() } else { self.remaining() };
        vec::bytes::copy_memory(bytes, (*self.vec).slice(*self.pos, (*self.vec).len()), n);
        *self.pos += n;
        Some(n)
    }

    fn eof(&mut self) -> bool {
        fail!();
    }
}



struct CountWriter {
    count: @mut uint,
}

impl CountWriter {
    pub fn new() -> @mut CountWriter {
        @mut CountWriter {
            count: @mut 0,
        }
    }
}

impl Writer for CountWriter {
    fn write(&mut self, v: &[u8]) {
        *self.count += v.len();
    }
}


#[cfg(test)]
mod test {
    use super::*;

    use std::rt::io::Writer;

    #[test]
    fn test_vec_writer() {
        let w = VecWriter::new();
        fn foo(writer: @mut Writer) {
            writer.write("hi".as_bytes());
        }
        foo(w as @mut Writer);
        assert_eq!(~['h' as u8, 'i' as u8], w.vec.to_owned());
    }

    fn test_count_writer() {
        let w = CountWriter::new();
        (w as @mut Writer).write("hi".as_bytes());
        (w as @mut Writer).write("there".as_bytes());
        assert_eq!(7, *w.count);
    }
}
