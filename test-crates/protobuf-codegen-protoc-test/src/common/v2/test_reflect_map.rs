use protobuf::reflect::ReflectValueBox;
use protobuf::MessageFull;

use super::test_reflect_map_pb::TestMap;

#[test]
fn test_map() {
    let mut message = TestMap::new();
    message
        .map_string_string
        .insert("foo".to_owned(), "bar".to_owned());
    message.map_int32_bool.insert(1, true);

    let map_string_string = TestMap::descriptor()
        .field_by_name("map_string_string")
        .unwrap();
    let map_int32_bool = TestMap::descriptor()
        .field_by_name("map_int32_bool")
        .unwrap();

    let mut reflect_message = TestMap::descriptor().new_instance();
    map_string_string.mut_map(&mut *reflect_message).insert(
        ReflectValueBox::String("foo".to_owned()),
        ReflectValueBox::String("bar".to_owned()),
    );
    map_int32_bool
        .mut_map(&mut *reflect_message)
        .insert(ReflectValueBox::I32(1), ReflectValueBox::Bool(true));

    assert!(TestMap::descriptor().eq(&message, &*reflect_message));
}
