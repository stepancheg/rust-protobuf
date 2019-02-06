use std::mem;

pub unsafe fn remove_lifetime_mut<A: ?Sized>(a: &mut A) -> &'static mut A {
    mem::transmute(a)
}
