use protobuf_test_common::test_serialize_deserialize_no_hex_with_dynamic;

use super::test_all_types_pb::*;

#[test]
fn test_types_singular() {
    let mut message = TestTypesSingular::new();
    message.set_double_field(19f64);
    message.set_float_field(20f32);
    message.set_int32_field(21);
    message.set_int64_field(-22);
    message.set_uint32_field(23);
    message.set_uint64_field(24);
    message.set_sint32_field(-25);
    message.set_sint64_field(26);
    message.set_fixed32_field(27);
    message.set_fixed64_field(28);
    message.set_sfixed32_field(-29);
    message.set_sfixed64_field(30);
    message.set_bool_field(true);
    message.set_string_field("thirty two".to_string());
    message.set_bytes_field([33u8, 34].to_vec());
    message.set_enum_field(SomeEnum::BLUE);
    test_serialize_deserialize_no_hex_with_dynamic(&message);
}

#[test]
fn test_types_repeated() {
    let mut message = TestTypesRepeated::new();
    message.set_double_field([19f64, 20f64].to_vec());
    message.set_float_field([20f32].to_vec());
    message.set_int32_field([21i32, -22, 23].to_vec());
    message.set_int64_field([22i64].to_vec());
    message.set_uint32_field([23u32, 24].to_vec());
    message.set_uint64_field([24u64].to_vec());
    message.set_sint32_field([25i32].to_vec());
    message.set_sint64_field([26i64, -27].to_vec());
    message.set_fixed32_field([27u32].to_vec());
    message.set_fixed64_field([28u64].to_vec());
    message.set_sfixed32_field([29i32, -30].to_vec());
    message.set_sfixed64_field([30i64].to_vec());
    message.set_bool_field([true, true].to_vec());
    message.set_string_field(vec!["thirty two".to_string(), "thirty three".to_string()]);
    message.set_bytes_field(vec![[33u8, 34].to_vec(), [35u8].to_vec()]);
    message.set_enum_field([SomeEnum::BLUE.into(), SomeEnum::GREEN.into()].to_vec());
    test_serialize_deserialize_no_hex_with_dynamic(&message);
}

#[test]
fn test_types_repeated_packed() {
    let mut message = TestTypesRepeatedPacked::new();
    message.set_double_field([19f64, 20f64].to_vec());
    message.set_float_field([20f32].to_vec());
    message.set_int32_field([21i32, -22, 23].to_vec());
    message.set_int64_field([22i64].to_vec());
    message.set_uint32_field([23u32, 24].to_vec());
    message.set_uint64_field([24u64].to_vec());
    message.set_sint32_field([-25i32, 26].to_vec());
    message.set_sint64_field([26i64, -27, 28].to_vec());
    message.set_fixed32_field([27u32].to_vec());
    message.set_fixed64_field([28u64].to_vec());
    message.set_sfixed32_field([29i32, -30].to_vec());
    message.set_sfixed64_field([30i64].to_vec());
    message.set_bool_field([true, true].to_vec());
    message.set_string_field(vec!["thirty two".to_string(), "thirty three".to_string()]);
    message.set_bytes_field(vec![[33u8, 34].to_vec(), [35u8].to_vec()]);
    message.set_enum_field([SomeEnum::BLUE.into(), SomeEnum::GREEN.into()].to_vec());
    test_serialize_deserialize_no_hex_with_dynamic(&message);
}
