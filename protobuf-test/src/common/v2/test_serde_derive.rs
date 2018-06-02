use super::test_serde_derive_pb::*;
use serde_json;
use std::collections::HashMap;

#[test]
fn serialize_deserialize () {
    let mut original_data = TestSerde::new();
    original_data.set_test_enum(SerdeEnum::TEST);

    let mut map = HashMap::new();
    map.insert(5, 10);
    original_data.set_somemap(map);

    let serialized = serde_json::to_string(&original_data).unwrap();

    assert_eq!(serialized, r#"{"test_enum":"TEST","somemap":{"5":10}}"#);

    let deserialized: TestSerde = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, original_data);
}
