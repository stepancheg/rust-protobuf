use protobuf::*;

use super::test_enum_values_pb::*;

#[test]
fn test_enum_values() {
    let expected = [
        TestEnumValuesEnum::UNKNOWN,
        TestEnumValuesEnum::WINTER,
        TestEnumValuesEnum::SPRING,
        TestEnumValuesEnum::SUMMER,
        TestEnumValuesEnum::AUTUMN,
    ];
    assert_eq!(expected, TestEnumValuesEnum::values());
}
