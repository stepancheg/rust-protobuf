use protobuf::reflect::FileDescriptor;
use protobuf::Message;
use protobuf::MessageDyn;

/// Serialize/deserialize test for `DynamicMessage`.
pub fn serialize_and_parse_as_dynamic_and_serialize<M: Message>(m: &M) -> Box<dyn MessageDyn> {
    // Recreate generated message file descriptor as dynamic descriptor.
    let deps = M::descriptor_static().file_descriptor().deps().to_vec();
    let file_descriptor =
        FileDescriptor::new_dynamic(M::descriptor_static().file_descriptor_proto().clone(), deps);

    // Find the dynamic version of the generated message.
    let description_dynamic = file_descriptor
        .message_by_package_relative_name(M::descriptor_static().name_to_package())
        .unwrap();

    // Serialize message as bytes.
    let bytes = m.write_to_bytes().unwrap();
    // Parse it as dynamic message.
    let parsed = description_dynamic.parse_from_bytes(&bytes).unwrap();
    // Now serialize dynamic message.
    let serialized_again = parsed.write_to_bytes_dyn().unwrap();
    // And compare serialized dynamic message with serialized generated message.
    assert_eq!(bytes, serialized_again);
    parsed
}
