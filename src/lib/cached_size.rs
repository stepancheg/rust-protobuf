/// Cached size field used in generated code.
/// It is always equal to itself to simplify generated code.
/// (Generated code can use `#[derive(Eq)]`).
#[derive(Debug,Default,Clone)]
pub struct CachedSize {
    size: ::std::cell::Cell<u32>,
}

impl CachedSize {
    pub fn get(&self) -> u32 {
        self.size.get()
    }

    pub fn set(&self, size: u32) {
        self.size.set(size)
    }
}

impl PartialEq<CachedSize> for CachedSize {
    fn eq(&self, _other: &CachedSize) -> bool {
        true
    }
}

impl Eq for CachedSize {}
