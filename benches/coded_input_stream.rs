#![feature(test)]

extern crate test;
extern crate protobuf;

use protobuf::stream::CodedInputStream;

use self::test::Bencher;

#[bench]
fn bench_read_byte(b: &mut Bencher) {
    let v = test::black_box(vec![17; 10000]);
    b.iter(|| {
        let mut is = CodedInputStream::from_bytes(&v);
        while !is.eof().unwrap() {
            test::black_box(is.read_raw_byte().unwrap());
        }
    });
}

