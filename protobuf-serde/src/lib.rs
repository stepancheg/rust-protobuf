//! Runtime for generated code with serde options.
//! Should not be used directly.

extern crate serde;
extern crate protobuf;

use protobuf::SingularPtrField;
use protobuf::RepeatedField;

use serde::Serialize;
use serde::Deserialize;

use serde::ser::Serializer;
use serde::Deserializer;

pub fn serialize_singular_ptr_field<T, S>(spf: &SingularPtrField<T>, serializer: S)
        -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where S: Serializer, T: Serialize
{
    match spf.as_ref() {
        Some(value) => value.serialize(serializer),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_singular_ptr_field<'de, D, T>(deserializer: D)
        -> Result<SingularPtrField<T>, D::Error>
    where D: Deserializer<'de>, T: Deserialize<'de>
{
    Option::deserialize(deserializer).map(From::from)
}

pub fn serialize_repeated_field<T, S>(repeated: &RepeatedField<T>, serializer: S)
    -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where S: Serializer, T: Serialize
{
    repeated.as_slice().serialize(serializer)
}

pub fn deserialize_repeated_field<'de, D, T>(deserializer: D)
    -> Result<RepeatedField<T>, D::Error>
    where D: Deserializer<'de>, T: Deserialize<'de>
{
    Vec::deserialize(deserializer).map(From::from)
}
