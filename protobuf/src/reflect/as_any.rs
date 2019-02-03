//! Work around Rust not yet implemented trait upcast
//! https://github.com/rust-lang/rust/issues/5665#issuecomment-31582946

use std::any::Any;


/// Hack against lack of upcasting in Rust
pub trait AsAny {
    fn as_any(&self) -> &Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &Any {
        self
    }
}
