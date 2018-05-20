#![allow(dead_code)]

use std::any::Any;
use std::mem;

/// Check if types `F` and `T` are the same.
pub fn transmute_eq<F : 'static, T : 'static>(mut from: F) -> Result<T, F> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(Ok("ab".to_owned()), transmute_eq::<String, String>("ab".to_owned()))
    }

    #[test]
    fn test_err() {
        assert_eq!(Err("ab".to_owned()), transmute_eq::<String, Box<u32>>("ab".to_owned()))
    }
}
