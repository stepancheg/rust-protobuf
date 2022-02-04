#![allow(dead_code)] // TODO: use it

use protobuf_parse::ProtobufAbsPath;
use protobuf_parse::ProtobufRelPath;

use crate::Customize;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PathMatcher {
    Prefix(ProtobufAbsPath),
    Middle(ProtobufRelPath),
}

impl PathMatcher {
    pub(crate) fn new(path: &str) -> PathMatcher {
        if let Some(path) = ProtobufAbsPath::try_new(path) {
            PathMatcher::Prefix(path)
        } else {
            PathMatcher::Middle(ProtobufRelPath::new(path))
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct CustomizeByPath {
    map: Vec<(PathMatcher, Customize)>,
}

impl CustomizeByPath {
    pub(crate) fn apply_one(&self, path: &ProtobufAbsPath) -> (Customize, CustomizeByPath) {
        let mut customize = Customize::default();
        let mut map = Vec::with_capacity(self.map.len());
        for (matcher, next_customize) in &self.map {
            match matcher {
                PathMatcher::Prefix(prefix) => {
                    if path == prefix {
                        customize.update_with(next_customize);
                    } else if prefix.starts_with(path) {
                        map.push((matcher.clone(), next_customize.clone()));
                    } else {
                        // discard this matcher
                    }
                }
                PathMatcher::Middle(middle) => {
                    if path.ends_with(middle) {
                        customize.update_with(next_customize);
                    } else {
                        map.push((matcher.clone(), next_customize.clone()));
                    }
                }
            }
        }
        (customize, CustomizeByPath { map })
    }
}

#[cfg(test)]
mod test {
    use protobuf_parse::ProtobufAbsPath;

    use crate::customize::by_path::CustomizeByPath;
    use crate::customize::by_path::PathMatcher;
    use crate::Customize;

    fn one(matcher: &str, expose_fields: bool) -> CustomizeByPath {
        CustomizeByPath {
            map: vec![(
                PathMatcher::new(matcher),
                Customize {
                    expose_fields: Some(expose_fields),
                    ..Customize::default()
                },
            )],
        }
    }

    #[test]
    fn apply_one_abs() {
        let by_path = one(".foo.bar", true);
        let (cu, rem) = by_path.apply_one(&ProtobufAbsPath::new(".foo.bar"));
        assert_eq!(
            Customize {
                expose_fields: Some(true),
                ..Customize::default()
            },
            cu
        );
        assert_eq!(CustomizeByPath::default(), rem);
    }

    #[test]
    fn apply_one_rel() {
        let by_path = one("bar", true);
        let (cu, rem) = by_path.apply_one(&ProtobufAbsPath::new(".foo.bar"));
        assert_eq!(
            Customize {
                expose_fields: Some(true),
                ..Customize::default()
            },
            cu
        );
        assert_eq!(CustomizeByPath::default(), rem);
    }
}
