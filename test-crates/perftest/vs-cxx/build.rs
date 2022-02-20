use protobuf_codegen::Codegen;
use protobuf_codegen::Customize;

fn main() {
    Codegen::new()
        .pure()
        .out_dir(".")
        .include(".")
        .input("perftest_data.proto")
        .customize(Customize::default().gen_mod_rs(false))
        .run_from_script();
}
