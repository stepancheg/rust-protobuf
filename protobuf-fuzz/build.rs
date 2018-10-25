extern crate protobuf_test_common;
extern crate protoc_rust;

fn main() {
    protobuf_test_common::build::clean_old_files();

    protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        includes: &["src"],
        input: &["src/all_types_pb.proto"],
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    }).expect("protoc_rust");
}
