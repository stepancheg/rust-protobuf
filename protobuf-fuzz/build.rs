fn main() {
    protobuf_test_common::build::clean_old_files();

    protobuf_codegen_pure::Codegen::new()
        .out_dir("src")
        .include("src")
        .input("src/all_types_pb.proto")
        .run()
        .expect("protoc_rust");
}
