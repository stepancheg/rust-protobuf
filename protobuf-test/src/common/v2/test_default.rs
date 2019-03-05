use super::test_default_pb::*;

#[test]
fn test_default_for_amp_message() {
    let none: Option<&TestDefault> = None;
    assert_eq!(0, none.unwrap_or_default().get_i());
}
