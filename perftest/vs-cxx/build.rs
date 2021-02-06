fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir(".")
        .include(".")
        .input("perftest_data.proto")
        .run()
        .expect("protoc");
}
