use std::collections::hash_map::HashMap;

use descriptor::*;
use core::Message;
use compiler_plugin;
use code_writer::CodeWriter;
use paginate::PaginatableIterator;
use descriptorx::*;

mod message;
mod enums;

use self::message::*;
use self::enums::*;


fn write_file_descriptor_data(file: &FileDescriptorProto, w: &mut CodeWriter) {
    let fdp_bytes = file.write_to_bytes().unwrap();
    w.write_line("static file_descriptor_proto_data: &'static [u8] = &[");
    for groups in fdp_bytes.iter().paginate(16) {
        let fdp_bytes_str = groups.iter()
                .map(|&b| format!("0x{:02x}", *b))
                .collect::<Vec<String>>()
                .join(", ");
        w.write_line(&format!("    {},", fdp_bytes_str));
    }
    w.write_line("];");
    w.write_line("");
    w.lazy_static("file_descriptor_proto_lazy", "::protobuf::descriptor::FileDescriptorProto");
    w.write_line("");
    w.def_fn("parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto", |w| {
        w.write_line("::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()");
    });
    w.write_line("");
    w.pub_fn("file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto", |w| {
        w.unsafe_expr(|w| {
            w.block("file_descriptor_proto_lazy.get(|| {", "})", |w| {
                w.write_line("parse_descriptor_proto()");
            });
        });
    });
}

fn gen_file(
    file: &FileDescriptorProto,
    _files_map: &HashMap<&str, &FileDescriptorProto>,
    root_scope: &RootScope,
)
    -> Option<compiler_plugin::GenResult>
{
    let scope = FileScope { file_descriptor: file } .to_scope();

    if scope.get_messages().is_empty() && scope.get_enums().is_empty() {
        // protoc generates empty file descriptors for directories: skip them
        return None;
    }

    let mut v = Vec::new();

    {
        let mut w = CodeWriter::new(&mut v);

        w.write_generated();

        w.write_line("");
        w.write_line("use protobuf::Message as Message_imported_for_functions;");
        w.write_line("use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;");

        for message in &scope.get_messages() {
            // ignore map entries, because they are not used in map fields
            if message.map_entry().is_none() {
                w.write_line("");
                MessageGen::new(message, &root_scope).write(&mut w);
            }
        }
        for enum_type in &scope.get_enums() {
            w.write_line("");
            EnumGen::new(enum_type, file).write(&mut w);
        }

        if file.get_options().get_optimize_for() != FileOptions_OptimizeMode::LITE_RUNTIME {
            w.write_line("");
            write_file_descriptor_data(file, &mut w);
        }
    }

    Some(compiler_plugin::GenResult {
        name: format!("{}.rs", proto_path_to_rust_mod(file.get_name())),
        content: v,
    })
}

// This function is also used externally by cargo plugin
// https://github.com/plietar/rust-protobuf-build
// So be careful changing its signature.
pub fn gen(file_descriptors: &[FileDescriptorProto], files_to_generate: &[String])
        -> Vec<compiler_plugin::GenResult>
{
    let root_scope = RootScope { file_descriptors: file_descriptors };

    let mut results: Vec<compiler_plugin::GenResult> = Vec::new();
    let files_map: HashMap<&str, &FileDescriptorProto> =
        file_descriptors.iter().map(|f| (f.get_name(), f)).collect();

    for file_name in files_to_generate {
        let file = files_map[&file_name[..]];
        results.extend(gen_file(file, &files_map, &root_scope));
    }
    results
}

pub fn protoc_gen_rust_main() {
    compiler_plugin::plugin_main(gen);
}
