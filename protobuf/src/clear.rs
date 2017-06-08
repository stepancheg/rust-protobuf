#[cfg(feature = "bytes")]
use bytes::Bytes;

/// anything that can be cleared
pub trait Clear {
    fn clear(&mut self);
}

impl<T> Clear for Option<T> {
    fn clear(&mut self) {
        self.take();
    }
}

impl Clear for String {
    fn clear(&mut self) {
        String::clear(self);
    }
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) {
        Vec::clear(self);
    }
}

#[cfg(feature = "bytes")]
impl Clear for Bytes {
    fn clear(&mut self) {
        Bytes::clear(self);
    }
}
