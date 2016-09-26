use std::fmt;
use std::fmt::Write;
use core::Message;
use reflect::ReflectFieldRef;
use reflect::ProtobufValueRef;


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
                if let Some(v) = optional {
                    print_field(buf, pretty, indent, &mut first, f.name(), v);
                }
            }
            ReflectFieldRef::RepeatedOld(..) => {
                for i in 0..f.len_field(m) {
                    let v = f.get_rep_item(m, i);
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
