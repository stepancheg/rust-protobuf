extern crate protobuf_parser;
extern crate protobuf;
extern crate protobuf_codegen;

mod convert;

use std::path::Path;
use std::io;
use std::io::Read;
use std::fs;


// TODO: merge with protoc-rust def
#[derive(Debug, Default)]
pub struct Args<'a> {
    /// --lang_out= param
    pub out_dir: &'a str,
    /// -I args
    pub includes: &'a [&'a str],
    /// List of .proto files to compile
    pub input: &'a [&'a str],
}

fn strip_prefixes<'a>(file_name: &'a str, prefixes: &'a [&'a str]) -> io::Result<&'a str> {
    for prefix in prefixes {
        match Path::new(file_name).strip_prefix(prefix) {
            Ok(stripped) => return Ok(stripped.to_str().expect("back to str")),
            Err(..) => {}
        }
    }
    Err(io::Error::new(io::ErrorKind::Other,
        format!("file name {:?} is not a in include path {:?}", file_name, prefixes)))
}

/// Like `protoc --rust_out=...` but without requiring `protoc` or `protoc-gen-rust`
/// commands in `$PATH`.
pub fn run(args: Args) -> io::Result<()> {
    let mut file_descriptors = Vec::new();
    let mut relative_paths = Vec::new();

    for input in args.input {
        let relative = strip_prefixes(input, args.includes)?;
        relative_paths.push(relative.to_owned());
        let mut content = Vec::new();
        fs::File::open(input)?.read_to_end(&mut content)?;
        let file_descriptor = protobuf_parser::FileDescriptor::parse(&content)
            .map_err(|_e| io::Error::new(io::ErrorKind::Other, "failed to parse"))?;

        let import_paths = file_descriptor.import_paths.clone();

        let proto = convert::file_descriptor(relative.to_owned(), file_descriptor);
        file_descriptors.push(proto);

        for import_path in import_paths {
            // TODO: handle imports
        }
    }

    // TODO: resolve includes

    protobuf_codegen::gen_and_write(&file_descriptors, &relative_paths, &Path::new(&args.out_dir))
}
