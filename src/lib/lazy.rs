use std::mem;
use std::sync;
use std::boxed;

pub struct Lazy<T> {
    pub lock: sync::Once,
    pub ptr: *const T,
}

impl<T> Lazy<T> {
    pub fn get<F>(&'static self, init: F) -> &'static T
        where F : Fn() -> T
    {
        unsafe {
            self.lock.call_once(|| {
                mem::transmute::<&Lazy<T>, &mut Lazy<T>>(self).ptr = boxed::into_raw(Box::new(init()));
            });
            mem::transmute(self.ptr)
        }
    }
}

pub const ONCE_INIT: sync::Once = sync::ONCE_INIT;
