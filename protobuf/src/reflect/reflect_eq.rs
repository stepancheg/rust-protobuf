#[derive(Debug)]
pub(crate) struct ReflectEqMode {
    pub nan_equal: bool,
}

/// Special version of eq
pub(crate) trait ReflectEq {
    fn reflect_eq(&self, that: &Self, mode: &ReflectEqMode) -> bool;
}
