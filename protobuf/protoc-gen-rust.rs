extern crate protobuf;

use protobuf::codegen;


fn main() {
    codegen::protoc_gen_rust_main();
}
