// copy-paste from Google Protobuf
pub fn camel_case(input: &str) -> String {
    let mut capitalize_next = true;
    let mut result = String::new();
    result.reserve(input.len());

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!("FooBarBaz", camel_case("foo_barBaz"));
    }
}
