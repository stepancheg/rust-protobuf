extern crate glob;
extern crate log;
extern crate env_logger;

extern crate protobuf_codegen_pure;

extern crate protobuf_test_common;

use protobuf_test_common::build::*;


fn generate_pb_rs() {

    fn gen_v2_v3(dir: &str) {
        gen_in_dir(dir, |GenInDirArgs { out_dir, input, includes, customize }| {
            protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
                out_dir, input, includes, customize
            })
        });
    }

    gen_v2_v3("src/v2");

//    gen_v2_v3("src/v3");
//
//    let protos = glob_simple("src/google/protobuf/*.proto");
//    protoc_rust::run(protoc_rust::Args {
//        out_dir: "src/google/protobuf",
//        input: &protos.iter().map(|a| a.as_ref()).collect::<Vec<&str>>(),
//        includes: &["../proto", "src"],
//    }).expect("protoc");
}


fn main() {
    env_logger::init();

    clean_old_files();
    generate_pb_rs();
}
