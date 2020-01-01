// `cargo test --benches` and `#[feature(test)]` work only in nightly
#![cfg(rustc_nightly)]
#![cfg(feature = "bytes")]
#![feature(test)]

extern crate bytes;
extern crate test;

extern crate protobuf;

extern crate perftest_bytes;

use std::fmt::Write;

use bytes::Bytes;

use perftest_bytes::messages;
use protobuf::Message;

fn make_string_of_len(len: usize) -> String {
    let mut s = String::new();
    for i in 0..len {
        write!(s, "{}", i % 10).expect("unreachable");
    }
    s
}

fn make_repeated(len: usize) -> Vec<u8> {
    let mut m = messages::TestMessage::new();

    for i in 0..100 {
        m.sr.push(make_string_of_len(i % len));
        m.br.push(make_string_of_len((i + len / 2) % len).into_bytes());
    }

    m.write_to_bytes().expect("write")
}

#[bench]
fn parse_repeated_small_regular(b: &mut test::Bencher) {
    let bs = make_repeated(30);
    b.iter(|| protobuf::parse_from_bytes::<messages::TestMessage>(&bs).expect("parse"))
}

#[bench]
fn parse_repeated_small_bytes(b: &mut test::Bencher) {
    let bs = Bytes::from(make_repeated(30));
    b.iter(|| {
        protobuf::parse_from_carllerche_bytes::<messages::TestMessageWithBytes>(&bs).expect("parse")
    })
}

#[bench]
fn parse_repeated_medium_regular(b: &mut test::Bencher) {
    let bs = make_repeated(300);
    b.iter(|| protobuf::parse_from_bytes::<messages::TestMessage>(&bs).expect("parse"))
}

#[bench]
fn parse_repeated_medium_bytes(b: &mut test::Bencher) {
    let bs = Bytes::from(make_repeated(300));
    b.iter(|| {
        protobuf::parse_from_carllerche_bytes::<messages::TestMessageWithBytes>(&bs).expect("parse")
    })
}

#[bench]
fn parse_repeated_large_regular(b: &mut test::Bencher) {
    let bs = make_repeated(3000);
    b.iter(|| protobuf::parse_from_bytes::<messages::TestMessage>(&bs).expect("parse"))
}

#[bench]
fn parse_repeated_large_bytes(b: &mut test::Bencher) {
    let bs = Bytes::from(make_repeated(3000));
    b.iter(|| {
        protobuf::parse_from_carllerche_bytes::<messages::TestMessageWithBytes>(&bs).expect("parse")
    })
}

#[bench]
fn parse_repeated_huge_regular(b: &mut test::Bencher) {
    let bs = make_repeated(30000);
    b.iter(|| protobuf::parse_from_bytes::<messages::TestMessage>(&bs).expect("parse"))
}

#[bench]
fn parse_repeated_huge_bytes(b: &mut test::Bencher) {
    let bs = Bytes::from(make_repeated(30000));
    b.iter(|| {
        protobuf::parse_from_carllerche_bytes::<messages::TestMessageWithBytes>(&bs).expect("parse")
    })
}
