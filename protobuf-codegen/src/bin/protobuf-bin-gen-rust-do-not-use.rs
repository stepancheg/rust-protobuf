extern crate protobuf;
extern crate protobuf_codegen;

use std::fs::*;
use std::io::Read;
use std::path::{Path, PathBuf};

use protobuf::descriptor::*;
use protobuf::parse_from_reader;
use protobuf_codegen::*;

fn write_file(bin: &str) {
    let mut is = File::open(&Path::new(bin)).unwrap();
    let fds = parse_from_reader::<FileDescriptorSet>(&mut is as &mut dyn Read).unwrap();

    let file_names: Vec<PathBuf> = fds.file.iter().map(|f| f.get_name().into()).collect();
    gen_and_write(
        &fds.file,
        &format!("unknown, file {}", bin),
        &file_names,
        Path::new("."),
        &Default::default(),
    )
    .expect("gen_and_write");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("must have exactly one argument");
    }
    let ref pb_bin = args[1];
    write_file(&pb_bin);
}
