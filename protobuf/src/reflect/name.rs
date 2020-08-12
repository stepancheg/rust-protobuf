pub(crate) fn append_path(name: &mut String, append: &str) {
    if !append.is_empty() {
        if !name.is_empty() {
            name.push('.');
        }
        name.push_str(append);
    }
}

pub(crate) fn concat_paths(a: &str, b: &str) -> String {
    if a.is_empty() {
        b.to_owned()
    } else if b.is_empty() {
        b.to_owned()
    } else {
        format!("{}.{}", a, b)
    }
}

pub(crate) fn compute_full_name(package: &str, path_to_package: &str, name: &str) -> String {
    assert!(!name.is_empty());

    let mut full_name = package.to_owned();
    append_path(&mut full_name, path_to_package);
    append_path(&mut full_name, name);
    full_name
}
