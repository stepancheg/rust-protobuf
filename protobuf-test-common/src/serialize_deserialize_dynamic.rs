use protobuf::Message;
use protobuf::MessageDyn;

use crate::dynamic_descriptor_for_descriptor;
use crate::hex::encode_hex;

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
