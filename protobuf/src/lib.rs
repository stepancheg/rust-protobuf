//! Library to read and write protocol buffers data.

#![deny(missing_docs)]
#![deny(broken_intra_doc_links)]
// Because we need compat with Rust 1.26
#![allow(bare_trait_objects)]

#[cfg(feature = "bytes")]
extern crate bytes;
#[cfg(feature = "with-serde")]
extern crate serde;
#[macro_use]
#[cfg(feature = "with-serde")]
extern crate serde_derive;
pub use crate::cached_size::CachedSize;
#[cfg(feature = "bytes")]
pub use crate::chars::Chars;
pub use crate::clear::Clear;
pub use crate::enums::ProtobufEnum;
pub use crate::error::ProtobufError;
pub use crate::error::ProtobufResult;
pub use crate::message::parse_from_bytes;
#[cfg(feature = "bytes")]
pub use crate::message::parse_from_carllerche_bytes;
pub use crate::message::parse_from_reader;
#[allow(deprecated)]
pub use crate::message::parse_length_delimited_from;
#[allow(deprecated)]
pub use crate::message::parse_length_delimited_from_bytes;
#[allow(deprecated)]
pub use crate::message::parse_length_delimited_from_reader;
pub use crate::message::Message;
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
mod enums;
pub mod error;
pub mod ext;
pub mod json;
pub mod lazy;
mod lazy_v2;
mod message;
pub mod reflect;
mod repeated;
pub mod rt;
mod singular;
pub mod stream;
pub mod text_format;
pub mod types;
pub mod well_known_types;
mod well_known_types_util;

// used by test
#[cfg(test)]
#[path = "../../protobuf-test-common/src/hex.rs"]
mod hex;

// used by rust-grpc
pub mod descriptorx;

mod cached_size;
mod chars;
#[doc(hidden)] // used by codegen
pub mod rust;
mod strx;
mod unknown;
mod varint;
mod zigzag;

mod misc;

mod buf_read_iter;

/// This symbol is in generated `version.rs`, include here for IDE
#[cfg(never)]
pub const VERSION: &str = "";
/// This symbol is in generated `version.rs`, include here for IDE
#[cfg(never)]
#[doc(hidden)]
pub const VERSION_IDENT: &str = "";
include!(concat!(env!("OUT_DIR"), "/version.rs"));
