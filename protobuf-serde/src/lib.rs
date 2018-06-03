extern crate serde;
extern crate protobuf;

use protobuf::SingularPtrField;

use serde::Serialize;
use serde::ser::Serializer;

pub fn serialize_singular_ptr_field<'a, T: Serialize, S>(spf: &SingularPtrField<T>, serializer: &'a mut S)
    -> Result<<&'a mut S as Serializer>::Ok, <&'a mut S as Serializer>::Error>
    where &'a mut S: Serializer {
    // spf.unwrap().serialize(serializer)
    "hello".serialize(serializer)
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
