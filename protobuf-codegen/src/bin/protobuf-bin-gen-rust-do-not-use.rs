extern crate protobuf;
extern crate protobuf_codegen;

use std::fs::*;
use std::io::Read;
use std::path::Path;

use protobuf::descriptor::*;
use protobuf::Message;
use protobuf_codegen::*;
use protobuf_parse::ProtoPathBuf;

fn write_file(bin: &str) -> anyhow::Result<()> {
    let mut is = File::open(&Path::new(bin)).unwrap();
    let fds = FileDescriptorSet::parse_from_reader(&mut is as &mut dyn Read).unwrap();

    let file_names: Vec<ProtoPathBuf> = fds
        .file
        .iter()
        .map(|f| ProtoPathBuf::new(f.get_name().to_owned()))
        .collect::<anyhow::Result<_>>()?;
    gen_and_write(
        &fds.file,
        &format!("unknown, file {}", bin),
        &file_names,
        Path::new("."),
        &Default::default(),
    )?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("must have exactly one argument");
    }
    let ref pb_bin = args[1];
    write_file(&pb_bin).unwrap();
}
