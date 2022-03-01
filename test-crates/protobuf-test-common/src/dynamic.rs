use protobuf::reflect::FileDescriptor;
use protobuf::reflect::MessageDescriptor;
use protobuf::MessageDyn;
use protobuf::MessageFull;

/// Recreate generated message file descriptor as dynamic descriptor.
pub fn dynamic_descriptor_for_descriptor<M: MessageFull>() -> MessageDescriptor {
    let deps = M::descriptor_static().file_descriptor().deps().to_vec();
    let dynamic_file_descriptor =
        FileDescriptor::new_dynamic(M::descriptor_static().file_descriptor_proto().clone(), deps)
            .unwrap();

    // Find the dynamic version of the generated message.
    let dynamic_descriptor = dynamic_file_descriptor
        .message_by_package_relative_name(M::descriptor_static().name_to_package())
        .unwrap();

    // This descriptor is equivalent to `M::descriptor_static()`, but created dynamically
    // using descriptor data stored in generated files.
    dynamic_descriptor
}

/// Serialize message and parse it back as dynamic message.
pub fn recreate_as_dynamic<M: MessageFull>(m: &M) -> Box<dyn MessageDyn> {
    let bytes = m.write_to_bytes().unwrap();
    let dynamic_descriptor = dynamic_descriptor_for_descriptor::<M>();
    dynamic_descriptor.parse_from_bytes(&bytes).unwrap()
}
