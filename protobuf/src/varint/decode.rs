//! Varint decode utilities.

use crate::error::WireError;
use crate::varint::MAX_VARINT_ENCODED_LEN;

/// Decode a varint, and return decoded value and decoded byte count.
#[inline]
fn decode_varint64_full(rem: &[u8]) -> crate::Result<Option<(u64, usize)>> {
    let mut r: u64 = 0;
    for (i, &b) in rem.iter().enumerate() {
        if i == MAX_VARINT_ENCODED_LEN {
            return Err(WireError::IncorrectVarint.into());
        }

        if i == 9 && (b & 0x7f) > 1 {
            return Err(WireError::IncorrectVarint.into());
        }
        r = r | (((b & 0x7f) as u64) << (i as u64 * 7));
        if b < 0x80 {
            return Ok(Some((r, i + 1)));
        }
    }
    Ok(None)
}

/// Try decode a varint. Return `None` if the buffer does not contain complete varint.
#[inline]
pub(crate) fn decode_varint64(buf: &[u8]) -> crate::Result<Option<(u64, usize)>> {
    if buf.len() >= 1 && buf[0] < 0x80 {
        // The the most common case.
        let ret = buf[0] as u64;
        let consume = 1;
        Ok(Some((ret, consume)))
    } else if buf.len() >= 2 && buf[1] < 0x80 {
        // Handle the case of two bytes too.
        let ret = (buf[0] & 0x7f) as u64 | (buf[1] as u64) << 7;
        let consume = 2;
        Ok(Some((ret, consume)))
    } else {
        // Read from array when buf at at least 10 bytes,
        // max len for varint.
        decode_varint64_full(buf)
    }
}

/// Try decode a varint. Return `None` if the buffer does not contain complete varint.
#[inline]
pub(crate) fn decode_varint32(buf: &[u8]) -> crate::Result<Option<(u32, usize)>> {
    // TODO: optimize
    match decode_varint64(buf)? {
        Some((v, consumed)) => {
            if v > u32::MAX as u64 {
                return Err(WireError::U32Overflow(v).into());
            }
            Ok(Some((v as u32, consumed)))
        }
        None => Ok(None),
    }
}
