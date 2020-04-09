use std::fs;
use std::path::Path;

use walkdir::WalkDir;

// Verify that protoc-rust and protobuf-codegen-pure produce byte-for-byte
// identical output.
#[test]
fn test_diff() {
    let mut failed = false;

    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        let path = entry.path().to_str().unwrap();
        let name = entry.file_name().to_os_string().into_string().unwrap();

        if !(name.ends_with("_pb.rs")
            || name.ends_with("_pb_proto3.rs")
            || (path.starts_with("src/google/protobuf") && name.ends_with(".rs")))
        {
            continue;
        }

        // Many files are expected to fail right now.
        let expect_fail = match path {
            | "src/google/protobuf/unittest_lazy_dependencies_enum.rs"
            | "src/google/protobuf/unittest_proto3_arena.rs"
            | "src/google/protobuf/unittest_no_arena_lite.rs"
            | "src/google/protobuf/unittest_drop_unknown_fields.rs"
            | "src/google/protobuf/unittest_lite_imports_nonlite.rs"
            | "src/google/protobuf/unittest_import.rs"
            | "src/google/protobuf/unittest_well_known_types.rs"
            | "src/google/protobuf/unittest_arena.rs"
            | "src/google/protobuf/unittest_no_field_presence.rs"
            | "src/google/protobuf/unittest_import_lite.rs"
            | "src/google/protobuf/unittest_no_generic_services.rs"
            | "src/google/protobuf/unittest_proto3_arena_lite.rs"
            | "src/google/protobuf/unittest.rs"
            | "src/google/protobuf/unittest_mset_wire_format.rs"
            | "src/google/protobuf/unittest_lazy_dependencies_custom_option.rs"
            | "src/google/protobuf/unittest_import_public.rs"
            | "src/google/protobuf/unittest_custom_options.rs"
            | "src/google/protobuf/unittest_lazy_dependencies.rs"
            | "src/google/protobuf/unittest_embed_optimize_for.rs"
            | "src/google/protobuf/unittest_no_arena.rs"
            | "src/google/protobuf/unittest_preserve_unknown_enum.rs"
            | "src/google/protobuf/unittest_proto3_lite.rs"
            | "src/google/protobuf/unittest_lite.rs"
            | "src/google/protobuf/unittest_optimize_for.rs"
            | "src/google/protobuf/unittest_mset.rs"
            | "src/google/protobuf/unittest_import_public_lite.rs"
            | "src/v2/test_oneof_default_value_pb.rs"
            | "src/v2/test_group_pb.rs"
            | "src/v2/test_oneof_group_pb.rs"
            | "src/v2/test_default_values_pb.rs"
            | "src/common/v2/test_lite_runtime_pb.rs"
            | "src/common/v2/test_service_pb.rs"
            | "src/common/v2/test_ext_pb.rs"
            | "src/common/v3/test_lite_runtime_pb.rs"
            | "src/common/v3/test_service_pb.rs"
            | "src/common/v3/test_ext_pb.rs" => true,
            _ => false,
        };

        print!("{}... ", path);

        // Delete the line that mentions which generator was used, since that
        // will obviously be different.
        let munge = |s: String| {
            s.lines()
                .filter(|l| !l.contains(".proto file is parsed by"))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let expected_path = Path::new("../protobuf-test").join(path);
        let expected = munge(fs::read_to_string(&expected_path).unwrap());
        let actual = munge(fs::read_to_string(&path).unwrap());
        if expected == actual && expect_fail {
            println!("FAIL (unexpectedly matched)");
            failed = true;
        } else if expected != actual && !expect_fail {
            println!("FAIL");
            failed = true;
        } else if expect_fail {
            println!("SKIP");
        } else {
            println!("PASS");
        }
    }

    if failed {
        panic!("at least one file did not match expected outcome");
    }
}
