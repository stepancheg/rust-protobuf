extern crate serde;
extern crate protobuf;

use protobuf::SingularPtrField;

use serde::Serialize;
use serde::Deserialize;

use serde::ser::Serializer;
use serde::Deserializer;

pub fn serialize_singular_ptr_field<'a, T: Serialize, S>(spf: &SingularPtrField<T>, serializer: S)
        -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where S: Serializer
{
    spf.get_ref().serialize(serializer)
}

pub fn deserialize_singular_ptr_field<'de, D, T>(deserializer: D)
        -> Result<SingularPtrField<T>, D::Error>
    where D: Deserializer<'de>, T: Deserialize<'de>
{
    Ok(SingularPtrField::some(Option::deserialize(deserializer).unwrap().unwrap()))
}

