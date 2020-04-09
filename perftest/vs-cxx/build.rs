extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir(".")
        .input("perftest_data.proto")
        .run()
        .expect("protoc");
}
