use crate::rt::bytes_size_no_tag;
use crate::rt::tag_size;
use crate::rt::ProtobufVarint;
use crate::wire_format::Tag;
use crate::wire_format::WireType;
use crate::CodedInputStream;
use crate::UnknownFields;

fn skip_group(is: &mut CodedInputStream) -> crate::Result<()> {
    loop {
        let (_, wire_type) = is.read_tag_unpack()?;
        if wire_type == WireType::EndGroup {
            return Ok(());
        }
        is.skip_field(wire_type)?;
    }
}

/// Size of encoded unknown fields size.
pub fn unknown_fields_size(unknown_fields: &UnknownFields) -> u64 {
    let mut r = 0;
    for (number, values) in unknown_fields {
        r += (tag_size(number) + 4) * values.fixed32.len() as u64;
        r += (tag_size(number) + 8) * values.fixed64.len() as u64;

        r += tag_size(number) * values.varint.len() as u64;
        for varint in &values.varint {
            r += varint.len_varint();
        }

        r += tag_size(number) * values.length_delimited.len() as u64;
        for bytes in &values.length_delimited {
            r += bytes_size_no_tag(&bytes);
        }
    }
    r
}

/// Handle unknown field in generated code.
/// Either store a value in unknown, or skip a group.
pub(crate) fn read_unknown_or_skip_group_with_tag_unpacked(
    field_number: u32,
    wire_type: WireType,
    is: &mut CodedInputStream,
    unknown_fields: &mut UnknownFields,
) -> crate::Result<()> {
    match wire_type {
        WireType::StartGroup => skip_group(is),
        _ => {
            let unknown = is.read_unknown(wire_type)?;
            unknown_fields.add_value(field_number, unknown);
            Ok(())
        }
    }
}

/// Handle unknown field in generated code.
/// Either store a value in unknown, or skip a group.
/// Return error if tag is incorrect.
pub fn read_unknown_or_skip_group(
    tag: u32,
    is: &mut CodedInputStream,
    unknown_fields: &mut UnknownFields,
) -> crate::Result<()> {
    let (field_humber, wire_type) = Tag::new(tag)?.unpack();
    read_unknown_or_skip_group_with_tag_unpacked(field_humber, wire_type, is, unknown_fields)
}
