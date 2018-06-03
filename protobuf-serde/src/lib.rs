extern crate serde;
extern crate protobuf;

use protobuf::SingularPtrField;

use serde::Serialize;
use serde::Serializer;
use serde::Deserializer;
use serde::Deserialize;

// TODO: Work in progress
pub fn serialize_singular_ptr_field<'a, T: Serialize, S>(spf: &SingularPtrField<T>, serializer: &'a mut S)
    -> Result<<&'a mut S as Serializer>::Ok, <&'a mut S as Serializer>::Error>
    where &'a mut S: Serializer {
    // spf.unwrap().serialize(serializer)
    "hello".serialize(serializer)
}

// TODO: Work in progress
pub fn deserialize_singular_ptr_field<'de, D, T>(deserializer: D)
    -> Result<SingularPtrField<T>, D::Error>
    where D: Deserializer<'de> {
    // spf.unwrap().serialize(serializer)
    Ok(SingularPtrField::none())
}

/*
pub struct SerializableDeserializable<T: Serialize>(SingularPtrField<T>);

pub trait SerializeWith: Sized {
    fn serialize_with<'a, S>(self, ser: &'a mut S) -> Result<<&'a mut S as serde::Serializer>::Ok, <&'a mut S as serde::Serializer>::Error>
        where &'a mut S: Serializer;
}

impl<T: Serialize> SerializeWith for SerializableDeserializable<T> {
    fn serialize_with<'a, S>(self, serializer: &'a mut S) -> Result<<&'a mut S as serde::Serializer>::Ok, <&'a mut S as serde::Serializer>::Error>
        where &'a mut S: Serializer {
        self.0.unwrap().serialize(serializer)
    }
}
*/
