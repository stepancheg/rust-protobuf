use std::fmt;

/// Trait implemented by all protobuf enum types.
///
/// Additionally, generated enums also implement [`EnumFull`](crate::EnumFull) trait,
/// which provides access to reflection.
pub trait Enum: Eq + Sized + Copy + fmt::Debug + Default + Send + Sync + 'static {
    /// Get enum `i32` value.
    fn value(&self) -> i32;

    /// Try to create an enum from `i32` value.
    /// Return `None` if value is unknown.
    fn from_i32(v: i32) -> Option<Self>;

    /// Get all enum values for enum type.
    fn values() -> &'static [Self];
}
