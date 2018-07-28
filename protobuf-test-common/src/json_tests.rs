use protobuf::Message;
use protobuf::json;
use protobuf::text_format;

pub fn test_json_print_parse_message(s: &str, m: &Message) {
    assert_eq!(s, json::print_to_string(m));

    test_json_parse_message(s, m);
}

pub fn test_json_parse_message(s: &str, m: &Message) {
    let descriptor = m.descriptor();

    let mut new = descriptor.new_instance();
    json::merge_from_str(&mut *new, s).expect("parse");
    assert!(
        descriptor.deep_eq(m, &*new),
        "{:?} should be == {:?}",
        text_format::print_to_string(m),
        text_format::print_to_string(&*new));
}

