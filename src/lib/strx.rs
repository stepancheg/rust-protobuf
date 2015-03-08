pub fn remove_to<'s>(s: &'s str, c: char) -> &'s str {
    match s.rfind(c) {
        Some(pos) => &s[pos + 1 ..],
        None => s
    }
}

#[allow(dead_code)]
pub fn remove_from_last<'s>(s: &'s str, c: char) -> &'s str {
    match s.rfind(c) {
        Some(pos) => &s[..pos],
        None => s,
    }
}

pub fn remove_suffix<'s>(s: &'s str, suffix: &str) -> &'s str {
    if !s.ends_with(suffix) {
        panic!();
    }
    &s[.. s.len() - suffix.len()]
}

pub fn remove_prefix<'s>(s: &'s str, prefix: &str) -> &'s str {
    if !s.starts_with(prefix) {
        panic!();
    }
    &s[prefix.len()..]
}

#[cfg(test)]
mod test {

    use super::remove_to;
    use super::remove_from_last;
    use super::remove_prefix;
    use super::remove_suffix;

    #[test]
    fn test_remove_to() {
        assert_eq!("aaa", remove_to("aaa", '.'));
        assert_eq!("bbb", remove_to("aaa.bbb", '.'));
        assert_eq!("ccc", remove_to("aaa.bbb.ccc", '.'));
    }

    #[test]
    fn test_remove_from_last() {
        assert_eq!("aaa", remove_from_last("aaa", '.'));
        assert_eq!("aaa", remove_from_last("aaa.bbb", '.'));
        assert_eq!("aaa.bbb", remove_from_last("aaa.bbb.ccc", '.'));
    }

    #[test]
    fn test_remove_prefix() {
        assert_eq!("aaa", remove_prefix("bbbaaa", "bbb"));
    }

    #[test]
    #[should_fail]
    fn test_remove_prefix_fail() {
        remove_prefix("aaa", "bbb");
    }

    #[test]
    fn test_remove_suffix() {
        assert_eq!("bbb", remove_suffix("bbbaaa", "aaa"));
    }

    #[test]
    #[should_fail]
    fn test_remove_suffix_fail() {
        remove_suffix("aaa", "bbb");
    }
}
