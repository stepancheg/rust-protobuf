use std::fmt;
use std::fmt::Write;
use core::Message;
use reflect::FieldDescriptor;
use reflect::EnumValueDescriptor;
use reflect::ReflectFieldRef;
use reflect::ProtobufValueRef;
use descriptor::*;

fn print_bytes_to(bytes: &[u8], buf: &mut String) {
    buf.push('"');
    for &c in bytes {
        match c {
            b'\n' => buf.push_str(r"\n"),
            b'\r' => buf.push_str(r"\r"),
            b'\t' => buf.push_str(r"\t"),
            b'"' => buf.push_str("\\\""),
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

    fn get_rep_item_x<'a>(&self, m: &'a Message, index: usize) -> ProtobufValueRef<'a> {
        match self.proto().get_field_type() {
            FieldDescriptorProto_Type::TYPE_MESSAGE =>
                ProtobufValueRef::Message(self.get_rep_message_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_ENUM =>
                ProtobufValueRef::Enum(self.get_rep_enum_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_STRING =>
                ProtobufValueRef::String(self.get_rep_str_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_BYTES =>
                ProtobufValueRef::Bytes(self.get_rep_bytes_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_INT32 |
            FieldDescriptorProto_Type::TYPE_SINT32 |
            FieldDescriptorProto_Type::TYPE_SFIXED32 =>
                ProtobufValueRef::I32(self.get_rep_i32_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_INT64 |
            FieldDescriptorProto_Type::TYPE_SINT64 |
            FieldDescriptorProto_Type::TYPE_SFIXED64 =>
                ProtobufValueRef::I64(self.get_rep_i64_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_UINT32 |
            FieldDescriptorProto_Type::TYPE_FIXED32 =>
                ProtobufValueRef::U32(self.get_rep_u32_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_UINT64 |
            FieldDescriptorProto_Type::TYPE_FIXED64 =>
                ProtobufValueRef::U64(self.get_rep_u64_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_BOOL =>
                ProtobufValueRef::Bool(self.get_rep_bool_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_FLOAT =>
                ProtobufValueRef::F32(self.get_rep_f32_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_DOUBLE =>
                ProtobufValueRef::F64(self.get_rep_f64_item_x(m, index)),
            FieldDescriptorProto_Type::TYPE_GROUP =>
                unimplemented!(),
        }

    }
}

fn print_start_field(buf: &mut String, pretty: bool, indent: usize, first: &mut bool,
    field_name: &str)
{
    if !*first && !pretty {
        buf.push_str(" ");
    }
    do_indent(buf, pretty, indent);
    *first = false;
    buf.push_str(field_name);
}

fn print_end_field(buf: &mut String, pretty: bool) {
    if pretty {
        buf.push_str("\n");
    }
}

fn print_field(buf: &mut String, pretty: bool, indent: usize, first: &mut bool,
    field_name: &str, value: ProtobufValueRef)
{
    print_start_field(buf, pretty, indent, first, field_name);

    match value {
        ProtobufValueRef::Message(m) => {
            buf.push_str(" {");
            if pretty {
                buf.push_str("\n");
            }
            print_to_internal(m, buf, pretty, indent + 1);
            do_indent(buf, pretty, indent);
            buf.push_str("}");
        }
        ProtobufValueRef::Enum(e) => {
            buf.push_str(": ");
            buf.push_str(e.name());
        }
        ProtobufValueRef::String(s) => {
            buf.push_str(": ");
            print_str_to(s, buf);
        }
        ProtobufValueRef::Bytes(b) => {
            buf.push_str(": ");
            print_bytes_to(b, buf);
        },
        ProtobufValueRef::I32(v) => {
            write!(buf, ": {}", v).unwrap();
        },
        ProtobufValueRef::I64(v) => {
            write!(buf, ": {}", v).unwrap();
        },
        ProtobufValueRef::U32(v) => {
            write!(buf, ": {}", v).unwrap();
        },
        ProtobufValueRef::U64(v) => {
            write!(buf, ": {}", v).unwrap();
        },
        ProtobufValueRef::Bool(v) => {
            write!(buf, ": {}", v).unwrap();
        },
        ProtobufValueRef::F32(v) => {
            write!(buf, ": {}", v).unwrap();
        },
        ProtobufValueRef::F64(v) => {
            write!(buf, ": {}", v).unwrap();
        },
    }

    print_end_field(buf, pretty);
}

fn print_to_internal(m: &Message, buf: &mut String, pretty: bool, indent: usize) {
    let d = m.descriptor();
    let mut first = true;
    for f in d.fields() {
        match f.get_reflect(m) {
            ReflectFieldRef::Map(map) => {
                for (k, v) in map {
                    print_start_field(buf, pretty, indent, &mut first, f.name());
                    buf.push_str(" {");
                    if pretty {
                        buf.push_str("\n");
                    }

                    let mut entry_first = true;

                    print_field(buf, pretty, indent + 1, &mut entry_first, "key", k.as_ref());
                    print_field(buf, pretty, indent + 1, &mut entry_first, "value", v.as_ref());
                    do_indent(buf, pretty, indent);
                    buf.push_str("}");
                    print_end_field(buf, pretty);
                }
            },
            ReflectFieldRef::Repeated(repeated) => {
                // TODO: do not print zeros for v3
                for v in repeated {
                    print_field(buf, pretty, indent, &mut first, f.name(), v.as_ref());
                }
            },
            ReflectFieldRef::Optional(optional) => {
                if let Some(v) = optional.to_option() {
                    print_field(buf, pretty, indent, &mut first, f.name(), v.as_ref());
                }
            }
            ReflectFieldRef::Singular(v) => {
                if v.as_ref().is_non_zero() {
                    print_field(buf, pretty, indent, &mut first, f.name(), v.as_ref());
                }
            }
            ReflectFieldRef::Old => {
                for i in 0..f.len_field_x(m) {
                    let v = f.get_rep_item_x(m, i);
                    print_field(buf, pretty, indent, &mut first, f.name(), v);

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
        assert_eq!("\"a\\r\\n\\t '\\\"\\\\\"", escape(b"a\r\n\t '\"\\"));
        assert_eq!("\"\\344\\275\\240\\345\\245\\275\"", escape("你好".as_bytes()));
    }
}
