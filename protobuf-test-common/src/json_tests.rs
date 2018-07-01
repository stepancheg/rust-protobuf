use protobuf::Message;
use protobuf::json;
use protobuf::text_format;

pub fn test_print_parse_message(s: &str, m: &Message) {
    let descriptor = m.descriptor();

    assert_eq!(s, json::print_to_string(m));

    let mut new = descriptor.new_instance();
    json::merge_from_str(&mut *new, s).expect("parse");
    assert!(
        descriptor.eq(m, &*new),
        "{:?} should be == {:?}",
        text_format::print_to_string(m),
        text_format::print_to_string(&*new));
}

