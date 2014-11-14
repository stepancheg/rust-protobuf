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
