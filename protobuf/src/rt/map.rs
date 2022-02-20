use std::collections::HashMap;
use std::hash::Hash;

use crate::reflect::types::ProtobufType;
use crate::rt::compute_raw_varint64_size;
use crate::rt::tag_size;
use crate::rt::unexpected_wire_type;
use crate::wire_format::WireType;
use crate::CodedInputStream;
use crate::CodedOutputStream;

/// Compute serialized size of `map` field and cache nested field sizes.
pub fn compute_map_size<K, V>(
    field_number: u32,
    map: &HashMap<K::ProtobufValue, V::ProtobufValue>,
) -> u64
where
    K: ProtobufType,
    V: ProtobufType,
    K::ProtobufValue: Eq + Hash,
{
    let mut sum = 0;
    for (k, v) in map {
        let key_tag_size = 1;
        let value_tag_size = 1;

        let key_len = K::compute_size_with_length_delimiter(k);
        let value_len = V::compute_size_with_length_delimiter(v);

        let entry_len = key_tag_size + key_len + value_tag_size + value_len;
        sum += tag_size(field_number) + compute_raw_varint64_size(entry_len) + entry_len;
    }
    sum
}

/// Write map, message sizes must be already known.
pub fn write_map_with_cached_sizes<K, V>(
    field_number: u32,
    map: &HashMap<K::ProtobufValue, V::ProtobufValue>,
    os: &mut CodedOutputStream,
) -> crate::Result<()>
where
    K: ProtobufType,
    V: ProtobufType,
    K::ProtobufValue: Eq + Hash,
{
    for (k, v) in map {
        let key_tag_size = 1;
        let value_tag_size = 1;

        let key_len = K::get_cached_size_with_length_delimiter(k);
        let value_len = V::get_cached_size_with_length_delimiter(v);

        let entry_len = key_tag_size + key_len + value_tag_size + value_len;

        os.write_tag(field_number, WireType::LengthDelimited)?;
        os.write_raw_varint32(entry_len)?;
        K::write_with_cached_size(1, k, os)?;
        V::write_with_cached_size(2, v, os)?;
    }
    Ok(())
}

pub(crate) fn read_map_template_new(
    is: &mut CodedInputStream,
    mut key: impl FnMut(WireType, &mut CodedInputStream) -> crate::Result<()>,
    mut value: impl FnMut(WireType, &mut CodedInputStream) -> crate::Result<()>,
) -> crate::Result<()> {
    let len = is.read_raw_varint32()?;
    let old_limit = is.push_limit(len as u64)?;
    while !is.eof()? {
        let (field_number, wire_type) = is.read_tag_unpack()?;
        match field_number {
            1 => key(wire_type, is)?,
            2 => value(wire_type, is)?,
            _ => is.skip_field(wire_type)?,
        }
    }
    is.pop_limit(old_limit);
    Ok(())
}

pub(crate) fn read_map_template(
    wire_type: WireType,
    is: &mut CodedInputStream,
    key: impl FnMut(WireType, &mut CodedInputStream) -> crate::Result<()>,
    value: impl FnMut(WireType, &mut CodedInputStream) -> crate::Result<()>,
) -> crate::Result<()> {
    if wire_type != WireType::LengthDelimited {
        return Err(unexpected_wire_type(wire_type));
    }

    read_map_template_new(is, key, value)
}

/// Read `map` field.
pub fn read_map_into<K, V>(
    is: &mut CodedInputStream,
    target: &mut HashMap<K::ProtobufValue, V::ProtobufValue>,
) -> crate::Result<()>
where
    K: ProtobufType,
    V: ProtobufType,
    K::ProtobufValue: Eq + Hash,
{
    let mut key = Default::default();
    let mut value = Default::default();

    read_map_template_new(
        is,
        |wire_type, is| {
            if wire_type != K::WIRE_TYPE {
                return Err(unexpected_wire_type(wire_type));
            }
            key = K::read(is)?;
            Ok(())
        },
        |wire_type, is| {
            if wire_type != V::WIRE_TYPE {
                return Err(unexpected_wire_type(wire_type));
            }
            value = V::read(is)?;
            Ok(())
        },
    )?;

    target.insert(key, value);

    Ok(())
}
