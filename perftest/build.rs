extern crate protoc;

fn main() {
    protoc::run(protoc::Args {
        lang: "rust",
        out_dir: ".",
        plugin: Some("../target/debug/protoc-gen-rust"),
        input: &["perftest_data.proto"],
        ..Default::default()
    }).expect("protoc");

    
}
