extern crate protoc;

fn main() {
    protoc::run(protoc::Args {
        lang: "rust",
        out_dir: "src",
        plugin: Some("../../target/debug/protoc-gen-rust"),
        input: &["src/data.proto"],
        ..Default::default()
    }).expect("protoc");
}
