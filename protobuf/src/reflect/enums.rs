use std::fmt;
use std::collections::HashMap;

use descriptor::EnumValueDescriptorProto;
use descriptor::EnumDescriptorProto;
use ProtobufEnum;
use descriptor::FileDescriptorProto;
use descriptorx::find_enum_by_rust_name;
use reflect::ProtobufValue;


#[derive(Clone)]
pub struct EnumValueDescriptor {
    proto: &'static EnumValueDescriptorProto,
    protobuf_value: &'static ProtobufValue,
}

fn _assert_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<EnumValueDescriptor>();
}

impl fmt::Debug for EnumValueDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("EnumValueDescriptor")
            .field("proto", self.proto)
            .field("value", &"...")
            .finish()
    }
}

impl Copy for EnumValueDescriptor {}

impl EnumValueDescriptor {
    /// Name of enum variant as specified in proto file
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    /// `i32` value of the enum variant
    pub fn value(&self) -> i32 {
        self.proto.get_number()
    }

    /// Convert to generic `ProtobufValue`
    pub fn protobuf_value(&self) -> &'static ProtobufValue {
        self.protobuf_value
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

    pub fn new<E>(rust_name: &'static str, file: &'static FileDescriptorProto) -> EnumDescriptor
        where E : ProtobufEnum
    {
        let proto = find_enum_by_rust_name(file, rust_name);
        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, v) in proto.en.get_value().iter().enumerate() {
            index_by_number.insert(v.get_number(), i);
            index_by_name.insert(v.get_name().to_string(), i);
        }

        let proto_values = proto.en.get_value();
        let code_values = E::values();
        assert_eq!(proto_values.len(), code_values.len());

        let values = proto_values.iter().zip(code_values).map(|(p, c)| {
            EnumValueDescriptor {
                proto: p,
                protobuf_value: c,
            }
        }).collect();

        EnumDescriptor {
            proto: proto.en,
            values,
            index_by_name,
            index_by_number,
        }
    }

    pub fn values(&self) -> &[EnumValueDescriptor] {
        &self.values
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
