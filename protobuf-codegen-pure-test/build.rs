extern crate glob;
extern crate log;
extern crate env_logger;

extern crate protobuf_codegen_pure;

extern crate protobuf_test_common;

use std::io::Read;
use std::io::Write;

use std::fs;
use std::path::Path;

use protobuf_test_common::build::*;


fn copy_test<P1 : AsRef<Path>, P2 : AsRef<Path>>(src: P1, dst: P2) {
    let mut content = Vec::new();
    fs::File::open(src.as_ref()).expect(&format!("open {}", src.as_ref().display()))
        .read_to_end(&mut content).expect("read_to_end");

    let mut write = fs::File::create(dst).expect("create");
    writeln!(write, "// generated").expect("write");
    writeln!(write, "// copied from {}", src.as_ref().display()).expect("write");
    writeln!(write, "").expect("write");
    write.write_all(&content).expect("write_all");
    write.flush().expect("flush");
}


// Copy tests from `protobuf-test` directory to the same directory here
fn copy_tests(dir: &str) {
    let src_dir = format!("../protobuf-test/{}", dir);
    for entry in fs::read_dir(&src_dir).expect(&format!("read_dir {}", src_dir)) {
        let file_name = entry.expect("entry").file_name().into_string().unwrap();

        // Only copy rust and proto files
        if !file_name.ends_with(".rs")
            && !file_name.ends_with(".proto") && !file_name.ends_with(".proto3")
        {
            continue;
        }

        // Generated proto files
        if file_name.ends_with("_pb.rs") || file_name.ends_with("_pb_proto3.rs") {
            continue;
        }

        if file_name.starts_with(".") || file_name == "mod.rs" {
            continue;
        }

        copy_test(
            &format!("../protobuf-test/{}/{}", dir, file_name),
            &format!("{}/{}", dir, file_name),
        )
    }
}


fn gen_in_dir(dir: &str, include_dir: &str) {
    gen_in_dir_impl(dir, include_dir, true, |GenInDirArgs { out_dir, input, includes, customize }| {
        protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
            out_dir, input, includes, customize
        })
    });
}


fn generate_pb_rs() {
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
}


fn main() {
    env_logger::init();

    clean_old_files();
    generate_pb_rs();

    println!("cargo:rustc-cfg=protoc3");
}
