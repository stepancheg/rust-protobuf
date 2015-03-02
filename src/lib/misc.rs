use std::old_io::Writer;
use std::old_io as io;

pub struct VecWriter<'a> {
    vec: &'a mut Vec<u8>,
}

impl<'a> VecWriter<'a> {
    pub fn new(vec: &'a mut Vec<u8>) -> VecWriter<'a> {
        VecWriter {
            vec: vec
        }
    }
}

impl<'a> Writer for VecWriter<'a> {
    fn write_all(&mut self, v: &[u8]) -> io::IoResult<()> {
        self.vec.extend(v.iter().map(|b| *b));
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use std::old_io::Writer;
    use misc::VecWriter;

    #[test]
    fn test_vec_writer() {
        let mut v = Vec::new();
        {
            let mut w = VecWriter::new(&mut v);
            fn foo(writer: &mut Writer) {
                writer.write_all(b"hi").unwrap();
            }
            foo(&mut w as &mut Writer);
        }
        assert_eq!(b"hi".to_vec(), v);
    }
}
