use protobuf_codegen::Codegen;
use protobuf_codegen::Customize;

fn main() {
    protobuf_test_common::build::clean_old_files();

    Codegen::new()
        .pure()
        .out_dir("src")
        .include("src")
        .input("src/all_types_pb.proto")
        .customize(Customize::default().gen_mod_rs(false))
        .run_from_script();
}
