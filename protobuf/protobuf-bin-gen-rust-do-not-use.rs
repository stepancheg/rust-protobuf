extern crate protobuf;

use std::fs::*;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use protobuf::parse_from_reader;
use protobuf::descriptor::*;
use protobuf::codegen::*;


fn write_file(bin: &str) {
    let mut is = File::open(&Path::new(bin)).unwrap();
    let fds = parse_from_reader::<FileDescriptorSet>(&mut is as &mut Read).unwrap();

    let file_names: Vec<String> = fds.get_file()
        .iter()
        .map(|f| f.get_name().to_string())
        .collect();
    let results = gen(fds.get_file(), &file_names);

    for r in &results {
        let mut file_writer = File::create(&Path::new(&r.name)).unwrap();
        file_writer.write(&r.content).unwrap();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("must have exactly one argument");
    }
    let ref pb_bin = args[1];
    write_file(&pb_bin);
}
