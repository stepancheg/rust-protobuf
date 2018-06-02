use super::test_serde_derive_pb::*;
use serde_json;

#[test]
fn serialize_deserialize () {
    let mut data = TestSerde::new();
    data.set_test(SerdeEnum::TEST);

    let serialized = serde_json::to_string(&data).unwrap();

    assert_eq!(serialized, "{\"test\":\"TEST\"}");

    let deserialized: TestSerde = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, data);
}
