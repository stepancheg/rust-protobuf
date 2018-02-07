use std::cmp;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::mem;
use std::u64;

#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "bytes")]
use bytes::BytesMut;
#[cfg(feature = "bytes")]
use bytes::BufMut;

use ProtobufResult;
use ProtobufError;
use error::WireError;


// If an input stream is constructed with a `Read`, we create a
// `BufReader` with an internal buffer of this size.
const INPUT_STREAM_BUFFER_SIZE: usize = 4096;

const USE_UNSAFE_FOR_SPEED: bool = true;

const NO_LIMIT: u64 = u64::MAX;


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
    pos_within_buf: usize,
    limit_within_buf: usize,
    pos_of_buf_start: u64,
    limit: u64,
}

impl<'a> Drop for BufReadIter<'a> {
    fn drop(&mut self) {
        match self.input_source {
            InputSource::BufRead(ref mut buf_read) => buf_read.consume(self.pos_within_buf),
            InputSource::Read(_) => {
                // Nothing to flush, because we own BufReader
            }
            _ => {}
        }
    }
}

impl<'ignore> BufReadIter<'ignore> {
    pub fn from_read<'a>(read: &'a mut Read) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Read(
                BufReader::with_capacity(INPUT_STREAM_BUFFER_SIZE, read),
            ),
            buf: &[],
            pos_within_buf: 0,
            limit_within_buf: 0,
            pos_of_buf_start: 0,
            limit: NO_LIMIT,
        }
    }

    pub fn from_buf_read<'a>(buf_read: &'a mut BufRead) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::BufRead(buf_read),
            buf: &[],
            pos_within_buf: 0,
            limit_within_buf: 0,
            pos_of_buf_start: 0,
            limit: NO_LIMIT,
        }
    }

    pub fn from_byte_slice<'a>(bytes: &'a [u8]) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Slice(bytes),
            buf: bytes,
            pos_within_buf: 0,
            limit_within_buf: bytes.len(),
            pos_of_buf_start: 0,
            limit: NO_LIMIT,
        }
    }

    #[cfg(feature = "bytes")]
    pub fn from_bytes<'a>(bytes: &'a Bytes) -> BufReadIter<'a> {
        BufReadIter {
            input_source: InputSource::Bytes(bytes),
            buf: &bytes,
            pos_within_buf: 0,
            limit_within_buf: bytes.len(),
            pos_of_buf_start: 0,
            limit: NO_LIMIT,
        }
    }

    #[inline]
    fn assertions(&self) {
        debug_assert!(self.pos_within_buf <= self.limit_within_buf);
        debug_assert!(self.limit_within_buf <= self.buf.len());
        debug_assert!(self.pos_of_buf_start + self.pos_within_buf as u64 <= self.limit);
    }

    #[inline(always)]
    pub fn pos(&self) -> u64 {
        self.pos_of_buf_start + self.pos_within_buf as u64
    }

    /// Recompute `limit_within_buf` after update of `limit`
    #[inline]
    fn update_limit_within_buf(&mut self) {
        if self.pos_of_buf_start + (self.buf.len() as u64) <= self.limit {
            self.limit_within_buf = self.buf.len();
        } else {
            self.limit_within_buf = (self.limit - self.pos_of_buf_start) as usize;
        }

        self.assertions();
    }

    pub fn push_limit(&mut self, limit: u64) -> ProtobufResult<u64> {
        // TODO: return error instead of panic
        let new_limit = match self.pos().checked_add(limit) {
            Some(new_limit) => new_limit,
            None => return Err(ProtobufError::WireError(WireError::Other)),
        };

        if new_limit > self.limit {
            return Err(ProtobufError::WireError(WireError::Other));
        }

        let prev_limit = mem::replace(&mut self.limit, new_limit);

        self.update_limit_within_buf();

        Ok(prev_limit)
    }

    #[inline]
    pub fn pop_limit(&mut self, limit: u64) {
        assert!(limit >= self.limit);

        self.limit = limit;

        self.update_limit_within_buf();
    }

    #[inline]
    pub fn remaining_in_buf(&self) -> &[u8] {
        if USE_UNSAFE_FOR_SPEED {
            unsafe {
                &self.buf
                    .get_unchecked(self.pos_within_buf..self.limit_within_buf)
            }
        } else {
            &self.buf[self.pos_within_buf..self.limit_within_buf]
        }
    }

    #[inline(always)]
    pub fn remaining_in_buf_len(&self) -> usize {
        self.limit_within_buf - self.pos_within_buf
    }

    pub fn bytes_until_limit(&self) -> u64 {
        if self.limit == NO_LIMIT {
            NO_LIMIT
        } else {
            self.limit - (self.pos_of_buf_start + self.pos_within_buf as u64)
        }
    }

    #[inline(always)]
    pub fn eof(&mut self) -> ProtobufResult<bool> {
        if self.pos_within_buf == self.limit_within_buf {
            Ok(self.fill_buf()?.is_empty())
        } else {
            Ok(false)
        }
    }

    #[inline(always)]
    pub fn read_byte(&mut self) -> ProtobufResult<u8> {
        if self.pos_within_buf == self.limit_within_buf {
            self.do_fill_buf()?;
            if self.remaining_in_buf_len() == 0 {
                return Err(ProtobufError::WireError(WireError::UnexpectedEof));
            }
        }

        let r = if USE_UNSAFE_FOR_SPEED {
            unsafe { *self.buf.get_unchecked(self.pos_within_buf) }
        } else {
            self.buf[self.pos_within_buf]
        };
        self.pos_within_buf += 1;
        Ok(r)
    }

    #[cfg(feature = "bytes")]
    pub fn read_exact_bytes(&mut self, len: usize) -> ProtobufResult<Bytes> {
        if let InputSource::Bytes(bytes) = self.input_source {
            if self.pos_within_buf + len > self.limit_within_buf {
                return Err(ProtobufError::WireError(WireError::UnexpectedEof));
            }
            let r = bytes.slice(self.pos_within_buf, self.pos_within_buf + len);
            self.pos_within_buf += len;
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

    pub fn read(&mut self, buf: &mut [u8]) -> ProtobufResult<usize> {
        self.fill_buf()?;

        let rem = &self.buf[self.pos_within_buf..self.limit_within_buf];

        let len = cmp::min(rem.len(), buf.len());
        &mut buf[..len].copy_from_slice(&rem[..len]);
        self.pos_within_buf += len;
        Ok(len)
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> ProtobufResult<()> {
        if self.remaining_in_buf_len() >= buf.len() {
            let buf_len = buf.len();
            buf.copy_from_slice(
                &self.buf[self.pos_within_buf..self.pos_within_buf + buf_len],
            );
            self.pos_within_buf += buf_len;
            return Ok(());
        }

        if self.bytes_until_limit() < buf.len() as u64 {
            return Err(ProtobufError::WireError(WireError::UnexpectedEof));
        }

        let consume = self.pos_within_buf;
        self.pos_of_buf_start += self.pos_within_buf as u64;
        self.pos_within_buf = 0;
        self.buf = &[];
        self.limit_within_buf = 0;

        match self.input_source {
            InputSource::Read(ref mut buf_read) => {
                buf_read.consume(consume);
                buf_read.read_exact(buf)?;
            }
            InputSource::BufRead(ref mut buf_read) => {
                buf_read.consume(consume);
                buf_read.read_exact(buf)?;
            }
            _ => {
                return Err(ProtobufError::WireError(WireError::UnexpectedEof));
            }
        }

        self.pos_of_buf_start += buf.len() as u64;

        self.assertions();

        Ok(())
    }

    fn do_fill_buf(&mut self) -> ProtobufResult<()> {
        debug_assert!(self.pos_within_buf == self.limit_within_buf);

        // Limit is reached, do not fill buf, because otherwise
        // synchronous read from `CodedInputStream` may block.
        if self.limit == self.pos() {
            return Ok(());
        }

        let consume = self.buf.len();
        self.pos_of_buf_start += self.buf.len() as u64;
        self.buf = &[];
        self.pos_within_buf = 0;
        self.limit_within_buf = 0;

        match self.input_source {
            InputSource::Read(ref mut buf_read) => {
                buf_read.consume(consume);
                self.buf = unsafe { mem::transmute(buf_read.fill_buf()?) };
            }
            InputSource::BufRead(ref mut buf_read) => {
                buf_read.consume(consume);
                self.buf = unsafe { mem::transmute(buf_read.fill_buf()?) };
            }
            _ => {
                return Ok(());
            }
        }

        self.update_limit_within_buf();

        Ok(())
    }

    #[inline(always)]
    pub fn fill_buf(&mut self) -> ProtobufResult<&[u8]> {
        if self.pos_within_buf == self.limit_within_buf {
            self.do_fill_buf()?;
        }

        Ok(if USE_UNSAFE_FOR_SPEED {
            unsafe {
                self.buf
                    .get_unchecked(self.pos_within_buf..self.limit_within_buf)
            }
        } else {
            &self.buf[self.pos_within_buf..self.limit_within_buf]
        })
    }

    #[inline(always)]
    pub fn consume(&mut self, amt: usize) {
        assert!(amt <= self.limit_within_buf - self.pos_within_buf);
        self.pos_within_buf += amt;
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

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufRead;
    use std::io::Read;
    use std::io;

    #[test]
    fn eof_at_limit() {
        struct Read5ThenPanic {
            pos: usize,
        }

        impl Read for Read5ThenPanic {
            fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                unreachable!();
            }
        }

        impl BufRead for Read5ThenPanic {
            fn fill_buf(&mut self) -> io::Result<&[u8]> {
                assert_eq!(0, self.pos);
                static ZERO_TO_FIVE: &'static [u8] = &[0, 1, 2, 3, 4];
                Ok(ZERO_TO_FIVE)
            }

            fn consume(&mut self, amt: usize) {
                if amt == 0 {
                    // drop of BufReadIter
                    return;
                }

                assert_eq!(0, self.pos);
                assert_eq!(5, amt);
                self.pos += amt;
            }
        }

        let mut read = Read5ThenPanic { pos : 0 };
        let mut buf_read_iter = BufReadIter::from_buf_read(&mut read);
        assert_eq!(0, buf_read_iter.pos());
        let _prev_limit = buf_read_iter.push_limit(5);
        buf_read_iter.read_byte().expect("read_byte");
        buf_read_iter.read_exact(&mut [1, 2, 3, 4]).expect("read_exact");
        assert!(buf_read_iter.eof().expect("eof"));
    }
}
