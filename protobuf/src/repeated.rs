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


/// Implemented for both `Vec` and `RepeatedField`.
/// Used to simplify codegen, should not be used directly.
pub(crate) trait VecLike<T> {
    fn push(&mut self, item: T);
    fn push_default(&mut self) -> &mut T where T : Default + Clear;
}

impl<T> VecLike<T> for Vec<T> {
    fn push(&mut self, item: T) {
        Vec::push(self, item)
    }

    fn push_default(&mut self) -> &mut T where T : Default + Clear {
        self.push(Default::default());
        self.last_mut().unwrap()
    }
}


/// Wrapper around vector to avoid deallocations on clear.
///
/// It helps when needed to read lots of messages very quickly.
///
/// `RepeatedField` has the same API as `Vec`.
///
/// Note there's a codegen option to disable generation of `RepeatedField`
/// and generate `Vec` instead.
pub struct RepeatedField<T> {
    vec: Vec<T>,
    len: usize,
}

impl<T> RepeatedField<T> {
    /// Return number of elements in this container.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Clear.
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T> VecLike<T> for RepeatedField<T> {
    fn push(&mut self, item: T) {
        RepeatedField::push(self, item)
    }

    fn push_default(&mut self) -> &mut T where T : Default + Clear {
        if self.len == self.vec.len() {
            self.vec.push(Default::default());
        } else {
            self.vec[self.len].clear();
        }
        self.len += 1;
        &mut self.vec[self.len - 1]
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
    /// Create new empty container.
    #[inline]
    pub fn new() -> RepeatedField<T> {
        Default::default()
    }

    /// Create a contained with data from given vec.
    #[inline]
    pub fn from_vec(vec: Vec<T>) -> RepeatedField<T> {
        let len = vec.len();
        RepeatedField { vec: vec, len: len }
    }

    /// Convert data into vec.
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        let mut vec = self.vec;
        vec.truncate(self.len);
        vec
    }

    /// Return current capacity.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    /// View data as mutable slice.
    #[inline]
    pub fn as_slice<'a>(&'a self) -> &'a [T] {
        &self.vec[..self.len]
    }

    #[inline]
    pub fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
        &mut self.vec[..self.len]
    }

    /// Get subslice of this container.
    #[inline]
    pub fn slice(&self, start: usize, end: usize) -> &[T] {
        &self.as_ref()[start..end]
    }

    /// Get mutable subslice of this container.
    #[inline]
    pub fn slice_mut(&mut self, start: usize, end: usize) -> &mut [T] {
        &mut self.as_mut_slice()[start..end]
    }

    /// Get slice from given index.
    #[inline]
    pub fn slice_from(&self, start: usize) -> &[T] {
        &self.as_ref()[start..]
    }

    /// Get mutable slice from given index.
    #[inline]
    pub fn slice_from_mut(&mut self, start: usize) -> &mut [T] {
        &mut self.as_mut_slice()[start..]
    }

    /// Get slice to given index.
    #[inline]
    pub fn slice_to(&self, end: usize) -> &[T] {
        &self.as_ref()[..end]
    }

    /// Get mutable slice to given index.
    #[inline]
    pub fn slice_to_mut(&mut self, end: usize) -> &mut [T] {
        &mut self.as_mut_slice()[..end]
    }

    /// View this container as two slices split at given index.
    #[inline]
    pub fn split_at<'a>(&'a self, mid: usize) -> (&'a [T], &'a [T]) {
        self.as_ref().split_at(mid)
    }

    /// View this container as two mutable slices split at given index.
    #[inline]
    pub fn split_at_mut<'a>(&'a mut self, mid: usize) -> (&'a mut [T], &'a mut [T]) {
        self.as_mut_slice().split_at_mut(mid)
    }

    /// View all but first elements of this container.
    #[inline]
    pub fn tail(&self) -> &[T] {
        &self.as_ref()[1..]
    }

    /// Last element of this container.
    #[inline]
    pub fn last(&self) -> Option<&T> {
        self.as_ref().last()
    }

    /// Mutable last element of this container.
    #[inline]
    pub fn last_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        self.as_mut_slice().last_mut()
    }

    /// View all but last elements of this container.
    #[inline]
    pub fn init<'a>(&'a self) -> &'a [T] {
        let s = self.as_ref();
        &s[0..s.len() - 1]
    }

    /// Push an element to the end.
    #[inline]
    pub fn push(&mut self, value: T) {
        if self.len == self.vec.len() {
            self.vec.push(value);
        } else {
            self.vec[self.len] = value;
        }
        self.len += 1;
    }

    /// Pop last element.
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

    /// Insert an element at specified position.
    #[inline]
    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len);
        self.vec.insert(index, value);
        self.len += 1;
    }

    /// Remove an element from specified position.
    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);
        self.len -= 1;
        self.vec.remove(index)
    }

    /// Truncate at specified length.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        if self.len > len {
            self.len = len;
        }
    }

    /// Reverse in place.
    #[inline]
    pub fn reverse(&mut self) {
        self.as_mut_slice().reverse()
    }

    /// Into owned iterator.
    #[inline]
    pub fn into_iter(mut self) -> vec::IntoIter<T> {
        self.vec.truncate(self.len);
        self.vec.into_iter()
    }

    /// Immutable data iterator.
    #[inline]
    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.as_ref().iter()
    }

    /// Mutable data iterator.
    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Sort elements with given comparator.
    #[inline]
    pub fn sort_by<F>(&mut self, compare: F)
    where
        F : Fn(&T, &T) -> Ordering,
    {
        self.as_mut_slice().sort_by(compare)
    }

    /// Get data as raw pointer.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.vec.as_ptr()
    }

    /// Get data a mutable raw pointer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.vec.as_mut_ptr()
    }
}

impl<T : Default + Clear> RepeatedField<T> {
    /// Push default value.
    /// This operation could be faster than `rf.push(Default::default())`,
    /// because it may reuse previously allocated and cleared element.
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

impl<T> From<Vec<T>> for RepeatedField<T> {
    #[inline]
    fn from(values: Vec<T>) -> RepeatedField<T> {
        RepeatedField::from_vec(values)
    }
}

impl<'a, T : Clone> From<&'a [T]> for RepeatedField<T> {
    #[inline]
    fn from(values: &'a [T]) -> RepeatedField<T> {
        RepeatedField::from_slice(values)
    }
}

impl<T> Into<Vec<T>> for RepeatedField<T> {
    #[inline]
    fn into(self) -> Vec<T> {
        self.into_vec()
    }
}


impl<T : Clone> RepeatedField<T> {

    /// Copy slice data to `RepeatedField`
    #[inline]
    pub fn from_slice(values: &[T]) -> RepeatedField<T> {
        RepeatedField::from_vec(values.to_vec())
    }

    /// Copy slice data to `RepeatedField`
    #[inline]
    pub fn from_ref<X: AsRef<[T]>>(values: X) -> RepeatedField<T> {
        RepeatedField::from_slice(values.as_ref())
    }

    /// Copy this data into new vec.
    #[inline]
    pub fn to_vec(&self) -> Vec<T> {
        self.as_ref().to_vec()
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
    /// True iff this container contains given element.
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
