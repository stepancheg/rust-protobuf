use std::f32;
use std::f64;

use super::test_default_values_pb::*;

#[test]
fn test_default_value_simple() {
    let d = TestDefaultValues::new();
    assert_eq!(1.0, d.get_double_field());
    assert_eq!(2.0, d.get_float_field());
    assert_eq!(3, d.get_int32_field());
    assert_eq!(4, d.get_int64_field());
    assert_eq!(5, d.get_uint32_field());
    assert_eq!(6, d.get_uint64_field());
    assert_eq!(7, d.get_sint32_field());
    assert_eq!(8, d.get_sint64_field());
    assert_eq!(9, d.get_fixed32_field());
    assert_eq!(10, d.get_fixed64_field());
    assert_eq!(11, d.get_sfixed32_field());
    assert_eq!(12, d.get_sfixed64_field());
    assert_eq!(true, d.get_bool_field());
    assert_eq!("abc\n22", d.get_string_field());
    assert_eq!(b"cde\n33", d.get_bytes_field());
    assert_eq!(EnumForDefaultValue::TWO, d.get_enum_field());
    assert_eq!(EnumForDefaultValue::ONE, d.get_enum_field_without_default());
}

#[test]
fn test_default_value_extreme() {
    let d = TestExtremeDefaultValues::new();
    assert_eq!(f64::INFINITY, d.get_inf_double());
    assert_eq!(f64::NEG_INFINITY, d.get_neg_inf_double());
    assert!(d.get_nan_double().is_nan());
    assert_eq!(f32::INFINITY, d.get_inf_float());
    assert_eq!(f32::NEG_INFINITY, d.get_neg_inf_float());
    assert!(d.get_nan_float().is_nan());
    assert_eq!(
        b"\0\x01\x07\x08\x0c\n\r\t\x0b\\\'\"\xfe",
        d.get_escaped_bytes()
    );
    assert_eq!("'", d.get_quote1());
    assert_eq!("\"", d.get_quote2());
    assert_eq!(b"'", d.get_bquote1());
    assert_eq!(b"\"", d.get_bquote2());
}
