//use std::cmp;
//use std::io;
//use std::io::Read;
//use std::io::BufRead;
//use std::mem;
//
//use input_source::InputSource;
//
//pub struct InputSourceIter<'a> {
//    input_source: InputSource<'a>,
//    pub buf: &'a [u8],
//}
//
//impl<'a> InputSourceIter<'a> {
//    pub fn new(buf_read: &'a mut BufRead) -> InputSourceIter<'a> {
//        InputSourceIter {
//            buf_read: buf_read,
//            buf: &[],
//        }
//    }
//
//    pub fn fill_buf(&mut self) -> io::Result<()> {
//        if self.buf.is_empty() {
//            return Ok(());
//        }
//
//        // since we own unique reference to `buf_read`, buf cannot be moved
//        self.buf = unsafe { mem::transmute(self.buf_read.fill_buf()?) };
//        Ok(())
//    }
//}
//
//impl<'a> Read for InputSourceIter<'a> {
//    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//        self.fill_buf()?;
//
//        let len = cmp::min(self.buf.len(), buf.len());
//        &mut buf[..len].copy_from_slice(&self.buf[..len]);
//        Ok((len))
//    }
//}
