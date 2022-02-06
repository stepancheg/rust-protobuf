extern crate env_logger;
extern crate glob;
extern crate log;

extern crate protoc;

extern crate protobuf_test_common;

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use protobuf_codegen::Codegen;
use protobuf_test_common::build::*;
use protoc::Protoc;

fn test_protoc_bin_path() -> PathBuf {
    let path = protoc_bin_vendored::protoc_bin_path().unwrap();
    assert!(Protoc::from_path(&path).version().unwrap().is_3());
    path
}

fn codegen() -> Codegen {
    let mut codegen = Codegen::new();
    codegen.protoc();
    codegen.protoc_path(test_protoc_bin_path());
    codegen.extra_arg("--experimental_allow_proto3_optional");
    codegen
}

fn gen_in_dir(dir: &str, include_dir: &str) {
    gen_in_dir_impl(
        dir,
        |GenInDirArgs {
             out_dir,
             input,
             customize,
         }| {
            codegen()
                .out_dir(out_dir)
                .inputs(input)
                .includes(&["../proto", include_dir])
                .customize(customize)
                .run_from_script()
        },
    );
}

fn generate_in_common() {
    gen_in_dir("src/common/v2", "src/common/v2");

    copy_tests_v2_v3("src/common/v2", "src/common/v3");
    gen_in_dir("src/common/v3", "src/common/v3");
}

fn generate_in_v2_v3() {
    gen_in_dir("src/v2", "src/v2");

    gen_in_dir("src/v3", "src/v3");

    gen_in_dir("src/google/protobuf", "src");
}

fn generate_interop() {
    codegen()
        .out_dir("src/interop")
        .includes(&["../interop/cxx", "../proto"])
        .input("../interop/cxx/interop_pb.proto")
        .run_from_script();
}

fn generate_include_generated() {
    let dir = format!("{}/include_generated", env::var("OUT_DIR").unwrap());
    if Path::new(&dir).exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    fs::create_dir(&dir).unwrap();
    Codegen::new()
        .protoc()
        .out_dir(dir)
        .input("src/include_generated/v2.proto")
        .input("src/include_generated/v3.proto")
        .customize(Customize::default().gen_mod_rs(true))
        .include("src/include_generated")
        .run_from_script();
}

fn generate_pb_rs() {
    generate_in_common();
    generate_in_v2_v3();
    generate_interop();
    generate_include_generated();
}

fn main() {
    env_logger::init();

    clean_old_files();

    generate_pb_rs();
}
