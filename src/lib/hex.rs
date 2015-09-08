// hex encoder and decoder used by rust-protobuf unittests

use std::char;

fn decode_hex_digit(digit: char) -> u8 {
    match digit {
        '0' ... '9' => digit as u8 - '0' as u8,
        'a' ... 'f' => digit as u8 - 'a' as u8 + 10,
        'A' ... 'F' => digit as u8 - 'A' as u8 + 10,
        _ => panic!(),
    }
}

pub fn decode_hex(hex: &str) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    let mut chars = hex.chars().enumerate();
    loop {
        let (pos, first) = match chars.next() {
            None => break,
            Some(elt) => elt
        };
        if first == ' ' { continue; }
        let (_, second) = match chars.next() {
            None => panic!("pos = {}d", pos),
            Some(elt) => elt
        };
        r.push((decode_hex_digit(first) << 4) | decode_hex_digit(second));
    }
    r
}

fn encode_hex_digit(digit: u8) -> char {
    match char::from_digit(digit as u32, 16) {
        Some(c) => c,
        _ => panic!()
    }
}

fn encode_hex_byte(byte: u8) -> [char; 2] {
    [encode_hex_digit(byte >> 4), encode_hex_digit(byte & 0x0Fu8)]
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|byte| {
        encode_hex_byte(*byte).iter().map(|c| *c).collect()
    }).collect();
    strs.connect(" ")
}

#[cfg(test)]
mod test {

    use super::decode_hex;
    use super::encode_hex;

    #[test]
    fn test_decode_hex() {
        assert_eq!(decode_hex(""), [].to_vec());
        assert_eq!(decode_hex("00"), [0x00u8].to_vec());
        assert_eq!(decode_hex("ff"), [0xffu8].to_vec());
        assert_eq!(decode_hex("AB"), [0xabu8].to_vec());
        assert_eq!(decode_hex("fa 19"), [0xfau8, 0x19].to_vec());
    }

    #[test]
    fn test_encode_hex() {
        assert_eq!("".to_owned(), encode_hex(&[]));
        assert_eq!("00".to_owned(), encode_hex(&[0x00]));
        assert_eq!("ab".to_owned(), encode_hex(&[0xab]));
        assert_eq!("01 a2 1a fe".to_owned(), encode_hex(&[0x01, 0xa2, 0x1a, 0xfe]));
    }
}
