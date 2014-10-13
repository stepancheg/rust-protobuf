use std::io::IoError;

pub type ProtobufResult<T> = Result<T, ProtobufError>;

#[deriving(Show,Eq,PartialEq)]
pub enum ProtobufError {
    ProtobufIoError(IoError),
    ProtobufWireError(String),
}
