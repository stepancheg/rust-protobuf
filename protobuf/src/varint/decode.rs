//! Varint decode utilities.

use crate::error::WireError;

/// Decode a varint, and return decoded value and decoded byte count.
pub(crate) fn decode_varint64(rem: &[u8]) -> crate::Result<(u64, usize)> {
    assert!(rem.len() >= 10);

    let mut r: u64 = 0;
    let mut i: usize = 0;
    loop {
        if i == 10 {
            return Err(WireError::IncorrectVarint.into());
        }

        let b = unsafe { *rem.get_unchecked(i) };

        if i == 9 && (b & 0x7f) > 1 {
            return Err(WireError::IncorrectVarint.into());
        }
        r = r | (((b & 0x7f) as u64) << (i as u64 * 7));
        i += 1;
        if b < 0x80 {
            break;
        }
    }
    Ok((r, i))
}
