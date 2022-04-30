use crate::gen::inside::protobuf_crate_path;
use crate::gen::rust_types_values::RustType;
use crate::Customize;

/// Optional fields can be stored are `Option<T>` or `SingularPtrField<T>`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OptionKind {
    /// Field is `Option<T>`
    Option,
    /// Field is `SingularPtrField<T>`
    MessageField,
}

impl OptionKind {
    pub(crate) fn wrap_element(&self, element_type: RustType) -> RustType {
        let element_type = Box::new(element_type);
        match self {
            OptionKind::Option => RustType::Option(element_type),
            OptionKind::MessageField => RustType::MessageField(element_type),
        }
    }

    // Type of `as_ref()` operation
    pub(crate) fn as_ref_type(&self, element_type: RustType) -> RustType {
        match self {
            OptionKind::Option => RustType::Option(Box::new(element_type.ref_type())),
            OptionKind::MessageField => RustType::MessageField(Box::new(element_type.ref_type())),
        }
    }

    fn _as_option_ref(&self, v: &str) -> String {
        match self {
            OptionKind::Option | OptionKind::MessageField => format!("{}.as_ref()", v),
        }
    }

    pub(crate) fn unwrap_or_else(&self, what: &str, default_value: &str) -> String {
        match self {
            _ => format!("{}.unwrap_or_else(|| {})", what, default_value),
        }
    }

    pub(crate) fn unwrap_ref_or_else(&self, what: &str, default_value: &str) -> String {
        match self {
            _ => format!("{}.unwrap_or_else(|| {})", what, default_value),
        }
    }

    pub(crate) fn wrap_value(&self, value: &str, customize: &Customize) -> String {
        match self {
            OptionKind::Option => format!("::std::option::Option::Some({})", value),
            OptionKind::MessageField => format!(
                "{}::MessageField::some({})",
                protobuf_crate_path(customize),
                value
            ),
        }
    }
}
