// `cargo test --benches` and `#[feature(test)]` work only in nightly
#![cfg(rustc_nightly)]
#![feature(test)]

extern crate test;

use protobuf::well_known_types::value;
use protobuf::well_known_types::Struct;
use protobuf::well_known_types::Value;
use protobuf::MessageFull;
use test::Bencher;

#[bench]
fn write_to_bytes(b: &mut Bencher) {
    let mut value = Value::new();
    value.kind = Some(value::Kind::number_value(10.0));
    let mut value2 = Value::new();
    value2.kind = Some(value::Kind::bool_value(true));
    let mut s = Struct::new();
    s.fields.insert("foo".to_owned(), value);
    s.fields.insert("bar".to_owned(), value2);
    b.iter(|| s.write_to_bytes());
}
