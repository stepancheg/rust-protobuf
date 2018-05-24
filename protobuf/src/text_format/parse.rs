use std::str;

#[doc(hidden)]
pub fn unescape_string(string: &str) -> Vec<u8> {
    fn parse_if_digit(chars: &mut str::Chars) -> u8 {
        let mut copy = chars.clone();
        let f = match copy.next() {
            None => return 0,
            Some(f) => f,
        };
        let d = match f {
            '0'...'9' => (f as u8 - b'0'),
            _ => return 0,
        };
        *chars = copy;
        d
    }

    fn parse_hex_digit(chars: &mut str::Chars) -> u8 {
        match chars.next().unwrap() {
            c @ '0'...'9' => (c as u8) - b'0',
            c @ 'a'...'f' => (c as u8) - b'a' + 10,
            c @ 'A'...'F' => (c as u8) - b'A' + 10,
            _ => panic!("incorrect hex escape"),
        }
    }

    fn parse_escape_rem(chars: &mut str::Chars) -> u8 {
        let n = chars.next().unwrap();
        match n {
            'a' => return b'\x07',
            'b' => return b'\x08',
            'f' => return b'\x0c',
            'n' => return b'\n',
            'r' => return b'\r',
            't' => return b'\t',
            'v' => return b'\x0b',
            '"' => return b'"',
            '\'' => return b'\'',
            '0'...'9' => {
                let d1 = n as u8 - b'0';
                let d2 = parse_if_digit(chars);
                let d3 = parse_if_digit(chars);
                return (d1 * 64 + d2 * 8 + d3) as u8;
            },
            'x' => {
                let d1 = parse_hex_digit(chars);
                let d2 = parse_hex_digit(chars);
                return d1 * 16 + d2;
            }
            c => return c as u8, // TODO: validate ASCII
        };
    }

    let mut chars = string.chars();
    let mut r = Vec::new();

    loop {
        let f = match chars.next() {
            None => return r,
            Some(f) => f,
        };

        if f == '\\' {
            r.push(parse_escape_rem(&mut chars));
        } else {
            r.push(f as u8); // TODO: escape UTF-8
        }
    }
}
