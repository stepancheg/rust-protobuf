use protobuf::descriptor::field_descriptor_proto;

use crate::gen::scope::FieldWithContext;
use crate::gen::scope::MessageWithScope;

/// Pair of (key, value) if this message is map entry
pub(crate) fn map_entry<'a>(
    d: &'a MessageWithScope,
) -> Option<(FieldWithContext<'a>, FieldWithContext<'a>)> {
    if d.message.is_map_entry() {
        // Must be consistent with
        // DescriptorBuilder::ValidateMapEntry

        // TODO: error, not panic
        assert!(d.message.get_proto().name().ends_with("Entry"));

        assert_eq!(0, d.message.get_proto().extension.len());
        assert_eq!(0, d.message.get_proto().extension_range.len());
        assert_eq!(0, d.message.get_proto().nested_type.len());
        assert_eq!(0, d.message.get_proto().enum_type.len());

        assert_eq!(2, d.message.fields().count());
        let key = d.fields()[0].clone();
        let value = d.fields()[1].clone();

        assert_eq!("key", key.name());
        assert_eq!("value", value.name());

        assert_eq!(1, key.number());
        assert_eq!(2, value.number());

        assert_eq!(
            field_descriptor_proto::Label::LABEL_OPTIONAL,
            key.field.get_proto().label()
        );
        assert_eq!(
            field_descriptor_proto::Label::LABEL_OPTIONAL,
            value.field.get_proto().label()
        );

        Some((key, value))
    } else {
        None
    }
}
