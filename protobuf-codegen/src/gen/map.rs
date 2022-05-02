use crate::gen::scope::FieldWithContext;
use crate::gen::scope::MessageWithScope;

/// Pair of (key, value) if this message is map entry
pub(crate) fn map_entry<'a>(
    d: &'a MessageWithScope,
) -> Option<(FieldWithContext<'a>, FieldWithContext<'a>)> {
    if d.message.is_map_entry() {
        // `MessageDescriptor` validated the fields.
        let key = d.fields()[0].clone();
        let value = d.fields()[1].clone();
        Some((key, value))
    } else {
        None
    }
}
