use regex::Regex;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR;
use std::str;

use protobuf_test_common::build::copy_tests_v2_v3;
use protobuf_test_common::build::glob_simple;
use std::process::Command;
use std::process::Stdio;

fn to_paths(iter: impl IntoIterator<Item = impl Into<String>>) -> Vec<PathBuf> {
    iter.into_iter()
        .map(|item| item.into().replace(MAIN_SEPARATOR, "/").into())
        .collect()
}

#[derive(Default, Debug)]
struct TestStats {
    passed: u32,
    passed_marked_skip: u32,
    skipped: u32,
    failed: u32,
}

fn normalize_generated_file(contents: &str) -> String {
    let parsed_by = Regex::new("^// \\.proto file is parsed by.*").unwrap();

    let mut r = String::new();
    let mut inside_descriptor_data = false;
    for line in contents.lines() {
        let line = if inside_descriptor_data {
            if line == "\";" {
                inside_descriptor_data = false;
                line.to_owned()
            } else {
                continue;
            }
        } else if line.starts_with("static file_descriptor_proto_data") {
            inside_descriptor_data = true;
            line.to_owned()
        } else {
            parsed_by.replace(line, "").into_owned()
        };

        writeln!(&mut r, "{}", line).unwrap();
    }

    // sanity check
    assert!(!inside_descriptor_data);

    r
}

fn normalize_generated_file_in_place(path: &Path) {
    let contents = fs::read_to_string(path).unwrap();
    let contents = normalize_generated_file(&contents);
    fs::write(path, &contents).unwrap();
}

fn print_diff(dir: &Path, a: &Path, b: &Path) {
    if cfg!(windows) {
        // likely we don't have `diff` command on Windows
        return;
    }

    // Not using difference crate because it's slow for large files
    let output = Command::new("diff")
        .current_dir(dir)
        .arg("-u")
        .arg(a)
        .arg(b)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    print!("{}", str::from_utf8(&output.stdout).unwrap());
    print!("{}", str::from_utf8(&output.stderr).unwrap());
}

fn test_diff_in<F>(root: &str, s: &str, include: &str, should_skip: F)
where
    F: Fn(&str) -> bool,
{
    let mut stats = TestStats {
        passed: 0,
        passed_marked_skip: 0,
        skipped: 0,
        failed: 0,
    };

    let include_full = format!("{}/{}", root, include);
    let s_full = format!("{}/{}", root, s);

    let inputs_glob = format!("{}/*.proto", s_full);
    let inputs = to_paths(glob_simple(&inputs_glob));
    assert!(!inputs.is_empty(), "glob is empty: {}", inputs_glob);
    let includes = to_paths(vec![include_full.as_str(), "../proto"]);

    let temp_dir = tempfile::Builder::new()
        .prefix("protobuf-codegen-identical-test")
        .tempdir()
        .unwrap();

    let protoc_dir = format!("{}/protoc", temp_dir.path().to_str().unwrap());
    let pure_dir = format!("{}/pure", temp_dir.path().to_str().unwrap());

    fs::create_dir(&protoc_dir).unwrap();
    fs::create_dir(&pure_dir).unwrap();

    protoc_rust::Codegen::new()
        .inputs(&inputs)
        .includes(&includes)
        .out_dir(&protoc_dir)
        .run()
        .unwrap();

    protobuf_codegen_pure::Codegen::new()
        .inputs(&inputs)
        .includes(&includes)
        .out_dir(&pure_dir)
        .run()
        .unwrap();

    for input in &inputs {
        let label = input.strip_prefix(root).unwrap().to_str().unwrap();
        let proto_name = input.file_name().unwrap().to_str().unwrap();
        let rs_name = protobuf_codegen::proto_name_to_rs(proto_name);
        let protoc_rs = format!("{}/{}", protoc_dir, rs_name);
        let pure_rs = format!("{}/{}", pure_dir, rs_name);

        normalize_generated_file_in_place(Path::new(&protoc_rs));
        normalize_generated_file_in_place(Path::new(&pure_rs));

        let protoc_rs_contents =
            fs::read_to_string(&protoc_rs).expect(&format!("while reading {}", protoc_rs));
        let pure_rs_contents =
            fs::read_to_string(&pure_rs).expect(&format!("while reading {}", pure_rs));
        let skip = should_skip(input.to_str().unwrap());
        if protoc_rs_contents == pure_rs_contents {
            if !skip {
                stats.passed += 1;
                println!("{}: PASSED", label);
            } else {
                stats.passed_marked_skip += 1;
                println!("{}: PASSED BUT MARKED SKIP", label);
            }
        } else {
            if skip {
                stats.skipped += 1;
                println!("{} SKIPPED", label);
            } else {
                stats.failed += 1;
                println!("{} FAILED", label);
            }

            print_diff(
                temp_dir.path(),
                Path::new(&protoc_rs).strip_prefix(temp_dir.path()).unwrap(),
                Path::new(&pure_rs).strip_prefix(temp_dir.path()).unwrap(),
            );
        }
    }

    println!("{:?}", stats);
    assert!(
        stats.passed != 0 || s == "src/google/protobuf",
        "sanity check"
    );
    assert!(stats.failed == 0, "at least one test failed");
}

fn should_skip_with_marker(path: &str) -> bool {
    fs::read_to_string(path)
        .unwrap()
        .contains("@skip-codegen-identical-test")
}

#[test]
fn common_v2() {
    test_diff_in(
        "../protobuf-test",
        "src/common/v2",
        "src/common/v2",
        should_skip_with_marker,
    );
}

#[test]
fn common_v3() {
    let common_v3_root = tempfile::Builder::new()
        .prefix("common-v3")
        .tempdir()
        .unwrap();
    fs::create_dir_all(format!(
        "{}/src/common/v3",
        common_v3_root.path().to_str().unwrap()
    ))
    .unwrap();

    copy_tests_v2_v3(
        "../protobuf-test/src/common/v2",
        &format!("{}/src/common/v3", common_v3_root.path().to_str().unwrap()),
    );

    test_diff_in(
        common_v3_root.path().to_str().unwrap(),
        "src/common/v3",
        "src/common/v3",
        should_skip_with_marker,
    );
}

#[test]
fn v2() {
    test_diff_in(
        "../protobuf-test",
        "src/v2",
        "src/v2",
        should_skip_with_marker,
    );
}

#[test]
fn v3() {
    test_diff_in(
        "../protobuf-test",
        "src/v3",
        "src/v3",
        should_skip_with_marker,
    );
}

#[test]
fn interop() {
    test_diff_in(
        "../protobuf-test",
        "../interop/cxx",
        "../interop/cxx",
        should_skip_with_marker,
    );
}

#[test]
fn google() {
    test_diff_in("../protobuf-test", "src/google/protobuf", "src", |_| true);
}
