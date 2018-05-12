use super::test_generate_accessors_pb::*;

#[test]
fn test() {
    // Check accessors are generated
    WithAccessors::new().get_f();
    WithAccessors::new().set_i(10);

    // Check that field is public
    // even if it's not requested explicitly
    WithoutAccessors::new().f;
}
