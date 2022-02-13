use std::env;
use std::fs;

use protobuf::text_format;
use protobuf_parse::Parser;

enum Which {
    Protoc,
    Pure,
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let (path, out_protoc, out_pure) = match args.as_slice() {
        // Just invoke protoc.
        [path, out_protoc, out_pure] => (path, out_protoc, out_pure),
        _ => panic!("wrong args"),
    };

    for which in [Which::Pure, Which::Protoc] {
        let mut parser = Parser::new();
        match which {
            Which::Protoc => {
                parser.protoc();
            }
            Which::Pure => {
                parser.pure();
            }
        }

        parser.input(path);
        parser.include(".");
        let fds = parser.file_descriptor_set().unwrap();
        let fds = text_format::print_to_string_pretty(&fds);
        let out = match which {
            Which::Protoc => out_protoc,
            Which::Pure => out_pure,
        };
        fs::write(out, fds).unwrap();
    }
}
