#[crate_type = "bin"];
#[feature(globs)];
#[feature(managed_boxes)];

extern mod protobuf;

use std::rt::io;
use std::rt::io::Reader;
use std::rt::io::Writer;
use std::str;
use plugin::*;
use protobuf::*;
use protobuf::codegen::*;

mod descriptor {
    pub use protobuf::descriptor::*;
}

mod plugin;

fn main() {
    // io::stdin() hangs on EOF on Linux
    // https://github.com/mozilla/rust/issues/10237
    let req = parse_from_reader::<CodeGeneratorRequest>(@io::native::stdio::stdin() as @Reader);
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
    resp.write_to_writer(@io::stdout() as @Writer);
}
