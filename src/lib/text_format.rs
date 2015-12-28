use std::fmt;
use core::Message;
use descriptor::*;

fn print_bytes_to(bytes: &[u8], buf: &mut String) {
    buf.push('"');
    for &b in bytes.iter() {
        if b < 0x20 || b >= 0x7f {
            buf.push('\\');
            buf.push((b'0' + ((b >> 6) & 3)) as char);
            buf.push((b'0' + ((b >> 3) & 7)) as char);
            buf.push((b'0' + (b & 7)) as char);
        } else if b == b'"' {
            buf.push_str("\\\"");
        } else if b == b'\\' {
            buf.push_str("\\\\");
        } else {
            buf.push(b as char);
        }
    }
    buf.push('"');
}

fn print_str_to(s: &str, buf: &mut String) {
    // TODO: keep printable Unicode
    print_bytes_to(s.as_bytes(), buf);
}

fn do_indent(buf: &mut String, pretty: bool, indent: usize) {
    if pretty && indent > 0 {
        for _ in 0..indent {
            buf.push_str("  ");
        }
    }
}

fn print_to_internal(m: &Message, buf: &mut String, pretty: bool, indent: usize) {
    let d = m.descriptor();
    let mut first = true;
    for f in d.fields().iter() {
        if f.is_repeated() {
            for i in 0..f.len_field(m) {
                if !first && !pretty {
                    buf.push_str(" ");
                }
                do_indent(buf, pretty, indent);
                first = false;
                buf.push_str(f.name());
                match f.proto().get_field_type() {
                    FieldDescriptorProto_Type::TYPE_MESSAGE => {
                        buf.push_str(" {");
                        if pretty {
                            buf.push_str("\n");
                        }
                        print_to_internal(f.get_rep_message_item(m, i), buf, pretty, indent+1);
                        do_indent(buf, pretty, indent);
                        buf.push_str("}");
                    },
                    FieldDescriptorProto_Type::TYPE_ENUM => {
                        buf.push_str(": ");
                        buf.push_str(f.get_rep_enum_item(m, i).name());
                    },
                    FieldDescriptorProto_Type::TYPE_STRING => {
                        buf.push_str(": ");
                        print_str_to(f.get_rep_str_item(m, i), buf);
                    },
                    FieldDescriptorProto_Type::TYPE_BYTES => {
                        buf.push_str(": ");
                        print_bytes_to(f.get_rep_bytes_item(m, i), buf);
                    },
                    FieldDescriptorProto_Type::TYPE_INT32 |
                    FieldDescriptorProto_Type::TYPE_SINT32 |
                    FieldDescriptorProto_Type::TYPE_SFIXED32 => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_rep_i32(m)[i].to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_INT64 |
                    FieldDescriptorProto_Type::TYPE_SINT64 |
                    FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_rep_i64(m)[i].to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_UINT32 |
                    FieldDescriptorProto_Type::TYPE_FIXED32 => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_rep_u32(m)[i].to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_UINT64 |
                    FieldDescriptorProto_Type::TYPE_FIXED64 => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_rep_u64(m)[i].to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_BOOL => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_rep_bool(m)[i].to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_FLOAT => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_rep_f32(m)[i].to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_DOUBLE => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_rep_f64(m)[i].to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_GROUP => {
                        buf.push_str(": <TYPE_GROUP>");
                    }
                }
                if pretty {
                    buf.push_str("\n");
                }
            }
        } else {
            if f.has_field(m) {
                if !first && !pretty {
                    buf.push_str(" ");
                }
                do_indent(buf, pretty, indent);
                first = false;
                buf.push_str(f.name());
                match f.proto().get_field_type() {
                    FieldDescriptorProto_Type::TYPE_MESSAGE => {
                        buf.push_str(" {");
                        if pretty {
                            buf.push_str("\n");
                        }
                        print_to_internal(f.get_message(m), buf, pretty, indent+1);
                        do_indent(buf, pretty, indent);
                        buf.push_str("}");
                    },
                    FieldDescriptorProto_Type::TYPE_ENUM => {
                        buf.push_str(": ");
                        buf.push_str(f.get_enum(m).name());
                    },
                    FieldDescriptorProto_Type::TYPE_STRING => {
                        buf.push_str(": ");
                        print_str_to(f.get_str(m), buf);
                    },
                    FieldDescriptorProto_Type::TYPE_BYTES => {
                        buf.push_str(": ");
                        print_bytes_to(f.get_bytes(m), buf);
                    },
                    FieldDescriptorProto_Type::TYPE_INT32 |
                    FieldDescriptorProto_Type::TYPE_SINT32 |
                    FieldDescriptorProto_Type::TYPE_SFIXED32 => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_i32(m).to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_INT64 |
                    FieldDescriptorProto_Type::TYPE_SINT64 |
                    FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_i64(m).to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_UINT32 |
                    FieldDescriptorProto_Type::TYPE_FIXED32 => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_u32(m).to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_UINT64 |
                    FieldDescriptorProto_Type::TYPE_FIXED64 => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_u64(m).to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_BOOL => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_bool(m).to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_FLOAT => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_f32(m).to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_DOUBLE => {
                        buf.push_str(": ");
                        buf.push_str(&f.get_f64(m).to_string());
                    },
                    FieldDescriptorProto_Type::TYPE_GROUP => {
                        buf.push_str(": <TYPE_GROUP>");
                    }
                }
                if pretty {
                    buf.push_str("\n");
                }
            }
        }
    }

    // TODO: unknown fields
}

pub fn print_to(m: &Message, buf: &mut String) {
    print_to_internal(m, buf, false, 0)
}

fn print_to_string_internal(m: &Message, pretty: bool) -> String {
    let mut r = String::new();
    print_to_internal(m, &mut r, pretty, 0);
    r.to_string()
}

pub fn print_to_string(m: &Message) -> String {
    print_to_string_internal(m, false)
}

pub fn fmt(m: &Message, f: &mut fmt::Formatter) -> fmt::Result {
    let pretty = f.flags() & 4 != 0; // private: fmt::FlagV1::Alternate
    f.write_str(&print_to_string_internal(m, pretty))
}
