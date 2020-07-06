use protobuf_codegen_pure::Customize;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            ..Default::default()
        })
        .out_dir("src/protos")
        .input("src/protos/example.proto")
        .include("src/protos")
        .run()
        .expect("protoc");
}
