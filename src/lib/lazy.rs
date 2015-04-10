use std::mem;
use std::sync;

pub struct Lazy<T> {
    pub lock: sync::Once,
    pub ptr: *const T,
}

impl<T> Lazy<T> {
    pub fn get<F>(&'static self, init: F) -> &'static T
        where F : Fn() -> T
    {
        self.lock.call_once(|| {
            let mut vec = Vec::with_capacity(1);
            vec.push(init());
            let ptr = vec.as_mut_ptr();
            unsafe {
                mem::forget(vec);
                mem::transmute::<&Lazy<T>, &mut Lazy<T>>(self).ptr = ptr;
            }
        });
        unsafe { mem::transmute(self.ptr) }
    }
}

pub const ONCE_INIT: sync::Once = sync::ONCE_INIT;
