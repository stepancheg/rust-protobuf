pub(crate) const EXPR_NONE: &str = "::std::option::Option::None";
pub(crate) const EXPR_VEC_NEW: &str = "::std::vec::Vec::new()";

pub(crate) fn expr_vec_with_capacity(capacity: &str) -> String {
    format!("::std::vec::Vec::with_capacity({})", capacity)
}
