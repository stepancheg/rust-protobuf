extern crate protoc;

use std::path::PathBuf;

fn main() {
    let exe_suffix = if cfg!(windows) {
        ".exe"
    } else if cfg!(unix) {
        ""
    } else {
        panic!("unknown OS")
    };

    let protoc_gen_rust = PathBuf::from(format!("../../target/debug/protoc-gen-rust{}", exe_suffix));
    let protoc_gen_rust = protoc_gen_rust.canonicalize().expect("canonicalize");

    assert!(protoc_gen_rust.is_file(), "{:?}", protoc_gen_rust);

    protoc::run(protoc::Args {
        lang: "rust",
        out_dir: "src",
        plugin: Some(&format!("protoc-gen-rust={}", protoc_gen_rust.as_os_str().to_str().unwrap())),
        input: &["src/data.proto"],
        ..Default::default()
    }).expect("protoc");
}
