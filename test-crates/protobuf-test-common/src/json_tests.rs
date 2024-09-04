use protobuf::reflect::ReflectEqMode;
use protobuf::text_format;
use protobuf::MessageDyn;

pub fn test_json_print_parse_message(s: &str, m: &dyn MessageDyn) {
    assert_eq!(
        s,
        protobuf_json_mapping::print_to_string(m).expect("print_to_string")
    );

    test_json_parse_message(s, m);
}

pub fn test_json_parse_message(s: &str, m: &dyn MessageDyn) {
    let descriptor = m.descriptor_dyn();

    let mut new = descriptor.new_instance();
    protobuf_json_mapping::merge_from_str(&mut *new, s).expect("parse");
    assert!(
        m.reflect_eq_dyn(&*new, &ReflectEqMode::nan_equal()),
        "{:?} should be == {:?}",
        text_format::print_to_string(m),
        text_format::print_to_string(&*new)
    );
}

/// Print message to string, parse the string,
/// then check resulting message is equal to the original.
pub fn test_json_message(m: &dyn MessageDyn) {
    let descriptor = m.descriptor_dyn();

    let s = protobuf_json_mapping::print_to_string(m).expect("print_to_string");
    let mut new = descriptor.new_instance();
    protobuf_json_mapping::merge_from_str(&mut *new, &s)
        .unwrap_or_else(|_| panic!("failed to parse serialized: {}; from message: {:?}", s, m));
    assert!(
        m.reflect_eq_dyn(&*new, &ReflectEqMode::nan_equal()),
        "{:?} should be == {:?}",
        text_format::print_to_string(m),
        text_format::print_to_string(&*new)
    );
}
