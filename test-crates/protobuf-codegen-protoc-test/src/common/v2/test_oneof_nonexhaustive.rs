use protobuf::OneofFull;
use protobuf_test_common::*;

use super::test_oneof_nonexhaustive_pb::*;

#[test]
fn test_oneof_nonexhaustive_disabled() {
    use message_with_oneof_nonexhaustive_disabled::One;
    match MessageWithOneofNonexhaustiveDisabled::default().one {
        None => (),
        Some(one) => match one {
            One::FirstField(_) | One::SecondField(_) => (),
        },
    }
}
