use crate::descriptor::DescriptorProto;
use crate::json::json_name;
use std::collections::HashMap;

pub(crate) struct MessageIndices {
    pub index_by_name: HashMap<String, usize>,
    pub index_by_name_or_json_name: HashMap<String, usize>,
    pub index_by_number: HashMap<u32, usize>,
    pub json_names: Vec<String>,
}

impl MessageIndices {
    pub fn index(proto: &DescriptorProto) -> MessageIndices {
        let mut index_by_name = HashMap::new();
        let mut index_by_name_or_json_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        let mut json_names = Vec::new();

        for (i, f) in proto.field.iter().enumerate() {
            assert!(index_by_number.insert(f.get_number() as u32, i).is_none());
            assert!(index_by_name.insert(f.get_name().to_owned(), i).is_none());
            assert!(index_by_name_or_json_name
                .insert(f.get_name().to_owned(), i)
                .is_none());

            let json_name = if !f.get_json_name().is_empty() {
                f.get_json_name().to_owned()
            } else {
                json_name(f.get_name())
            };

            json_names.push(json_name.clone());

            if json_name != f.get_name() {
                assert!(index_by_name_or_json_name.insert(json_name, i).is_none());
            }
        }

        MessageIndices {
            index_by_name,
            index_by_name_or_json_name,
            index_by_number,
            json_names,
        }
    }

    pub fn fields_len(&self) -> usize {
        self.json_names.len()
    }
}
