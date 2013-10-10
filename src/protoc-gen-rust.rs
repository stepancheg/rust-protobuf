#[crate_type = "bin"];
#[feature(globs)];

extern mod protobuf;

use std::io;
use std::str;
use plugin::*;
use protobuf::*;
use protobuf::codegen::*;

mod descriptor {
    pub use protobuf::descriptor::*;
}

mod plugin;

fn main() {
    let req = parse_from_reader::<CodeGeneratorRequest>(io::stdin());
    let gen_options = GenOptions {
        dummy: false,
    };
    let result = gen(req.proto_file, &gen_options);
    let mut resp = CodeGeneratorResponse::new();
    resp.file = do result.map |file| {
        let mut r = CodeGeneratorResponse_File::new();
        r.name = Some(file.name.to_owned());
        r.content = Some(str::from_utf8(file.content));
        r
    };
    resp.write_to_writer(io::stdout());
}
