use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .pure()
        .out_dir(".")
        .include(".")
        .input("perftest_data.proto")
        .run_from_script();
}
