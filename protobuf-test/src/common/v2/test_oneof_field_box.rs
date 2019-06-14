use super::test_oneof_field_box_pb::*;

#[test]
fn test_without_option() {
    let inner = Inner::new();
    let _x1 = without_option::X::inner(inner);

    let s = String::new();
    let _x2 = without_option::X::s(s);
}

#[test]
fn test_without_option_recursive() {
    let w = WithoutOptionRecursive::new();
    let _x1 = without_option_recursive::X::inner(Box::new(w));

    let s = String::new();
    let _x2 = without_option_recursive::X::s(s);
}

#[test]
fn test_with_message_option() {
    let inner = Inner::new();
    let _x1 = with_message_option::X::inner(Box::new(inner));

    let s = String::new();
    let _x2 = with_message_option::X::s(s);
}

#[test]
fn test_with_field_option() {
    let inner = Inner::new();
    let _x1 = with_field_option::X::inner(inner);

    let inner = Inner::new();
    let _x2 = with_field_option::X::inner_box(Box::new(inner));
}
