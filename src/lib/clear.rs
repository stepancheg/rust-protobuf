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

// work around name collision
fn collection_clear<C : collections::Mutable>(c: &mut C) {
    c.clear();
}

impl Clear for String {
    fn clear(&mut self) {
        collection_clear(self);
    }
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) {
        collection_clear(self);
    }
}
