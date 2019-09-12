#[cfg(test)]
mod test {
    use crate::text_format;

    fn escape(data: &[u8]) -> String {
        let mut s = String::with_capacity(data.len() * 4);
        text_format::quote_bytes_to(data, &mut s);
        s
    }

    fn test_escape_unescape(text: &str, escaped: &str) {
        assert_eq!(text.as_bytes(), &text_format::unescape_string(escaped)[..]);
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
        assert_eq!(
            b"aaa\x01bbb",
            &text_format::unescape_string("aaa\\x01bbb")[..]
        );
        assert_eq!(
            b"aaa\xcdbbb",
            &text_format::unescape_string("aaa\\xCDbbb")[..]
        );
        assert_eq!(
            b"aaa\xcdbbb",
            &text_format::unescape_string("aaa\\xCDbbb")[..]
        );
        // quotes
        assert_eq!(b"aaa\"bbb", &text_format::unescape_string("aaa\\\"bbb")[..]);
        assert_eq!(b"aaa\'bbb", &text_format::unescape_string("aaa\\\'bbb")[..]);
    }
}
