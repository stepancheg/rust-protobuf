// hex encoder and decoder used by rust-protobuf unittests

#![allow(dead_code)]

use std::char;

fn decode_hex_digit(digit: char) -> u8 {
    match char::to_digit(digit, 16) {
        Some(d) => d as u8,
        _ => panic!()
    }
}

pub fn decode_hex(hex: &str) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    let mut pos = 0;
    loop {
        while pos < hex.char_len() && hex.to_ascii().get(pos).unwrap().to_byte() == b' ' {
            pos += 1;
        }
        if hex.char_len() - pos >= 2 {
            r.push((decode_hex_digit(hex.char_at(pos)) << 4) | decode_hex_digit(hex.char_at(pos + 1)));
            pos += 2;
            continue;
        }
        if pos == hex.char_len() {
            break;
        }
        panic!("pos = {:u}d", pos);
    }
    r
}

fn encode_hex_digit(digit: u8) -> char {
    match char::from_digit(digit as uint, 16) {
        Some(c) => c,
        _ => panic!()
    }
}

fn encode_hex_byte(byte: u8) -> [char, ..2] {
    [encode_hex_digit(byte >> 4), encode_hex_digit(byte & 0x0Fu8)]
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|byte| {
        String::from_chars(encode_hex_byte(*byte).as_slice())
    }).collect();
    strs.connect(" ")
}

#[cfg(test)]
mod test {

    use super::decode_hex;
    use super::encode_hex;

    #[test]
    fn test_decode_hex() {
        assert_eq!([].to_vec(), decode_hex(""));
        assert_eq!([0x00u8].to_vec(), decode_hex("00"));
        assert_eq!([0xffu8].to_vec(), decode_hex("ff"));
        assert_eq!([0xabu8].to_vec(), decode_hex("AB"));
        assert_eq!([0xfau8, 0x19].to_vec(), decode_hex("fa 19"));
    }

    #[test]
    fn test_encode_hex() {
        assert_eq!("".to_string(), encode_hex([]));
        assert_eq!("00".to_string(), encode_hex([0x00]));
        assert_eq!("ab".to_string(), encode_hex([0xab]));
        assert_eq!("01 a2 1a fe".to_string(), encode_hex([0x01, 0xa2, 0x1a, 0xfe]));
    }
}
