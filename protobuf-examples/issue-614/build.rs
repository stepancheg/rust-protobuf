use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .input("foos.proto")
        .include(".")
        .cargo_out_dir("p")
        .run_from_script();
}
