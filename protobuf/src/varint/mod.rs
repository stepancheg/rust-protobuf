pub(crate) mod decode;
pub(crate) mod encode;

/// Encoded varint message is not longer than 10 bytes.
pub(crate) const MAX_VARINT_ENCODED_LEN: usize = 10;
