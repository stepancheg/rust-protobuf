use std::mem;
use sync::one;

pub struct Lazy<T> {
    pub lock: one::Once,
    pub ptr: *const T,
}

impl<T> Lazy<T> {
    pub fn get(&self, init: || -> T) -> &'static T {
        unsafe {
            self.lock.doit(|| {
                mem::transmute::<&Lazy<T>, &mut Lazy<T>>(self).ptr = mem::transmute(box init())
            });
            mem::transmute(self.ptr)
        }
    }
}

pub const ONCE_INIT: one::Once = one::ONCE_INIT;
