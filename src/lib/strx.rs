pub fn remove_to<'s>(s: &'s str, c: char) -> &'s str {
    match s.rfind(c) {
        Some(pos) => &s[(pos + 1)..],
        None => s
    }
}

pub fn remove_suffix<'s>(s: &'s str, suffix: &str) -> &'s str {
    if !s.ends_with(suffix) {
        panic!();
    }
    &s[..(s.len() - suffix.len())]
}

#[cfg(test)]
mod test {

    use super::remove_to;
    use super::remove_suffix;

    #[test]
    fn test_remove_to() {
        assert_eq!("aaa", remove_to("aaa", '.'));
        assert_eq!("bbb", remove_to("aaa.bbb", '.'));
        assert_eq!("ccc", remove_to("aaa.bbb.ccc", '.'));
    }

    #[test]
    #[should_panic]
    fn test_remove_prefix_fail() {
        remove_prefix("aaa", "bbb");
    }

    #[test]
    fn test_remove_suffix() {
        assert_eq!("bbb", remove_suffix("bbbaaa", "aaa"));
    }

    #[test]
    #[should_panic]
    fn test_remove_suffix_fail() {
        remove_suffix("aaa", "bbb");
    }
}
