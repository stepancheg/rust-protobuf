extern crate env_logger;
extern crate glob;
extern crate log;

extern crate protobuf_codegen_pure;

extern crate protobuf_test_common;

use std::io::Read;
use std::io::Write;

use std::fs;
use std::path::Path;

use protobuf_test_common::build::*;

fn copy_test<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dst: P2) {
    eprintln!("copy {:?} to {:?}", src.as_ref(), dst.as_ref());
    let mut content = Vec::new();
    fs::File::open(src.as_ref())
        .expect(&format!("open {}", src.as_ref().display()))
        .read_to_end(&mut content)
        .expect("read_to_end");

    let mut write = fs::File::create(dst).expect("create");
    writeln!(write, "// generated").expect("write");
    writeln!(write, "// copied from {}", src.as_ref().display()).expect("write");
    writeln!(write, "").expect("write");
    write.write_all(&content).expect("write_all");
    writeln!(write, "// generated").expect("write");
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
        include_dir,
        |GenInDirArgs {
             out_dir,
             input,
             includes,
             customize,
         }| {
            protobuf_codegen_pure::Codegen::new()
                .out_dir(out_dir)
                .inputs(input)
                .includes(includes)
                .customize(customize)
                .run()
        },
    );
}

fn print_rerun_if_changed<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    // Doesn't seem to do anything
    println!("rerun-if-changed={}", path.to_str().expect("to_str"));
    if path.is_dir() {
        for child in fs::read_dir(path).expect("read_dir") {
            let child = child.expect("child").path();
            print_rerun_if_changed(child);
        }
    }
}

fn generate_interop() {
    copy_from_protobuf_test("src/interop/mod.rs");
    copy_from_protobuf_test("src/interop/json.rs");

    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/interop")
        .includes(&["../interop/cxx", "../proto"])
        .input("../interop/cxx/interop_pb.proto")
        .run()
        .unwrap();
}

fn generate_pb_rs() {
    print_rerun_if_changed("../protobuf-test");

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
}

fn main() {
    env_logger::init();

    cfg_serde();

    clean_old_files();
    generate_pb_rs();
}
