use core::*;
use hex::*;
use descriptor;

use shrug::*;

fn test_serialize_deserialize_length_delimited<M : Message>(msg: &M) {
    let serialized_bytes = msg.write_length_delimited_to_bytes();
    let parsed = parse_length_delimited_from_bytes::<M>(serialized_bytes.as_slice());
    assert!(*msg == parsed);
}

fn test_serialize_deserialize_no_hex<M : Message>(msg: &M) {
    let serialized_bytes = msg.write_to_bytes();
    let parsed = parse_from_bytes::<M>(serialized_bytes.as_slice());
    assert!(*msg == parsed);
}

fn test_serialize_deserialize<M : Message>(hex: &str, msg: &M) {
    let expected_bytes = decode_hex(hex);
    let expected_hex = encode_hex(expected_bytes.as_slice());
    let serialized = msg.write_to_bytes();
    let serialized_hex = encode_hex(serialized.as_slice());
    assert_eq!(expected_hex, serialized_hex);
    let parsed = parse_from_bytes::<M>(expected_bytes.as_slice());
    assert!(*msg == parsed);

    assert_eq!(expected_bytes.len(), msg.serialized_size() as uint);

    test_serialize_deserialize_length_delimited(msg);
}

fn test_deserialize<M : Message>(hex: &str, msg: &M) {
    let bytes = decode_hex(hex);
    let parsed = parse_from_bytes::<M>(bytes.as_slice());
    assert!(*msg == parsed);
}

#[test]
fn test1() {
    let mut test1 = Test1::new();
    test1.set_a(150);
    test_serialize_deserialize("08 96 01", &test1);
}

#[test]
fn test2() {
    let mut test2 = Test2::new();
    test2.set_b(~"testing");
    test_serialize_deserialize("12 07 74 65 73 74 69 6e 67", &test2);
}

#[test]
fn test3() {
    let mut test1 = Test1::new();
    test1.set_a(150);
    let mut test3 = Test3::new();
    test3.set_c(test1);
    test_serialize_deserialize("1a 03 08 96 01", &test3);
}

#[test]
fn test4() {
    let mut test4 = Test4::new();
    test4.set_d(Vec::from_slice([3i32, 270, 86942]));
    test_serialize_deserialize("22 06 03 8E 02 9E A7 05", &test4);
}

#[test]
fn test_read_unpacked_expect_packed() {
    let mut test_packed_unpacked = TestPackedUnpacked::new();
    test_packed_unpacked.set_packed(Vec::new());
    test_packed_unpacked.set_unpacked(Vec::from_slice([17i32, 1000]));
    test_deserialize("20 11 20 e8 07", &test_packed_unpacked);
}

#[test]
fn test_read_packed_expect_unpacked() {
    let mut test_packed_unpacked = TestPackedUnpacked::new();
    test_packed_unpacked.set_packed(Vec::from_slice([17i32, 1000]));
    test_packed_unpacked.set_unpacked(Vec::new());
    test_deserialize("2a 03 11 e8 07", &test_packed_unpacked);
}

#[test]
fn test_empty() {
    test_serialize_deserialize("", &TestEmpty::new());
}

#[test]
#[should_fail]
fn test_write_missing_required() {
    TestRequired::new().write_to_bytes();
}

#[test]
#[should_fail]
fn test_read_missing_required() {
    parse_from_bytes::<TestRequired>([]);
}

#[test]
#[should_fail]
fn test_read_junk() {
    parse_from_bytes::<Test1>(decode_hex("00").as_slice());
}

#[test]
fn test_unknown_fields_length_delimited() {
    let mut message = TestUnknownFields::new();
    message.set_a(150);
    message.mut_unknown_fields().add_length_delimited(4, Vec::from_slice([0x10u8, 0x20, 0x30]));
    test_serialize_deserialize("08 96 01 22 03 10 20 30", &message);
}

#[test]
fn test_unknown_fields_fixed32() {
    let mut message = TestUnknownFields::new();
    message.set_a(150);
    message.mut_unknown_fields().add_fixed32(4, 0x01020304);
    message.mut_unknown_fields().add_fixed32(4, 0xA1A2A3A4);
    test_serialize_deserialize("08 96 01 25 04 03 02 01 25 A4 A3 A2 A1", &message);
}

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
    message.set_string_field(~"thirty two");
    message.set_bytes_field(Vec::from_slice([33u8, 34]));
    test_serialize_deserialize_no_hex(&message);
}

#[test]
fn test_types_repeated() {
    let mut message = TestTypesRepeated::new();
    message.set_double_field(Vec::from_slice([19f64, 20f64]));
    message.set_float_field(Vec::from_slice([20f32]));
    message.set_int32_field(Vec::from_slice([21i32, -22, 23]));
    message.set_int64_field(Vec::from_slice([22i64]));
    message.set_uint32_field(Vec::from_slice([23u32, 24]));
    message.set_uint64_field(Vec::from_slice([24u64]));
    message.set_sint32_field(Vec::from_slice([25i32]));
    message.set_sint64_field(Vec::from_slice([26i64, -27]));
    message.set_fixed32_field(Vec::from_slice([27u32]));
    message.set_fixed64_field(Vec::from_slice([28u64]));
    message.set_sfixed32_field(Vec::from_slice([29i32, -30]));
    message.set_sfixed64_field(Vec::from_slice([30i64]));
    message.set_bool_field(Vec::from_slice([true, true]));
    message.set_string_field(Vec::from_slice([StrBuf::from_str("thirty two"), StrBuf::from_str("thirty three")]));
    message.set_bytes_field(Vec::from_slice([Vec::from_slice([33u8, 34]), Vec::from_slice([35u8])]));
    test_serialize_deserialize_no_hex(&message);
}

#[test]
fn test_types_repeated_packed() {
    let mut message = TestTypesRepeatedPacked::new();
    message.set_double_field(Vec::from_slice([19f64, 20f64]));
    message.set_float_field(Vec::from_slice([20f32]));
    message.set_int32_field(Vec::from_slice([21i32, -22, 23]));
    message.set_int64_field(Vec::from_slice([22i64]));
    message.set_uint32_field(Vec::from_slice([23u32, 24]));
    message.set_uint64_field(Vec::from_slice([24u64]));
    message.set_sint32_field(Vec::from_slice([-25i32, 26]));
    message.set_sint64_field(Vec::from_slice([26i64, -27, 28]));
    message.set_fixed32_field(Vec::from_slice([27u32]));
    message.set_fixed64_field(Vec::from_slice([28u64]));
    message.set_sfixed32_field(Vec::from_slice([29i32, -30]));
    message.set_sfixed64_field(Vec::from_slice([30i64]));
    message.set_bool_field(Vec::from_slice([true, true]));
    message.set_string_field(Vec::from_slice([StrBuf::from_str("thirty two"), StrBuf::from_str("thirty three")]));
    message.set_bytes_field(Vec::from_slice([Vec::from_slice([33u8, 34]), Vec::from_slice([35u8])]));
    test_serialize_deserialize_no_hex(&message);
}

#[test]
fn test_file_descriptor_proto() {
    let p: &'static descriptor::FileDescriptorProto = file_descriptor_proto();
    assert!(p.has_name());
    assert_eq!("proto/shrug.proto", p.get_name());
}
