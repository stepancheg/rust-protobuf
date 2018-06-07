use protobuf::descriptorx::FieldWithContext;
use protobuf::descriptorx::MessageWithScope;
use protobuf::descriptor::FieldDescriptorProto_Label;


/// Pair of (key, value) if this message is map entry
pub fn map_entry<'a>(d: &'a MessageWithScope) -> Option<(FieldWithContext<'a>, FieldWithContext<'a>)> {
    if d.message.get_options().get_map_entry() {
        // Must be consistent with
        // DescriptorBuilder::ValidateMapEntry

        assert!(d.message.get_name().ends_with("Entry"));

        assert_eq!(0, d.message.get_extension().len());
        assert_eq!(0, d.message.get_extension_range().len());
        assert_eq!(0, d.message.get_nested_type().len());
        assert_eq!(0, d.message.get_enum_type().len());

        assert_eq!(2, d.fields().len());
        let key = d.fields()[0].clone();
        let value = d.fields()[1].clone();

        assert_eq!("key", key.name());
        assert_eq!("value", value.name());

        assert_eq!(1, key.number());
        assert_eq!(2, value.number());

        assert_eq!(FieldDescriptorProto_Label::LABEL_OPTIONAL, key.field.get_label().unwrap());
        assert_eq!(FieldDescriptorProto_Label::LABEL_OPTIONAL, value.field.get_label().unwrap());

        Some((key, value))
    } else {
        None
    }
}
