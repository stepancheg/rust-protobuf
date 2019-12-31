use std::io::Read;
use std::io::Write;
use std::process;

use super::interop_pb::*;

use protobuf::json;
use protobuf::Message;

use protobuf_test_common::*;

fn test_parse_message(m: &InteropMessageList) {
    let m_bytes = m.write_to_bytes().expect("write_to_bytes");

    let mut interop = process::Command::new("../interop/cxx/interop")
        .args(&["json-encode"])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn()
        .expect("interop");

    interop
        .stdin
        .take()
        .unwrap()
        .write_all(&m_bytes)
        .expect("write to process");

    let mut json = String::new();
    interop
        .stdout
        .take()
        .unwrap()
        .read_to_string(&mut json)
        .expect("read json");

    let exit_status = interop.wait().expect("wait_with_output");
    assert!(exit_status.success(), "{}", exit_status);

    let mut mm = InteropMessageList::new();

    json::merge_from_str(&mut mm, &json).expect("parse json");

    assert!(
        Message::reflect_eq(m, &mm),
        "{:?} != {:?}; json: {}",
        m,
        mm,
        json
    );
}

fn test_print_message(m: &InteropMessageList) {
    let m_json = json::print_to_string(m).unwrap();

    let mut interop = process::Command::new("../interop/cxx/interop")
        .args(&["json-decode"])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit())
        .spawn()
        .expect("interop");

    interop
        .stdin
        .take()
        .unwrap()
        .write_all(&m_json.as_bytes())
        .expect("write to process");

    let mut bytes = Vec::new();
    interop
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut bytes)
        .expect("read json");

    let exit_status = interop.wait().expect("wait_with_output");
    assert!(
        exit_status.success(),
        "{} when parsing: {}",
        exit_status,
        m_json
    );

    let mut mm = InteropMessageList::new();

    mm.merge_from_bytes(&bytes).expect("parse bytes");

    assert!(Message::reflect_eq(m, &mm), "{:?} != {:?}", m, mm);
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
