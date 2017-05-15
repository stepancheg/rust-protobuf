use std::cmp;
use std::io;
use std::io::Read;
use std::io::BufRead;
use std::mem;

#[cfg(feature = "bytes")]
use bytes::Bytes;

#[cfg(feature = "bytes")]
struct BytesWhenFeature(Bytes);

#[cfg(feature = "bytes")]
impl Default for BytesWhenFeature {
    fn default() -> Self {
        BytesWhenFeature(Bytes::new())
    }
}

#[cfg(not(feature = "bytes"))]
#[derive(Default)]
struct BytesWhenFeature();

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
pub struct BufReadIter<R : BufRead> {
    buf_read: R,
    buf: &'static [u8],
    pos: usize, // within buf
    _bytes: BytesWhenFeature,
}

impl<R : BufRead> BufReadIter<R> {
    pub fn new(buf_read: R) -> BufReadIter<R> {
        BufReadIter {
            buf_read: buf_read,
            buf: &[],
            pos: 0,
            _bytes: BytesWhenFeature::default(),
        }
    }

    #[inline]
    pub fn remaining(&self) -> &[u8] {
        &self.buf[self.pos..]
    }

    fn remaining_len(&self) -> usize {
        self.buf.len() - self.pos
    }

    #[inline]
    pub fn eof(&mut self) -> io::Result<bool> {
        self.fill_buf()?;
        Ok(self.buf.is_empty())
    }

    #[inline]
    pub fn read_byte(&mut self) -> io::Result<u8> {
        self.fill_buf()?;
        if self.pos == self.buf.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF"));
        }

        let r = self.buf[self.pos];
        self.pos += 1;
        Ok(r)
    }

    #[cfg(feature = "bytes")]
    pub fn read_exact_bytes(&mut self, len: usize) -> io::Result<Bytes> {
        let end = self.pos + len;
        if end > self._bytes.0.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF"));
        }
        let r = self._bytes.0.slice(self.pos, end);
        self.pos = end;
        Ok(r)
    }
}

impl<R : BufRead> Drop for BufReadIter<R> {
    fn drop(&mut self) {
        self.buf_read.consume(self.pos);
    }
}

impl<R : BufRead> Read for BufReadIter<R> {
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
        if self.remaining_len() >= buf.len() {
            let buf_len = buf.len();
            buf.copy_from_slice(&self.buf[self.pos .. self.pos + buf_len]);
            self.pos += buf_len;
            return Ok(());
        }

        if self.pos != 0 {
            self.buf_read.consume(self.pos);
            self.pos = 0;
            self.buf = &[];
        }

        self.buf_read.read_exact(buf)
    }
}

impl<R : BufRead> BufRead for BufReadIter<R> {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.pos == self.buf.len() {
            self.buf_read.consume(self.pos);

            // Danger! `buf_read.buf` must not be moved!
            self.buf = unsafe { mem::transmute(self.buf_read.fill_buf()?) };
            self.pos = 0;
        }

        Ok(&self.buf[self.pos..])
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        assert!(amt <= self.buf.len() - self.pos);
        self.pos += amt;
    }
}
