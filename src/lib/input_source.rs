use std::io;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;

pub enum InputSource<'a> {
    BufRead(&'a mut BufRead),
    Read(BufReader<&'a mut Read>),
    Cursor(io::Cursor<&'a [u8]>),
}

impl<'a> Read for InputSource<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            &mut InputSource::BufRead(ref mut br) => br.read(buf),
            &mut InputSource::Read(ref mut br) => br.read(buf),
            &mut InputSource::Cursor(ref mut c) => c.read(buf),
        }
    }
}

impl<'a> BufRead for InputSource<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            &mut InputSource::BufRead(ref mut br) => br.fill_buf(),
            &mut InputSource::Read(ref mut br) => br.fill_buf(),
            &mut InputSource::Cursor(ref mut c) => c.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            &mut InputSource::BufRead(ref mut br) => br.consume(amt),
            &mut InputSource::Read(ref mut br) => br.consume(amt),
            &mut InputSource::Cursor(ref mut c) => c.consume(amt),
        }
    }
}

impl<'a> InputSource<'a> {
    #[allow(dead_code)]
    pub fn eof(&mut self) -> io::Result<bool> {
        let res = self.fill_buf()?;
        Ok(res.is_empty())
    }
}

