use protobuf::json;
use protobuf::text_format;
use protobuf::Message;

pub fn test_json_print_parse_message(s: &str, m: &dyn Message) {
    assert_eq!(s, json::print_to_string(m).expect("print_to_string"));

    test_json_parse_message(s, m);
}

pub fn test_json_parse_message(s: &str, m: &dyn Message) {
    let descriptor = m.descriptor();

    let mut new = descriptor.new_instance();
    json::merge_from_str(&mut *new, s).expect("parse");
    assert!(
        m.reflect_eq(&*new),
        "{:?} should be == {:?}",
        text_format::print_to_string(m),
        text_format::print_to_string(&*new)
    );
}

/// Print message to string, parse the string,
/// then check resulting message is equal to the original.
pub fn test_json_message(m: &dyn Message) {
    let descriptor = m.descriptor();

    let s = json::print_to_string(m).expect("print_to_string");
    let mut new = descriptor.new_instance();
    json::merge_from_str(&mut *new, &s).expect(&format!(
        "failed to parse serialized: {}; from message: {:?}",
        s, m
    ));
    assert!(
        m.reflect_eq(&*new),
        "{:?} should be == {:?}",
        text_format::print_to_string(m),
        text_format::print_to_string(&*new)
    );
}
