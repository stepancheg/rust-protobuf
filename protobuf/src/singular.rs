use std::hash::Hash;
use std::hash::Hasher;
use std::option;
use std::default::Default;
use std::fmt;
use std::mem;

use clear::Clear;


pub struct SingularField<T> {
    value: T,
    set: bool,
}

pub struct SingularPtrField<T> {
    value: Option<Box<T>>,
    set: bool,
}

impl<T> SingularField<T> {
    #[inline]
    pub fn some(value: T) -> SingularField<T> {
        SingularField {
            value: value,
            set: true,
        }
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        self.set
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    #[inline]
    pub fn into_option(self) -> Option<T> {
        if self.set {
            Some(self.value)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ref<'a>(&'a self) -> Option<&'a T> {
        if self.set {
            Some(&self.value)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        if self.set {
            Some(&mut self.value)
        } else {
            None
        }
    }

    #[inline]
    pub fn get_ref<'a>(&'a self) -> &'a T {
        self.as_ref().unwrap()
    }

    #[inline]
    pub fn get_mut_ref<'a>(&'a mut self) -> &'a mut T {
        self.as_mut().unwrap()
    }

    #[inline]
    pub fn unwrap(self) -> T {
        if self.set {
            self.value
        } else {
            panic!();
        }
    }

    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        if self.set {
            self.value
        } else {
            def
        }
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
        where F : FnOnce() -> T
    {
        if self.set {
            self.value
        } else {
            f()
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> SingularPtrField<U>
        where F: FnOnce(T) -> U
    {
        SingularPtrField::from_option(self.into_option().map(f))
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> option::IntoIter<&'a T> {
        self.as_ref().into_iter()
    }

    #[inline]
    pub fn mut_iter<'a>(&'a mut self) -> option::IntoIter<&'a mut T> {
        self.as_mut().into_iter()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.set = false;
    }
}

impl<T : Default> SingularField<T> {
    #[inline]
    pub fn none() -> SingularField<T> {
        SingularField {
            value: Default::default(),
            set: false,
        }
    }

    #[inline]
    pub fn from_option(option: Option<T>) -> SingularField<T> {
        match option {
            Some(x) => SingularField::some(x),
            None => SingularField::none(),
        }
    }

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
    #[inline]
    pub fn some(value: T) -> SingularPtrField<T> {
        SingularPtrField {
            value: Some(Box::new(value)),
            set: true,
        }
    }

    #[inline]
    pub fn none() -> SingularPtrField<T> {
        SingularPtrField {
            value: None,
            set: false,
        }
    }

    #[inline]
    pub fn from_option(option: Option<T>) -> SingularPtrField<T> {
        match option {
            Some(x) => SingularPtrField::some(x),
            None => SingularPtrField::none(),
        }
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        self.set
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        !self.is_some()
    }

    #[inline]
    pub fn into_option(self) -> Option<T> {
        if self.set {
            Some(*self.value.unwrap())
        } else {
            None
        }
    }

    #[inline]
    pub fn as_ref<'a>(&'a self) -> Option<&'a T> {
        if self.set {
            Some(&**self.value.as_ref().unwrap())
        } else {
            None
        }
    }

    #[inline]
    pub fn as_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        if self.set {
            Some(&mut **self.value.as_mut().unwrap())
        } else {
            None
        }
    }

    #[inline]
    pub fn get_ref<'a>(&'a self) -> &'a T {
        self.as_ref().unwrap()
    }

    #[inline]
    pub fn get_mut_ref<'a>(&'a mut self) -> &'a mut T {
        self.as_mut().unwrap()
    }

    #[inline]
    pub fn unwrap(self) -> T {
        if self.set {
            *self.value.unwrap()
        } else {
            panic!();
        }
    }

    #[inline]
    pub fn unwrap_or(self, def: T) -> T {
        if self.set {
            *self.value.unwrap()
        } else {
            def
        }
    }

    #[inline]
    pub fn unwrap_or_else<F>(self, f: F) -> T
        where F : FnOnce() -> T
    {
        if self.set {
            *self.value.unwrap()
        } else {
            f()
        }
    }

    #[inline]
    pub fn map<U, F>(self, f: F) -> SingularPtrField<U>
        where F: FnOnce(T) -> U
    {
        SingularPtrField::from_option(self.into_option().map(f))
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> option::IntoIter<&'a T> {
        self.as_ref().into_iter()
    }

    #[inline]
    pub fn mut_iter<'a>(&'a mut self) -> option::IntoIter<&'a mut T> {
        self.as_mut().into_iter()
    }

    #[inline]
    pub fn take(&mut self) -> Option<T> {
        if self.set {
            self.set = false;
            Some(*self.value.take().unwrap())
        } else {
            None
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.set = false;
    }
}

impl<T : Default+Clear> SingularField<T> {
    #[inline]
    pub fn unwrap_or_default(mut self) -> T {
        self.value.clear();
        self.value
    }

    #[inline]
    pub fn set_default<'a>(&'a mut self) -> &'a mut T {
        self.set = true;
        self.value.clear();
        &mut self.value
    }
}

impl<T : Default+Clear> SingularPtrField<T> {
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

    #[inline]
    pub fn set_default<'a>(&'a mut self) -> &'a mut T {
        self.set = true;
        if self.value.is_some() {
            self.value.as_mut().unwrap().clear();
        } else {
            self.value = Some(Default::default());
        }
        self.as_mut().unwrap()
    }
}

impl<T : Default> Default for SingularField<T> {
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

impl<T : Clone+Default> Clone for SingularField<T> {
    #[inline]
    fn clone(&self) -> SingularField<T> {
        if self.set {
            SingularField::some(self.value.clone())
        } else {
            SingularField::none()
        }
    }
}

impl<T : Clone> Clone for SingularPtrField<T> {
    #[inline]
    fn clone(&self) -> SingularPtrField<T> {
        if self.set {
            SingularPtrField::some(self.as_ref().unwrap().clone())
        } else {
            SingularPtrField::none()
        }
    }
}

impl<T : fmt::Debug> fmt::Debug for SingularField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_some() {
            write!(f, "Some({:?})", *self.as_ref().unwrap())
        } else {
            write!(f, "None")
        }
    }
}

impl<T : fmt::Debug> fmt::Debug for SingularPtrField<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_some() {
            write!(f, "Some({:?})", *self.as_ref().unwrap())
        } else {
            write!(f, "None")
        }
    }
}

impl<T : PartialEq> PartialEq for SingularField<T> {
    #[inline]
    fn eq(&self, other: &SingularField<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T : Eq> Eq for SingularField<T> {}

impl<T : PartialEq> PartialEq for SingularPtrField<T> {
    #[inline]
    fn eq(&self, other: &SingularPtrField<T>) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T : Eq> Eq for SingularPtrField<T> {}


impl<T : Hash> Hash for SingularField<T> {
    fn hash<H : Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<T : Hash> Hash for SingularPtrField<T> {
    fn hash<H : Hasher>(&self, state: &mut H) {
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


#[cfg(test)]
mod test {
    use clear::Clear;
    use super::SingularField;

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
