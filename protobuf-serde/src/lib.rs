extern crate serde;
extern crate protobuf;

use protobuf::SingularPtrField;

use serde::Serialize;
use serde::Deserialize;

use serde::ser::Serializer;
use serde::Deserializer;

pub fn serialize_singular_ptr_field<T: Serialize, S>(spf: &SingularPtrField<T>, serializer: S)
        -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where S: Serializer
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

