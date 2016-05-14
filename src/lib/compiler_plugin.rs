use std::io::stdin;
use std::io::stdout;
use std::str;
use plugin::*;
use protobuf::parse_from_reader;
use protobuf::Message;
use protobuf::descriptor::FileDescriptorProto;


pub struct GenResult {
    pub name: String,
    pub content: Vec<u8>,
}

pub fn plugin_main(
    gen: fn(file_descriptors: &[FileDescriptorProto], files_to_generate: &[String]) -> Vec<GenResult>)
{
    let req = parse_from_reader::<CodeGeneratorRequest>(&mut stdin()).unwrap();
    let result = gen(req.get_proto_file(), req.get_file_to_generate());
    let mut resp = CodeGeneratorResponse::new();
    resp.set_file(result.iter().map(|file| {
        let mut r = CodeGeneratorResponse_File::new();
        r.set_name(file.name.to_string());
        r.set_content(str::from_utf8(file.content.as_ref()).unwrap().to_string());
        r
    }).collect());
    resp.write_to(&mut stdout()).unwrap();
}
