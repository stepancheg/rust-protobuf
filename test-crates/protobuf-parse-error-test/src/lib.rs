use std::fs;

use protobuf_parse::Parser;
use protobuf_test_common::build::glob_simple;
use protoc_bin_vendored::protoc_bin_path;

#[test]
fn tests() {
    for f in glob_simple("test-data/*.proto") {
        test_file(&f)
    }
}

fn test_file(file: &str) {
    println!("Testing {}", file);

    let (expected_protoc, expected_pure) = parse_expected(file);

    let protoc_error = Parser::new()
        .protoc()
        .protoc_path(&protoc_bin_path().unwrap())
        .input(file)
        .include("test-data")
        .capture_stderr()
        .parse_and_typecheck()
        .err()
        .unwrap_or_else(|| {
            panic!("Protoc parse of `{file}` is expected to fail, but it was successful")
        });

    let pure_error = Parser::new()
        .pure()
        .input(file)
        .include("test-data")
        .parse_and_typecheck()
        .err()
        .unwrap_or_else(|| {
            panic!("Pure parse of `{file}` is expected to fail, but it was successful")
        });

    let protoc_error = format!("{:?}", protoc_error);
    let pure_error = format!("{:?}", pure_error);

    assert!(protoc_error.contains(&expected_protoc));
    assert!(
        pure_error.contains(&expected_pure),
        "pure error expected to contain {expected_pure:?}, but was {pure_error:?}"
    );
}

fn parse_expected(file: &str) -> (String, String) {
    let mut expected_protoc = None;
    let mut expected_pure = None;

    for line in fs::read_to_string(file).unwrap().lines() {
        let marker_protoc = "// expected protoc: ";
        let marker_pure = "// expected pure: ";
        if line.starts_with(marker_protoc) {
            assert!(expected_protoc.is_none());
            expected_protoc = Some(line[marker_protoc.len()..].to_owned());
        }
        if line.starts_with(marker_pure) {
            assert!(expected_pure.is_none());
            expected_pure = Some(line[marker_pure.len()..].to_owned());
        }
    }

    (expected_protoc.unwrap(), expected_pure.unwrap())
}
