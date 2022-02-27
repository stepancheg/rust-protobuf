use anyhow::ensure;
use protobuf::descriptor::field_descriptor_proto;

use crate::gen::scope::FieldWithContext;
use crate::gen::scope::MessageWithScope;

/// Pair of (key, value) if this message is map entry
pub(crate) fn map_entry<'a>(
    d: &'a MessageWithScope,
) -> anyhow::Result<Option<(FieldWithContext<'a>, FieldWithContext<'a>)>> {
    if d.message.is_map_entry() {
        // Must be consistent with
        // DescriptorBuilder::ValidateMapEntry

        // TODO: error, not panic
        ensure!(d.message.proto().name().ends_with("Entry"));

        ensure!(d.message.proto().extension.is_empty());
        ensure!(d.message.proto().extension_range.is_empty());
        ensure!(d.message.proto().nested_type.is_empty());
        ensure!(d.message.proto().enum_type.is_empty());

        ensure!(
            d.message.fields().count() == 2,
            "expecting two fields, got: {}",
            d.message
                .fields()
                .map(|f| format!("{}", f.name()))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let key = d.fields()[0].clone();
        let value = d.fields()[1].clone();

        ensure!(
            "key" == key.name(),
            "first field must be named 'key', got: '{}'",
            key.name()
        );
        ensure!(
            "value" == value.name(),
            "second field must be named 'value', got: '{}'",
            value.name()
        );

        ensure!(
            1 == key.number(),
            "key field number must be 1, got: {}",
            key.number()
        );
        ensure!(
            2 == value.number(),
            "value field number must be 2, got: {}",
            value.number()
        );

        ensure!(
            field_descriptor_proto::Label::LABEL_OPTIONAL == key.field.proto().label(),
            "key field must be optional, got: {:?}",
            key.field.proto().label()
        );
        ensure!(
            field_descriptor_proto::Label::LABEL_OPTIONAL == value.field.proto().label(),
            "value field must be optional, got: {:?}",
            value.field.proto().label()
        );

        Ok(Some((key, value)))
    } else {
        Ok(None)
    }
}
