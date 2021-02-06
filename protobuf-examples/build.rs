use std::env;
use std::fs;

use protobuf_codegen_pure::Customize;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // We generate descriptors twice: with pure rust codegen
    // and with codegen depending on `protoc` binary.
    // This is for demonstration purposes; in practice you'd need either of them.
    //
    // Note there's a third option: using `protoc` binary directly and `protoc-gen-rust`
    // plugin, this is a canonical way to generate protobuf sources.
    // This is not possible to do with Cargo (since Cargo cannot depend on binaries)
    // but can be used with some other build system.

    let generated_with_pure_dir = format!("{}/generated_with_pure", out_dir);
    let generated_with_native_dir = format!("{}/generated_with_native", out_dir);

    if Path::new(&generated_with_pure_dir).exists() {
        fs::remove_dir_all(&generated_with_pure_dir).unwrap();
    }
    if Path::new(&generated_with_native_dir).exists() {
        fs::remove_dir_all(&generated_with_native_dir).unwrap();
    }
    fs::create_dir(&generated_with_pure_dir).unwrap();
    fs::create_dir(&generated_with_native_dir).unwrap();

    protoc_rust::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir(generated_with_native_dir)
        .input("src/protos/example.proto")
        .include("src/protos")
        .run_from_script();

    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            gen_mod_rs: Some(true),
            ..Default::default()
        })
        .out_dir(generated_with_pure_dir)
        .input("src/protos/example.proto")
        .include("src/protos")
        .run_from_script();
}
