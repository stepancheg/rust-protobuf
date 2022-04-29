//! # Parse `.proto` files
//!
//! Parse `.proto` file definitions, **not** the protobuf text format serialization.
//!
//! Files can be parsed using pure Rust parser (mod `pure`)
//! or using the `protoc` command (mod `protoc`).
//!
//! This crate is not meant to be used directly, but rather through the `protobuf-codegen` crate.
//! If you think this crate might be useful to you,
//! please [consider creating an issue](https://github.com/stepancheg/rust-protobuf/issues/new),
//! until that this crate is considered to have **no stable API**.

extern crate core;

mod case_convert;
mod parse_and_typecheck;
mod parser;
mod path;
mod proto;
mod proto_path;
mod protobuf_abs_path;
mod protobuf_ident;
mod protobuf_path;
mod protobuf_rel_path;
pub(crate) mod protoc;
pub mod pure;
mod rel_path;
mod test_against_protobuf_protos;
mod which_parser;

// Public API
// Non-public API used by codegen crate.
pub use case_convert::*;
pub use parse_and_typecheck::*;
pub use parser::Parser;
pub use proto_path::*;
use protobuf::reflect::FileDescriptor;
pub use protobuf_abs_path::*;
pub use protobuf_ident::*;
pub use protobuf_rel_path::*;

use crate::pure::model;

#[derive(Clone)]
pub(crate) struct FileDescriptorPair {
    pub(crate) parsed: model::FileDescriptor,
    pub(crate) descriptor_proto: protobuf::descriptor::FileDescriptorProto,
    pub(crate) descriptor: FileDescriptor,
}
