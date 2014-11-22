use std::mem;

pub enum MaybeOwnedSlice<'a, T : 'a> {
    Ref(&'a [T]),
    Owned(Vec<T>),
}

impl<'a, T : 'a> MaybeOwnedSlice<'a, T> {
    pub fn from_vec(vec: Vec<T>) -> MaybeOwnedSlice<'static, T> {
        MaybeOwnedSlice::Owned(vec)
    }

    pub fn from_slice(slice: &'a [T]) -> MaybeOwnedSlice<'a, T> {
        MaybeOwnedSlice::Ref(slice)
    }

    #[inline]
    pub fn as_mut_slice<'b>(&'b mut self) -> &'b mut [T] {
        match *self {
            MaybeOwnedSlice::Ref(ref mut slice) => unsafe { mem::transmute(slice.as_slice()) },
            MaybeOwnedSlice::Owned(ref mut vec) => vec.as_mut_slice(),
        }
    }

    #[inline]
    pub fn slice<'a>(&'a self, start: uint, end: uint) -> &'a [T] {
        self.as_slice().slice(start, end)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn slice_from<'a>(&'a self, start: uint) -> &'a [T] {
        self.as_slice().slice_from(start)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn slice_to<'a>(&'a self, end: uint) -> &'a [T] {
        self.as_slice().slice_to(end)
    }
}

impl<'a, T : 'a> AsSlice<T> for MaybeOwnedSlice<'a, T> {
    fn as_slice<'b>(&'b self) -> &'b [T] {
        match *self {
            MaybeOwnedSlice::Ref(slice) => slice,
            MaybeOwnedSlice::Owned(ref vec) => vec.as_slice(),
        }
    }
}

impl<'a, T : 'a> Index<uint, T> for MaybeOwnedSlice<'a, T> {
    #[inline]
    fn index<'b>(&'b self, index: &uint) -> &'b T {
        &self.as_slice()[*index]
    }
}

impl<'a, T : 'a> IndexMut<uint, T> for MaybeOwnedSlice<'a, T> {
    #[inline]
    fn index_mut<'b>(&'b mut self, index: &uint) -> &'b mut T {
        &mut self.as_mut_slice()[*index]
    }
}
