use super::test_repeated_field_vec_pb::*;

#[test]
fn test() {
    let basket = Basket::new();
    let _eggs: Vec<Egg> = basket.eggs;
}
