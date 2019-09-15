//! Trait for well-known wrapper types

use crate::well_known_types::BoolValue;
use crate::well_known_types::BytesValue;
use crate::well_known_types::DoubleValue;
use crate::well_known_types::FloatValue;
use crate::well_known_types::Int32Value;
use crate::well_known_types::Int64Value;
use crate::well_known_types::StringValue;
use crate::well_known_types::UInt32Value;
use crate::well_known_types::UInt64Value;

/// Well-known wrapper types have single field.
/// This trait operations return pointers to that field.
pub(crate) trait WellKnownWrapper {
    type Underlying;

    fn get_ref(&self) -> &Self::Underlying;
    fn get_mut(&mut self) -> &mut Self::Underlying;
}

impl WellKnownWrapper for DoubleValue {
    type Underlying = f64;

    fn get_ref(&self) -> &f64 {
        &self.value
    }

    fn get_mut(&mut self) -> &mut f64 {
        &mut self.value
    }
}

impl WellKnownWrapper for FloatValue {
    type Underlying = f32;

    fn get_ref(&self) -> &f32 {
        &self.value
    }

    fn get_mut(&mut self) -> &mut f32 {
        &mut self.value
    }
}

impl WellKnownWrapper for Int64Value {
    type Underlying = i64;

    fn get_ref(&self) -> &i64 {
        &self.value
    }

    fn get_mut(&mut self) -> &mut i64 {
        &mut self.value
    }
}

impl WellKnownWrapper for UInt64Value {
    type Underlying = u64;

    fn get_ref(&self) -> &u64 {
        &self.value
    }

    fn get_mut(&mut self) -> &mut u64 {
        &mut self.value
    }
}

impl WellKnownWrapper for Int32Value {
    type Underlying = i32;

    fn get_ref(&self) -> &i32 {
        &self.value
    }

    fn get_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}

impl WellKnownWrapper for UInt32Value {
    type Underlying = u32;

    fn get_ref(&self) -> &u32 {
        &self.value
    }

    fn get_mut(&mut self) -> &mut u32 {
        &mut self.value
    }
}

impl WellKnownWrapper for BoolValue {
    type Underlying = bool;

    fn get_ref(&self) -> &bool {
        &self.value
    }

    fn get_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

impl WellKnownWrapper for StringValue {
    type Underlying = String;

    fn get_ref(&self) -> &String {
        &self.value
    }

    fn get_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl WellKnownWrapper for BytesValue {
    type Underlying = Vec<u8>;

    fn get_ref(&self) -> &Vec<u8> {
        &self.value
    }

    fn get_mut(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }
}
