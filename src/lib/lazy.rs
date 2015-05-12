use std::mem;
use std::sync;

pub struct Lazy<T> {
    pub lock: sync::Once,
    pub ptr: *const T,
}

impl<T> Lazy<T> {
    pub fn get<F>(&'static mut self, init: F) -> &'static T
        where F : FnOnce() -> T
    {
        self.lock.call_once(|| {
            unsafe {
                self.ptr = mem::transmute(Box::new(init()));
            }
        });
        unsafe { &*self.ptr }
    }
}

pub const ONCE_INIT: sync::Once = sync::ONCE_INIT;
