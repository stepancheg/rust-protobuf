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
            MaybeOwnedSlice::Ref(ref mut slice) => unsafe { mem::transmute(slice.as_ref()) },
            MaybeOwnedSlice::Owned(ref mut vec) => &mut *vec,
        }
    }

    #[inline]
    pub fn slice<'b>(&'b self, start: usize, end: usize) -> &'b [T] {
        &self.as_ref()[start..end]
    }
}

impl<'a, T : 'a> AsRef<[T]> for MaybeOwnedSlice<'a, T> {
    fn as_ref<'b>(&'b self) -> &'b [T] {
        match *self {
            MaybeOwnedSlice::Ref(slice) => slice,
            MaybeOwnedSlice::Owned(ref vec) => &vec,
        }
    }
}

impl<'a, T : 'a> Index<usize> for MaybeOwnedSlice<'a, T> {
    type Output = T;

    #[inline]
    fn index<'b>(&'b self, index: usize) -> &'b T {
        &self.as_ref()[index]
    }
}

impl<'a, T : 'a> IndexMut<usize> for MaybeOwnedSlice<'a, T> {
    #[inline]
    fn index_mut<'b>(&'b mut self, index: usize) -> &'b mut T {
        &mut self.as_mut_slice()[index]
    }
}
