/// Special version of eq, which
/// * considers `NaN` values equal
/// * panics if parameters have different types at runtime
///   (e. g. `ReflectRepeated` of `u32` and `u64` must not be compared.
pub(crate) trait ReflectDeepEq {
    fn reflect_deep_eq(&self, that: &Self) -> bool;
}
