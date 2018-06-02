use super::test_serde_derive_pb::*;
use serde_json;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Proto(TestSerde);

#[test]
fn serialize () {
    let mut data = TestSerde::new();
    data.set_test(SerdeEnum::TEST);

    let serialized = serde_json::to_string(&data).unwrap();

    assert_eq!(serialized, "{\"test\":\"TEST\"}");
    println!("{:#?}", serialized);
}
