use keywords::*;

use protobuf::text_format::print_to_string;

fn t<F : FnMut(&mut Keywords)>(expected: &str, mut setter: F) {
    let mut m = Keywords::new();
    setter(&mut m);
    assert_eq!(&*print_to_string(&m), expected);
}

#[test]
fn test_type() {
    t("type: 1", |m| m.set_type(1));
}
