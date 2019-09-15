use std::fmt;
use std::fmt::Write;

use crate::core::Message;
use crate::reflect::ReflectFieldRef;
use crate::reflect::ReflectValueRef;

fn quote_bytes_to(bytes: &[u8], buf: &mut String) {
    for &c in bytes {
        match c {
            b'\n' => buf.push_str(r"\n"),
            b'\r' => buf.push_str(r"\r"),
            b'\t' => buf.push_str(r"\t"),
            b'"' => buf.push_str("\\\""),
            b'\\' => buf.push_str(r"\\"),
            b'\x20'..=b'\x7e' => buf.push(c as char),
            _ => {
                buf.push('\\');
                buf.push((b'0' + (c >> 6)) as char);
                buf.push((b'0' + ((c >> 3) & 7)) as char);
                buf.push((b'0' + (c & 7)) as char);
            }
        }
    }
}

fn quote_escape_bytes_to(bytes: &[u8], buf: &mut String) {
    buf.push('"');
    quote_bytes_to(bytes, buf);
    buf.push('"');
}

#[doc(hidden)]
pub fn quote_escape_bytes(bytes: &[u8]) -> String {
    let mut r = String::new();
    quote_escape_bytes_to(bytes, &mut r);
    r
}

fn print_str_to(s: &str, buf: &mut String) {
    // TODO: keep printable Unicode
    quote_escape_bytes_to(s.as_bytes(), buf);
}

fn do_indent(buf: &mut String, pretty: bool, indent: usize) {
    if pretty && indent > 0 {
        for _ in 0..indent {
            buf.push_str("  ");
        }
    }
}

fn print_start_field(
    buf: &mut String,
    pretty: bool,
    indent: usize,
    first: &mut bool,
    field_name: &str,
) {
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

fn print_field(
    buf: &mut String,
    pretty: bool,
    indent: usize,
    first: &mut bool,
    field_name: &str,
    value: ReflectValueRef,
) {
    print_start_field(buf, pretty, indent, first, field_name);

    match value {
        ReflectValueRef::Message(m) => {
            buf.push_str(" {");
            if pretty {
                buf.push_str("\n");
            }
            print_to_internal(m, buf, pretty, indent + 1);
            do_indent(buf, pretty, indent);
            buf.push_str("}");
        }
        ReflectValueRef::Enum(d, v) => {
            buf.push_str(": ");
            match d.get_value_by_number(v) {
                Some(e) => buf.push_str(e.name()),
                None => write!(buf, ": {}", v).unwrap(),
            }
        }
        ReflectValueRef::String(s) => {
            buf.push_str(": ");
            print_str_to(s, buf);
        }
        ReflectValueRef::Bytes(b) => {
            buf.push_str(": ");
            quote_escape_bytes_to(b, buf);
        }
        ReflectValueRef::I32(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::I64(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::U32(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::U64(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::Bool(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::F32(v) => {
            write!(buf, ": {}", v).unwrap();
        }
        ReflectValueRef::F64(v) => {
            write!(buf, ": {}", v).unwrap();
        }
    }

    print_end_field(buf, pretty);
}

fn print_to_internal(m: &dyn Message, buf: &mut String, pretty: bool, indent: usize) {
    let d = m.descriptor();
    let mut first = true;
    for f in d.fields() {
        match f.get_reflect(m) {
            ReflectFieldRef::Map(map) => {
                for (k, v) in &map {
                    print_start_field(buf, pretty, indent, &mut first, f.name());
                    buf.push_str(" {");
                    if pretty {
                        buf.push_str("\n");
                    }

                    let mut entry_first = true;

                    print_field(buf, pretty, indent + 1, &mut entry_first, "key", k);
                    print_field(buf, pretty, indent + 1, &mut entry_first, "value", v);
                    do_indent(buf, pretty, indent);
                    buf.push_str("}");
                    print_end_field(buf, pretty);
                }
            }
            ReflectFieldRef::Repeated(repeated) => {
                // TODO: do not print zeros for v3
                for v in repeated {
                    print_field(buf, pretty, indent, &mut first, f.name(), v);
                }
            }
            ReflectFieldRef::Optional(optional) => {
                if let Some(v) = optional {
                    print_field(buf, pretty, indent, &mut first, f.name(), v);
                }
            }
        }
    }

    // TODO: unknown fields
}

/// Text-format
pub fn print_to(m: &dyn Message, buf: &mut String) {
    print_to_internal(m, buf, false, 0)
}

fn print_to_string_internal(m: &dyn Message, pretty: bool) -> String {
    let mut r = String::new();
    print_to_internal(m, &mut r, pretty, 0);
    r.to_string()
}

/// Text-format
pub fn print_to_string(m: &dyn Message) -> String {
    print_to_string_internal(m, false)
}

/// Text-format to `fmt::Formatter`.
pub fn fmt(m: &dyn Message, f: &mut fmt::Formatter) -> fmt::Result {
    let pretty = f.alternate();
    f.write_str(&print_to_string_internal(m, pretty))
}

#[cfg(test)]
mod test {

    use crate::text_format::lexer::StrLit;

    fn escape(data: &[u8]) -> String {
        let mut s = String::with_capacity(data.len() * 4);
        super::quote_bytes_to(data, &mut s);
        s
    }

    fn unescape_string(escaped: &str) -> Vec<u8> {
        StrLit {
            escaped: escaped.to_owned(),
        }
        .decode_bytes()
        .expect("decode_bytes")
    }

    fn test_escape_unescape(text: &str, escaped: &str) {
        assert_eq!(text.as_bytes(), &unescape_string(escaped)[..]);
        assert_eq!(escaped, &escape(text.as_bytes())[..]);
    }

    #[test]
    fn test_print_to_bytes() {
        assert_eq!("ab", escape(b"ab"));
        assert_eq!("a\\\\023", escape(b"a\\023"));
        assert_eq!("a\\r\\n\\t '\\\"\\\\", escape(b"a\r\n\t '\"\\"));
        assert_eq!("\\344\\275\\240\\345\\245\\275", escape("你好".as_bytes()));
    }

    #[test]
    fn test_unescape_string() {
        test_escape_unescape("", "");
        test_escape_unescape("aa", "aa");
        test_escape_unescape("\n", "\\n");
        test_escape_unescape("\r", "\\r");
        test_escape_unescape("\t", "\\t");
        test_escape_unescape("你好", "\\344\\275\\240\\345\\245\\275");
        // hex
        assert_eq!(b"aaa\x01bbb", &unescape_string("aaa\\x01bbb")[..]);
        assert_eq!(b"aaa\xcdbbb", &unescape_string("aaa\\xCDbbb")[..]);
        assert_eq!(b"aaa\xcdbbb", &unescape_string("aaa\\xCDbbb")[..]);
        // quotes
        assert_eq!(b"aaa\"bbb", &unescape_string("aaa\\\"bbb")[..]);
        assert_eq!(b"aaa\'bbb", &unescape_string("aaa\\\'bbb")[..]);
    }
}
