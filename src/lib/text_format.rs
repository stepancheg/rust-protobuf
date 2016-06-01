use std::fmt;
use std::fmt::Write;
use core::Message;
use reflect::FieldDescriptor;
use reflect::EnumValueDescriptor;
use descriptor::*;

fn print_bytes_to(bytes: &[u8], buf: &mut String) {
    buf.push('"');
    for &c in bytes {
        match c {
            b'\n' => buf.push_str(r"\n"),
            b'\r' => buf.push_str(r"\r"),
            b'\t' => buf.push_str(r"\t"),
            b'"' => buf.push_str("\\\""),
            b'\'' => buf.push_str(r"\'"),
            b'\\' => buf.push_str(r"\\"),
            b'\x20'...b'\x7e' => buf.push(c as char),
            _ => {
                buf.push('\\');
                buf.push((b'0' + (c >> 6)) as char);
                buf.push((b'0' + ((c >> 3) & 7)) as char);
                buf.push((b'0' + (c & 7)) as char);
            }
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

#[allow(unused_must_use)] // because write!(..) to String need to to be checked
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
                    write!(buf, ": {}", f.get_rep_i32_item_x(m, i));
                },
                FieldDescriptorProto_Type::TYPE_INT64 |
                FieldDescriptorProto_Type::TYPE_SINT64 |
                FieldDescriptorProto_Type::TYPE_SFIXED64 => {
                    write!(buf, ": {}", f.get_rep_i64_item_x(m, i));
                },
                FieldDescriptorProto_Type::TYPE_UINT32 |
                FieldDescriptorProto_Type::TYPE_FIXED32 => {
                    write!(buf, ": {}", f.get_rep_u32_item_x(m, i));
                },
                FieldDescriptorProto_Type::TYPE_UINT64 |
                FieldDescriptorProto_Type::TYPE_FIXED64 => {
                    write!(buf, ": {}", f.get_rep_u64_item_x(m, i));
                },
                FieldDescriptorProto_Type::TYPE_BOOL => {
                    write!(buf, ": {}", f.get_rep_bool_item_x(m, i));
                },
                FieldDescriptorProto_Type::TYPE_FLOAT => {
                    write!(buf, ": {}", f.get_rep_f32_item_x(m, i));
                },
                FieldDescriptorProto_Type::TYPE_DOUBLE => {
                    write!(buf, ": {}", f.get_rep_f64_item_x(m, i));
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

#[cfg(test)]
mod test {
    
    fn escape(data: &[u8]) -> String {
        let mut s = String::with_capacity(data.len() * 4);
        super::print_bytes_to(data, &mut s);
        s
    }
    
    #[test]
    fn test_print_to_bytes() {
        assert_eq!("\"ab\"", escape(b"ab"));
        assert_eq!("\"a\\\\023\"", escape(b"a\\023"));
        assert_eq!("\"a\\r\\n\\t \\'\\\"\\\\\"", escape(b"a\r\n\t '\"\\"));
        assert_eq!("\"\\342\\235\\244\\360\\237\\220\\267\"", escape("❤🐷".as_bytes()));
    }
}
