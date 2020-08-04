use crate::inside::protobuf_crate_path;
use crate::rust_name::RustIdent;
use crate::rust_name::RustPath;
use crate::strx;
use crate::rust;
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

    let name = if rust::is_rust_keyword(&name) {
        format!("{}_pb", name)
    } else {
        name
    };
    RustIdent::from(name)
}

/// Used in protobuf-codegen-identical-test
pub fn proto_name_to_rs(proto_file_path: &str) -> String {
    format!("{}.rs", proto_path_to_rust_mod(proto_file_path))
}

pub(crate) fn proto_path_to_rust_mod_rel(proto_path: &str, customize: &Customize) -> RustPath {
    let protobuf_crate = protobuf_crate_path(customize);
    match proto_path {
        "rustproto.proto" => protobuf_crate.append_ident("rustproto".into()),
        "google/protobuf/descriptor.proto" => protobuf_crate.append_ident("descriptor".into()),
        "google/protobuf/any.proto" => protobuf_crate.append("well_known_types::any".into()),
        "google/protobuf/api.proto" => protobuf_crate.append("well_known_types::api".into()),
        "google/protobuf/duration.proto" => {
            protobuf_crate.append("well_known_types::duration".into())
        }
        "google/protobuf/empty.proto" => protobuf_crate.append("well_known_types::empty".into()),
        "google/protobuf/field_mask.proto" => {
            protobuf_crate.append("well_known_types::field_mask".into())
        }
        "google/protobuf/source_context.proto" => {
            protobuf_crate.append("well_known_types::source_context".into())
        }
        "google/protobuf/struct.proto" => {
            protobuf_crate.append("well_known_types::struct_pb".into())
        }
        "google/protobuf/timestamp.proto" => {
            protobuf_crate.append("well_known_types::timestamp".into())
        }
        "google/protobuf/type.proto" => protobuf_crate.append("well_known_types::type_pb".into()),
        "google/protobuf/wrappers.proto" => {
            protobuf_crate.append("well_known_types::wrappers".into())
        }
        s => RustPath::super_path().append_ident(proto_path_to_rust_mod(s)),
    }
}

#[cfg(test)]
mod test {

    use super::proto_path_to_rust_mod;
    use crate::rust_name::RustIdent;

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
