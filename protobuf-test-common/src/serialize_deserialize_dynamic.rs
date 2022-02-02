use protobuf::reflect::FileDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf::Message;
use protobuf::MessageDyn;

use crate::hex::encode_hex;

/// Recreate generated message file descriptor as dynamic descriptor.
fn dynamic_descriptor_for_descriptor<M: Message>() -> MessageDescriptor {
    let deps = M::descriptor_static().file_descriptor().deps().to_vec();
    let dynamic_file_descriptor =
        FileDescriptor::new_dynamic(M::descriptor_static().file_descriptor_proto().clone(), deps);

    // Find the dynamic version of the generated message.
    let dynamic_descriptor = dynamic_file_descriptor
        .message_by_package_relative_name(M::descriptor_static().name_to_package())
        .unwrap();

    // This descriptor is equivalent to `M::descriptor_static()`, but created dynamically
    // using descriptor data stored in generated files.
    dynamic_descriptor
}

/// Serialize/deserialize test for `DynamicMessage`.
pub fn serialize_and_parse_as_dynamic_and_serialize<M: Message>(m: &M) -> Box<dyn MessageDyn> {
    // Find the dynamic version of the generated message.
    let description_dynamic = dynamic_descriptor_for_descriptor::<M>();

    // Serialize message as bytes.
    let bytes = m.write_to_bytes().unwrap();
    // Parse it as dynamic message.
    let parsed = description_dynamic.parse_from_bytes(&bytes).unwrap();
    // Now serialize dynamic message.
    let serialized_again = parsed.write_to_bytes_dyn().unwrap();
    // And compare serialized dynamic message with serialized generated message.
    assert_eq!(
        encode_hex(&bytes),
        encode_hex(&serialized_again),
        "serialized({}) != serialized({})",
        m,
        parsed
    );
    parsed
}
