use protobuf_codegen::Codegen;

fn main() {
    protobuf_test_common::build::clean_old_files();

    Codegen::new()
        .pure()
        .out_dir("src")
        .include("src")
        .input("src/all_types_pb.proto")
        .run_from_script();
}
