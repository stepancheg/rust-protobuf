use std::mem;
use std::ops::Index;
use std::ops::IndexMut;

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
    pub fn slice<'b>(&'b self, start: usize, end: usize) -> &'b [T] {
        self.as_slice().slice(start, end)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn slice_from<'b>(&'b self, start: usize) -> &'b [T] {
        self.as_slice().slice_from(start)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn slice_to<'b>(&'b self, end: usize) -> &'b [T] {
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

impl<'a, T : 'a> Index<usize> for MaybeOwnedSlice<'a, T> {
    type Output = T;

    #[inline]
    fn index<'b>(&'b self, index: &usize) -> &'b T {
        &self.as_slice()[*index]
    }
}

impl<'a, T : 'a> IndexMut<usize> for MaybeOwnedSlice<'a, T> {
    #[inline]
    fn index_mut<'b>(&'b mut self, index: &usize) -> &'b mut T {
        &mut self.as_mut_slice()[*index]
    }
}
