use std::collections::HashMap;
use std::hash::Hash;

use crate::error::WireError;
use crate::reflect::types::ProtobufType;
use crate::wire_format::WireType;
use crate::CodedInputStream;

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
        return Err(WireError::UnexpectedWireType(wire_type).into());
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
                return Err(WireError::UnexpectedWireType(wire_type).into());
            }
            key = K::read(is)?;
            Ok(())
        },
        |wire_type, is| {
            if wire_type != V::WIRE_TYPE {
                return Err(WireError::UnexpectedWireType(wire_type).into());
            }
            value = V::read(is)?;
            Ok(())
        },
    )?;

    target.insert(key, value);

    Ok(())
}
