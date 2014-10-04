#![crate_type = "bin"]
#![feature(globs)]
#![allow(non_camel_case_types)]

extern crate protobuf;
extern crate sync;

use std::io;
use std::io::Reader;
use std::io::Writer;
use std::str;
use plugin::*;
use protobuf::parse_from_reader;
use protobuf::Message;
use protobuf::codegen::*;

mod descriptor {
    pub use protobuf::descriptor::*;
}

mod plugin;

fn main() {
    let req = parse_from_reader::<CodeGeneratorRequest>(&mut io::stdio::stdin() as &mut Reader);
    let gen_options = GenOptions {
        dummy: false,
    };
    let result = gen(req.get_proto_file(), &gen_options);
    let mut resp = CodeGeneratorResponse::new();
    resp.set_file(result.iter().map(|file| {
        let mut r = CodeGeneratorResponse_File::new();
        r.set_name(file.name.to_string());
        r.set_content(str::from_utf8(file.content.as_slice()).unwrap().to_string());
        r
    }).collect());
    resp.write_to_writer(&mut io::stdout() as &mut Writer);
}
