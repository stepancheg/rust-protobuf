use std::io;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct IoErrorWithMessage {
    message: String,
    underlying: io::Error,
}

impl fmt::Display for IoErrorWithMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.message, self.underlying)
    }
}

impl Error for IoErrorWithMessage {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.underlying)
    }
}


pub fn amend_io_error<M : Into<String>>(error: io::Error, message: M) -> io::Error {
    io::Error::new(error.kind(), Box::new(IoErrorWithMessage {
        message: message.into(),
        underlying: error,
    }))
}
