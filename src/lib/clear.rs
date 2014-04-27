/// anything that can be cleared
pub trait Clear {
    fn clear(&mut self) {
        fail!("TODO");
    }
}

impl<T> Clear for Option<T> {
    fn clear(&mut self) {
        self.take();
    }
}
