//! Work around Rust not yet implemented trait upcast
//! https://github.com/rust-lang/rust/issues/5665#issuecomment-31582946

use std::any::Any;


/// Hack against lack of upcasting in Rust
pub trait AsAny {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
    fn into_any_box(self: Box<Self>) -> Box<Any>;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn into_any_box(self: Box<Self>) -> Box<Any> {
        Box::new(*self)
    }
}
