//! Library to read and write protocol buffers data.

#![deny(missing_docs)]
#![deny(intra_doc_link_resolution_failure)]
// Because we need compat with Rust 1.26
#![allow(bare_trait_objects)]
#![cfg_attr(rustc_nightly, feature(specialization))]

#[cfg(feature = "bytes")]
extern crate bytes;
#[cfg(feature = "with-serde")]
extern crate serde;
#[macro_use]
#[cfg(feature = "with-serde")]
extern crate serde_derive;
pub use crate::cached_size::CachedSize;
#[cfg(feature = "bytes")]
pub use chars::Chars;
pub use crate::clear::Clear;
pub use crate::core::parse_from_bytes;
#[cfg(feature = "bytes")]
pub use core::parse_from_carllerche_bytes;
pub use crate::core::parse_from_reader;
pub use crate::core::parse_length_delimited_from;
pub use crate::core::parse_length_delimited_from_bytes;
pub use crate::core::parse_length_delimited_from_reader;
pub use crate::core::Message;
pub use crate::enums::ProtobufEnum;
pub use crate::error::ProtobufError;
pub use crate::error::ProtobufResult;
pub use crate::repeated::RepeatedField;
pub use crate::singular::SingularField;
pub use crate::singular::SingularPtrField;
pub use crate::stream::wire_format;
pub use crate::stream::CodedInputStream;
pub use crate::stream::CodedOutputStream;
pub use crate::unknown::UnknownFields;
pub use crate::unknown::UnknownFieldsIter;
pub use crate::unknown::UnknownValue;
pub use crate::unknown::UnknownValueRef;
pub use crate::unknown::UnknownValues;
pub use crate::unknown::UnknownValuesIter;

// generated
pub mod descriptor;
pub mod plugin;
pub mod rustproto;

mod clear;
pub mod compiler_plugin;
mod core;
mod enums;
pub mod error;
pub mod ext;
pub mod lazy;
pub mod reflect;
mod repeated;
pub mod rt;
mod singular;
pub mod stream;
pub mod text_format;
pub mod types;
pub mod well_known_types;

// used by test
#[cfg(test)]
#[path = "../../protobuf-test-common/src/hex.rs"]
mod hex;

// used by rust-grpc
pub mod descriptorx;

mod cached_size;
#[cfg(feature = "bytes")]
mod chars;
#[doc(hidden)] // used by codegen
pub mod rust;
mod strx;
mod unknown;
mod varint;
mod zigzag;

mod misc;

mod buf_read_iter;

// so `use protobuf::*` could work in mod descriptor and well_known_types
mod protobuf {
    pub use crate::cached_size::CachedSize;
    pub use crate::clear::Clear;
    pub use crate::core::*;
    pub use crate::descriptor;
    pub use crate::descriptorx;
    pub use crate::enums::ProtobufEnum;
    pub use crate::error::*;
    pub use crate::ext;
    pub use crate::lazy;
    pub use crate::reflect;
    pub use crate::repeated::RepeatedField;
    pub use crate::rt;
    pub use crate::singular::SingularField;
    pub use crate::singular::SingularPtrField;
    pub use crate::stream::*;
    pub use crate::text_format;
    pub use crate::types;
    pub use crate::unknown::UnknownFields;
    pub use crate::unknown::UnknownFieldsIter;
    pub use crate::unknown::UnknownValue;
    pub use crate::unknown::UnknownValueRef;
    pub use crate::unknown::UnknownValues;
    pub use crate::unknown::UnknownValuesIter;
    pub use crate::well_known_types;
}

/// This symbol is in generated `version.rs`, include here for IDE
#[cfg(never)]
pub const VERSION: &str = "";
/// This symbol is in generated `version.rs`, include here for IDE
#[cfg(never)]
#[doc(hidden)]
pub const VERSION_IDENT: &str = "";
include!(concat!(env!("OUT_DIR"), "/version.rs"));
