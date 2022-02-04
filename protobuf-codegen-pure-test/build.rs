extern crate env_logger;
extern crate glob;
extern crate log;

extern crate protobuf_test_common;

use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use protobuf_codegen::Codegen;
use protobuf_test_common::build::*;
use protobuf_test_common::print_rerun_if_changed_recursively;

fn copy_test<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dst: P2) {
    eprintln!("copy {:?} to {:?}", src.as_ref(), dst.as_ref());
    let mut content = Vec::new();
    fs::File::open(src.as_ref())
        .expect(&format!("open {}", src.as_ref().display()))
        .read_to_end(&mut content)
        .expect(&format!("read_to_end {}", src.as_ref().display()));

    let mut write = fs::File::create(dst).expect("create");
    writeln!(write, "// @generated").expect("write");
    writeln!(write, "// copied from {}", src.as_ref().display()).expect("write");
    writeln!(write, "").expect("write");
    write.write_all(&content).expect("write_all");
    // Print generated twice to avoid overlooking it accidentally
    writeln!(write, "// @generated").expect("write");
    write.flush().expect("flush");
}

fn copy_from_protobuf_test(path: &str) {
    copy_test(&format!("../protobuf-test/{}", path), &format!("{}", path))
}

enum FileNameClass {
    ModRs,
    TestRs,
    Proto,
    GeneratedRs,
    Ignore,
}

fn classify_file_name(dir: &str, name: &str) -> FileNameClass {
    if name.starts_with(".") || name.ends_with(".md") || name.ends_with(".sh") {
        FileNameClass::Ignore
    } else if name.ends_with("_pb.rs") || name.ends_with("_pb_proto3.rs") {
        FileNameClass::GeneratedRs
    } else if name == "mod.rs" {
        FileNameClass::ModRs
    } else if name.ends_with(".proto") || name.ends_with(".proto3") {
        FileNameClass::Proto
    } else if name.ends_with(".rs") {
        if dir == "src/google/protobuf" {
            FileNameClass::GeneratedRs
        } else {
            FileNameClass::TestRs
        }
    } else {
        panic!("unknown test file: {}", name);
    }
}

// Copy tests from `protobuf-test` directory to the same directory here
fn copy_tests(dir: &str) {
    let src_dir = format!("../protobuf-test/{}", dir);
    for entry in fs::read_dir(&src_dir).expect(&format!("read_dir {}", src_dir)) {
        let file_name = entry.expect("entry").file_name().into_string().unwrap();

        match classify_file_name(dir, &file_name) {
            FileNameClass::ModRs | FileNameClass::Ignore | FileNameClass::GeneratedRs => {}
            FileNameClass::TestRs | FileNameClass::Proto => {
                copy_from_protobuf_test(&format!("{}/{}", dir, file_name))
            }
        }
    }
}

fn gen_in_dir(dir: &str, include_dir: &str) {
    gen_in_dir_impl(
        dir,
        |GenInDirArgs {
             out_dir,
             input,
             customize,
         }| {
            Codegen::new()
                .pure()
                .out_dir(out_dir)
                .inputs(input)
                .includes(&[include_dir])
                .customize(customize)
                .run_from_script()
        },
    );
}

fn generate_interop() {
    copy_from_protobuf_test("src/interop/mod.rs");
    copy_from_protobuf_test("src/interop/json.rs");
    copy_from_protobuf_test("src/interop/bin.rs");

    Codegen::new()
        .pure()
        .out_dir("src/interop")
        .includes(&["../interop/cxx", "../proto"])
        .input("../interop/cxx/interop_pb.proto")
        .run_from_script();
}

fn generate_include_generated() {
    copy_from_protobuf_test("src/include_generated/mod.rs");

    let dir = format!("{}/include_generated", env::var("OUT_DIR").unwrap());
    if Path::new(&dir).exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    fs::create_dir(&dir).unwrap();
    Codegen::new()
        .pure()
        .out_dir(dir)
        .input("../protobuf-test/src/include_generated/v2.proto")
        .input("../protobuf-test/src/include_generated/v3.proto")
        .customize(Customize::default().gen_mod_rs(true))
        .include("../protobuf-test/src/include_generated")
        .run_from_script();
}

fn generate_pb_rs() {
    print_rerun_if_changed_recursively("../protobuf-test");

    copy_tests("src/v2");
    gen_in_dir("src/v2", "src/v2");

    copy_tests("src/v3");
    gen_in_dir("src/v3", "src/v3");

    copy_tests("src/common/v2");
    gen_in_dir("src/common/v2", "src/common/v2");

    copy_tests_v2_v3("src/common/v2", "src/common/v3");
    gen_in_dir("src/common/v3", "src/common/v3");

    copy_tests("src/google/protobuf");
    gen_in_dir("src/google/protobuf", "src");

    generate_interop();

    generate_include_generated();
}

fn main() {
    env_logger::init();

    cfg_serde();

    clean_old_files();
    generate_pb_rs();
}
