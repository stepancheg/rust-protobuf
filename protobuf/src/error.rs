use std::error::Error;
use std::fmt;
use std::io;
use std::str;

use crate::wire_format::WireType;

/// `Result` alias for `ProtobufError`
pub type ProtobufResult<T> = Result<T, ProtobufError>;

/// Enum values added here for diagnostic purposes.
/// Users should not depend on specific values.
#[derive(Debug, thiserror::Error)]
pub enum WireError {
    #[error("Unexpected EOF")]
    UnexpectedEof,
    #[error("Unexpected wire type")]
    UnexpectedWireType(WireType),
    #[error("Incorrect tag")]
    IncorrectTag(u32),
    // unused since https://github.com/stepancheg/rust-protobuf/issues/318
    #[error("Incomplete map")]
    IncompleteMap,
    #[error("Incorrect varint")]
    IncorrectVarint,
    #[error("Invalid UTF-8 sequence")]
    Utf8Error,
    #[error("Invalid enum value: {}", .0)]
    InvalidEnumValue(i32),
    #[error("Over recursion limit")]
    OverRecursionLimit,
    #[error("Truncated message")]
    TruncatedMessage,
    // not really possible
    #[error("Limit overflow")]
    LimitOverflow,
    #[error("New limit must not be greater than current limit")]
    LimitIncrease,
}

/// Generic protobuf error
#[derive(Debug)]
pub enum ProtobufError {
    /// I/O error when reading or writing
    IoError(io::Error),
    /// Malformed input
    WireError(WireError),
    /// Protocol contains a string which is not valid UTF-8 string
    Utf8(str::Utf8Error),
    /// Not all required fields of message set.
    MessageNotInitialized(String),
}

impl fmt::Display for ProtobufError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // not sure that cause should be included in message
            &ProtobufError::IoError(ref e) => write!(f, "IO error: {}", e),
            &ProtobufError::WireError(ref e) => fmt::Display::fmt(e, f),
            &ProtobufError::Utf8(ref e) => write!(f, "{}", e),
            &ProtobufError::MessageNotInitialized { .. } => write!(f, "not all message fields set"),
        }
    }
}

impl Error for ProtobufError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            &ProtobufError::IoError(ref e) => Some(e),
            &ProtobufError::Utf8(ref e) => Some(e),
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

impl From<str::Utf8Error> for ProtobufError {
    fn from(err: str::Utf8Error) -> Self {
        ProtobufError::Utf8(err)
    }
}

impl From<ProtobufError> for io::Error {
    fn from(err: ProtobufError) -> Self {
        match err {
            ProtobufError::IoError(e) => e,
            ProtobufError::WireError(e) => {
                io::Error::new(io::ErrorKind::InvalidData, ProtobufError::WireError(e))
            }
            ProtobufError::MessageNotInitialized(message) => io::Error::new(
                io::ErrorKind::InvalidInput,
                ProtobufError::MessageNotInitialized(message),
            ),
            e => io::Error::new(io::ErrorKind::Other, Box::new(e)),
        }
    }
}
