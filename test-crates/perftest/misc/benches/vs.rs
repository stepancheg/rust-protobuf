// `cargo test --benches` and `#[feature(test)]` work only in nightly
#![cfg(rustc_nightly)]
#![feature(test)]

extern crate test;

use protobuf::MessageField;
use protobuf_perftest_misc::black_box;
use protobuf_perftest_misc::messages;

#[bench]
fn clone_and_drop_message_field(b: &mut test::Bencher) {
    let mut m = messages::TestMessageField::new();
    m.m1 = MessageField::some(messages::SimpleMessage::new());
    m.m2 = MessageField::some(messages::SimpleMessage::new());
    m.m3 = MessageField::some(messages::SimpleMessage::new());
    m.m4 = MessageField::some(messages::SimpleMessage::new());
    b.iter(|| {
        let clone = black_box(m.clone());
        drop(clone);
    })
}

#[bench]
fn clone_and_drop_message_field_with_option(b: &mut test::Bencher) {
    let mut m = messages::TestMessageFieldWithOption::new();
    m.m1 = Some(messages::SimpleMessage::new());
    m.m2 = Some(messages::SimpleMessage::new());
    m.m3 = Some(messages::SimpleMessage::new());
    m.m4 = Some(messages::SimpleMessage::new());
    b.iter(|| {
        let clone = black_box(m.clone());
        drop(clone);
    })
}
