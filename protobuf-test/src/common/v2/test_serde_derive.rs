use super::test_serde_derive_pb::*;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Proto(TestSerde);

