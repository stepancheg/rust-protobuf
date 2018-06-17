use super::test_singular_field_option_pb::*;
use protobuf::SingularPtrField;

#[test]
fn test_option_box() {
    let w = WithOptionBox::new();
    let _inner: &Option<Box<Inner>> = &w.inner;
}

#[test]
fn test_option() {
    let w = WithOption::new();
    let _inner: &Option<Inner> = &w.inner;
}

#[test]
fn test_singular_ptr_field() {
    let w = WithSingularField::new();
    let _inner: &SingularPtrField<Inner> = &w.inner;
}
