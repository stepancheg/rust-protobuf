use protobuf::prelude::*;

use crate::scope::FieldWithContext;
use crate::scope::MessageWithScope;
use protobuf::descriptor::field_descriptor_proto;

/// Pair of (key, value) if this message is map entry
pub(crate) fn map_entry<'a>(
    d: &'a MessageWithScope,
) -> Option<(FieldWithContext<'a>, FieldWithContext<'a>)> {
    if d.message.options.get_message().get_map_entry() {
        // Must be consistent with
        // DescriptorBuilder::ValidateMapEntry

        assert!(d.message.get_name().ends_with("Entry"));

        assert_eq!(0, d.message.extension.len());
        assert_eq!(0, d.message.extension_range.len());
        assert_eq!(0, d.message.nested_type.len());
        assert_eq!(0, d.message.enum_type.len());

        assert_eq!(2, d.fields().len());
        let key = d.fields()[0].clone();
        let value = d.fields()[1].clone();

        assert_eq!("key", key.name());
        assert_eq!("value", value.name());

        assert_eq!(1, key.number());
        assert_eq!(2, value.number());

        assert_eq!(
            field_descriptor_proto::Label::LABEL_OPTIONAL,
            key.field.get_label()
        );
        assert_eq!(
            field_descriptor_proto::Label::LABEL_OPTIONAL,
            value.field.get_label()
        );

        Some((key, value))
    } else {
        None
    }
}
