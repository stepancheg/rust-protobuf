#[crate_type = "bin"];
#[feature(globs)];
#[feature(managed_boxes)];

extern mod protobuf;
extern mod extra;

use std::rt::io;
use std::rt::io::file::*;
use std::rt::io::Reader;
use std::rt::io::Writer;
use std::path::Path;
use std::os;
use extra::getopts;

use protobuf::*;
use protobuf::descriptor::*;
use protobuf::codegen::*;


fn write_file(bin: &str, gen_options: &GenOptions) {
    let is: @Reader = @Path::new(bin).open_reader(io::Open).unwrap() as @Reader;
    let fds = parse_from_reader::<FileDescriptorSet>(is);

    let results = gen(fds.file, gen_options);

    for r in results.iter() {
        let file_writer = @mut Path::new(r.name.to_owned()).open_writer(io::CreateOrTruncate).unwrap()
                as @mut Writer;
        file_writer.write(r.content);
    }
}

fn main() {
    let args = os::args();
    let opts: ~[getopts::Opt] = ~[
    ];
    let matches = getopts::getopts(args.tail(), opts).unwrap();
    let pb_bin = match matches.free {
        [ref pb_bin] => pb_bin.to_owned(),
        _ => fail!("must have exactly one argument")
    };
    let gen_options = GenOptions {
        dummy: false,
    };
    write_file(pb_bin, &gen_options);
}
