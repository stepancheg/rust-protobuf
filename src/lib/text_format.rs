use std::fmt;
use core::Message;
use reflect::FieldDescriptor;
use reflect::EnumValueDescriptor;
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

// internal helper
impl FieldDescriptor {
    // len fo repeated field, or 1 or 0 for singular field
    fn len_field_x(&self, m: &Message) -> usize {
        if self.is_repeated() {
            self.len_field(m)
        } else {
            match self.has_field(m) {
                true => 1,
                false => 0,
            }
        }
    }

    // get item by index for repeated fields, or just field for singular fields

    fn get_rep_message_item_x<'a>(&self, m: &'a Message, index: usize) -> &'a Message {
        if self.is_repeated() {
            self.get_rep_message_item(m, index)
        } else {
            assert_eq!(0, index);
            self.get_message(m)
        }
    }

    fn get_rep_enum_item_x(&self, m: &Message, index: usize) -> &'static EnumValueDescriptor {
        if self.is_repeated() {
            self.get_rep_enum_item(m, index)
        } else {
            assert_eq!(0, index);
            self.get_enum(m)
        }
    }

    fn get_rep_str_item_x<'a>(&self, m: &'a Message, index: usize) -> &'a str {
        if self.is_repeated() {
            self.get_rep_str_item(m, index)
        } else {
            assert_eq!(0, index);
            self.get_str(m)
        }
    }

    fn get_rep_bytes_item_x<'a>(&self, m: &'a Message, index: usize) -> &'a [u8] {
        if self.is_repeated() {
            self.get_rep_bytes_item(m, index)
        } else {
            assert_eq!(0, index);
            self.get_bytes(m)
        }
    }

    fn get_rep_i32_item_x(&self, m: &Message, index: usize) -> i32 {
        if self.is_repeated() {
            self.get_rep_i32(m)[index]
        } else {
            assert_eq!(0, index);
            self.get_i32(m)
        }
    }

    fn get_rep_u32_item_x(&self, m: &Message, index: usize) -> u32 {
        if self.is_repeated() {
            self.get_rep_u32(m)[index]
        } else {
            assert_eq!(0, index);
            self.get_u32(m)
        }
    }

    fn get_rep_i64_item_x(&self, m: &Message, index: usize) -> i64 {
        if self.is_repeated() {
            self.get_rep_i64(m)[index]
        } else {
            assert_eq!(0, index);
            self.get_i64(m)
        }
    }

    fn get_rep_u64_item_x(&self, m: &Message, index: usize) -> u64 {
        if self.is_repeated() {
            self.get_rep_u64(m)[index]
        } else {
            assert_eq!(0, index);
            self.get_u64(m)
        }
    }

    fn get_rep_bool_item_x(&self, m: &Message, index: usize) -> bool {
        if self.is_repeated() {
            self.get_rep_bool(m)[index]
        } else {
            assert_eq!(0, index);
            self.get_bool(m)
        }
    }

    fn get_rep_f32_item_x(&self, m: &Message, index: usize) -> f32 {
        if self.is_repeated() {
            self.get_rep_f32(m)[index]
        } else {
            assert_eq!(0, index);
            self.get_f32(m)
        }
    }

    fn get_rep_f64_item_x(&self, m: &Message, index: usize) -> f64 {
        if self.is_repeated() {
            self.get_rep_f64(m)[index]
        } else {
            assert_eq!(0, index);
            self.get_f64(m)
        }
    }

}

fn print_to_internal(m: &Message, buf: &mut String, pretty: bool, indent: usize) {
    let d = m.descriptor();
    let mut first = true;
    for f in d.fields().iter() {
        for i in 0..f.len_field_x(m) {
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
                    print_to_internal(f.get_rep_message_item_x(m, i), buf, pretty, indent+1);
                    do_indent(buf, pretty, indent);
                    buf.push_str("}");
                },
                FieldDescriptorProto_Type::TYPE_ENUM => {
                    buf.push_str(": ");
                    buf.push_str(f.get_rep_enum_item_x(m, i).name());
                },
                FieldDescriptorProto_Type::TYPE_STRING => {
                    buf.push_str(": ");
                    print_str_to(f.get_rep_str_item_x(m, i), buf);
                },
                FieldDescriptorProto_Type::TYPE_BYTES => {
                    buf.push_str(": ");
                    print_bytes_to(f.get_rep_bytes_item_x(m, i), buf);
                },
                FieldDescriptorProto_Type::TYPE_INT32 |
                FieldDescriptorProto_Type::TYPE_SINT32 |
                FieldDescriptorProto_Type::TYPE_SFIXED32 => {
                    buf.push_str(": ");
                    buf.push_str(&f.get_rep_i32_item_x(m, i).to_string());
                },
                FieldDescriptorProto_Type::TYPE_INT64 |
                FieldDescriptorProto_Type::TYPE_SINT64 |
                FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                    buf.push_str(": ");
                    buf.push_str(&f.get_rep_i64_item_x(m, i).to_string());
                },
                FieldDescriptorProto_Type::TYPE_UINT32 |
                FieldDescriptorProto_Type::TYPE_FIXED32 => {
                    buf.push_str(": ");
                    buf.push_str(&f.get_rep_u32_item_x(m, i).to_string());
                },
                FieldDescriptorProto_Type::TYPE_UINT64 |
                FieldDescriptorProto_Type::TYPE_FIXED64 => {
                    buf.push_str(": ");
                    buf.push_str(&f.get_rep_u64_item_x(m, i).to_string());
                },
                FieldDescriptorProto_Type::TYPE_BOOL => {
                    buf.push_str(": ");
                    buf.push_str(&f.get_rep_bool_item_x(m, i).to_string());
                },
                FieldDescriptorProto_Type::TYPE_FLOAT => {
                    buf.push_str(": ");
                    buf.push_str(&f.get_rep_f32_item_x(m, i).to_string());
                },
                FieldDescriptorProto_Type::TYPE_DOUBLE => {
                    buf.push_str(": ");
                    buf.push_str(&f.get_rep_f64_item_x(m, i).to_string());
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
    let pretty = f.alternate();
    f.write_str(&print_to_string_internal(m, pretty))
}
