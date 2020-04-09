extern crate protobuf_test_common;
extern crate protoc_rust;

fn main() {
    protobuf_test_common::build::clean_old_files();

    protoc_rust::Codegen::new()
        .out_dir("src")
        .include("src")
        .input("src/all_types_pb.proto")
        .run()
        .expect("protoc_rust");
}
