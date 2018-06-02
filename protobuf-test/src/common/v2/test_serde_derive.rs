use super::test_serde_derive_pb::*;

#[derive(Serialize, Deserialize)]
struct Proto(TestSerde);

#[test]
fn test_serialize() {
    assert!(false);
}

#[test]
fn test_deserialize() {
}
