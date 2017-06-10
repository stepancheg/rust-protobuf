#![crate_type = "lib"]

#[cfg(feature = "bytes")]
extern crate bytes;

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
pub use core::MessageStatic;
pub use core::ProtobufEnum;
pub use core::parse_from_bytes;
pub use core::parse_from_reader;
#[cfg(feature = "bytes")]
pub use core::parse_from_carllerche_bytes;
pub use core::parse_length_delimited_from;
pub use core::parse_length_delimited_from_bytes;
pub use stream::CodedInputStream;
pub use stream::CodedOutputStream;
pub use stream::wire_format;
pub use error::ProtobufResult;
pub use error::ProtobufError;
pub use cached_size::CachedSize;
#[cfg(feature = "bytes")]
pub use chars::Chars;

// generated
pub mod descriptor;
pub mod plugin;
mod rustproto;

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
pub mod well_known_types;
pub mod ext;

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
mod varint;
#[cfg(feature = "bytes")]
pub mod chars; // TODO: make private

mod misc;

mod buf_read_iter;


// so `use protobuf::*` could work in mod descriptor and well_known_types
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
    pub use well_known_types;
    pub use ext;
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
