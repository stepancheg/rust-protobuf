use crate::descriptor::{EnumDescriptorProto, EnumValueDescriptorProto, FileDescriptorProto};
use crate::descriptorx::find_enum_by_rust_name;
use crate::ProtobufEnum;
use std::collections::HashMap;
use std::fmt;

/// Description for enum variant.
///
/// Used in reflection.
#[derive(Clone)]
pub struct EnumValueDescriptor {
    proto: &'static EnumValueDescriptorProto,
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
}

/// Dynamic representation of enum type.
///
/// Can be used in reflective operations.
pub struct EnumDescriptor {
    proto: &'static EnumDescriptorProto,
    values: Vec<EnumValueDescriptor>,

    index_by_name: HashMap<String, usize>,
    index_by_number: HashMap<i32, usize>,
}

impl EnumDescriptor {
    /// Enum name as given in `.proto` file
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    /// `EnumDescriptor` for enum type
    pub fn for_type<E: ProtobufEnum>() -> &'static EnumDescriptor {
        E::enum_descriptor_static()
    }

    /// Separate function to reduce generated code size bloat.
    fn make_indices(proto: &EnumDescriptorProto) -> (HashMap<String, usize>, HashMap<i32, usize>) {
        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, v) in proto.value.iter().enumerate() {
            index_by_number.insert(v.get_number(), i);
            index_by_name.insert(v.get_name().to_string(), i);
        }

        (index_by_name, index_by_number)
    }

    /// Construct `EnumDescriptor` given enum name and `FileDescriptorProto`.
    ///
    /// This function is called from generated code, and should rarely be called directly.
    ///
    /// This function is not a part of public API.
    #[doc(hidden)]
    pub fn new<E: ProtobufEnum>(
        rust_name: &'static str,
        file: &'static FileDescriptorProto,
    ) -> EnumDescriptor {
        let proto = find_enum_by_rust_name(file, rust_name);
        let (index_by_name, index_by_number) = EnumDescriptor::make_indices(proto.en);
        EnumDescriptor {
            proto: proto.en,
            values: proto
                .en
                .get_value()
                .iter()
                .map(|v| EnumValueDescriptor { proto: v })
                .collect(),
            index_by_name,
            index_by_number,
        }
    }

    /// This enum values
    pub fn values(&self) -> &[EnumValueDescriptor] {
        &self.values
    }

    /// Find enum value by name
    pub fn value_by_name<'a>(&'a self, name: &str) -> &'a EnumValueDescriptor {
        // TODO: clone is weird
        let &index = self.index_by_name.get(&name.to_string()).unwrap();
        &self.values[index]
    }

    /// Find enum value by number
    pub fn value_by_number<'a>(&'a self, number: i32) -> &'a EnumValueDescriptor {
        let &index = self.index_by_number.get(&number).unwrap();
        &self.values[index]
    }
}
