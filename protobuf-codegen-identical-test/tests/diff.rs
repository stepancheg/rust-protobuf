use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io;
use std::path::{PathBuf, MAIN_SEPARATOR};

use difference::Changeset;

use protobuf::descriptor::{FileDescriptorProto, FileDescriptorSet};
use protobuf_test_common::build::glob_simple;
use protoc::Protoc;

fn to_paths(iter: impl IntoIterator<Item = impl Into<String>>) -> Vec<PathBuf> {
    iter.into_iter()
        .map(|item| item.into().replace(MAIN_SEPARATOR, "/").into())
        .collect()
}

fn test_diff_in<F>(s: &str, include: &str, should_skip: F, failed: &mut bool)
where
    F: Fn(&str) -> bool,
{
    let inputs = to_paths(glob_simple(&format!("{}/*.proto", s)));
    let includes = to_paths(vec![include, "../proto"]);

    #[derive(Debug)]
    struct Expectation {
        skip: bool,
        descriptor: FileDescriptorProto,
    }

    let expectations: BTreeMap<String, Expectation> = {
        let temp_dir = tempfile::Builder::new()
            .prefix("protoc-rust")
            .tempdir()
            .unwrap();
        let temp_file = temp_dir.path().join("descriptor.pbbin");

        Protoc::from_env_path()
            .descriptor_set_out_args()
            .out(&temp_file)
            .inputs(&inputs)
            .includes(&includes)
            .write_descriptor_set()
            .unwrap();

        let fds = fs::read(temp_file).unwrap();
        let fds: FileDescriptorSet = protobuf::parse_from_bytes(&fds)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            .unwrap();

        fds.file
            .into_iter()
            .map(|descriptor| {
                let name = String::from(descriptor.get_name());
                let expectation = Expectation {
                    skip: should_skip(&format!("{}/{}", include, name)),
                    descriptor,
                };
                (name, expectation)
            })
            .collect()
    };

    let pure = protobuf_codegen_pure::parse_and_typecheck(&includes, &inputs).unwrap();

    let actuals: BTreeMap<String, FileDescriptorProto> = pure
        .file_descriptors
        .into_iter()
        .map(|fd| (fd.get_name().into(), fd))
        .collect();

    for name in pure.relative_paths {
        print!("{}/{}... ", include, name.display());
        let name = name.to_str().unwrap();
        let expectation = &expectations[name];
        let actual = &actuals[name];
        if expectation.descriptor == *actual && expectation.skip {
            println!("FAIL (unexpectedly matched)");
            *failed = true;
        } else if expectation.descriptor != *actual && !expectation.skip {
            println!("FAIL");
            println!(
                "{}",
                Changeset::new(
                    &format!("{:#?}", expectation.descriptor),
                    &format!("{:#?}", actual),
                    "\n"
                )
            );
            *failed = true;
        } else if expectation.skip {
            println!("SKIP");
        } else {
            println!("PASS");
        }
    }
}

#[test]
fn test_diff() {
    env::set_current_dir("../protobuf-test").unwrap();

    let should_skip = |path: &str| {
        fs::read_to_string(path)
            .unwrap()
            .contains("@skip-codegen-identical-test")
    };

    let mut failed = false;
    test_diff_in("src/v2", "src/v2", should_skip, &mut failed);
    test_diff_in("src/v3", "src/v3", should_skip, &mut failed);
    test_diff_in("src/common/v2", "src/common/v2", should_skip, &mut failed);
    test_diff_in("src/common/v3", "src/common/v3", should_skip, &mut failed);
    test_diff_in("../interop/cxx", "../interop/cxx", should_skip, &mut failed);
    test_diff_in("src/google/protobuf", "src", |_| true, &mut failed);

    assert!(!failed, "at least one test failed");
}
