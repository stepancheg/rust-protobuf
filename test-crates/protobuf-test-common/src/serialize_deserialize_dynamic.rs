use protobuf::MessageDyn;
use protobuf::MessageFull;

use crate::dynamic_descriptor_for_descriptor;
use crate::hex::encode_hex;
use crate::recreate_as_dynamic;

/// Message comparison test for dynamic messages.
/// - serialize message
/// - deserialize message as dynamic
/// - serialize dynamic message
/// - compare bytes output of generated and dynamic messages
pub fn serialize_then_parse_as_dynamic_then_serialize<M: MessageFull>(
    m: &M,
) -> Box<dyn MessageDyn> {
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

/// Message comparison test for dynamic message.
/// - serialize message
/// - parse it as dynamic message
/// - serialize dynamic message
/// - parse serialized dynamic message
/// - compare dynamic messages
pub fn serialize_then_parse_as_dynamic_and_serialize_and_parse<M: MessageFull>(m: &M) {
    let parsed_1 = recreate_as_dynamic(m);
    let bytes_1 = parsed_1.write_to_bytes_dyn().unwrap();
    let parsed_2 = parsed_1
        .descriptor_dyn()
        .parse_from_bytes(&bytes_1)
        .unwrap();
    assert_eq!(&parsed_1, &parsed_2);
}
