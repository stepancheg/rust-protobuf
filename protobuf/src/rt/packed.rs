use crate::rt::compute_raw_varint32_size;
use crate::rt::tag_size;
use crate::rt::ProtobufFixed;
use crate::rt::ProtobufVarint;
use crate::rt::ProtobufVarintZigzag;
use crate::Enum;
use crate::EnumOrUnknown;

/// Size of serialized repeated packed field, excluding length and tag.
pub fn vec_packed_varint_data_size<T: ProtobufVarint>(vec: &[T]) -> u64 {
    vec.iter()
        .map(|v| v.len_varint() as u64)
        .fold(0, |a, i| a + i)
}

/// Size of serialized repeated packed field, excluding length and tag.
pub fn vec_packed_varint_zigzag_data_size<T: ProtobufVarintZigzag>(vec: &[T]) -> u64 {
    vec.iter()
        .map(|v| v.len_varint_zigzag())
        .fold(0, |a, i| a + i)
}

/// Size of serialized repeated packed enum field, excluding length and tag.
pub fn vec_packed_enum_or_unknown_data_size<E: Enum>(vec: &[EnumOrUnknown<E>]) -> u64 {
    vec.iter()
        .map(|e| compute_raw_varint32_size(e.value() as u32))
        .fold(0, |a, i| a + i)
}

/// Size of serialized data with length prefix and tag
pub fn vec_packed_varint_size<T: ProtobufVarint>(field_number: u32, vec: &[T]) -> u64 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_varint_data_size(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

/// Size of serialized data with length prefix and tag
pub fn vec_packed_varint_zigzag_size<T: ProtobufVarintZigzag>(field_number: u32, vec: &[T]) -> u64 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_varint_zigzag_data_size(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

/// Size of serialized data with length prefix and tag
pub fn vec_packed_enum_or_unknown_size<E: Enum>(
    field_number: u32,
    vec: &[EnumOrUnknown<E>],
) -> u64 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_enum_or_unknown_data_size(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}

/// Compute data size of fixed encoding of repeated field data.
pub(crate) fn vec_packed_fixed_data_size<V: ProtobufFixed>(vec: &[V]) -> u64 {
    (vec.len() as u64) * (V::LEN as u64)
}

/// Compute field size (data plus header) of fixed encoding of repeated field.
pub fn vec_packed_fixed_size<V: ProtobufFixed>(field_number: u32, vec: &[V]) -> u64 {
    if vec.is_empty() {
        0
    } else {
        let data_size = vec_packed_fixed_data_size::<V>(vec);
        tag_size(field_number) + data_size.len_varint() + data_size
    }
}
