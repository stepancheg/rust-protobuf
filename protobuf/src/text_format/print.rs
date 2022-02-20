use std::fmt;
use std::fmt::Write;

use crate::message_dyn::MessageDyn;
use crate::reflect::MessageRef;
use crate::reflect::ReflectFieldRef;
use crate::reflect::ReflectValueRef;

#[doc(hidden)]
pub fn quote_bytes_to(bytes: &[u8], buf: &mut String) {
    for &c in bytes {
        match c {
            b'\n' => buf.push_str(r"\n"),
            b'\r' => buf.push_str(r"\r"),
            b'\t' => buf.push_str(r"\t"),
            b'\'' => buf.push_str("\\\'"),
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

trait FieldName: fmt::Display {}
impl<'a> FieldName for &'a str {}
impl FieldName for u32 {}

fn print_start_field<F: FieldName>(
    buf: &mut String,
    pretty: bool,
    indent: usize,
    first: &mut bool,
    field_name: F,
) {
    if !*first && !pretty {
        buf.push_str(" ");
    }
    do_indent(buf, pretty, indent);
    *first = false;
    write!(buf, "{}", field_name).unwrap();
}

fn print_end_field(buf: &mut String, pretty: bool) {
    if pretty {
        buf.push_str("\n");
    }
}

fn print_field<F: FieldName>(
    buf: &mut String,
    pretty: bool,
    indent: usize,
    first: &mut bool,
    field_name: F,
    value: ReflectValueRef,
) {
    print_start_field(buf, pretty, indent, first, field_name);

    match value {
        ReflectValueRef::Message(m) => {
            buf.push_str(" {");
            if pretty {
                buf.push_str("\n");
            }
            print_to_internal(&m, buf, pretty, indent + 1);
            do_indent(buf, pretty, indent);
            buf.push_str("}");
        }
        ReflectValueRef::Enum(d, v) => {
            buf.push_str(": ");
            match d.value_by_number(v) {
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

fn print_to_internal(m: &MessageRef, buf: &mut String, pretty: bool, indent: usize) {
    let d = m.descriptor_dyn();
    let mut first = true;
    for f in d.fields() {
        match f.get_reflect(&**m) {
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

    let unknown_fields = m.unknown_fields_dyn();
    let mut numbers: Vec<u32> = m.unknown_fields_dyn().iter().map(|(n, _)| n).collect();
    // Sort for stable output
    numbers.sort();
    for &n in &numbers {
        for v in unknown_fields.get(n).unwrap() {
            // TODO: try decode nested message for length-delimited
            print_field(buf, pretty, indent, &mut first, n, v.to_reflect_value_ref());
        }
    }
}

/// Text-format
pub fn print_to(m: &dyn MessageDyn, buf: &mut String) {
    print_to_internal(&MessageRef::from(m), buf, false, 0)
}

fn print_to_string_internal(m: &dyn MessageDyn, pretty: bool) -> String {
    let mut r = String::new();
    print_to_internal(&MessageRef::from(m), &mut r, pretty, 0);
    r.to_string()
}

/// Text-format
pub fn print_to_string(m: &dyn MessageDyn) -> String {
    print_to_string_internal(m, false)
}

/// Text-format
pub fn print_to_string_pretty(m: &dyn MessageDyn) -> String {
    print_to_string_internal(m, true)
}

/// Text-format to `fmt::Formatter`.
pub fn fmt(m: &dyn MessageDyn, f: &mut fmt::Formatter) -> fmt::Result {
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
        assert_eq!("a\\r\\n\\t \\'\\\"\\\\", escape(b"a\r\n\t '\"\\"));
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
