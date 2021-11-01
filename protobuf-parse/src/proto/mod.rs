//! This folder contains copy of .proto files
//! needed for pure codegen.
//!
//! Files are copied here because when publishing to crates,
//! referencing files from outside is not allowed.

pub(crate) const RUSTPROTO_PROTO: &str = include_str!("rustproto.proto");
pub(crate) const ANY_PROTO: &str = include_str!("google/protobuf/any.proto");
pub(crate) const API_PROTO: &str = include_str!("google/protobuf/api.proto");
pub(crate) const DESCRIPTOR_PROTO: &str = include_str!("google/protobuf/descriptor.proto");
pub(crate) const DURATION_PROTO: &str = include_str!("google/protobuf/duration.proto");
pub(crate) const EMPTY_PROTO: &str = include_str!("google/protobuf/empty.proto");
pub(crate) const FIELD_MASK_PROTO: &str = include_str!("google/protobuf/field_mask.proto");
pub(crate) const SOURCE_CONTEXT_PROTO: &str = include_str!("google/protobuf/source_context.proto");
pub(crate) const STRUCT_PROTO: &str = include_str!("google/protobuf/struct.proto");
pub(crate) const TIMESTAMP_PROTO: &str = include_str!("google/protobuf/timestamp.proto");
pub(crate) const TYPE_PROTO: &str = include_str!("google/protobuf/type.proto");
pub(crate) const WRAPPERS_PROTO: &str = include_str!("google/protobuf/wrappers.proto");
