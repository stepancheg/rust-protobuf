mod case_convert;
mod linked_hash_map;
mod parse_and_typecheck;
mod parser;
mod path;
mod proto;
mod proto_path;
mod protobuf_abs_path;
mod protobuf_ident;
mod protobuf_path;
mod protobuf_rel_path;
pub mod protoc;
pub mod pure;
mod rel_path;
mod test_against_protobuf_protos;
mod which_parser;

// Public API
// Non-public API used by codegen crate.
pub use case_convert::*;
pub use parse_and_typecheck::*;
pub use proto_path::*;
pub use protobuf_abs_path::*;
pub use protobuf_ident::*;
pub use protobuf_rel_path::*;
pub use which_parser::*;

use crate::pure::model;

#[derive(Clone)]
pub(crate) struct FileDescriptorPair {
    parsed: model::FileDescriptor,
    descriptor: protobuf::descriptor::FileDescriptorProto,
}
