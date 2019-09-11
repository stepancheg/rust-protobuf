extern crate protobuf_codegen_pure;

use std::env;
use std::path::PathBuf;

fn main() {
    let args = env::args_os()
        .skip(1)
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    assert!(args.len() >= 2);
    let (input, includes) = args.split_at(1);
    let t =
        protobuf_codegen_pure::parse_and_typecheck(includes, input).expect("parse_and_typecheck");
    for fd in t.file_descriptors {
        println!("{:#?}", fd);
    }
}
