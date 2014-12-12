use std::io::Writer;
use std::io;

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
        let mut v = Vec::new();
        {
            let mut w = VecWriter::new(&mut v);
            fn foo(writer: &mut Writer) {
                writer.write(b"hi").unwrap();
            }
            foo(&mut w as &mut Writer);
        }
        assert_eq!(b"hi".to_vec(), v);
    }
}
