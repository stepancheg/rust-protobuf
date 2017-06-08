extern crate protoc;

fn main() {
    protoc::run(protoc::Args {
        lang: "rust".to_owned(),
        out_dir: ".".to_owned(),
        plugin: Some("../target/debug/protoc-gen-rust".to_owned()),
        input: vec!["perftest_data.proto".to_owned()],
        ..Default::default()
    }).expect("protoc");

    
}
