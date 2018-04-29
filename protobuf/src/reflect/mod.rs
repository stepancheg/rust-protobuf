//! Reflection implementation for protobuf types.

use std::collections::HashMap;
use std::default::Default;
use std::marker;

use core::Message;
use enums::ProtobufEnum;
use descriptor::FileDescriptorProto;
use descriptor::DescriptorProto;
use descriptor::FieldDescriptorProto;
use descriptor::EnumDescriptorProto;
use descriptor::EnumValueDescriptorProto;
use descriptor::FieldDescriptorProto_Label;
use descriptorx::find_enum_by_rust_name;
use descriptorx::find_message_by_rust_name;
use reflect::accessor::FieldAccessor;


pub mod accessor;
mod map;
mod repeated;
mod value;
mod optional;

use self::repeated::ReflectRepeated;
use self::map::ReflectMap;

pub use self::value::ProtobufValue;
pub use self::value::ProtobufValueRef;


pub struct FieldDescriptor {
    proto: &'static FieldDescriptorProto,
    accessor: Box<FieldAccessor + 'static>,
}

impl FieldDescriptor {
    fn new(
        a: Box<FieldAccessor + 'static>,
        proto: &'static FieldDescriptorProto,
    ) -> FieldDescriptor {
        assert_eq!(proto.get_name(), a.name_generic());
        FieldDescriptor {
            proto: proto,
            accessor: a,
        }
    }

    pub fn proto(&self) -> &'static FieldDescriptorProto {
        self.proto
    }

    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    pub fn is_repeated(&self) -> bool {
        self.proto.get_label() == FieldDescriptorProto_Label::LABEL_REPEATED
    }

    pub fn has_field(&self, m: &Message) -> bool {
        self.accessor.has_field_generic(m)
    }

    pub fn len_field(&self, m: &Message) -> usize {
        self.accessor.len_field_generic(m)
    }

    pub fn get_message<'a>(&self, m: &'a Message) -> &'a Message {
        self.accessor.get_message_generic(m)
    }

    pub fn get_enum(&self, m: &Message) -> &'static EnumValueDescriptor {
        self.accessor.get_enum_generic(m)
    }

    pub fn get_str<'a>(&self, m: &'a Message) -> &'a str {
        self.accessor.get_str_generic(m)
    }

    pub fn get_bytes<'a>(&self, m: &'a Message) -> &'a [u8] {
        self.accessor.get_bytes_generic(m)
    }

    pub fn get_u32(&self, m: &Message) -> u32 {
        self.accessor.get_u32_generic(m)
    }

    pub fn get_u64(&self, m: &Message) -> u64 {
        self.accessor.get_u64_generic(m)
    }

    pub fn get_i32(&self, m: &Message) -> i32 {
        self.accessor.get_i32_generic(m)
    }

    pub fn get_i64(&self, m: &Message) -> i64 {
        self.accessor.get_i64_generic(m)
    }

    pub fn get_bool(&self, m: &Message) -> bool {
        self.accessor.get_bool_generic(m)
    }

    pub fn get_f32(&self, m: &Message) -> f32 {
        self.accessor.get_f32_generic(m)
    }

    pub fn get_f64(&self, m: &Message) -> f64 {
        self.accessor.get_f64_generic(m)
    }

    pub fn get_reflect<'a>(&self, m: &'a Message) -> ReflectFieldRef<'a> {
        self.accessor.get_reflect(m)
    }
}


trait MessageFactory {
    fn new_instance(&self) -> Box<Message>;
}

struct MessageFactoryTyped<M> {
    _dummy: (),
    _phantom_data: marker::PhantomData<M>,
}

impl<M> MessageFactoryTyped<M> {
    fn new() -> MessageFactoryTyped<M> {
        MessageFactoryTyped {
            _dummy: (),
            _phantom_data: marker::PhantomData,
        }
    }
}

impl<M : 'static + Message + Default> MessageFactory for MessageFactoryTyped<M> {
    fn new_instance(&self) -> Box<Message> {
        let m: M = Default::default();
        Box::new(m)
    }
}

pub struct MessageDescriptor {
    full_name: String,
    proto: &'static DescriptorProto,
    factory: Box<MessageFactory + 'static>,
    fields: Vec<FieldDescriptor>,

    index_by_name: HashMap<String, usize>,
    index_by_number: HashMap<u32, usize>,
}

impl MessageDescriptor {
    pub fn for_type<M : Message>() -> &'static MessageDescriptor {
        M::descriptor_static()
    }

    pub fn new<M : 'static + Message + Default>(
        rust_name: &'static str,
        fields: Vec<Box<FieldAccessor + 'static>>,
        file: &'static FileDescriptorProto,
    ) -> MessageDescriptor {
        let proto = find_message_by_rust_name(file, rust_name);

        let mut field_proto_by_name = HashMap::new();
        for field_proto in proto.message.get_field() {
            field_proto_by_name.insert(field_proto.get_name(), field_proto);
        }

        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, f) in proto.message.get_field().iter().enumerate() {
            index_by_number.insert(f.get_number() as u32, i);
            index_by_name.insert(f.get_name().to_string(), i);
        }

        let mut full_name = file.get_package().to_string();
        if full_name.len() > 0 {
            full_name.push('.');
        }
        full_name.push_str(proto.message.get_name());

        MessageDescriptor {
            full_name: full_name,
            proto: proto.message,
            factory: Box::new(MessageFactoryTyped::<M>::new()),
            fields: fields
                .into_iter()
                .map(|f| {
                    let proto = *field_proto_by_name.get(&f.name_generic()).unwrap();
                    FieldDescriptor::new(f, proto)
                })
                .collect(),
            index_by_name: index_by_name,
            index_by_number: index_by_number,
        }
    }

    pub fn new_instance(&self) -> Box<Message> {
        self.factory.new_instance()
    }

    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name[..]
    }

    pub fn fields<'a>(&'a self) -> &'a [FieldDescriptor] {
        &self.fields
    }

    pub fn field_by_name<'a>(&'a self, name: &str) -> &'a FieldDescriptor {
        // TODO: clone is weird
        let &index = self.index_by_name.get(&name.to_string()).unwrap();
        &self.fields[index]
    }

    pub fn field_by_number<'a>(&'a self, number: u32) -> &'a FieldDescriptor {
        let &index = self.index_by_number.get(&number).unwrap();
        &self.fields[index]
    }
}

#[derive(Clone)]
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


pub enum ReflectFieldRef<'a> {
    Repeated(&'a ReflectRepeated),
    Map(&'a ReflectMap),
    Optional(Option<ProtobufValueRef<'a>>),
}
