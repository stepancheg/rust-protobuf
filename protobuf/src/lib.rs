//! Library to read and write protocol buffers data.

// TODO: add docs
//#![deny(missing_docs)]

#![cfg_attr(rustc_nightly, feature(specialization))]

#[cfg(feature = "bytes")]
extern crate bytes;
#[cfg(feature = "with-serde")]
extern crate serde;
#[macro_use]
#[cfg(feature = "with-serde")]

extern crate serde_derive;
pub use clear::Clear;
pub use core::parse_from_bytes;
#[cfg(feature = "bytes")]
pub use core::parse_from_carllerche_bytes;
pub use core::parse_from_reader;
pub use core::Message;
pub use enums::ProtobufEnum;
pub use enums::ProtobufEnumOrUnknown;
pub use oneof::Oneof;
pub use repeated::RepeatedField;
pub use singular::SingularField;
pub use singular::SingularPtrField;
pub use stream::CodedInputStream;
pub use stream::CodedOutputStream;
pub use unknown::UnknownFields;
pub use unknown::UnknownFieldsIter;
pub use unknown::UnknownValue;
pub use unknown::UnknownValueRef;
pub use unknown::UnknownValues;
pub use unknown::UnknownValuesIter;
pub mod wire_format;
pub use cached_size::CachedSize;
#[cfg(feature = "bytes")]
pub use chars::Chars;
pub use error::ProtobufError;
pub use error::ProtobufResult;

pub use reflect::types;

// generated
pub mod descriptor;
// TODO: move plugin to mod codegen
pub mod plugin;
pub mod rustproto;

mod clear;
mod core;
mod enums;
pub mod error;
pub mod ext;
pub mod json;
mod lazy;
mod oneof;
pub mod prelude;
pub mod reflect;
mod repeated;
pub mod rt;
mod singular;
pub mod stream;
pub mod text_format;
pub mod well_known_types;

// used by test
#[cfg(test)]
#[path = "../../protobuf-test-common/src/hex.rs"]
mod hex;

mod cached_size;
mod chars;
mod paginate;
mod unknown;
mod varint;
mod zigzag;

mod misc;

mod buf_read_iter;

// so `use protobuf::*` could work in mod descriptor and well_known_types
mod protobuf {
    pub use cached_size::CachedSize;
    pub use clear::Clear;
    pub use core::*;
    pub use descriptor;
    pub use enums::ProtobufEnum;
    pub use error::*;
    pub use ext;
    pub use oneof::Oneof;
    pub use reflect;
    pub use reflect::types;
    pub use repeated::RepeatedField;
    pub use rt;
    pub use singular::SingularField;
    pub use singular::SingularPtrField;
    pub use stream::*;
    pub use text_format;
    pub use unknown::UnknownFields;
    pub use unknown::UnknownFieldsIter;
    pub use unknown::UnknownValue;
    pub use unknown::UnknownValueRef;
    pub use unknown::UnknownValues;
    pub use unknown::UnknownValuesIter;
    pub use well_known_types;
    pub use wire_format;
}
