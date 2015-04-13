/// anything that can be cleared
pub trait Clear {
    fn clear(&mut self) -> &mut Self
        where Self: Sized;
}

impl<T> Clear for Option<T> {
    fn clear(&mut self) -> &mut Self {
        self.take();
        self
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
    fn clear(&mut self) -> &mut Self {
        util::clear_string(self);
        self
    }
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) -> &mut Self {
        util::clear_vec(self);
        self
    }
}
