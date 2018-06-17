use super::test_singular_field_option_pb::*;

#[test]
fn test_option_box() {
    let _ = WithOptionBox::new();
}

#[test]
fn test_option() {
    let _ = WithOption::new();
}

#[test]
fn test_singular_field() {
    let _ = WithSingularField::new();
}
