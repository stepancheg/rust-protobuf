use crate::reflect::RuntimeTypeBox;

/// Reflective representation of field type.
pub enum RuntimeFieldType {
    /// Singular field (required, optional for proto2 or singular for proto3)
    Singular(RuntimeTypeBox),
    /// Repeated field
    Repeated(RuntimeTypeBox),
    /// Map field
    Map(RuntimeTypeBox, RuntimeTypeBox),
}
