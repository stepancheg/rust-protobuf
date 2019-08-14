#[cfg(not(rustc_nightly))]
mod transmute_eq_impl {
    use std::any::Any;
    use std::mem;

    #[inline(always)]
    pub fn transmute_eq<F: 'static, T: 'static>(mut from: F) -> Result<T, F> {
        // call downcast twice to work around borrow checked
        if (&mut from as &mut Any).downcast_mut::<T>().is_none() {
            return Err(from);
        }

        let to = unsafe {
            let from_as_to = (&mut from as &mut Any).downcast_mut().unwrap();
            let mut to = mem::uninitialized();
            mem::swap(from_as_to, &mut to);
            to
        };
        mem::forget(from);
        Ok(to)
    }
}

#[cfg(rustc_nightly)]
mod transmute_eq_impl {
    use std::marker;

    trait TransmuteEq<From, To> {
        fn transmute_eq(from: From) -> Result<To, From>;
    }

    struct TransmuteEqImpl<F, T>(marker::PhantomData<(F, T)>);

    impl<F, T> TransmuteEq<F, T> for TransmuteEqImpl<F, T> {
        #[inline(always)]
        default fn transmute_eq(from: F) -> Result<T, F> {
            Err(from)
        }
    }

    impl<S> TransmuteEq<S, S> for TransmuteEqImpl<S, S> {
        #[inline(always)]
        fn transmute_eq(from: S) -> Result<S, S> {
            Ok(from)
        }
    }

    #[inline(always)]
    pub fn transmute_eq<F: 'static, T: 'static>(from: F) -> Result<T, F> {
        TransmuteEqImpl::<F, T>::transmute_eq(from)
    }
}

/// Check if types `F` and `T` are the same.
#[inline(always)]
pub fn transmute_eq<F: 'static, T: 'static>(from: F) -> Result<T, F> {
    transmute_eq_impl::transmute_eq(from)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(
            Ok("ab".to_owned()),
            transmute_eq::<String, String>("ab".to_owned())
        )
    }

    #[test]
    fn test_err() {
        assert_eq!(
            Err("ab".to_owned()),
            transmute_eq::<String, Box<u32>>("ab".to_owned())
        )
    }
}
