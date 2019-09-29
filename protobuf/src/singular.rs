#[cfg(feature = "with-serde")]
use serde;

use std::default::Default;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem;
use std::option;

use crate::clear::Clear;

/// Option-like objects
#[doc(hidden)]
pub trait OptionLike<T> {
    fn into_option(self) -> Option<T>;
    fn as_option_ref(&self) -> Option<&T>;
    fn as_option_mut(&mut self) -> Option<&mut T>;
    fn set_value(&mut self, value: T);
    fn set_default(&mut self) -> &mut T
    where
        T: Default + Clear;
}

impl<T> OptionLike<T> for Option<T> {
    fn into_option(self) -> Option<T> {
        self
    }

    fn as_option_ref(&self) -> Option<&T> {
        self.as_ref()
    }

    fn as_option_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }

    fn set_value(&mut self, value: T) {
        *self = Some(value);
    }

    fn set_default(&mut self) -> &mut T
    where
        T: Default + Clear,
    {
        if self.is_some() {
            let v = self.as_mut().unwrap();
            v.clear();
            v
        } else {
            *self = Some(Default::default());
            self.as_mut().unwrap()
        }
    }
}

impl<T> OptionLike<T> for Option<Box<T>> {
    fn into_option(self) -> Option<T> {
        self.map(|b| *b)
    }

    fn as_option_ref(&self) -> Option<&T> {
        self.as_ref().map(|b| b.as_ref())
    }

    fn as_option_mut(&mut self) -> Option<&mut T> {
        self.as_mut().map(|b| b.as_mut())
    }

    fn set_value(&mut self, value: T) {
        // TODO: reuse allocation
        *self = Some(Box::new(value))
    }

    fn set_default(&mut self) -> &mut T
    where
        T: Default + Clear,
    {
        if self.is_some() {
            let v = self.as_mut().unwrap();
            v.clear();
            v
        } else {
            *self = Some(Box::new(Default::default()));
            self.as_mut().unwrap()
        }
    }
}

/// Like `Option<T>`, but keeps the actual element on `clear`.
pub struct SingularField<T> {
    value: T,
    set: bool,
}

/// Like `Option<Box<T>>`, but keeps the actual element on `clear`.
///
/// # Examples
///
/// ```no_run
/// # use protobuf::SingularPtrField;
/// # use std::ops::Add;
/// # struct Address {
/// # }
/// # struct Customer {
/// #     address: SingularPtrField<Address>,
/// # }
/// # impl Customer {
/// #     fn new() -> Customer { unimplemented!() }
/// # }
/// #
/// #
/// # fn make_address() -> Address { unimplemented!() }
/// let mut customer = Customer::new();
///
/// // field of type `SingularPtrField` can be initialized like this
/// customer.address = SingularPtrField::some(make_address());
/// // or using `Option` and `Into`
/// customer.address = Some(make_address()).into();
/// ```
pub struct SingularPtrField<T> {
    value: Option<Box<T>>,
    set: bool,
}

impl<T> SingularField<T> {
    /// Construct this object from given value.
    #[inline]
    pub fn some(value: T) -> SingularField<T> {
        SingularField { value, set: true }
    }

    /// True iff this object contains data.
    #[inline]
    pub fn is_some(&self) -> bool {
        self.set
    }

    /// True iff this object contains no data.
    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Convert this object into `Option`.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        if self.set {
            Some(self.value)
        } else {
            None
        }
    }

    /// View data as `Option`.
    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        if self.set {
            Some(&self.value)
        } else {
            None
        }
    }

    /// View data as mutable `Option`.
    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.set {
            Some(&mut self.value)
        } else {
            None
        }
    }

    /// Unwrap data as reference.
    #[inline]
    pub fn unwrap_ref(&self) -> &T {
        self.as_ref().unwrap()
    }

    /// Unwrap data as mutable reference.
    #[inline]
    pub fn unwrap_mut_ref(&mut self) -> &mut T {
        self.as_mut().unwrap()
    }

    /// Unwrap data, panic if not set.
    #[inline]
    pub fn unwrap(self) -> T {
        if self.set {
            self.value
        } else {
            panic!();
        }
    }

    /// Unwrap data or return given default value.
    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        if self.set {
            self.value
        } else {
            def
        }
    }

    /// Unwrap data or return given default value.
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        if self.set {
            self.value
        } else {
            f()
        }
    }

    /// Apply a function to contained element and store result in new `SingularPtrField`.
    #[inline]
    pub fn map<U, F>(self, f: F) -> SingularPtrField<U>
    where
        F: FnOnce(T) -> U,
    {
        SingularPtrField::from_option(self.into_option().map(f))
    }

    /// View as iterator over references.
    #[inline]
    pub fn iter(&self) -> option::IntoIter<&T> {
        self.as_ref().into_iter()
    }

    /// View as iterator over mutable references.
    #[inline]
    pub fn mut_iter(&mut self) -> option::IntoIter<&mut T> {
        self.as_mut().into_iter()
    }

    /// Clear this object.
    /// Note, contained object destructor is not called, so allocated memory could be reused.
    #[inline]
    pub fn clear(&mut self) {
        self.set = false;
    }
}

impl<T: Default> SingularField<T> {
    /// Construct a `SingularField` with no data.
    #[inline]
    pub fn none() -> SingularField<T> {
        SingularField {
            value: Default::default(),
            set: false,
        }
    }

    /// Construct `SingularField` from `Option`.
    #[inline]
    pub fn from_option(option: Option<T>) -> SingularField<T> {
        match option {
            Some(x) => SingularField::some(x),
            None => SingularField::none(),
        }
    }

    /// Return data as option, clear this object.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        if self.set {
            self.set = false;
            Some(mem::replace(&mut self.value, Default::default()))
        } else {
            None
        }
    }
}

impl<T> SingularPtrField<T> {
    /// Construct `SingularPtrField` from given object.
    #[inline]
    pub fn some(value: T) -> SingularPtrField<T> {
        SingularPtrField {
            value: Some(Box::new(value)),
            set: true,
        }
    }

    /// Construct an empty `SingularPtrField`.
    #[inline]
    pub fn none() -> SingularPtrField<T> {
        SingularPtrField {
            value: None,
            set: false,
        }
    }

    /// Construct `SingularPtrField` from optional.
    #[inline]
    pub fn from_option(option: Option<T>) -> SingularPtrField<T> {
        match option {
            Some(x) => SingularPtrField::some(x),
            None => SingularPtrField::none(),
        }
    }

    /// True iff this object contains data.
    #[inline]
    pub fn is_some(&self) -> bool {
        self.set
    }

    /// True iff this object contains no data.
    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Convert into `Option<T>`.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        if self.set {
            Some(*self.value.unwrap())
        } else {
            None
        }
    }

    /// View data as reference option.
    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        if self.set {
            Some(&**self.value.as_ref().unwrap())
        } else {
            None
        }
    }

    /// View data as mutable reference option.
    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.set {
            Some(&mut **self.value.as_mut().unwrap())
        } else {
            None
        }
    }

    /// Get data as reference.
    /// Panics if empty.
    #[inline]
    pub fn get_ref(&self) -> &T {
        self.as_ref().unwrap()
    }

    /// Get data as mutable reference.
    /// Panics if empty.
    #[inline]
    pub fn get_mut_ref(&mut self) -> &mut T {
        self.as_mut().unwrap()
    }

    /// Take the data.
    /// Panics if empty
    #[inline]
    pub fn unwrap(self) -> T {
        if self.set {
            *self.value.unwrap()
        } else {
            panic!();
        }
    }

    /// Take the data or return supplied default element if empty.
    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        if self.set {
            *self.value.unwrap()
        } else {
            def
        }
    }

    /// Take the data or return supplied default element if empty.
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        if self.set {
            *self.value.unwrap()
        } else {
            f()
        }
    }

    /// Apply given function to contained data to construct another `SingularPtrField`.
    /// Returns empty `SingularPtrField` if this object is empty.
    #[inline]
    pub fn map<U, F>(self, f: F) -> SingularPtrField<U>
    where
        F: FnOnce(T) -> U,
    {
        SingularPtrField::from_option(self.into_option().map(f))
    }

    /// View data as iterator.
    #[inline]
    pub fn iter(&self) -> option::IntoIter<&T> {
        self.as_ref().into_iter()
    }

    /// View data as mutable iterator.
    #[inline]
    pub fn mut_iter(&mut self) -> option::IntoIter<&mut T> {
        self.as_mut().into_iter()
    }

    /// Take data as option, leaving this object empty.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        if self.set {
            self.set = false;
            Some(*self.value.take().unwrap())
        } else {
            None
        }
    }

    /// Clear this object, but do not call destructor of underlying data.
    #[inline]
    pub fn clear(&mut self) {
        self.set = false;
    }
}

impl<T: Default + Clear> SingularField<T> {
    /// Get contained data, consume self. Return default value for type if this is empty.
    #[inline]
    pub fn unwrap_or_default(mut self) -> T {
        self.value.clear();
        self.value
    }

    /// Set object to `Some(T::default())`.
    // TODO: inline
    #[inline]
    pub fn set_default(&mut self) -> &mut T {
        OptionLike::set_default(self)
    }
}

impl<T: Default + Clear> SingularPtrField<T> {
    /// Get contained data, consume self. Return default value for type if this is empty.
    #[inline]
    pub fn unwrap_or_default(mut self) -> T {
        if self.set {
            self.unwrap()
        } else if self.value.is_some() {
            self.value.clear();
            *self.value.unwrap()
        } else {
            Default::default()
        }
    }

    /// Set object to `Some(T::default())`.
    // TODO: inline
    #[inline]
    pub fn set_default(&mut self) -> &mut T {
        OptionLike::set_default(self)
    }
}

impl<T: Default> Default for SingularField<T> {
    #[inline]
    fn default() -> SingularField<T> {
        SingularField::none()
    }
}

impl<T> Default for SingularPtrField<T> {
    #[inline]
    fn default() -> SingularPtrField<T> {
        SingularPtrField::none()
    }
}

impl<T: Default> From<Option<T>> for SingularField<T> {
    fn from(o: Option<T>) -> Self {
        SingularField::from_option(o)
    }
}

impl<T> From<Option<T>> for SingularPtrField<T> {
    fn from(o: Option<T>) -> Self {
        SingularPtrField::from_option(o)
    }
}

impl<T: Clone + Default> Clone for SingularField<T> {
    #[inline]
    fn clone(&self) -> SingularField<T> {
        if self.set {
            SingularField::some(self.value.clone())
        } else {
            SingularField::none()
        }
    }
}

impl<T: Clone> Clone for SingularPtrField<T> {
    #[inline]
    fn clone(&self) -> SingularPtrField<T> {
        if self.set {
            SingularPtrField::some(self.as_ref().unwrap().clone())
        } else {
            SingularPtrField::none()
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for SingularField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_some() {
            write!(f, "Some({:?})", *self.as_ref().unwrap())
        } else {
            write!(f, "None")
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for SingularPtrField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_some() {
            write!(f, "Some({:?})", *self.as_ref().unwrap())
        } else {
            write!(f, "None")
        }
    }
}

impl<T: PartialEq> PartialEq for SingularField<T> {
    #[inline]
    fn eq(&self, other: &SingularField<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T: Eq> Eq for SingularField<T> {}

impl<T: PartialEq> PartialEq for SingularPtrField<T> {
    #[inline]
    fn eq(&self, other: &SingularPtrField<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T: Eq> Eq for SingularPtrField<T> {}

impl<T: Hash> Hash for SingularField<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<T: Hash> Hash for SingularPtrField<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<'a, T> IntoIterator for &'a SingularField<T> {
    type Item = &'a T;
    type IntoIter = option::IntoIter<&'a T>;

    fn into_iter(self) -> option::IntoIter<&'a T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a SingularPtrField<T> {
    type Item = &'a T;
    type IntoIter = option::IntoIter<&'a T>;

    fn into_iter(self) -> option::IntoIter<&'a T> {
        self.iter()
    }
}

impl<T> OptionLike<T> for SingularField<T> {
    fn into_option(self) -> Option<T> {
        self.into_option()
    }

    fn as_option_ref(&self) -> Option<&T> {
        self.as_ref()
    }

    fn as_option_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }

    fn set_value(&mut self, value: T) {
        *self = SingularField::some(value);
    }

    /// Initialize this object with default value.
    /// This operation can be more efficient then construction of clear element,
    /// because it may reuse previously contained object.
    #[inline]
    fn set_default(&mut self) -> &mut T
    where
        T: Default + Clear,
    {
        self.set = true;
        self.value.clear();
        &mut self.value
    }
}

impl<T> OptionLike<T> for SingularPtrField<T> {
    fn into_option(self) -> Option<T> {
        self.into_option()
    }

    fn as_option_ref(&self) -> Option<&T> {
        self.as_ref()
    }

    fn as_option_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }

    fn set_value(&mut self, value: T) {
        // TODO: unnecessary malloc
        *self = SingularPtrField::some(value);
    }

    /// Initialize this object with default value.
    /// This operation can be more efficient then construction of clear element,
    /// because it may reuse previously contained object.
    #[inline]
    fn set_default(&mut self) -> &mut T
    where
        T: Default + Clear,
    {
        self.set = true;
        if self.value.is_some() {
            self.value.as_mut().unwrap().clear();
        } else {
            self.value = Some(Default::default());
        }
        self.as_mut().unwrap()
    }
}

#[cfg(feature = "with-serde")]
impl<T: serde::Serialize> serde::Serialize for SingularPtrField<T> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

#[cfg(feature = "with-serde")]
impl<T: serde::Serialize> serde::Serialize for SingularField<T> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

#[cfg(feature = "with-serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for SingularPtrField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(SingularPtrField::from)
    }
}

#[cfg(feature = "with-serde")]
impl<'de, T: serde::Deserialize<'de> + Default> serde::Deserialize<'de> for SingularField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(SingularField::from)
    }
}

#[cfg(test)]
mod test {
    use crate::clear::Clear;
    use crate::singular::SingularField;

    #[test]
    fn test_set_default_clears() {
        #[derive(Default)]
        struct Foo {
            b: isize,
        }

        impl Clear for Foo {
            fn clear(&mut self) {
                self.b = 0;
            }
        }

        let mut x = SingularField::some(Foo { b: 10 });
        x.clear();
        x.set_default();
        assert_eq!(0, x.as_ref().unwrap().b);

        x.as_mut().unwrap().b = 11;
        // without clear
        x.set_default();
        assert_eq!(0, x.as_ref().unwrap().b);
    }
}
