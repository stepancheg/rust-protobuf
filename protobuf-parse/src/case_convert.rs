// copy-paste from Google Protobuf
// must be kept in sync with Google for JSON interop
#[doc(hidden)]
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

#[doc(hidden)]
pub fn snake_case(input: &str) -> String {
    let mut result = String::new();

    let mut last_lower = false;

    for c in input.chars() {
        if c.is_ascii_uppercase() && last_lower {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
        last_lower = c.is_lowercase();
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!("FooBarBazQuxQUUX", camel_case("foo_barBaz_QuxQUUX"));
        assert_eq!("FooBarBazQuxQUUX", camel_case("Foo_barBaz_QuxQUUX"));
    }

    #[test]
    fn test_snake_case() {
        assert_eq!("foo_bar_baz_qux_quux", snake_case("foo_barBaz_QuxQUUX"));
        assert_eq!("foo_bar_baz_qux_quux", snake_case("Foo_barBaz_QuxQUUX"));
    }
}
