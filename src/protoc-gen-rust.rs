#[crate_type = "bin"];
#[feature(globs)];
#[feature(managed_boxes)];

extern mod protobuf;

use std::io;
use std::io::Reader;
use std::io::Writer;
use std::str;
use plugin::*;
use protobuf::*;
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
    let result = gen(req.proto_file, &gen_options);
    let mut resp = CodeGeneratorResponse::new();
    resp.file = result.map(|file| {
        let mut r = CodeGeneratorResponse_File::new();
        r.name = Some(file.name.to_owned());
        r.content = Some(str::from_utf8(file.content).unwrap().to_owned());
        r
    });
    resp.write_to_writer(&mut io::stdout() as &mut Writer);
}
