use protobuf::Message;

use super::test_ext_pb::*;

#[test]
fn test_get() {
    let descriptor = MyMessage::descriptor_static();
    let message = descriptor.get_proto().get_options();
    assert_eq!(10.5, exts::double_field.get(message).unwrap_or_default());
    assert_eq!(-8.5, exts::float_field.get(message).unwrap_or_default());
    assert_eq!(-3, exts::int32_field.get(message).unwrap_or_default());
    assert_eq!(-13, exts::int64_field.get(message).unwrap_or_default());
    assert_eq!(-4, exts::sint32_field.get(message).unwrap_or_default());
    assert_eq!(-14, exts::sint64_field.get(message).unwrap_or_default());
    assert_eq!(5, exts::uint32_field.get(message).unwrap_or_default());
    assert_eq!(15, exts::uint64_field.get(message).unwrap_or_default());
    assert_eq!(6, exts::fixed32_field.get(message).unwrap_or_default());
    assert_eq!(16, exts::fixed64_field.get(message).unwrap_or_default());
    assert_eq!(7, exts::sfixed32_field.get(message).unwrap_or_default());
    assert_eq!(-17, exts::sfixed64_field.get(message).unwrap_or_default());
    assert_eq!(true, exts::bool_field.get(message).unwrap_or_default());
    assert_eq!("Hello world!", exts::string_field.get(message).unwrap_or_default());
    if false {
        // TODO: only implemented in `protoc`-based codegen
        assert_eq!(TestEnum::RED, exts::enum_field.get(message).unwrap_or_default());
        assert_eq!(22, exts::message_field.get(message).unwrap().get_n());
    }
}
