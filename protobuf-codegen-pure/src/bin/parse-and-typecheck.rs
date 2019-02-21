extern crate protobuf_codegen_pure;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    assert!(args.len() >= 2);
    let input = vec![&args[0][..]];
    let includes: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
    let t =
        protobuf_codegen_pure::parse_and_typecheck(&includes, &input).expect("parse_and_typecheck");
    for fd in t.file_descriptors {
        println!("{:#?}", fd);
    }
}
