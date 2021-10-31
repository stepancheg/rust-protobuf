use std::path::is_separator;
use std::path::Path;

pub(crate) fn fs_path_to_proto_path(path: &Path) -> String {
    let path = path.to_str().expect("not a valid UTF-8");

    path.chars()
        .map(|c| if is_separator(c) { '/' } else { c })
        .collect()
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::path::fs_path_to_proto_path;

    #[test]
    fn test_fs_path_to_proto_path() {
        assert_eq!("foo.proto", fs_path_to_proto_path(Path::new("foo.proto")));
        assert_eq!(
            "bar/foo.proto",
            fs_path_to_proto_path(Path::new("bar/foo.proto"))
        );

        if cfg!(windows) {
            assert_eq!(
                "bar/foo.proto",
                fs_path_to_proto_path(Path::new("bar\\foo.proto"))
            );
        }
    }
}
