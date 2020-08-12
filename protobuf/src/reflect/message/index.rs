use crate::descriptor::DescriptorProto;
use crate::reflect::field::index::FieldIndex;
use crate::reflect::file::building::FileDescriptorBuilding;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct MessageIndex {
    pub fields: Vec<FieldIndex>,
    pub index_by_name: HashMap<String, usize>,
    pub index_by_name_or_json_name: HashMap<String, usize>,
    pub index_by_number: HashMap<u32, usize>,
}

impl MessageIndex {
    pub fn index(proto: &DescriptorProto, building: &FileDescriptorBuilding) -> MessageIndex {
        let mut index_by_name = HashMap::new();
        let mut index_by_name_or_json_name = HashMap::new();
        let mut index_by_number = HashMap::new();

        let fields: Vec<FieldIndex> = proto
            .field
            .iter()
            .map(|f| FieldIndex::index(f, building))
            .collect();
        for (i, f) in proto.field.iter().enumerate() {
            let field_index = &fields[i];

            assert!(index_by_number.insert(f.get_number() as u32, i).is_none());
            assert!(index_by_name.insert(f.get_name().to_owned(), i).is_none());
            assert!(index_by_name_or_json_name
                .insert(f.get_name().to_owned(), i)
                .is_none());

            if field_index.json_name != f.get_name() {
                assert!(index_by_name_or_json_name
                    .insert(field_index.json_name.clone(), i)
                    .is_none());
            }
        }

        MessageIndex {
            fields,
            index_by_name,
            index_by_name_or_json_name,
            index_by_number,
        }
    }
}
