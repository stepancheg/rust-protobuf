//! Library to read and write protocol buffers data.
//!
//! Rust files from `.proto` files can be generated with
//! `protobuf-codegen`, `protobuf-codegen-pure` crates.
//! See readme on the [project github page](https://github.com/stepancheg/rust-protobuf).

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "bytes")]
extern crate bytes;
#[cfg(feature = "with-serde")]
extern crate serde;
#[macro_use]
#[cfg(feature = "with-serde")]
extern crate serde_derive;
pub use crate::clear::Clear;
pub use crate::coded_input_stream::CodedInputStream;
pub use crate::coded_output_stream::CodedOutputStream;
pub use crate::enums::ProtobufEnum;
pub use crate::enums::ProtobufEnumOrUnknown;
pub use crate::message::Message;
pub use crate::message_dyn::MessageDyn;
pub use crate::message_field::MessageField;
pub use crate::oneof::Oneof;
pub use crate::unknown::UnknownFields;
pub use crate::unknown::UnknownFieldsIter;
pub use crate::unknown::UnknownValue;
pub use crate::unknown::UnknownValueRef;
pub use crate::unknown::UnknownValues;
pub use crate::unknown::UnknownValuesIter;
pub mod wire_format;
#[cfg(feature = "bytes")]
pub use crate::chars::Chars;
pub use crate::error::ProtobufError;
pub use crate::error::ProtobufResult;

// generated
pub mod descriptor;
// TODO: move plugin to mod codegen
pub mod plugin;
pub mod rustproto;

mod clear;
mod coded_input_stream;
mod coded_output_stream;
mod enums;
mod error;
pub mod ext;
pub mod json;
mod lazy_v2;
mod message;
mod message_dyn;
mod message_field;
mod oneof;
pub mod reflect;
pub mod rt;
pub mod text_format;
pub mod well_known_types;
mod well_known_types_util;

// used by test
#[cfg(test)]
#[path = "../../protobuf-test-common/src/hex.rs"]
mod hex;

mod cached_size;
mod chars;
mod unknown;
mod varint;
mod zigzag;

mod misc;

mod buf_read_iter;
mod buf_read_or_reader;

// TODO: does not work: https://github.com/rust-lang/rust/issues/67295
#[cfg(doctest)]
mod doctest_pb;

/// This symbol is in generated `version.rs`, include here for IDE
#[cfg(never)]
pub const VERSION: &str = "";
/// This symbol is in generated `version.rs`, include here for IDE
#[cfg(never)]
#[doc(hidden)]
pub const VERSION_IDENT: &str = "";
include!(concat!(env!("OUT_DIR"), "/version.rs"));
