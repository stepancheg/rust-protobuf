use crate::wire_format::WireType;
use crate::CodedInputStream;
use crate::CodedOutputStream;
use crate::Message;
use crate::MessageField;

/// Read option `message` field.
pub fn read_option_message_into_field<M>(
    is: &mut CodedInputStream,
    target: &mut Option<M>,
) -> crate::Result<()>
where
    M: Message,
{
    let mut m = M::new();
    is.merge_message(&mut m)?;
    *target = Some(m);
    Ok(())
}

/// Read singular `message` field.
pub fn read_singular_message_into_field<M>(
    is: &mut CodedInputStream,
    target: &mut MessageField<M>,
) -> crate::Result<()>
where
    M: Message,
{
    let mut m = M::new();
    is.merge_message(&mut m)?;
    *target = MessageField::some(m);
    Ok(())
}

/// Write message with field number and length to the stream.
pub fn write_message_field_with_cached_size<M>(
    field_number: u32,
    message: &M,
    os: &mut CodedOutputStream,
) -> crate::Result<()>
where
    M: Message,
{
    os.write_tag(field_number, WireType::LengthDelimited)?;
    os.write_raw_varint32(message.cached_size())?;
    message.write_to_with_cached_sizes(os)
}
