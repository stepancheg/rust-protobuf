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
#[derive(Debug, thiserror::Error)]
pub enum ProtobufError {
    /// I/O error when reading or writing
    #[error(transparent)]
    IoError(#[from] io::Error),
    /// Malformed input
    #[error(transparent)]
    WireError(#[from] WireError),
    /// Protocol contains a string which is not valid UTF-8 string
    #[error("UTF-8 decode error")]
    Utf8(
        #[source]
        #[from]
        str::Utf8Error,
    ),
    /// Not all required fields of message set.
    #[error("Message `{}` is missing required fields", .0)]
    MessageNotInitialized(String),
    /// Protobuf type and runtime types mismatch.
    #[error("Protobuf type and runtime types are not compatible")]
    IncompatibleProtobufTypeAndRuntimeType,
    /// Group field type not implemented.
    #[error("Group field is not supported")]
    GroupIsNotImplemented,
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
