use std::collections::HashMap;
use std::hash::Hash;

use crate::descriptor::EnumDescriptorProto;

#[derive(Debug)]
pub(crate) struct EnumIndex<S: Hash + Eq> {
    pub index_by_name: HashMap<S, usize>,
    pub index_by_number: HashMap<i32, usize>,
}

impl<S: Hash + Eq> EnumIndex<S> {
    pub fn index<'a, T: From<&'a str> + Hash + Eq>(proto: &'a EnumDescriptorProto) -> EnumIndex<T> {
        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, v) in proto.value.iter().enumerate() {
            index_by_number.insert(v.number(), i);
            index_by_name.insert(T::from(v.name()), i);
        }

        EnumIndex {
            index_by_name,
            index_by_number,
        }
    }
}
