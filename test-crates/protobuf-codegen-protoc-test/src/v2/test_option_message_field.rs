use protobuf::*;

use super::test_option_message_field_file_option_pb::*;
use super::test_option_message_field_pb::*;

#[test]
fn test_option_message_field_field_option() {
    let msg = TestOptionMessageFieldFieldOption::default_instance();
    let _v: &Option<Msg> = &msg.v;
    let _p: &MessageField<Msg> = &msg.p;
    let _b: &Option<Vec<u8>> = &msg.b;
}

#[test]
fn test_option_message_field_message_option() {
    let msg = TestOptionMessageFieldMessageOption::default_instance();
    let _v: &Option<Msg> = &msg.v;
    let _vv: &Option<Msg> = &msg.vv;
}

#[test]
fn test_option_message_field_file_option() {
    let msg = TestOptionMessageFieldFileOption::default_instance();
    let _v: &Option<Msg> = &msg.v;
    let _vv: &Option<Msg> = &msg.vv;
}
