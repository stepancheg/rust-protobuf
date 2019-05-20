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
pub use crate::clear::Clear;
pub use crate::core::parse_from_bytes;
#[cfg(feature = "bytes")]
pub use crate::core::parse_from_carllerche_bytes;
pub use crate::core::parse_from_reader;
pub use crate::core::Message;
pub use crate::enums::ProtobufEnum;
pub use crate::enums::ProtobufEnumOrUnknown;
pub use crate::oneof::Oneof;
pub use crate::repeated::RepeatedField;
pub use crate::singular::SingularField;
pub use crate::singular::SingularPtrField;
pub use crate::stream::CodedInputStream;
pub use crate::stream::CodedOutputStream;
pub use crate::unknown::UnknownFields;
pub use crate::unknown::UnknownFieldsIter;
pub use crate::unknown::UnknownValue;
pub use crate::unknown::UnknownValueRef;
pub use crate::unknown::UnknownValues;
pub use crate::unknown::UnknownValuesIter;
pub mod wire_format;
pub use cached_size::CachedSize;
#[cfg(feature = "bytes")]
pub use crate::chars::Chars;
pub use crate::error::ProtobufError;
pub use crate::error::ProtobufResult;

pub use crate::reflect::types;

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
