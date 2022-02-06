fn main() {
    protobuf_codegen::Codegen::new()
        .cargo_out_dir("protos")
        .include("src")
        .inputs(&["src/customize_example.proto"])
        .run()
        .expect("protoc");
}
