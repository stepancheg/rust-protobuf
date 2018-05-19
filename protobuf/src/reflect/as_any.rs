//! Work around Rust not yet implemented trait upcast
//! https://github.com/rust-lang/rust/issues/5665#issuecomment-31582946

use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
    fn into_any_box(self: Box<Self>) -> Box<Any>;
    fn set_from_any(&mut self, any: Box<Any>);
}

impl<T : 'static> AsAny for T {
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }

    fn into_any_box(self: Box<Self>) -> Box<Any> {
        Box::new(*self)
    }

    fn set_from_any(&mut self, any: Box<Any>) {
        *self = *any.downcast().expect("wrong_type");
    }
}

