use crate::customize::Customize;
use crate::gen::rust::path::RustPath;

/// Path to `protobuf` crate, different when `.proto` file is
/// used inside or outside of protobuf crate.
pub(crate) fn protobuf_crate_path(customize: &Customize) -> RustPath {
    match customize.inside_protobuf {
        Some(true) => RustPath::from("crate"),
        _ => RustPath::from("::protobuf"),
    }
}
