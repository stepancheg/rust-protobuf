use std::io::stdin;
use std::io::stdout;
use std::str;

use protobuf::descriptor::FileDescriptorProto;
use protobuf::plugin::*;
use protobuf::Message;
use protobuf_parse::ProtoPathBuf;

pub struct GenRequest<'a> {
    pub file_descriptors: &'a [FileDescriptorProto],
    pub files_to_generate: &'a [ProtoPathBuf],
    pub parameter: &'a str,
}

pub struct GenResult {
    pub name: String,
    pub content: Vec<u8>,
}

pub fn plugin_main<F>(gen: F) -> anyhow::Result<()>
where
    F: Fn(&GenRequest) -> anyhow::Result<Vec<GenResult>>,
{
    let req = CodeGeneratorRequest::parse_from_reader(&mut stdin()).unwrap();
    let result = gen(&GenRequest {
        file_descriptors: &req.proto_file,
        files_to_generate: &req
            .file_to_generate
            .iter()
            .map(|n| ProtoPathBuf::new(n.to_owned()))
            .collect::<anyhow::Result<Vec<_>>>()?,
        parameter: req.parameter(),
    })?;
    let mut resp = CodeGeneratorResponse::new();
    resp.set_supported_features(code_generator_response::Feature::FEATURE_PROTO3_OPTIONAL as u64);
    resp.file = result
        .iter()
        .map(|file| {
            let mut r = code_generator_response::File::new();
            r.set_name(file.name.to_string());
            r.set_content(str::from_utf8(file.content.as_ref()).unwrap().to_string());
            r
        })
        .collect();
    resp.write_to_writer(&mut stdout()).unwrap();
    Ok(())
}
