use std::mem::MaybeUninit;

use crate::varint::MAX_VARINT32_ENCODED_LEN;
use crate::varint::MAX_VARINT_ENCODED_LEN;

struct VarInt64Iterator {
    num: u64,
    cont: bool,
}

impl VarInt64Iterator {
    fn new(num: u64) -> Self {
        Self { num, cont: true }
    }
}

impl Iterator for VarInt64Iterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cont {
            self.cont = (self.num & !0x7F) != 0;
            let num = self.num;
            let val = if self.cont { (num & 0x7F) | 0x80 } else { num } as u8;
            self.num >>= 7;
            Some(val)
        } else {
            None
        }
    }
}

fn encode_varint(value: u64, buf: &mut [MaybeUninit<u8>]) -> usize {
    let iter = VarInt64Iterator::new(value);
    let mut bytes_written = 0;
    for (val, slot) in iter.zip(buf.iter_mut()) {
        slot.write(val);
        bytes_written += 1;
    }
    bytes_written
}

/// Encode u64 as varint.
/// Panics if buffer length is less than 10.
#[inline]
pub(crate) fn encode_varint64(value: u64, buf: &mut [MaybeUninit<u8>]) -> usize {
    assert!(buf.len() >= MAX_VARINT_ENCODED_LEN);
    encode_varint(value, buf)
}

/// Encode u32 value as varint.
/// Panics if buffer length is less than 5.
#[inline]
pub(crate) fn encode_varint32(value: u32, buf: &mut [MaybeUninit<u8>]) -> usize {
    assert!(buf.len() >= MAX_VARINT32_ENCODED_LEN);
    encode_varint(value as u64, buf)
}

/// Encoded size of u64 value.
#[inline]
pub(crate) fn encoded_varint64_len(value: u64) -> usize {
    // Bitwise-or'ing by 1 allows the `value = zero` case to work without
    // affecting other cases.
    let significant_bits = 64 - (value | 1).leading_zeros();
    (significant_bits + 6) as usize / 7
}

#[cfg(test)]
mod test {
    use std::mem::MaybeUninit;

    use crate::varint::encode::encode_varint64;
    use crate::varint::encode::encoded_varint64_len;

    #[test]
    fn test_encoded_varint64_len() {
        fn test(n: u64) {
            let mut buf = [MaybeUninit::uninit(); 10];
            let expected = encode_varint64(n, &mut buf);
            assert_eq!(expected, encoded_varint64_len(n), "n={}", n);
        }

        for n in 0..1000 {
            test(n);
        }

        for p in 0.. {
            match 2u64.checked_pow(p) {
                Some(n) => test(n),
                None => break,
            }
        }

        for p in 0.. {
            match 3u64.checked_pow(p) {
                Some(n) => test(n),
                None => break,
            }
        }

        test(u64::MAX);
        test(u64::MAX - 1);
        test((i64::MAX as u64) + 1);
        test(i64::MAX as u64);
        test((i64::MAX as u64) - 1);
        test((u32::MAX as u64) + 1);
        test(u32::MAX as u64);
        test((u32::MAX as u64) - 1);
        test((i32::MAX as u64) + 1);
        test(i32::MAX as u64);
        test((i32::MAX as u64) - 1);
    }
}
