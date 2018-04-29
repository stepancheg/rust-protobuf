//! Library to read and write protocol buffers data.

// TODO: add docs
//#![deny(missing_docs)]

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
pub use core::parse_from_bytes;
pub use core::parse_from_reader;
#[cfg(feature = "bytes")]
pub use core::parse_from_carllerche_bytes;
pub use core::parse_length_delimited_from;
pub use core::parse_length_delimited_from_reader;
pub use core::parse_length_delimited_from_bytes;
pub use enums::ProtobufEnum;
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
pub mod rustproto;

mod core;
mod enums;
pub mod rt;
pub mod lazy;
pub mod compiler_plugin;
mod repeated;
mod singular;
mod clear;
pub mod reflect;
pub mod text_format;
pub mod stream;
pub mod error;
pub mod types;
pub mod well_known_types;
pub mod ext;

// used by test
#[cfg(test)]
#[path = "../../protobuf-test-common/src/hex.rs"]
mod hex;

// used by rust-grpc
pub mod descriptorx;

mod zigzag;
mod paginate;
mod unknown;
mod strx;
#[doc(hidden)] // used by codegen
pub mod rust;
mod cached_size;
mod varint;
#[cfg(feature = "bytes")]
mod chars;

mod misc;

mod buf_read_iter;


// so `use protobuf::*` could work in mod descriptor and well_known_types
mod protobuf {
    pub use descriptor;
    pub use descriptorx;
    pub use reflect;
    pub use core::*;
    pub use enums::ProtobufEnum;
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
