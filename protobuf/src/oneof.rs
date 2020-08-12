use crate::reflect::OneofDescriptor;

/// Trait implemented by all oneof types in generated code.
pub trait Oneof {
    /// Descriptor object for this oneof.
    fn descriptor() -> OneofDescriptor {
        unimplemented!("TODO")
    }
}
