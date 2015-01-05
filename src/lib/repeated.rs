use std::vec;
use std::slice;
use std::default::Default;
use std::ops::Index;
use std::ops::IndexMut;
use std::fmt;

use clear::Clear;

pub struct RepeatedField<T> {
    vec: Vec<T>,
    len: uint,
}

impl<T> RepeatedField<T> {
    #[inline]
    fn len(&self) -> uint {
        self.len
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T> Clear for RepeatedField<T> {
    #[inline]
    fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T> Default for RepeatedField<T> {
    #[inline]
    fn default() -> RepeatedField<T> {
        RepeatedField {
            vec: Vec::new(),
            len: 0,
        }
    }
}

impl<T> RepeatedField<T> {
    #[inline]
    pub fn new() -> RepeatedField<T> {
        Default::default()
    }

    #[inline]
    pub fn from_vec(vec: Vec<T>) -> RepeatedField<T> {
        let len = vec.len();
        RepeatedField {
            vec: vec,
            len: len,
        }
    }

    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        let mut vec = self.vec;
        vec.truncate(self.len);
        vec
    }

    #[inline]
    pub fn capacity(&self) -> uint {
        self.vec.capacity()
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
        self.vec.as_mut_slice()
    }

    #[inline]
    pub fn slice<'a>(&'a self, start: uint, end: uint) -> &'a [T] {
        self.as_slice().slice(start, end)
    }

    #[inline]
    pub fn slice_mut<'a>(&'a mut self, start: uint, end: uint) -> &'a mut [T] {
        self.as_mut_slice().slice_mut(start, end)
    }

    #[inline]
    pub fn slice_from<'a>(&'a self, start: uint) -> &'a [T] {
        self.as_slice().slice_from(start)
    }

    #[inline]
    pub fn slice_from_mut<'a>(&'a mut self, start: uint) -> &'a mut [T] {
        self.as_mut_slice().slice_from_mut(start)
    }

    #[inline]
    pub fn slice_to<'a>(&'a self, end: uint) -> &'a [T] {
        self.as_slice().slice_to(end)
    }

    #[inline]
    pub fn slice_to_mut<'a>(&'a mut self, end: uint) -> &'a mut [T] {
        self.as_mut_slice().slice_to_mut(end)
    }

    #[inline]
    pub fn split_at<'a>(&'a self, mid: uint) -> (&'a [T], &'a [T]) {
        self.as_slice().split_at(mid)
    }

    #[inline]
    pub fn split_at_mut<'a>(&'a mut self, mid: uint) -> (&'a mut [T], &'a mut [T]) {
        self.as_mut_slice().split_at_mut(mid)
    }

    #[inline]
    pub fn tail<'a>(&'a self) -> &'a [T] {
        self.as_slice().tail()
    }

    #[inline]
    pub fn last<'a>(&'a self) -> Option<&'a T> {
        self.as_slice().last()
    }

    #[inline]
    pub fn last_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        self.as_mut_slice().last_mut()
    }

    #[inline]
    pub fn init<'a>(&'a self) -> &'a [T] {
        self.as_slice().init()
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        if self.len == self.vec.len() {
            self.vec.push(value);
        } else {
            self.vec[self.len] = value;
        }
        self.len += 1;
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.vec.truncate(self.len);
            self.len -= 1;
            self.vec.pop()
        }
    }

    #[inline]
    pub fn insert(&mut self, index: uint, value: T) {
        assert!(index <= self.len);
        self.vec.insert(index, value);
        self.len += 1;
    }

    #[inline]
    pub fn remove(&mut self, index: uint) -> T {
        assert!(index < self.len);
        self.len -= 1;
        self.vec.remove(index)
    }

    #[inline]
    pub fn truncate(&mut self, len: uint) {
        if self.len > len {
            self.len = len;
        }
    }

    #[inline]
    pub fn reverse(&mut self) {
        self.as_mut_slice().reverse()
    }

    #[inline]
    pub fn into_iter(mut self) -> vec::IntoIter<T> {
        self.vec.truncate(self.len);
        self.vec.into_iter()
    }

    #[deprecated = "use `foo[index]` instead"]
    #[inline]
    pub fn get<'a>(&'a self, index: uint) -> &'a T {
        &self.as_slice()[index]
    }

    #[deprecated = "use `foo[index] = bar` instead"]
    #[inline]
    pub fn get_mut<'a>(&'a mut self, index: uint) -> &'a mut T {
        &mut self.as_mut_slice()[index]
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.as_slice().iter()
    }

    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, T> {
        self.as_mut_slice().iter_mut()
    }

    #[inline]
    pub fn sort_by<F>(&mut self, compare: F) where F: Fn(&T, &T) -> Ordering {
        self.as_mut_slice().sort_by(compare)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.vec.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.vec.as_mut_ptr()
    }
}

impl<T : Default+Clear> RepeatedField<T> {
    pub fn push_default<'a>(&'a mut self) -> &'a mut T {
        if self.len == self.vec.len() {
            self.vec.push(Default::default());
        } else {
            self.vec[self.len].clear();
        }
        self.len += 1;
        self.last_mut().unwrap()
    }
}

impl<T : Clone> RepeatedField<T> {
    // TODO: implement to_vec()
    #[inline]
    pub fn from_slice(values: &[T]) -> RepeatedField<T> {
        RepeatedField::from_vec(values.to_vec())
    }
}

impl<T : Clone> Clone for RepeatedField<T> {
    #[inline]
    fn clone(&self) -> RepeatedField<T> {
        RepeatedField {
            vec: self.as_slice().to_vec(),
            len: self.len(),
        }
    }
}

impl<T> FromIterator<T> for RepeatedField<T> {
    #[inline]
    fn from_iter<I : Iterator<T>>(iter: I) -> RepeatedField<T> {
        RepeatedField::from_vec(FromIterator::from_iter(iter))
    }
}

impl<T : PartialEq> PartialEq for RepeatedField<T> {
    #[inline]
    fn eq(&self, other: &RepeatedField<T>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T : Eq> Eq for RepeatedField<T> {}

impl<T : PartialEq> RepeatedField<T> {
    #[inline]
    pub fn contains(&self, value: &T) -> bool {
        self.as_slice().contains(value)
    }
}

impl<T> AsSlice<T> for RepeatedField<T> {
    #[inline]
    fn as_slice<'a>(&'a self) -> &'a [T] {
        self.vec.slice_to(self.len)
    }
}

impl<T> Index<uint> for RepeatedField<T> {
    type Output = T;

    #[inline]
    fn index<'a>(&'a self, index: &uint) -> &'a T {
        &self.as_slice()[*index]
    }
}

impl<T> IndexMut<uint> for RepeatedField<T> {
    type Output = T;

    #[inline]
    fn index_mut<'a>(&'a mut self, index: &uint) -> &'a mut T {
        &mut self.as_mut_slice()[*index]
    }
}

impl<T : fmt::Show> fmt::Show for RepeatedField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}
