mod case_convert;
mod convert;
mod linked_hash_map;
mod model;
mod parse_and_typecheck;
mod parse_dependencies;
mod parser;
mod path;
mod proto;
mod proto_path;
mod protobuf_abs_path;
mod protobuf_ident;
mod protobuf_path;
mod protobuf_rel_path;
mod rel_path;
mod test_against_protobuf_protos;

// Public API.
pub use case_convert::*;
pub use parse_and_typecheck::*;
pub use parse_dependencies::*;
// Non-public API used by codegen crate.
pub use proto_path::*;
pub use protobuf_abs_path::*;
pub use protobuf_ident::*;
pub use protobuf_rel_path::*;

#[derive(Clone)]
pub(crate) struct FileDescriptorPair {
    parsed: model::FileDescriptor,
    descriptor: protobuf::descriptor::FileDescriptorProto,
}
