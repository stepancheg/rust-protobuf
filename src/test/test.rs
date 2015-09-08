use protobuf::hex::encode_hex;
use protobuf::hex::decode_hex;

use shrug::*;
use test_lite_runtime;

use protobuf::*;

fn test_serialize_deserialize_length_delimited<M : Message + MessageStatic>(msg: &M) {
    let serialized_bytes = msg.write_length_delimited_to_bytes().unwrap();
    let parsed = parse_length_delimited_from_bytes::<M>(&serialized_bytes).unwrap();
    assert!(*msg == parsed);
}

pub fn test_serialize_deserialize_no_hex<M : Message + MessageStatic>(msg: &M) {
    let serialized_bytes = msg.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<M>(&serialized_bytes).unwrap();
    assert!(*msg == parsed);
}

pub fn test_serialize_deserialize<M : Message + MessageStatic>(hex: &str, msg: &M) {
    let expected_bytes = decode_hex(hex);
    let expected_hex = encode_hex(&expected_bytes);
    let serialized = msg.write_to_bytes().unwrap();
    let serialized_hex = encode_hex(&serialized);
    assert_eq!(expected_hex, serialized_hex);
    let parsed = parse_from_bytes::<M>(&expected_bytes).unwrap();
    assert!(*msg == parsed);

    assert_eq!(expected_bytes.len(), msg.compute_size() as usize);

    test_serialize_deserialize_length_delimited(msg);
}

fn test_deserialize<M : Message + MessageStatic>(hex: &str, msg: &M) {
    let bytes = decode_hex(hex);
    let parsed = parse_from_bytes::<M>(&bytes).unwrap();
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
    test2.set_b("testing".to_owned());
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
    test4.set_d([3i32, 270, 86942].to_vec());
    test_serialize_deserialize("22 06 03 8E 02 9E A7 05", &test4);
}

#[test]
fn test_read_unpacked_expect_packed() {
    let mut test_packed_unpacked = TestPackedUnpacked::new();
    test_packed_unpacked.set_packed(Vec::new());
    test_packed_unpacked.set_unpacked([17i32, 1000].to_vec());
    test_deserialize("20 11 20 e8 07", &test_packed_unpacked);
}

#[test]
fn test_read_packed_expect_unpacked() {
    let mut test_packed_unpacked = TestPackedUnpacked::new();
    test_packed_unpacked.set_packed([17i32, 1000].to_vec());
    test_packed_unpacked.set_unpacked(Vec::new());
    test_deserialize("2a 03 11 e8 07", &test_packed_unpacked);
}

#[test]
fn test_empty() {
    test_serialize_deserialize("", &TestEmpty::new());
}

#[test]
#[should_panic]
fn test_write_missing_required() {
    TestRequired::new().write_to_bytes().unwrap();
}

#[test]
#[should_panic]
fn test_read_missing_required() {
    parse_from_bytes::<TestRequired>(&[]).unwrap();
}

#[test]
#[should_panic]
fn test_read_junk() {
    parse_from_bytes::<Test1>(&decode_hex("00")).unwrap();
}

#[test]
fn test_unknown_fields_length_delimited() {
    let mut message = TestUnknownFields::new();
    message.set_a(150);
    message.mut_unknown_fields().add_length_delimited(4, [0x10u8, 0x20, 0x30].to_vec());
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
    message.set_string_field("thirty two".to_owned());
    message.set_bytes_field([33u8, 34].to_vec());
    message.set_enum_field(TestEnumDescriptor::BLUE);
    test_serialize_deserialize_no_hex(&message);
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
    message.set_string_field(RepeatedField::from_slice(&["thirty two".to_owned(), "thirty three".to_owned()]));
    message.set_bytes_field(RepeatedField::from_slice(&[[33u8, 34].to_vec(), [35u8].to_vec()]));
    message.set_enum_field([TestEnumDescriptor::BLUE, TestEnumDescriptor::GREEN].to_vec());
    test_serialize_deserialize_no_hex(&message);
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
    message.set_string_field(RepeatedField::from_slice(&["thirty two".to_owned(), "thirty three".to_owned()]));
    message.set_bytes_field(RepeatedField::from_slice(&[[33u8, 34].to_vec(), [35u8].to_vec()]));
    message.set_enum_field([TestEnumDescriptor::BLUE, TestEnumDescriptor::GREEN].to_vec());
    test_serialize_deserialize_no_hex(&message);
}

#[test]
fn test_file_descriptor_proto() {
    let p: &'static descriptor::FileDescriptorProto = file_descriptor_proto();
    assert!(p.has_name());
    assert_eq!("proto/shrug.proto", p.get_name());
}

#[test]
fn test_default_instance() {
    let d = TestDefaultInstance::new();
    assert_eq!("", d.get_field().get_s());
}

#[test]
fn test_message_descriptor() {
    assert_eq!("TestDescriptor", TestDescriptor::new().descriptor().name());

    let d = reflect::MessageDescriptor::for_type::<TestDescriptor>();
    assert_eq!("TestDescriptor", d.name());
    assert_eq!("shrug.TestDescriptor", d.full_name());

    let mut t = TestDescriptor::new();
    t.set_stuff(55);

    let field = d.field_by_name("stuff");
    assert_eq!(55, field.get_i32(&t));
}

#[test]
fn test_enum_descriptor() {
    let d = TestEnumDescriptor::RED.enum_descriptor();
    assert_eq!("TestEnumDescriptor", d.name());
    assert_eq!("TestEnumDescriptor", reflect::EnumDescriptor::for_type::<TestEnumDescriptor>().name());
    assert_eq!("GREEN", d.value_by_name("GREEN").name());
}

#[test]
fn test_default_value() {
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
fn test_lite_runtime() {
    let mut m = test_lite_runtime::TestLiteRuntime::new();
    m.set_v(10);
    test_serialize_deserialize("08 0a", &m);

    // test it doesn't crash
    format!("{:?}", m);
}

#[test]
fn test_invalid_tag() {
    // 01 is invalid tag, because field number for that tag would be 0
    let bytes = decode_hex("01 02 03");
    let r = parse_from_bytes::<TestInvalidTag>(&bytes);
    assert!(r.is_err());
}

#[test]
fn test_truncated_no_varint() {
    // 08 is valid tag that should be followed by varint
    let bytes = decode_hex("08");
    let r = parse_from_bytes::<TestTruncated>(&bytes);
    assert!(r.is_err());
}

#[test]
fn test_truncated_middle_of_varint() {
    // 08 is field 1, wire type varint
    // 96 is non-final byte of varint
    let bytes = decode_hex("08 96");
    let r = parse_from_bytes::<TestTruncated>(&bytes);
    assert!(r.is_err());
}

#[test]
fn test_truncated_middle_of_length_delimited() {
    // 0a is field 1, wire type length delimited
    // 03 is length 3
    let bytes = decode_hex("0a 03 10");
    let r = parse_from_bytes::<TestTruncated>(&bytes);
    assert!(r.is_err());
}

#[test]
fn test_truncated_repeated_packed() {
    // 12 is field 2, wire type length delimited
    // 04 is length 4
    let bytes = decode_hex("12 04 10 20");
    let r = parse_from_bytes::<TestTruncated>(&bytes);
    assert!(r.is_err());
}
