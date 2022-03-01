use std::collections::HashMap;

use crate::descriptor::DescriptorProto;
use crate::reflect::field::index::FieldIndex;
use crate::reflect::file::building::FileDescriptorBuilding;

#[derive(Debug)]
pub(crate) struct MessageIndex {
    pub(crate) fields: Vec<FieldIndex>,
    pub(crate) field_index_by_name: HashMap<String, usize>,
    pub(crate) field_index_by_name_or_json_name: HashMap<String, usize>,
    pub(crate) field_index_by_number: HashMap<u32, usize>,
    pub(crate) extensions: Vec<FieldIndex>,
}

impl MessageIndex {
    pub(crate) fn index(
        proto: &DescriptorProto,
        building: &FileDescriptorBuilding,
    ) -> crate::Result<MessageIndex> {
        let mut index_by_name = HashMap::new();
        let mut index_by_name_or_json_name = HashMap::new();
        let mut index_by_number = HashMap::new();

        let fields: Vec<FieldIndex> = proto
            .field
            .iter()
            .map(|f| FieldIndex::index(f, building))
            .collect::<crate::Result<_>>()?;
        for (i, f) in proto.field.iter().enumerate() {
            let field_index = &fields[i];

            assert!(index_by_number.insert(f.number() as u32, i).is_none());
            assert!(index_by_name.insert(f.name().to_owned(), i).is_none());
            assert!(index_by_name_or_json_name
                .insert(f.name().to_owned(), i)
                .is_none());

            if field_index.json_name != f.name() {
                assert!(index_by_name_or_json_name
                    .insert(field_index.json_name.clone(), i)
                    .is_none());
            }
        }

        let extensions: Vec<FieldIndex> = proto
            .extension
            .iter()
            .map(|f| FieldIndex::index(f, building))
            .collect::<crate::Result<Vec<_>>>()?;

        Ok(MessageIndex {
            fields,
            field_index_by_name: index_by_name,
            field_index_by_name_or_json_name: index_by_name_or_json_name,
            field_index_by_number: index_by_number,
            extensions,
        })
    }
}
