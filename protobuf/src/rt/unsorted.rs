use crate::{CodedInputStream, UnknownFields};
use crate::rt::read_unknown_or_skip_group_with_tag_unpacked;
use crate::wire_format::Tag;

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
