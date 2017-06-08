use std::vec;
use std::slice;
use std::borrow::Borrow;
use std::default::Default;
use std::hash::Hash;
use std::hash::Hasher;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Deref;
use std::ops::DerefMut;
use std::cmp::Ordering;
use std::fmt;

use clear::Clear;

pub struct RepeatedField<T> {
    vec: Vec<T>,
    len: usize,
}

impl<T> RepeatedField<T> {
    #[inline]
    pub fn len(&self) -> usize {
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
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
        &mut self.vec[..self.len]
    }

    #[inline]
    pub fn slice<'a>(&'a self, start: usize, end: usize) -> &'a [T] {
        &self.as_ref()[start..end]
    }

    #[inline]
    pub fn slice_mut<'a>(&'a mut self, start: usize, end: usize) -> &'a mut [T] {
        &mut self.as_mut_slice()[start..end]
    }

    #[inline]
    pub fn slice_from<'a>(&'a self, start: usize) -> &'a [T] {
        &self.as_ref()[start..]
    }

    #[inline]
    pub fn slice_from_mut<'a>(&'a mut self, start: usize) -> &'a mut [T] {
        &mut self.as_mut_slice()[start..]
    }

    #[inline]
    pub fn slice_to<'a>(&'a self, end: usize) -> &'a [T] {
        &self.as_ref()[..end]
    }

    #[inline]
    pub fn slice_to_mut<'a>(&'a mut self, end: usize) -> &'a mut [T] {
        &mut self.as_mut_slice()[..end]
    }

    #[inline]
    pub fn split_at<'a>(&'a self, mid: usize) -> (&'a [T], &'a [T]) {
        self.as_ref().split_at(mid)
    }

    #[inline]
    pub fn split_at_mut<'a>(&'a mut self, mid: usize) -> (&'a mut [T], &'a mut [T]) {
        self.as_mut_slice().split_at_mut(mid)
    }

    #[inline]
    pub fn tail<'a>(&'a self) -> &'a [T] {
        &self.as_ref()[1..]
    }

    #[inline]
    pub fn last<'a>(&'a self) -> Option<&'a T> {
        self.as_ref().last()
    }

    #[inline]
    pub fn last_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        self.as_mut_slice().last_mut()
    }

    #[inline]
    pub fn init<'a>(&'a self) -> &'a [T] {
        let s = self.as_ref();
        &s[0..s.len() - 1]
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
    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len);
        self.vec.insert(index, value);
        self.len += 1;
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);
        self.len -= 1;
        self.vec.remove(index)
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
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

    #[inline]
    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.as_ref().iter()
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
            vec: self.to_vec(),
            len: self.len(),
        }
    }
}

impl<T> FromIterator<T> for RepeatedField<T> {
    #[inline]
    fn from_iter<I : IntoIterator<Item = T>>(iter: I) -> RepeatedField<T> {
        RepeatedField::from_vec(FromIterator::from_iter(iter))
    }
}

impl<'a, T> IntoIterator for &'a RepeatedField<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self.iter()
    }
}

impl<T : PartialEq> PartialEq for RepeatedField<T> {
    #[inline]
    fn eq(&self, other: &RepeatedField<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T : Eq> Eq for RepeatedField<T> {}

impl<T : PartialEq> RepeatedField<T> {
    #[inline]
    pub fn contains(&self, value: &T) -> bool {
        self.as_ref().contains(value)
    }
}

impl<T : Hash> Hash for RepeatedField<T> {
    fn hash<H : Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<T> AsRef<[T]> for RepeatedField<T> {
    #[inline]
    fn as_ref<'a>(&'a self) -> &'a [T] {
        &self.vec[..self.len]
    }
}

impl<T> Borrow<[T]> for RepeatedField<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        &self.vec[..self.len]
    }
}

impl<T> Deref for RepeatedField<T> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T] {
        &self.vec[..self.len]
    }
}

impl<T> DerefMut for RepeatedField<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.vec[..self.len]
    }
}

impl<T> Index<usize> for RepeatedField<T> {
    type Output = T;

    #[inline]
    fn index<'a>(&'a self, index: usize) -> &'a T {
        &self.as_ref()[index]
    }
}

impl<T> IndexMut<usize> for RepeatedField<T> {
    #[inline]
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        &mut self.as_mut_slice()[index]
    }
}

impl<T : fmt::Debug> fmt::Debug for RepeatedField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::RepeatedField;

    #[test]
    fn as_mut_slice() {
        let mut v = RepeatedField::new();
        v.push(10);
        v.push(20);
        v.clear();
        assert_eq!(v.as_mut_slice(), &mut []);
        v.push(30);
        assert_eq!(v.as_mut_slice(), &mut [30]);
    }

    #[test]
    fn push_default() {
        let mut v = RepeatedField::new();
        v.push("aa".to_string());
        v.push("bb".to_string());
        v.clear();
        assert_eq!("".to_string(), *v.push_default());
    }
}
