use std::io::IoError;
use std::error::Error;
use std::fmt;

pub type ProtobufResult<T> = Result<T, ProtobufError>;

#[derive(Debug,Eq,PartialEq)]
pub enum ProtobufError {
    IoError(IoError),
    WireError(String),
}

impl Error for ProtobufError {
    fn description(&self) -> &str {
        match self {
            // not sure that cause should be included in message
            &ProtobufError::IoError(ref e) => e.description(),
            &ProtobufError::WireError(ref e) => e.as_slice(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ProtobufError::IoError(ref e) => Some(e as &Error),
            &ProtobufError::WireError(..) => None,
        }
    }
}

impl fmt::Display for ProtobufError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
