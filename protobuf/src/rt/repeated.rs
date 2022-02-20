use crate::error::WireError;
use crate::wire_format::WireType;
use crate::CodedInputStream;
use crate::Enum;
use crate::EnumOrUnknown;

fn read_repeated_packed_enum_or_unknown_into<E: Enum>(
    is: &mut CodedInputStream,
    target: &mut Vec<EnumOrUnknown<E>>,
) -> crate::Result<()> {
    let len = is.read_raw_varint64()?;
    let old_limit = is.push_limit(len)?;
    while !is.eof()? {
        target.push(is.read_enum_or_unknown()?);
    }
    is.pop_limit(old_limit);
    Ok(())
}

/// Read repeated `enum` field into given vec,
/// and when value is unknown store it in unknown fields
/// which matches proto2 spec.
///
/// See explanation
/// [here](https://github.com/stepancheg/rust-protobuf/issues/233#issuecomment-375142710)
pub fn read_repeated_enum_or_unknown_into<E: Enum>(
    wire_type: WireType,
    is: &mut CodedInputStream,
    target: &mut Vec<EnumOrUnknown<E>>,
) -> crate::Result<()> {
    match wire_type {
        WireType::LengthDelimited => read_repeated_packed_enum_or_unknown_into(is, target),
        WireType::Varint => {
            target.push(is.read_enum_or_unknown()?);
            Ok(())
        }
        _ => Err(WireError::UnexpectedWireType(wire_type).into()),
    }
}
