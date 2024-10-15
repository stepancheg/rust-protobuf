// `cargo test --benches` and `#[feature(test)]` work only in nightly
#![cfg(rustc_nightly)]
#![feature(test)]

extern crate test;

use protobuf::well_known_types::struct_::value;
use protobuf::well_known_types::struct_::Struct;
use protobuf::well_known_types::struct_::Value;
use protobuf::Message;
use test::Bencher;

#[bench]
fn write_to_bytes(b: &mut Bencher) {
    let mut value = Value::new();
    value.kind = Some(value::Kind::NumberValue(10.0));
    let mut value2 = Value::new();
    value2.kind = Some(value::Kind::BoolValue(true));
    let mut s = Struct::new();
    s.fields.insert("foo".to_owned(), value);
    s.fields.insert("bar".to_owned(), value2);
    b.iter(|| s.write_to_bytes());
}
