use std::fmt;
use core::Message;
use descriptor::*;

fn print_bytes_to(bytes: &[u8], buf: &mut String) {
    buf.push_char('"');
    for &b in bytes.iter() {
        if b < 0x20 || b >= 0x80 {
            buf.push_char('\\');
            buf.push_char((b'0' + ((b >> 6) & 3)) as char);
            buf.push_char((b'0' + ((b >> 3) & 7)) as char);
            buf.push_char((b'0' + (b & 7)) as char);
        } else {
            buf.push_char(b as char);
        }
    }
    buf.push_char('"');
}

fn print_str_to(s: &str, buf: &mut String) {
    // TODO: keep printable Unicode
    print_bytes_to(s.as_bytes(), buf);
}

pub fn print_to(m: &Message, buf: &mut String) {
    let d = m.descriptor();
    let mut first = true;
    for f in d.fields().iter() {
        if f.is_repeated() {
            for i in range(0, f.len_field(m)) {
                if !first {
                    buf.push_str(" ");
                }
                first = false;
                buf.push_str(f.name());
                match f.proto().get_field_type() {
                    FieldDescriptorProto_TYPE_MESSAGE => {
                        buf.push_str(" {");
                        print_to(f.get_rep_message_item(m, i), buf);
                        buf.push_str("}");
                    },
                    FieldDescriptorProto_TYPE_ENUM => {
                        buf.push_str(": ");
                        buf.push_str(f.get_rep_enum_item(m, i).name());
                    },
                    FieldDescriptorProto_TYPE_STRING => {
                        buf.push_str(": ");
                        print_str_to(f.get_rep_str_item(m, i), buf);
                    },
                    FieldDescriptorProto_TYPE_BYTES => {
                        buf.push_str(": ");
                        print_bytes_to(f.get_rep_bytes_item(m, i), buf);
                    },
                    FieldDescriptorProto_TYPE_INT32 |
                    FieldDescriptorProto_TYPE_SINT32 |
                    FieldDescriptorProto_TYPE_SFIXED32 => {
                        buf.push_str(": ");
                        buf.push_str(f.get_rep_i32(m)[i].to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_INT64 |
                    FieldDescriptorProto_TYPE_SINT64 |
                    FieldDescriptorProto_TYPE_SFIXED64 => {
                        buf.push_str(": ");
                        buf.push_str(f.get_rep_i64(m)[i].to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_UINT32 |
                    FieldDescriptorProto_TYPE_FIXED32 => {
                        buf.push_str(": ");
                        buf.push_str(f.get_rep_u32(m)[i].to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_UINT64 |
                    FieldDescriptorProto_TYPE_FIXED64 => {
                        buf.push_str(": ");
                        buf.push_str(f.get_rep_u64(m)[i].to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_BOOL => {
                        buf.push_str(": ");
                        buf.push_str(f.get_rep_bool(m)[i].to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_FLOAT => {
                        buf.push_str(": ");
                        buf.push_str(f.get_rep_f32(m)[i].to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_DOUBLE => {
                        buf.push_str(": ");
                        buf.push_str(f.get_rep_f64(m)[i].to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_GROUP => {
                        buf.push_str(": <TYPE_GROUP>");
                    }
                }
            }
        } else {
            if f.has_field(m) {
                if !first {
                    buf.push_str(" ");
                }
                first = false;
                buf.push_str(f.name());
                match f.proto().get_field_type() {
                    FieldDescriptorProto_TYPE_MESSAGE => {
                        buf.push_str(" {");
                        print_to(f.get_message(m), buf);
                        buf.push_str("}");
                    },
                    FieldDescriptorProto_TYPE_ENUM => {
                        buf.push_str(": ");
                        buf.push_str(f.get_enum(m).name());
                    },
                    FieldDescriptorProto_TYPE_STRING => {
                        buf.push_str(": ");
                        print_str_to(f.get_str(m), buf);
                    },
                    FieldDescriptorProto_TYPE_BYTES => {
                        buf.push_str(": ");
                        print_bytes_to(f.get_bytes(m), buf);
                    },
                    FieldDescriptorProto_TYPE_INT32 |
                    FieldDescriptorProto_TYPE_SINT32 |
                    FieldDescriptorProto_TYPE_SFIXED32 => {
                        buf.push_str(": ");
                        buf.push_str(f.get_i32(m).to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_INT64 |
                    FieldDescriptorProto_TYPE_SINT64 |
                    FieldDescriptorProto_TYPE_SFIXED64 => {
                        buf.push_str(": ");
                        buf.push_str(f.get_i64(m).to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_UINT32 |
                    FieldDescriptorProto_TYPE_FIXED32 => {
                        buf.push_str(": ");
                        buf.push_str(f.get_u32(m).to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_UINT64 |
                    FieldDescriptorProto_TYPE_FIXED64 => {
                        buf.push_str(": ");
                        buf.push_str(f.get_u64(m).to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_BOOL => {
                        buf.push_str(": ");
                        buf.push_str(f.get_bool(m).to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_FLOAT => {
                        buf.push_str(": ");
                        buf.push_str(f.get_f32(m).to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_DOUBLE => {
                        buf.push_str(": ");
                        buf.push_str(f.get_f64(m).to_string().as_slice());
                    },
                    FieldDescriptorProto_TYPE_GROUP => {
                        buf.push_str(": <TYPE_GROUP>");
                    }
                }
            }
        }
    }

    // TODO: unknown fields
}

pub fn print_to_string(m: &Message) -> String {
    let mut r = String::new();
    print_to(m, &mut r);
    r.to_string()
}

pub fn fmt(m: &Message, f: &mut fmt::Formatter) -> fmt::Result {
    f.write(print_to_string(m).as_bytes())
}

#[cfg(test)]
mod test {
    use std::default::Default;

    use text_format_test_data::*;

    use super::*;

    fn t(expected: &str, setter: |&mut TestTypes|) {
        let mut m = TestTypes::new();
        setter(&mut m);
        assert_eq!(expected, print_to_string(&m).as_slice());
    }

    #[test]
    fn test_singular() {
        t("int32_singular: 99",       |m| m.set_int32_singular(99));
        t("double_singular: 99",      |m| m.set_double_singular(99.0));
        t("float_singular: 99",       |m| m.set_float_singular(99.0));
        t("int32_singular: 99",       |m| m.set_int32_singular(99));
        t("int64_singular: 99",       |m| m.set_int64_singular(99));
        t("uint32_singular: 99",      |m| m.set_uint32_singular(99));
        t("uint64_singular: 99",      |m| m.set_uint64_singular(99));
        t("sint32_singular: 99",      |m| m.set_sint32_singular(99));
        t("sint64_singular: 99",      |m| m.set_sint64_singular(99));
        t("fixed32_singular: 99",     |m| m.set_fixed32_singular(99));
        t("fixed64_singular: 99",     |m| m.set_fixed64_singular(99));
        t("sfixed32_singular: 99",    |m| m.set_sfixed32_singular(99));
        t("sfixed64_singular: 99",    |m| m.set_sfixed64_singular(99));
        t("bool_singular: false",     |m| m.set_bool_singular(false));
        t("string_singular: \"abc\"", |m| m.set_string_singular("abc".to_string()));
        t("bytes_singular: \"def\"",  |m| m.set_bytes_singular(Vec::from_slice("def".as_bytes())));
        t("test_enum_singular: DARK", |m| m.set_test_enum_singular(DARK));
        t("test_message_singular {}", |m| { m.mut_test_message_singular(); });
    }

    #[test]
    fn test_repeated_one() {
        t("int32_repeated: 99",       |m| m.add_int32_repeated(99));
        t("double_repeated: 99",      |m| m.add_double_repeated(99.0));
        t("float_repeated: 99",       |m| m.add_float_repeated(99.0));
        t("int32_repeated: 99",       |m| m.add_int32_repeated(99));
        t("int64_repeated: 99",       |m| m.add_int64_repeated(99));
        t("uint32_repeated: 99",      |m| m.add_uint32_repeated(99));
        t("uint64_repeated: 99",      |m| m.add_uint64_repeated(99));
        t("sint32_repeated: 99",      |m| m.add_sint32_repeated(99));
        t("sint64_repeated: 99",      |m| m.add_sint64_repeated(99));
        t("fixed32_repeated: 99",     |m| m.add_fixed32_repeated(99));
        t("fixed64_repeated: 99",     |m| m.add_fixed64_repeated(99));
        t("sfixed32_repeated: 99",    |m| m.add_sfixed32_repeated(99));
        t("sfixed64_repeated: 99",    |m| m.add_sfixed64_repeated(99));
        t("bool_repeated: false",     |m| m.add_bool_repeated(false));
        t("string_repeated: \"abc\"", |m| m.add_string_repeated(String::from_str("abc")));
        t("bytes_repeated: \"def\"",  |m| m.add_bytes_repeated(Vec::from_slice("def".as_bytes())));
        t("test_enum_repeated: DARK", |m| m.add_test_enum_repeated(DARK));
        t("test_message_repeated {}", |m| { m.add_test_message_repeated(Default::default()); });
    }

    #[test]
    fn test_repeated_multiple() {
        t("uint32_singular: 30 int32_repeated: 10 int32_repeated: -20", |m| {
            m.set_uint32_singular(30);
            m.add_int32_repeated(10);
            m.add_int32_repeated(-20);
        });
    }

    #[test]
    fn test_complex_message() {
        t("test_message_singular {value: 30}", |m| m.mut_test_message_singular().set_value(30));
    }

    #[test]
    fn test_show() {
        let mut m = TestTypes::new();
        m.set_bool_singular(true);
        assert_eq!("bool_singular: true", m.to_string().as_slice());
    }
}
