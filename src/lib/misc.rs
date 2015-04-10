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

impl<'a> io::Write for VecWriter<'a> {
    fn write(&mut self, v: &[u8]) -> io::Result<usize> {
        self.vec.extend(v.iter().map(|x| *x));
        Ok(v.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use std::io;
    use misc::VecWriter;

    #[test]
    fn test_vec_writer() {
        let mut v = Vec::new();
        {
            let mut w = VecWriter::new(&mut v);
            fn foo(writer: &mut io::Write) {
                writer.write(b"hi").unwrap();
            }
            foo(&mut w as &mut io::Write);
        }
        assert_eq!(b"hi".to_vec(), v);
    }
}
