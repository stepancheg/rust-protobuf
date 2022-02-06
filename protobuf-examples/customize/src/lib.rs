include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

#[cfg(test)]
mod test {
    use crate::customize_example::Fruit;

    #[test]
    fn test() {
        let mut fruit = Fruit::new();
        fruit.set_name("Orange".to_owned());
        fruit.set_weight(1.5);

        // Serde works.
        // Note rust-protobuf has built in support for JSON,
        // which follows protobuf-JSON serialization more correctly than default serde-json.
        // This example here is for the demonstration of generation of custom derives.
        let json = serde_json::to_string(&fruit).unwrap();
        assert_eq!("{\"name\":\"Orange\",\"weight\":1.5}", json);
    }
}
