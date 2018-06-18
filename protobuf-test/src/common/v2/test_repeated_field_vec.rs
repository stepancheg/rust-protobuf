use super::test_repeated_field_vec_pb::*;
use protobuf::RepeatedField;

#[test]
fn test_vec() {
    let basket = BasketVec::new();
    let _eggs: Vec<Egg> = basket.eggs;
    let _s: Vec<String> = basket.s;
    let _b: Vec<Vec<u8>> = basket.b;
}

#[test]
fn test_repeated_field() {
    let basket = BasketRepeatedField::new();
    let _eggs: RepeatedField<Egg> = basket.eggs;
    let _s: RepeatedField<String> = basket.s;
    let _b: RepeatedField<Vec<u8>> = basket.b;
}
