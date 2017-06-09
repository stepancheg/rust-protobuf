extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: ".",
        input: &["perftest_data.proto"],
        ..Default::default()
    }).expect("protoc");
}
