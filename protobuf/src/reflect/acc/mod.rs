#![doc(hidden)]

use crate::reflect::acc::v1::FieldAccessorTrait;

pub(crate) mod v1;

// TODO: replace with struct
pub type FieldAccessor = Box<dyn FieldAccessorTrait + 'static>;
