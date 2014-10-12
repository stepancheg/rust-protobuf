#![crate_type = "bin"]
#![feature(globs)]

extern crate protobuf;
extern crate getopts;

use std::io::fs::*;
use std::io::Reader;
use std::io::Writer;
use std::path::Path;
use std::os;

use protobuf::parse_from_reader;
use protobuf::descriptor::*;
use protobuf::codegen::*;


fn write_file(bin: &str, gen_options: &GenOptions) {
    let mut is = File::open(&Path::new(bin)).unwrap();
    let fds = parse_from_reader::<FileDescriptorSet>(&mut is as &mut Reader).unwrap();

    let results = gen(fds.get_file(), gen_options);

    for r in results.iter() {
        let mut file_writer = File::create(&Path::new(r.name.as_slice())).unwrap();
        file_writer.write(r.content.as_slice()).unwrap();
    }
}

fn main() {
    let args = os::args();
    let opts = vec!();
    let matches = getopts::getopts(args.tail(), opts.as_slice()).unwrap();
    let pb_bin = match matches.free.as_slice() {
        [ref pb_bin] => pb_bin.to_string(),
        _ => fail!("must have exactly one argument")
    };
    let gen_options = GenOptions {
        dummy: false,
    };
    write_file(pb_bin.as_slice(), &gen_options);
}
