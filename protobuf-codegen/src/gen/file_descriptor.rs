use std::fmt::Write as _;

use protobuf::reflect::FileDescriptor;
use protobuf::Message;

use crate::gen::code_writer::CodeWriter;
use crate::gen::inside::protobuf_crate_path;
use crate::gen::paths::proto_path_to_fn_file_descriptor;
use crate::gen::rust::EXPR_VEC_NEW;
use crate::gen::scope::FileScope;
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

fn write_file_descriptor(
    file_descriptor: &FileDescriptor,
    customize: &Customize,
    w: &mut CodeWriter,
) {
    w.write_line("/// `FileDescriptor` object which allows dynamic access to files");
    w.pub_fn(
        &format!(
            "file_descriptor() -> {}::reflect::FileDescriptor",
            protobuf_crate_path(customize)
        ),
        |w| {
            w.lazy_static(
                "file_descriptor_lazy",
                &format!(
                    "{}::reflect::GeneratedFileDescriptor",
                    protobuf_crate_path(customize)
                ),
                &format!("{}", protobuf_crate_path(customize)),
            );
            w.block(
                "let file_descriptor = file_descriptor_lazy.get(|| {",
                "});",
                |w| {
                    w.write_line(&format!("let mut deps = {};", EXPR_VEC_NEW));
                    for f in &file_descriptor.proto().dependency {
                        w.write_line(&format!(
                            "deps.push({}());",
                            proto_path_to_fn_file_descriptor(f, customize)
                        ));
                    }

                    let scope = FileScope { file_descriptor };

                    w.write_line(&format!("let mut messages = {};", EXPR_VEC_NEW));
                    for m in scope.find_messages_except_map() {
                        if m.is_map() {
                            continue;
                        }
                        w.write_line(&format!(
                            "messages.push({}::generated_message_descriptor_data());",
                            m.rust_name_to_file(),
                        ));
                    }

                    w.write_line(&format!("let mut enums = {};", EXPR_VEC_NEW));
                    for e in scope.find_enums() {
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
                },
            );
            w.write_line(&format!(
                "{}::reflect::FileDescriptor::new_generated_2(file_descriptor)",
                protobuf_crate_path(&customize),
            ));
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
    w.write_line("/// `FileDescriptorProto` object which was a source for this generated file");
    w.pub_fn(
        &format!(
            "file_descriptor_proto() -> &'static {}::descriptor::FileDescriptorProto",
            protobuf_crate_path(customize)
        ),
        |w| {
            w.lazy_static(
                "file_descriptor_proto_lazy",
                &format!(
                    "{}::descriptor::FileDescriptorProto",
                    protobuf_crate_path(customize)
                ),
                &format!("{}", protobuf_crate_path(customize)),
            );
            w.block("file_descriptor_proto_lazy.get(|| {", "})", |w| {
                w.write_line(&format!(
                    "{}::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()",
                    protobuf_crate_path(customize)
                ));
            });
        },
    );
    w.write_line("");
    write_file_descriptor(file, &customize, w);
}
