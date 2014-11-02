use std::collections;

/// anything that can be cleared
pub trait Clear {
    fn clear(&mut self);
}

impl<T> Clear for Option<T> {
    fn clear(&mut self) {
        self.take();
    }
}

mod util {
    pub fn clear_string(s: &mut String) {
        s.clear();
    }

    pub fn clear_vec<T>(v: &mut Vec<T>) {
        v.clear();
    }
}

impl Clear for String {
    fn clear(&mut self) {
        util::clear_string(self);
    }
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) {
        util::clear_vec(self);
    }
}
