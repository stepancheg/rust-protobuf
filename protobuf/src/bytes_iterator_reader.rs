use std::io::Read;
use bytes::{Buf, Bytes};
use bytes::buf::Reader;

struct BytesIteratorReader<'a, T: Iterator<Item=&'a Bytes>> {
    iter: T,
    current_reader: Option<Reader<Bytes>>,
}

impl<'a, T: Iterator<Item=&'a Bytes>> BytesIteratorReader<'a, T> {
    fn new(iter: T) -> BytesIteratorReader<'a, T> {
        BytesIteratorReader {
            iter,
            current_reader: None,
        }
    }
}

impl<'a, T: Iterator<Item=&'a Bytes>> Read for BytesIteratorReader<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        loop {
            if self.current_reader.is_none() {
                if let Some(bytes) = self.iter.next() {
                    self.current_reader = Some(bytes.slice(..).reader());
                } else {
                    return Ok(0);
                }
            }
            let result = self.current_reader.as_mut().unwrap().read(buf);
            if let Ok(0) = result {
                self.current_reader = None;
                continue;
            }
            return result;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_exact_cross_bytes() {
        use std::io::Read;
        use bytes::Bytes;

        let bytes_vec = vec![
            Bytes::from_static(b"hello "),
            Bytes::from_static(b"protobuf "),
            Bytes::from_static(b"world!"),
        ];
        let mut reader = BytesIteratorReader::new(bytes_vec.iter());

        let mut buf = [0u8; 20];
        reader.read_exact(&mut buf).unwrap();

        use std::str;
        println!("{:?}", str::from_utf8(b"helloworld").unwrap());
        println!("{:?}", str::from_utf8(&buf[..]).unwrap());

        assert_eq!(b"hello protobuf world", &buf[..]); // Note that last ! is not read.
    }
}
