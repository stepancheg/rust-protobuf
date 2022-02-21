use protobuf::json;
use protobuf::reflect::ReflectEqMode;
use protobuf::Message;
use protobuf_test_common::*;

use super::interop_pb::*;
use crate::interop;

fn test_parse_message(m: &InteropMessageList) {
    let json = interop::interop_json_encode_typed(m);

    let mut mm = InteropMessageList::new();

    json::merge_from_str(&mut mm, &json).expect("parse json");

    assert!(
        Message::reflect_eq(m, &mm, &ReflectEqMode::nan_equal()),
        "{:?} != {:?}; json: {}",
        m,
        mm,
        json
    );
}

fn test_print_message(m: &InteropMessageList) {
    let m_json = json::print_to_string(m).unwrap();

    let mm = interop::interop_json_decode_typed(&m_json);

    assert!(
        Message::reflect_eq(m, &mm, &ReflectEqMode::nan_equal()),
        "{:?} != {:?}",
        m,
        mm
    );
}

#[test]
fn parse_empty() {
    test_parse_message(&InteropMessageList::new());
}

#[test]
fn parse_random() {
    for m in special_messages_typed::<InteropMessage>() {
        let mut l = InteropMessageList::new();
        l.ts.push(m);
        test_parse_message(&l);
        // This slow test is equivalent to the fast test below.
        // Do just one iteration as smoke test.
        // `break` statement can be commented out for easier debugging.
        break;
    }

    let mut l = InteropMessageList::new();
    l.ts = special_messages_typed().into();
    test_parse_message(&l);
}

#[test]
fn print_empty() {
    test_print_message(&InteropMessageList::new());
}

#[test]
fn print_random() {
    for m in special_messages_typed::<InteropMessage>() {
        let mut l = InteropMessageList::new();
        l.ts.push(m);
        test_print_message(&l);
        // This slow test is equivalent to the fast test below.
        // Do just one iteration as smoke test.
        // `break` statement can be commented out for easier debugging.
        break;
    }

    let mut l = InteropMessageList::new();
    l.ts = special_messages_typed().into();
    test_print_message(&l);
}
