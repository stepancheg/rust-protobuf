use std::cast;
use sync::one;

pub struct Lazy<T> {
    pub lock: one::Once,
    pub ptr: *T,
}

impl<T> Lazy<T> {
    pub fn get(&self, init: || -> T) -> &'static T {
        unsafe {
            self.lock.doit(|| {
                cast::transmute_mut(self).ptr = cast::transmute(~init())
            });
            cast::transmute(self.ptr)
        }
    }
}

pub static ONCE_INIT: one::Once = one::ONCE_INIT;
