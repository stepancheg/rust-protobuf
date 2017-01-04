#![crate_type = "lib"]

pub use unknown::UnknownFields;
pub use unknown::UnknownFieldsIter;
pub use unknown::UnknownValue;
pub use unknown::UnknownValueRef;
pub use unknown::UnknownValues;
pub use unknown::UnknownValuesIter;
pub use repeated::RepeatedField;
pub use singular::SingularField;
pub use singular::SingularPtrField;
pub use clear::Clear;
pub use core::Message;
pub use protobuf::CodedMessage;
pub use core::MessageStatic;
pub use core::ProtobufEnum;
pub use core::parse_from_bytes;
pub use core::parse_from_reader;
pub use core::parse_length_delimited_from;
pub use core::parse_length_delimited_from_bytes;
pub use stream::InputSource;
pub use stream::CodedInputStream;
pub use stream::CodedOutputStream;
pub use stream::wire_format;
pub use error::ProtobufResult;
pub use error::ProtobufError;
pub use cached_size::CachedSize;

// generated
pub mod descriptor;
pub mod plugin;

pub mod core;
pub mod rt;
pub mod lazy;
pub mod code_writer;
pub mod codegen;
pub mod compiler_plugin;
pub mod repeated;
pub mod singular;
pub mod clear;
pub mod reflect;
pub mod text_format;
pub mod stream;
pub mod error;
pub mod types;

// used by test
pub mod hex;

// used by rust-grpc
pub mod descriptorx;

mod zigzag;
mod paginate;
mod unknown;
mod strx;
mod rust;
mod cached_size;

// so `use protobuf::*` could work in descriptor mod
mod protobuf {
    pub use descriptor;
    pub use descriptorx;
    pub use codegen;
    pub use reflect;
    pub use core::*;
    pub use error::*;
    pub use stream::*;
    pub use rt;
    pub use text_format;
    pub use types;
    pub use lazy;
    pub use unknown::UnknownFields;
    pub use unknown::UnknownFieldsIter;
    pub use unknown::UnknownValue;
    pub use unknown::UnknownValueRef;
    pub use unknown::UnknownValues;
    pub use unknown::UnknownValuesIter;
    pub use repeated::RepeatedField;
    pub use singular::SingularField;
    pub use singular::SingularPtrField;
    pub use clear::Clear;
    pub use cached_size::CachedSize;
}
