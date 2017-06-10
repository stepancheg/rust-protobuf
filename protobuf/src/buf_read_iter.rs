use std::cmp;
use std::io;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::mem;
use std::slice;

#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "bytes")]
use bytes::BytesMut;
#[cfg(feature = "bytes")]
use bytes::BufMut;


// If an input stream is constructed with a `Read`, we create a
// `BufReader` with an internal buffer of this size.
const INPUT_STREAM_BUFFER_SIZE: usize = 4096;

const USE_UNSAFE_FOR_SPEED: bool = true;


/// Hold all possible combinations of input source
enum InputSource<'a> {
    BufRead(&'a mut BufRead),
    Read(BufReader<&'a mut Read>),
    Slice(&'a [u8]),
    #[cfg(feature = "bytes")]
    Bytes(&'a Bytes),
}

/// Dangerous implementation of `BufRead`.
///
/// Unsafe wrapper around BufRead which assumes that `BufRead` buf is
/// not moved when `BufRead` is moved.
///
/// This assumption is generally incorrect, however, in practice
/// `BufReadIter` is created either from `BufRead` reference (which
/// cannot  be moved, because it is locked by `CodedInputStream`) or from
/// `BufReader` which does not move its buffer (we know that from
/// inspecting rust standard library).
///
/// It is important for `CodedInputStream` performance that small reads
/// (e. g. 4 bytes reads) do not involve virtual calls or switches.
/// This is achievable with `BufReadIter`.
pub struct BufReadIter<'a> {
    input_source: InputSource<'a>,
    buf: &'a [u8],
    pos: usize, // within buf
}

impl<'ignore> BufReadIter<'ignore> {
    pub fn from_read<'a>(read: &'a mut Read) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Read(
                BufReader::with_capacity(INPUT_STREAM_BUFFER_SIZE, read)),
            buf: &[],
            pos: 0,
        }
    }

    pub fn from_buf_read<'a>(buf_read: &'a mut BufRead) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::BufRead(buf_read),
            buf: &[],
            pos: 0,
        }
    }

    pub fn from_byte_slice<'a>(bytes: &'a [u8]) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Slice(bytes),
            buf: bytes,
            pos: 0,
        }
    }

    #[cfg(feature = "bytes")]
    pub fn from_bytes<'a>(bytes: &'a Bytes) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Bytes(bytes),
            buf: &bytes,
            pos: 0,
        }
    }

    #[inline]
    pub fn remaining_in_buf(&self) -> &[u8] {
        if USE_UNSAFE_FOR_SPEED {
            unsafe {
                slice::from_raw_parts(
                    self.buf.as_ptr().offset(self.pos as isize),
                    self.buf.len() - self.pos)
            }
        } else {
            &self.buf[self.pos..]
        }
    }

    #[inline]
    pub fn remaining_in_buf_len(&self) -> usize {
        self.buf.len() - self.pos
    }

    #[inline]
    pub fn eof(&mut self) -> io::Result<bool> {
        if self.buf.len() == self.pos {
            Ok(self.fill_buf()?.is_empty())
        } else {
            Ok(false)
        }
    }

    #[inline]
    pub fn read_byte(&mut self) -> io::Result<u8> {
        if self.pos == self.buf.len() {
            if self.fill_buf()?.is_empty() {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF"));
            }
        }

        let r = if USE_UNSAFE_FOR_SPEED {
            unsafe { *self.buf.get_unchecked(self.pos) }
        } else {
            self.buf[self.pos]
        };
        self.pos += 1;
        Ok(r)
    }

    #[cfg(feature = "bytes")]
    pub fn read_exact_bytes(&mut self, len: usize) -> io::Result<Bytes> {
        if let InputSource::Bytes(bytes) = self.input_source {
            if self.pos + len > bytes.len() {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF"));
            }
            let r = bytes.slice(self.pos, self.pos + len);
            self.pos += len;
            Ok(r)
        } else {
            let mut r = BytesMut::with_capacity(len);
            unsafe {
                {
                    let mut buf = &mut r.bytes_mut()[..len];
                    self.read_exact(buf)?;
                }
                r.advance_mut(len);
            }
            Ok(r.freeze())
        }
    }

    fn do_fill_buf(&mut self) -> io::Result<()> {
        debug_assert!(self.pos == self.buf.len());
        match self.input_source {
            InputSource::Read(ref mut buf_read) => {
                buf_read.consume(self.pos);
                // Danger! `buf_read.buf` must not be moved!
                self.buf = unsafe { mem::transmute(buf_read.fill_buf()?) };
                self.pos = 0;
            }
            InputSource::BufRead(ref mut buf_read) => {
                buf_read.consume(self.pos);
                // Danger! `buf_read.buf` must not be moved!
                self.buf = unsafe { mem::transmute(buf_read.fill_buf()?) };
                self.pos = 0;
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'a> Drop for BufReadIter<'a> {
    fn drop(&mut self) {
        match self.input_source {
            InputSource::BufRead(ref mut buf_read) => buf_read.consume(self.pos),
            InputSource::Read(_) => {
                // Nothing to flush, because we own BufReader
            }
            _ => {},
        }
    }
}

impl<'a> Read for BufReadIter<'a> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.fill_buf()?;

        let rem = &self.buf[self.pos..];

        let len = cmp::min(rem.len(), buf.len());
        &mut buf[..len].copy_from_slice(&rem[..len]);
        self.pos += len;
        Ok((len))
    }

    fn read_exact(&mut self, mut buf: &mut [u8]) -> io::Result<()> {
        if self.remaining_in_buf_len() >= buf.len() {
            let buf_len = buf.len();
            buf.copy_from_slice(&self.buf[self.pos .. self.pos + buf_len]);
            self.pos += buf_len;
            return Ok(());
        }

        match self.input_source {
            InputSource::Read(ref mut buf_read) => {
                buf_read.consume(self.pos);
                self.pos = 0;
                self.buf = &[];
                buf_read.read_exact(buf)
            }
            InputSource::BufRead(ref mut buf_read) => {
                buf_read.consume(self.pos);
                self.pos = 0;
                self.buf = &[];
                buf_read.read_exact(buf)
            }
            _ => {
                Err(io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer"))
            }
        }
    }
}

impl<'a> BufRead for BufReadIter<'a> {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.pos == self.buf.len() {
            self.do_fill_buf()?;
        }

        let s = if USE_UNSAFE_FOR_SPEED {
            unsafe { self.buf.get_unchecked(self.pos..) }
        } else {
            &self.buf[self.pos..]
        };
        Ok(s)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        assert!(amt <= self.buf.len() - self.pos);
        self.pos += amt;
    }
}

#[cfg(all(test, feature = "bytes"))]
mod test_bytes {
    use super::*;
    use std::io::Write;

    fn make_long_string(len: usize) -> Vec<u8> {
        let mut s = Vec::new();
        while s.len() < len {
            let len = s.len();
            write!(&mut s, "{}", len).expect("unexpected");
        }
        s.truncate(len);
        s
    }

    #[test]
    fn read_exact_bytes_from_slice() {
        let bytes = make_long_string(100);
        let mut bri = BufReadIter::from_byte_slice(&bytes[..]);
        assert_eq!(&bytes[..90], &bri.read_exact_bytes(90).unwrap()[..]);
        assert_eq!(bytes[90], bri.read_byte().expect("read_byte"));
    }

    #[test]
    fn read_exact_bytes_from_bytes() {
        let bytes = Bytes::from(make_long_string(100));
        let mut bri = BufReadIter::from_bytes(&bytes);
        let read = bri.read_exact_bytes(90).unwrap();
        assert_eq!(&bytes[..90], &read[..]);
        assert_eq!(&bytes[..90].as_ptr(), &read.as_ptr());
        assert_eq!(bytes[90], bri.read_byte().expect("read_byte"));
    }

}
