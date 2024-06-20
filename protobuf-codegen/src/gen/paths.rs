use protobuf::descriptor::FileDescriptorProto;
use tracing::{instrument, Level};

use crate::gen::inside::protobuf_crate_path;
use crate::gen::rust::ident::RustIdent;
use crate::gen::rust::path::RustPath;
use crate::gen::strx;
use crate::gen::well_known_types::WELL_KNOWN_TYPES_PROTO_FILE_FULL_NAMES;
use crate::Customize;

// Copy-pasted from libsyntax.
fn ident_start(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

// Copy-pasted from libsyntax.
fn ident_continue(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_'
}

pub(crate) fn proto_path_to_rust_mod(path: &str) -> RustIdent {
    let without_dir = strx::remove_to(path, std::path::is_separator);
    let without_suffix = strx::remove_suffix(without_dir, ".proto");

    let name = without_suffix
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let valid = if i == 0 {
                ident_start(c)
            } else {
                ident_continue(c)
            };
            if valid {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();

    RustIdent::new(&name)
}

/// Used in protobuf-codegen-identical-test
pub fn proto_name_to_rs(proto_name: &str) -> String {
    format!("{}.rs", proto_path_to_rust_mod(proto_name))
}

/// Determines the one-for-one relative path in which a Rust source file
/// should be found for the provided `file_descriptor`.
pub fn file_descriptor_to_hierarchical_rs(file_descriptor: &FileDescriptorProto) -> String {
    match &file_descriptor.package {
        Some(package) => format!(
            "{}/{}",
            package.replace('.', "/"),
            proto_name_to_rs(file_descriptor.name())
        ),
        None => proto_name_to_rs(file_descriptor.name()),
    }
}

#[instrument(level = Level::DEBUG, skip(customize), ret(Display))]
pub(crate) fn proto_path_to_fn_file_descriptor(
    proto_path: &str,
    customize: &Customize,
) -> RustPath {
    let protobuf_crate = protobuf_crate_path(customize);
    match proto_path {
        "rustproto.proto" => protobuf_crate.append("rustproto::file_descriptor".into()),
        "google/protobuf/descriptor.proto" => {
            protobuf_crate.append("descriptor::file_descriptor".into())
        }
        s if WELL_KNOWN_TYPES_PROTO_FILE_FULL_NAMES.contains(&s) => protobuf_crate
            .append_ident("well_known_types".into())
            .append_ident(proto_path_to_rust_mod(s))
            .append_ident("file_descriptor".into()),
        s => {
            if let Some(mod_path) = &customize.gen_mod_rs_hierarchy_out_dir_mod_name {
                let mut rust_path = RustPath::from("crate");
                for mod_part in mod_path.split("::") {
                    rust_path = rust_path.append_ident(RustIdent::from(mod_part));
                }
                for component in proto_path.split("/").filter(|p| !p.ends_with(".proto")) {
                    rust_path = rust_path.append_ident(RustIdent::from(component));
                }
                rust_path.append_ident("file_descriptor".into())
            } else {
                RustPath::super_path()
                    .append_ident(proto_path_to_rust_mod(s))
                    .append_ident("file_descriptor".into())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::proto_path_to_rust_mod;
    use crate::gen::rust::ident::RustIdent;

    #[test]
    fn test_mod_path_proto_ext() {
        assert_eq!(
            RustIdent::from("proto"),
            proto_path_to_rust_mod("proto.proto")
        );
    }

    #[test]
    fn test_mod_path_unknown_ext() {
        assert_eq!(
            RustIdent::from("proto_proto3"),
            proto_path_to_rust_mod("proto.proto3")
        );
    }

    #[test]
    fn test_mod_path_empty_ext() {
        assert_eq!(RustIdent::from("proto"), proto_path_to_rust_mod("proto"));
    }

    #[test]
    fn test_mod_path_dir() {
        assert_eq!(
            RustIdent::from("baz"),
            proto_path_to_rust_mod("foo/bar/baz.proto"),
        )
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_mod_path_dir_backslashes() {
        assert_eq!(
            RustIdent::from("baz"),
            proto_path_to_rust_mod("foo\\bar\\baz.proto"),
        )
    }
}
