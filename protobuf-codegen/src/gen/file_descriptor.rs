use std::fmt::Write as _;

use protobuf::reflect::FileDescriptor;
use protobuf::Message;

use crate::gen::code_writer::CodeWriter;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::paths::proto_path_to_fn_file_descriptor;
use crate::gen::rust::snippets::expr_vec_with_capacity_const;
use crate::gen::scope::FileScope;
use crate::gen::scope::Scope;
use crate::gen::scope::WithScope;
use crate::Customize;

fn escape_byte(s: &mut String, b: u8) {
    if b == b'\n' {
        write!(s, "\\n").unwrap();
    } else if b == b'\r' {
        write!(s, "\\r").unwrap();
    } else if b == b'\t' {
        write!(s, "\\t").unwrap();
    } else if b == b'\\' || b == b'"' {
        write!(s, "\\{}", b as char).unwrap();
    } else if b == b'\0' {
        write!(s, "\\0").unwrap();
    // ASCII printable except space
    } else if b > 0x20 && b < 0x7f {
        write!(s, "{}", b as char).unwrap();
    } else {
        write!(s, "\\x{:02x}", b).unwrap();
    }
}

fn write_generate_file_descriptor(
    file_descriptor: &FileDescriptor,
    customize: &Customize,
    w: &mut CodeWriter,
) {
    let deps = &file_descriptor.proto().dependency;
    w.write_line(&format!(
        "let mut deps = {vec_with_capacity};",
        vec_with_capacity = expr_vec_with_capacity_const(deps.len())
    ));
    for f in deps {
        w.write_line(&format!(
            "deps.push({}().clone());",
            proto_path_to_fn_file_descriptor(f, customize)
        ));
    }

    let scope = FileScope { file_descriptor };

    let messages = scope.find_messages_except_map();
    w.write_line(&format!(
        "let mut messages = {vec_with_capacity};",
        vec_with_capacity = expr_vec_with_capacity_const(messages.len())
    ));
    for m in &messages {
        w.write_line(&format!(
            "messages.push({}::generated_message_descriptor_data());",
            m.rust_name_to_file(),
        ));
    }

    let enums = scope.find_enums();
    w.write_line(&format!(
        "let mut enums = {};",
        expr_vec_with_capacity_const(enums.len())
    ));
    for e in &enums {
        w.write_line(&format!(
            "enums.push({}::generated_enum_descriptor_data());",
            e.rust_name_to_file(),
        ));
    }

    w.write_line(&format!(
        "{}::reflect::GeneratedFileDescriptor::new_generated(",
        protobuf_crate_path(&customize),
    ));
    w.indented(|w| {
        w.write_line(&format!("file_descriptor_proto(),"));
        w.write_line(&format!("deps,"));
        w.write_line(&format!("messages,"));
        w.write_line(&format!("enums,"));
    });
    w.write_line(")");
}

fn write_file_descriptor(
    file_descriptor: &FileDescriptor,
    customize: &Customize,
    w: &mut CodeWriter,
) {
    w.write_line("/// `FileDescriptor` object which allows dynamic access to files");
    w.pub_fn(
        &format!(
            "file_descriptor() -> &'static {protobuf_crate}::reflect::FileDescriptor",
            protobuf_crate = protobuf_crate_path(customize)
        ),
        |w| {
            w.lazy_static(
                "generated_file_descriptor_lazy",
                &format!(
                    "{protobuf_crate}::reflect::GeneratedFileDescriptor",
                    protobuf_crate = protobuf_crate_path(customize)
                ),
                &format!("{}", protobuf_crate_path(customize)),
            );
            w.lazy_static_decl_get(
                "file_descriptor",
                &format!(
                    "{protobuf_crate}::reflect::FileDescriptor",
                    protobuf_crate = protobuf_crate_path(customize)
                ),
                &format!("{}", protobuf_crate_path(customize)),
                |w| {
                    w.block(
                        "let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {",
                        "});",
                        |w| write_generate_file_descriptor(file_descriptor, customize, w),
                    );
                    w.write_line(&format!(
                        "{protobuf_crate}::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)",
                        protobuf_crate=protobuf_crate_path(&customize),
                    ));
                }
            );
        },
    );
}

pub(crate) fn write_file_descriptor_data(
    file: &FileDescriptor,
    customize: &Customize,
    w: &mut CodeWriter,
) {
    let fdp_bytes = file.proto().write_to_bytes().unwrap();
    w.write_line("static file_descriptor_proto_data: &'static [u8] = b\"\\");
    w.indented(|w| {
        const MAX_LINE_LEN: usize = 72;

        let mut s = String::new();
        for &b in &fdp_bytes {
            let prev_len = s.len();
            escape_byte(&mut s, b);
            let truncate = s.len() > MAX_LINE_LEN;
            if truncate {
                s.truncate(prev_len);
            }
            if truncate || s.len() == MAX_LINE_LEN {
                write!(s, "\\").unwrap();
                w.write_line(&s);
                s.clear();
            }
            if truncate {
                escape_byte(&mut s, b);
            }
        }
        if !s.is_empty() {
            write!(s, "\\").unwrap();
            w.write_line(&s);
            s.clear();
        }
    });
    w.write_line("\";");
    w.write_line("");
    write_file_descriptor_proto(&customize, w);
    w.write_line("");
    write_file_descriptor(file, &customize, w);
}

fn write_file_descriptor_proto(customize: &Customize, w: &mut CodeWriter) {
    w.write_line("/// `FileDescriptorProto` object which was a source for this generated file");
    w.def_fn(
        &format!(
            "file_descriptor_proto() -> &'static {protobuf_crate}::descriptor::FileDescriptorProto",
            protobuf_crate=protobuf_crate_path(customize)
        ),
        |w| {
            w.lazy_static_decl_get(
                "file_descriptor_proto_lazy",
                &format!(
                    "{protobuf_crate}::descriptor::FileDescriptorProto",
                    protobuf_crate=protobuf_crate_path(customize)
                ),
                &format!("{}", protobuf_crate_path(customize)),
                |w| {
                    w.write_line(&format!(
                        "{protobuf_crate}::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()",
                        protobuf_crate=protobuf_crate_path(customize)
                    ));
                },
            );
        },
    );
}

/// Code to generate call `module::file_descriptor()`.
pub(crate) fn file_descriptor_call_expr(scope: &Scope) -> String {
    format!(
        "{}()",
        scope
            .rust_path_to_file()
            .to_reverse()
            .append_ident("file_descriptor".into())
    )
}
