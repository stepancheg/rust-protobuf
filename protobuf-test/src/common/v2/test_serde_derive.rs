use super::test_serde_derive_pb::*;
use serde_json;
use std::collections::HashMap;

#[test]
fn serialize_deserialize () {
    let mut original_data = TestSerde::new();
    original_data.set_test_enum(SerdeEnum::TEST);

    let mut map = HashMap::new();
    map.insert(5, 10);

    let mut oneof = SerdeOneOf::new();
    let pasta = Pasta::new();
    oneof.set_pasta(pasta);

    let mut repeated= vec![1, 2, 3];

    original_data.set_test_map(map);
    original_data.set_test_oneof(oneof);
    original_data.set_test_repeated(repeated);

    let serialized = serde_json::to_string(&original_data).unwrap();

    assert_eq!(serialized, r#"{"test_enum":"TEST","test_map":{"5":10},"test_oneof":{"food":{"pasta":{}}},"test_repeated":[1,2,3]}"#);

    let deserialized: TestSerde = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, original_data);
}
