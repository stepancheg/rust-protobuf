//! Copy-pasted from the internet
/// Available encoding character sets
#[derive(Clone, Copy, Debug)]
enum _CharacterSet {
    /// The standard character set (uses `+` and `/`)
    _Standard,
    /// The URL safe character set (uses `-` and `_`)
    _UrlSafe,
}

static STANDARD_CHARS: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                        abcdefghijklmnopqrstuvwxyz\
                                        0123456789+/";

static _URLSAFE_CHARS: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                       abcdefghijklmnopqrstuvwxyz\
                                       0123456789-_";

pub fn encode(input: &[u8]) -> String {
    let bytes = STANDARD_CHARS;

    let len = input.len();

    // Preallocate memory.
    let prealloc_len = (len + 2) / 3 * 4;
    let mut out_bytes = vec![b'='; prealloc_len];

    // Deal with padding bytes
    let mod_len = len % 3;

    // Use iterators to reduce branching
    {
        let mut s_in = input[..len - mod_len].iter().map(|&x| x as u32);
        let mut s_out = out_bytes.iter_mut();

        // Convenient shorthand
        let enc = |val| bytes[val as usize];
        let mut write = |val| *s_out.next().unwrap() = val;

        // Iterate though blocks of 4
        while let (Some(first), Some(second), Some(third)) = (s_in.next(), s_in.next(), s_in.next())
        {
            let n = first << 16 | second << 8 | third;

            // This 24-bit number gets separated into four 6-bit numbers.
            write(enc((n >> 18) & 63));
            write(enc((n >> 12) & 63));
            write(enc((n >> 6) & 63));
            write(enc((n >> 0) & 63));
        }

        // Heh, would be cool if we knew this was exhaustive
        // (the dream of bounded integer types)
        match mod_len {
            0 => (),
            1 => {
                let n = (input[len - 1] as u32) << 16;
                write(enc((n >> 18) & 63));
                write(enc((n >> 12) & 63));
            }
            2 => {
                let n = (input[len - 2] as u32) << 16 | (input[len - 1] as u32) << 8;
                write(enc((n >> 18) & 63));
                write(enc((n >> 12) & 63));
                write(enc((n >> 6) & 63));
            }
            _ => panic!("Algebra is broken, please alert the math police"),
        }
    }

    // `out_bytes` vec is prepopulated with `=` symbols and then only updated
    // with base64 chars, so this unsafe is safe.
    unsafe { String::from_utf8_unchecked(out_bytes) }
}

/// Errors that can occur when decoding a base64 encoded string
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum FromBase64Error {
    /// The input contained a character not part of the base64 format
    #[error("Invalid base64 byte")]
    InvalidBase64Byte(u8, usize),
    /// The input had an invalid length
    #[error("Invalid base64 length")]
    InvalidBase64Length,
}

pub fn decode(input: &str) -> Result<Vec<u8>, FromBase64Error> {
    let mut r = Vec::with_capacity(input.len());
    let mut buf: u32 = 0;
    let mut modulus = 0;

    let mut it = input.as_bytes().iter();
    for byte in it.by_ref() {
        let code = DECODE_TABLE[*byte as usize];
        if code >= SPECIAL_CODES_START {
            match code {
                NEWLINE_CODE => continue,
                EQUALS_CODE => break,
                INVALID_CODE => {
                    return Err(FromBase64Error::InvalidBase64Byte(
                        *byte,
                        (byte as *const _ as usize) - input.as_ptr() as usize,
                    ))
                }
                _ => unreachable!(),
            }
        }
        buf = (buf | code as u32) << 6;
        modulus += 1;
        if modulus == 4 {
            modulus = 0;
            r.push((buf >> 22) as u8);
            r.push((buf >> 14) as u8);
            r.push((buf >> 6) as u8);
        }
    }

    for byte in it {
        match *byte {
            b'=' | b'\r' | b'\n' => continue,
            _ => {
                return Err(FromBase64Error::InvalidBase64Byte(
                    *byte,
                    (byte as *const _ as usize) - input.as_ptr() as usize,
                ))
            }
        }
    }

    match modulus {
        2 => {
            r.push((buf >> 10) as u8);
        }
        3 => {
            r.push((buf >> 16) as u8);
            r.push((buf >> 8) as u8);
        }
        0 => (),
        _ => return Err(FromBase64Error::InvalidBase64Length),
    }

    Ok(r)
}

const DECODE_TABLE: [u8; 256] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFD, 0xFF, 0xFF, 0xFD, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x3E, 0xFF, 0x3E, 0xFF, 0x3F,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF,
    0xFF, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
    0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0xFF, 0xFF, 0xFF, 0xFF, 0x3F,
    0xFF, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30, 0x31, 0x32, 0x33, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];
const INVALID_CODE: u8 = 0xFF;
const EQUALS_CODE: u8 = 0xFE;
const NEWLINE_CODE: u8 = 0xFD;
const SPECIAL_CODES_START: u8 = NEWLINE_CODE;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"f"), "Zg==");
        assert_eq!(encode(b"fo"), "Zm8=");
        assert_eq!(encode(b"foo"), "Zm9v");
        assert_eq!(encode(b"foob"), "Zm9vYg==");
        assert_eq!(encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn test_encode_standard_safe() {
        assert_eq!(encode(&[251, 255]), "+/8=");
    }

    #[test]
    fn test_decode_basic() {
        assert_eq!(decode("").unwrap(), b"");
        assert_eq!(decode("Zg==").unwrap(), b"f");
        assert_eq!(decode("Zm8=").unwrap(), b"fo");
        assert_eq!(decode("Zm9v").unwrap(), b"foo");
        assert_eq!(decode("Zm9vYg==").unwrap(), b"foob");
        assert_eq!(decode("Zm9vYmE=").unwrap(), b"fooba");
        assert_eq!(decode("Zm9vYmFy").unwrap(), b"foobar");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("Zm9vYmFy").unwrap(), b"foobar");
    }

    #[test]
    fn test_decode_newlines() {
        assert_eq!(decode("Zm9v\r\nYmFy").unwrap(), b"foobar");
        assert_eq!(decode("Zm9vYg==\r\n").unwrap(), b"foob");
        assert_eq!(decode("Zm9v\nYmFy").unwrap(), b"foobar");
        assert_eq!(decode("Zm9vYg==\n").unwrap(), b"foob");
    }

    #[test]
    fn test_decode_urlsafe() {
        assert_eq!(decode("-_8").unwrap(), decode("+/8=").unwrap());
    }

    #[test]
    fn test_from_base64_invalid_char() {
        assert!(decode("Zm$=").is_err());
        assert!(decode("Zg==$").is_err());
    }

    #[test]
    fn test_decode_invalid_padding() {
        assert!(decode("Z===").is_err());
    }
}
