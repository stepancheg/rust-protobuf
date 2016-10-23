use std::io;
use std::error::Error;
use std::fmt;

pub type ProtobufResult<T> = Result<T, ProtobufError>;

#[derive(Debug)]
pub enum ProtobufError {
    IoError(io::Error),
    WireError(String),
    MessageNotInitialized { message: &'static str },
}

impl ProtobufError {
    pub fn message_not_initialized(message: &'static str) -> ProtobufError {
        ProtobufError::MessageNotInitialized {
            message: message
        }
    }
}

impl fmt::Display for ProtobufError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for ProtobufError {
    fn description(&self) -> &str {
        match self {
            // not sure that cause should be included in message
            &ProtobufError::IoError(ref e) => e.description(),
            &ProtobufError::WireError(ref e) => &e,
            &ProtobufError::MessageNotInitialized { .. } => "not all message fields set",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ProtobufError::IoError(ref e) => Some(e),
            &ProtobufError::WireError(..) => None,
            &ProtobufError::MessageNotInitialized { .. } => None,
        }
    }
}

impl From<io::Error> for ProtobufError {
    fn from(err: io::Error) -> Self {
        ProtobufError::IoError(err)
    }
}

impl From<ProtobufError> for io::Error {
    fn from(err: ProtobufError) -> Self {
        match err {
            ProtobufError::IoError(e) => e,
            ProtobufError::WireError(e) => io::Error::new(io::ErrorKind::InvalidData, ProtobufError::WireError(e)),
            ProtobufError::MessageNotInitialized { message: msg } => io::Error::new(io::ErrorKind::InvalidInput, ProtobufError::MessageNotInitialized { message: msg }),
        }
    }
}
