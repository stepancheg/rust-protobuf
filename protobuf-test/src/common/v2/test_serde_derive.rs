use super::test_serde_derive_pb::*;
use serde_json;
use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[test]
fn test_enum() {
    let serialized = serde_json::to_string(&AnEnum::TEST).unwrap();
    assert_eq!(serialized, r#""TEST""#);

    let deserialized: AnEnum = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, AnEnum::TEST);
}

#[test]
fn test_oneof() {
    let mut one_of = OneOf::new();
    one_of.set_rice(50);

    let serialized = serde_json::to_string(&one_of).unwrap();
    assert_eq!(serialized, r#"{"food":{"rice":50}}"#);

    let deserialized: OneOf = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, one_of);
}

#[test]
fn test_include_optional_singular_ptr_field() {
    let mut set_spf = TestSingularPtrField::new();
    let msg = SomeMessage::new();
    set_spf.set_test(msg);

    let serialized = serde_json::to_string(&set_spf).unwrap();
    assert_eq!(serialized, r#"{"test":{}}"#);

    let deserialized: TestSingularPtrField = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, set_spf);
}

#[test]
fn test_exclude_optional_singular_ptr_field() {
    let mut unset_spf = TestSingularPtrField::new();

    let serialized = serde_json::to_string(&unset_spf).unwrap();
    // TODO: Ideally we'd omit optional fields when serializing instead of setting to `null`.
    // so this test would be: r#"{}"#;
    assert_eq!(serialized, r#"{"test":null}"#);

    let deserialized: TestSingularPtrField = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, unset_spf);
}

#[test]
fn test_repeated() {
    let mut repeated = Repeated::new();
    repeated.set_test_repeated(vec![1, 2, 3]);

    let serialized = serde_json::to_string(&repeated).unwrap();
    assert_eq!(serialized, r#"{"test_repeated":[1,2,3]}"#);

    let deserialized: Repeated = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, repeated);
}

#[test]
fn test_map() {
    let mut map = TestSerdeMap::new();
    let mut hash = HashMap::new();
    hash.insert(1, 2);
    map.set_test_map(hash);

    let serialized = serde_json::to_string(&map).unwrap();
    assert_eq!(serialized, r#"{"test_map":{"1":2}}"#);

    let deserialized: TestSerdeMap = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, map);
}
