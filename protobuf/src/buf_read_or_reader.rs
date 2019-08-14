//! `BufRead` pointer or `BufReader` owned.

use std::io;
use std::io::{BufRead, BufReader, Read};

/// Helper type to simplify `BufReadIter` implementation.
pub(crate) enum BufReadOrReader<'a> {
    BufReader(BufReader<&'a mut dyn Read>),
    BufRead(&'a mut dyn BufRead),
}

impl<'a> Read for BufReadOrReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        match self {
            BufReadOrReader::BufReader(r) => r.read(buf),
            BufReadOrReader::BufRead(r) => r.read(buf),
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, io::Error> {
        match self {
            BufReadOrReader::BufReader(r) => r.read_to_end(buf),
            BufReadOrReader::BufRead(r) => r.read_to_end(buf),
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), io::Error> {
        match self {
            BufReadOrReader::BufReader(r) => r.read_exact(buf),
            BufReadOrReader::BufRead(r) => r.read_exact(buf),
        }
    }
}

impl<'a> BufRead for BufReadOrReader<'a> {
    fn fill_buf(&mut self) -> Result<&[u8], io::Error> {
        match self {
            BufReadOrReader::BufReader(r) => r.fill_buf(),
            BufReadOrReader::BufRead(r) => r.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            BufReadOrReader::BufReader(r) => r.consume(amt),
            BufReadOrReader::BufRead(r) => r.consume(amt),
        }
    }
}
