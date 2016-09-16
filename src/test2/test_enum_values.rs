use protobuf::*;

use pb_test_enum_values::*;

#[test]
fn test_enum_values() {
    let expected = [
        TestEnumValuesEnum::WINTER,
        TestEnumValuesEnum::SPRING,
        TestEnumValuesEnum::SUMMER,
        TestEnumValuesEnum::AUTUMN,
    ];
    assert_eq!(expected, TestEnumValuesEnum::values());
}
