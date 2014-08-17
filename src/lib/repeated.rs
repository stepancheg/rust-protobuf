use std::vec;
use std::slice;
use std::default::Default;
use std::fmt;

use clear::Clear;

pub struct RepeatedField<T> {
    vec: Vec<T>,
    len: uint,
}

impl<T> Collection for RepeatedField<T> {
    #[inline]
    fn len(&self) -> uint {
        self.len
    }
}

impl<T> Mutable for RepeatedField<T> {
    #[inline]
    fn clear(&mut self) {
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
        self.vec.mut_slice_to(self.len)
    }

    #[inline]
    pub fn slice<'a>(&'a self, start: uint, end: uint) -> &'a [T] {
        self.as_slice().slice(start, end)
    }

    #[inline]
    pub fn mut_slice<'a>(&'a mut self, start: uint, end: uint) -> &'a mut [T] {
        self.as_mut_slice().mut_slice(start, end)
    }

    #[inline]
    pub fn slice_from<'a>(&'a self, start: uint) -> &'a [T] {
        self.as_slice().slice_from(start)
    }

    #[inline]
    pub fn mut_slice_from<'a>(&'a mut self, start: uint) -> &'a mut [T] {
        self.as_mut_slice().mut_slice_from(start)
    }

    #[inline]
    pub fn slice_to<'a>(&'a self, end: uint) -> &'a [T] {
        self.as_slice().slice_to(end)
    }

    #[inline]
    pub fn mut_slice_to<'a>(&'a mut self, end: uint) -> &'a mut [T] {
        self.as_mut_slice().mut_slice_to(end)
    }

    #[inline]
    pub fn mut_split_at<'a>(&'a mut self, mid: uint) -> (&'a mut [T], &'a mut [T]) {
        self.as_mut_slice().mut_split_at(mid)
    }

    #[inline]
    pub fn tail<'a>(&'a self) -> &'a [T] {
        self.as_slice().tail()
    }

    #[inline]
    pub fn tailn<'a>(&'a self, n: uint) -> &'a [T] {
        self.as_slice().tailn(n)
    }

    #[inline]
    pub fn last<'a>(&'a self) -> Option<&'a T> {
        self.as_slice().last()
    }

    #[inline]
    pub fn mut_last<'a>(&'a mut self) -> Option<&'a mut T> {
        self.as_mut_slice().mut_last()
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
            *self.vec.get_mut(self.len) = value;
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
    pub fn remove(&mut self, index: uint) -> Option<T> {
        if index < self.len {
            self.len -= 1;
            self.vec.remove(index)
        } else {
            None
        }
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
    pub fn move_iter(mut self) -> vec::MoveItems<T> {
        self.vec.truncate(self.len);
        self.vec.move_iter()
    }

    #[inline]
    pub fn get<'a>(&'a self, index: uint) -> &'a T {
        &self.as_slice()[index]
    }

    #[inline]
    pub fn get_mut<'a>(&'a mut self, index: uint) -> &'a mut T {
        &mut self.as_mut_slice()[index]
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> slice::Items<'a, T> {
        self.as_slice().iter()
    }

    #[inline]
    pub fn mut_iter<'a>(&'a mut self) -> slice::MutItems<'a, T> {
        self.as_mut_slice().mut_iter()
    }

    #[inline]
    pub fn sort_by(&mut self, compare: |&T, &T| -> Ordering) {
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
            self.vec.get_mut(self.len).clear();
        }
        self.len += 1;
        self.mut_last().unwrap()
    }
}

impl<T : Clone> RepeatedField<T> {
    #[inline]
    pub fn from_slice(values: &[T]) -> RepeatedField<T> {
        RepeatedField::from_vec(Vec::from_slice(values))
    }
}

impl<T : Clone> Clone for RepeatedField<T> {
    #[inline]
    fn clone(&self) -> RepeatedField<T> {
        RepeatedField {
            vec: Vec::from_slice(self.as_slice()),
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

impl<T> Slice<T> for RepeatedField<T> {
    #[inline]
    fn as_slice<'a>(&'a self) -> &'a [T] {
        self.vec.slice_to(self.len)
    }
}

impl<T : fmt::Show> fmt::Show for RepeatedField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}
