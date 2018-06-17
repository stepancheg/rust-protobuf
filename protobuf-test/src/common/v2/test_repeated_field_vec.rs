use super::test_repeated_field_vec_pb::*;
use protobuf::RepeatedField;

#[test]
fn test_vec() {
    let basket = BasketVec::new();
    let _eggs: Vec<Egg> = basket.eggs;
}

#[test]
fn test_repeated_field() {
    let basket = BasketRepeatedField::new();
    let _eggs: RepeatedField<Egg> = basket.eggs;
}
