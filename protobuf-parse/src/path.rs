use std::path::is_separator;

use crate::proto_path::ProtoPath;

pub(crate) fn fs_path_to_proto_path(path: &ProtoPath) -> String {
    path.to_str()
        .chars()
        .map(|c| if is_separator(c) { '/' } else { c })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::path::fs_path_to_proto_path;
    use crate::ProtoPath;

    #[test]
    fn test_fs_path_to_proto_path() {
        assert_eq!(
            "foo.proto",
            fs_path_to_proto_path(ProtoPath::new("foo.proto").unwrap())
        );
        assert_eq!(
            "bar/foo.proto",
            fs_path_to_proto_path(ProtoPath::new("bar/foo.proto").unwrap())
        );
    }
}
