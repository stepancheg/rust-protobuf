use descriptor::EnumValueDescriptorProto;
use descriptor::EnumDescriptorProto;
use std::collections::HashMap;
use ProtobufEnum;
use descriptor::FileDescriptorProto;
use descriptorx::find_enum_by_rust_name;


// TODO: better `Debug`
#[derive(Clone, Debug)]
pub struct EnumValueDescriptor {
    proto: &'static EnumValueDescriptorProto,
}

impl Copy for EnumValueDescriptor {}

impl EnumValueDescriptor {
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    pub fn value(&self) -> i32 {
        self.proto.get_number()
    }
}

pub struct EnumDescriptor {
    proto: &'static EnumDescriptorProto,
    values: Vec<EnumValueDescriptor>,

    index_by_name: HashMap<String, usize>,
    index_by_number: HashMap<i32, usize>,
}

impl EnumDescriptor {
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    pub fn for_type<E : ProtobufEnum>() -> &'static EnumDescriptor {
        E::enum_descriptor_static()
    }

    pub fn new(rust_name: &'static str, file: &'static FileDescriptorProto) -> EnumDescriptor {
        let proto = find_enum_by_rust_name(file, rust_name);
        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, v) in proto.en.get_value().iter().enumerate() {
            index_by_number.insert(v.get_number(), i);
            index_by_name.insert(v.get_name().to_string(), i);
        }
        EnumDescriptor {
            proto: proto.en,
            values: proto
                .en
                .get_value()
                .iter()
                .map(|v| EnumValueDescriptor { proto: v })
                .collect(),
            index_by_name: index_by_name,
            index_by_number: index_by_number,
        }
    }

    pub fn value_by_name<'a>(&'a self, name: &str) -> &'a EnumValueDescriptor {
        // TODO: clone is weird
        let &index = self.index_by_name.get(&name.to_string()).unwrap();
        &self.values[index]
    }

    pub fn value_by_number<'a>(&'a self, number: i32) -> &'a EnumValueDescriptor {
        let &index = self.index_by_number.get(&number).unwrap();
        &self.values[index]
    }
}
