/// Custom attribute for element.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CustomAttr {
    pub(crate) attr: String,
}

impl CustomAttr {
    pub(crate) fn format(&self) -> String {
        self.attr.clone()
    }

    pub(crate) fn from_str(attr: &str) -> CustomAttr {
        assert!(attr.starts_with("#["));
        assert!(attr.ends_with("]"));
        CustomAttr {
            attr: attr.to_owned(),
        }
    }
}
