#[cfg(feature = "with-serde")]
use serde;

use std::default::Default;
use std::hash::Hash;
use std::option;

use crate::clear::Clear;
use crate::Message;

/// Option-like objects
#[doc(hidden)]
pub trait OptionLike<T> {
    fn into_option(self) -> Option<T>;
    fn as_option_ref(&self) -> Option<&T>;
    fn as_option_mut(&mut self) -> Option<&mut T>;
    fn set_value(&mut self, value: T);
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
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SingularPtrField<T> {
    value: Option<Box<T>>,
}

impl<T> SingularPtrField<T> {
    /// Construct `SingularPtrField` from given object.
    #[inline]
    pub fn some(value: T) -> SingularPtrField<T> {
        SingularPtrField {
            value: Some(Box::new(value)),
        }
    }

    /// Construct an empty `SingularPtrField`.
    #[inline]
    pub const fn none() -> SingularPtrField<T> {
        SingularPtrField { value: None }
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
        self.value.is_some()
    }

    /// True iff this object contains no data.
    #[inline]
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }

    /// Convert into `Option<T>`.
    #[inline]
    pub fn into_option(self) -> Option<T> {
        self.value.map(|v| *v)
    }

    /// View data as reference option.
    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        self.value.as_ref().map(|v| &**v)
    }

    /// View data as mutable reference option.
    #[inline]
    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut().map(|v| &mut **v)
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
        *self.value.unwrap()
    }

    /// Take the data or return supplied default element if empty.
    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        self.value.map(|v| *v).unwrap_or(def)
    }

    /// Take the data or return supplied default element if empty.
    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.value.map(|v| *v).unwrap_or_else(f)
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
        self.value.take().map(|v| *v)
    }

    /// Clear this object, but do not call destructor of underlying data.
    #[inline]
    pub fn clear(&mut self) {
        self.value = None;
    }
}

impl<T: Default + Clear> SingularPtrField<T> {
    /// Get contained data, consume self. Return default value for type if this is empty.
    #[inline]
    pub fn unwrap_or_default(self) -> T {
        *self.value.unwrap_or_default()
    }

    /// Set object to `Some(T::default())`.
    // TODO: inline
    #[inline]
    pub fn set_default(&mut self) -> &mut T {
        *self = SingularPtrField::some(Default::default());
        self.as_mut().unwrap()
    }
}

impl<M: Message + Default> SingularPtrField<M> {
    /// Get a reference to contained value or a default instance.
    pub fn get_or_default(&self) -> &M {
        self.as_ref().unwrap_or_else(|| M::default_instance())
    }

    /// Get a mutable reference to contained value, initialize if not initialized yet.
    pub fn mut_or_default(&mut self) -> &mut M {
        if self.is_none() {
            *self = SingularPtrField::some(Default::default());
        }
        self.get_mut_ref()
    }
}

impl<T> Default for SingularPtrField<T> {
    #[inline]
    fn default() -> SingularPtrField<T> {
        SingularPtrField::none()
    }
}

impl<T> From<Option<T>> for SingularPtrField<T> {
    fn from(o: Option<T>) -> Self {
        SingularPtrField::from_option(o)
    }
}

impl<'a, T> IntoIterator for &'a SingularPtrField<T> {
    type Item = &'a T;
    type IntoIter = option::IntoIter<&'a T>;

    fn into_iter(self) -> option::IntoIter<&'a T> {
        self.iter()
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
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for SingularPtrField<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(SingularPtrField::from)
    }
}
