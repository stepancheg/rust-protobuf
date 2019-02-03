//! Work around Rust not yet implemented trait upcast
//! https://github.com/rust-lang/rust/issues/5665#issuecomment-31582946

use std::any::TypeId;


/// Hack against lack of upcasting in Rust
pub trait AsAny {
    // TODO: replace with Any::type_id after 1.34
    fn get_type_id(&self) -> TypeId;
}

impl<T: 'static> AsAny for T {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
