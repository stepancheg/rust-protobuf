use core::message_down_cast;
use core::Message;
use core::ProtobufEnum;
use std::default::Default;
use descriptor::FileDescriptorProto;
use descriptor::DescriptorProto;
use descriptor::FieldDescriptorProto;
use descriptor::EnumDescriptorProto;
use descriptor::EnumValueDescriptorProto;
use descriptor::FieldDescriptorProto_Label;
use descriptorx::find_enum_by_rust_name;
use descriptorx::find_message_by_rust_name;
use std::collections::HashMap;


/// this trait should not be used directly, use `FieldDescriptor` instead
pub trait FieldAccessor<M : Message> {
    fn name(&self) -> &'static str;

    fn has_field(&self, _m: &M) -> bool {
        panic!();
    }

    fn len_field(&self, _m: &M) -> uint {
        panic!();
    }

    fn get_message<'a>(&self, _m: &'a M) -> &'a Message {
        panic!();
    }

    fn get_rep_message_item<'a>(&self, _m: &'a M, _index: uint) -> &'a Message {
        panic!();
    }

    fn get_enum(&self, _m: &M) -> &'static EnumValueDescriptor {
        panic!();
    }

    fn get_rep_enum_item(&self, _m: &M, _index: uint) -> &'static EnumValueDescriptor {
        panic!();
    }

    fn get_str<'a>(&self, _m: &'a M) -> &'a str {
        panic!();
    }

    fn get_rep_str<'a>(&self, _m: &'a M) -> &'a [String] {
        panic!();
    }

    fn get_bytes<'a>(&self, _m: &'a M) -> &'a [u8] {
        panic!();
    }

    fn get_rep_bytes<'a>(&self, _m: &'a M) -> &'a [Vec<u8>] {
        panic!();
    }

    fn get_u32(&self, _m: &M) -> u32 {
        panic!();
    }

    fn get_rep_u32<'a>(&self, _m: &'a M) -> &'a [u32] {
        panic!();
    }

    fn get_u64(&self, _m: &M) -> u64 {
        panic!();
    }

    fn get_rep_u64<'a>(&self, _m: &'a M) -> &'a [u64] {
        panic!();
    }

    fn get_i32(&self, _m: &M) -> i32 {
        panic!();
    }

    fn get_rep_i32<'a>(&self, _m: &'a M) -> &'a [i32] {
        panic!();
    }

    fn get_i64(&self, _m: &M) -> i64 {
        panic!();
    }

    fn get_rep_i64<'a>(&self, _m: &'a M) -> &'a [i64] {
        panic!();
    }

    fn get_bool(&self, _m: &M) -> bool {
        panic!();
    }

    fn get_rep_bool<'a>(&self, _m: &'a M) -> &'a [bool] {
        panic!();
    }

    fn get_f32(&self, _m: &M) -> f32 {
        panic!();
    }

    fn get_rep_f32<'a>(&self, _m: &'a M) -> &'a [f32] {
        panic!();
    }

    fn get_f64(&self, _m: &M) -> f64 {
        panic!();
    }

    fn get_rep_f64<'a>(&self, _m: &'a M) -> &'a [f64] {
        panic!();
    }
}


trait FieldAccessorGeneric {
    fn has_field_generic(&self, m: &Message) -> bool;
    fn len_field_generic(&self, m: &Message) -> uint;
    fn get_message_generic<'a>(&self, m: &'a Message) -> &'a Message;
    fn get_rep_message_item_generic<'a>(&self, m: &'a Message, index: uint) -> &'a Message;
    fn get_enum_generic(&self, m: &Message) -> &'static EnumValueDescriptor;
    fn get_rep_enum_item_generic(&self, m: &Message, index: uint) -> &'static EnumValueDescriptor;
    fn get_str_generic<'a>(&self, m: &'a Message) -> &'a str;
    fn get_rep_str_generic<'a>(&self, m: &'a Message) -> &'a [String];
    fn get_bytes_generic<'a>(&self, m: &'a Message) -> &'a [u8];
    fn get_rep_bytes_generic<'a>(&self, m: &'a Message) -> &'a [Vec<u8>];
    fn get_u32_generic(&self, m: &Message) -> u32;
    fn get_rep_u32_generic<'a>(&self, m: &'a Message) -> &'a [u32];
    fn get_u64_generic(&self, m: &Message) -> u64;
    fn get_rep_u64_generic<'a>(&self, m: &'a Message) -> &'a [u64];
    fn get_i32_generic(&self, m: &Message) -> i32;
    fn get_rep_i32_generic<'a>(&self, m: &'a Message) -> &'a [i32];
    fn get_i64_generic(&self, m: &Message) -> i64;
    fn get_rep_i64_generic<'a>(&self, m: &'a Message) -> &'a [i64];
    fn get_bool_generic(&self, m: &Message) -> bool;
    fn get_rep_bool_generic<'a>(&self, m: &'a Message) -> &'a [bool];
    fn get_f32_generic(&self, m: &Message) -> f32;
    fn get_rep_f32_generic<'a>(&self, m: &'a Message) -> &'a [f32];
    fn get_f64_generic(&self, m: &Message) -> f64;
    fn get_rep_f64_generic<'a>(&self, m: &'a Message) -> &'a [f64];
}

struct FieldAccessorGenericImpl<M> {
    accessor: &'static (FieldAccessor<M> + 'static)
}

impl<M : Message> FieldAccessorGenericImpl<M> {
    fn new(a: &'static (FieldAccessor<M> + 'static)) -> FieldAccessorGenericImpl<M> {
        FieldAccessorGenericImpl {
            accessor: a
        }
    }
}

impl<M : 'static + Message> FieldAccessorGeneric for FieldAccessorGenericImpl<M> {
    fn has_field_generic(&self, m: &Message) -> bool {
        self.accessor.has_field(message_down_cast(m))
    }

    fn len_field_generic(&self, m: &Message) -> uint {
        self.accessor.len_field(message_down_cast(m))
    }

    fn get_message_generic<'a>(&self, m: &'a Message) -> &'a Message {
        self.accessor.get_message(message_down_cast(m))
    }

    fn get_rep_message_item_generic<'a>(&self, m: &'a Message, index: uint) -> &'a Message {
        self.accessor.get_rep_message_item(message_down_cast(m), index)
    }

    fn get_enum_generic(&self, m: &Message) -> &'static EnumValueDescriptor {
        self.accessor.get_enum(message_down_cast(m))
    }

    fn get_rep_enum_item_generic(&self, m: &Message, index: uint) -> &'static EnumValueDescriptor {
        self.accessor.get_rep_enum_item(message_down_cast(m), index)
    }

    fn get_str_generic<'a>(&self, m: &'a Message) -> &'a str {
        self.accessor.get_str(message_down_cast(m))
    }

    fn get_rep_str_generic<'a>(&self, m: &'a Message) -> &'a [String] {
        self.accessor.get_rep_str(message_down_cast(m))
    }

    fn get_bytes_generic<'a>(&self, m: &'a Message) -> &'a [u8] {
        self.accessor.get_bytes(message_down_cast(m))
    }

    fn get_rep_bytes_generic<'a>(&self, m: &'a Message) -> &'a [Vec<u8>] {
        self.accessor.get_rep_bytes(message_down_cast(m))
    }

    fn get_u32_generic(&self, m: &Message) -> u32 {
        self.accessor.get_u32(message_down_cast(m))
    }

    fn get_rep_u32_generic<'a>(&self, m: &'a Message) -> &'a [u32] {
        self.accessor.get_rep_u32(message_down_cast(m))
    }

    fn get_u64_generic(&self, m: &Message) -> u64 {
        self.accessor.get_u64(message_down_cast(m))
    }

    fn get_rep_u64_generic<'a>(&self, m: &'a Message) -> &'a [u64] {
        self.accessor.get_rep_u64(message_down_cast(m))
    }

    fn get_i32_generic(&self, m: &Message) -> i32 {
        self.accessor.get_i32(message_down_cast(m))
    }

    fn get_rep_i32_generic<'a>(&self, m: &'a Message) -> &'a [i32] {
        self.accessor.get_rep_i32(message_down_cast(m))
    }

    fn get_i64_generic(&self, m: &Message) -> i64 {
        self.accessor.get_i64(message_down_cast(m))
    }

    fn get_rep_i64_generic<'a>(&self, m: &'a Message) -> &'a [i64] {
        self.accessor.get_rep_i64(message_down_cast(m))
    }

    fn get_bool_generic(&self, m: &Message) -> bool {
        self.accessor.get_bool(message_down_cast(m))
    }

    fn get_rep_bool_generic<'a>(&self, m: &'a Message) -> &'a [bool] {
        self.accessor.get_rep_bool(message_down_cast(m))
    }

    fn get_f32_generic(&self, m: &Message) -> f32 {
        self.accessor.get_f32(message_down_cast(m))
    }

    fn get_rep_f32_generic<'a>(&self, m: &'a Message) -> &'a [f32] {
        self.accessor.get_rep_f32(message_down_cast(m))
    }

    fn get_f64_generic(&self, m: &Message) -> f64 {
        self.accessor.get_f64(message_down_cast(m))
    }

    fn get_rep_f64_generic<'a>(&self, m: &'a Message) -> &'a [f64] {
        self.accessor.get_rep_f64(message_down_cast(m))
    }
}

pub struct FieldDescriptor {
    proto: &'static FieldDescriptorProto,
    accessor: Box<FieldAccessorGeneric + 'static>,
}

impl FieldDescriptor {
    fn new<M : 'static + Message>(a: &'static (FieldAccessor<M> + 'static), proto: &'static FieldDescriptorProto)
        -> FieldDescriptor
    {
        assert_eq!(proto.get_name(), a.name());
        FieldDescriptor {
            proto: proto,
            accessor: box FieldAccessorGenericImpl::new(a) as Box<FieldAccessorGeneric>,
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

    pub fn len_field(&self, m: &Message) -> uint {
        self.accessor.len_field_generic(m)
    }

    pub fn get_message<'a>(&self, m: &'a Message) -> &'a Message {
        self.accessor.get_message_generic(m)
    }

    pub fn get_rep_message_item<'a>(&self, m: &'a Message, index: uint) -> &'a Message {
        self.accessor.get_rep_message_item_generic(m, index)
    }

    pub fn get_enum(&self, m: &Message) -> &'static EnumValueDescriptor {
        self.accessor.get_enum_generic(m)
    }

    pub fn get_rep_enum_item(&self, m: &Message, index: uint) -> &'static EnumValueDescriptor {
        self.accessor.get_rep_enum_item_generic(m, index)
    }

    pub fn get_str<'a>(&self, m: &'a Message) -> &'a str {
        self.accessor.get_str_generic(m)
    }

    pub fn get_rep_str<'a>(&self, m: &'a Message) -> &'a [String] {
        self.accessor.get_rep_str_generic(m)
    }

    pub fn get_rep_str_item<'a>(&self, m: &'a Message, index: uint) -> &'a str {
        self.get_rep_str(m)[index].as_slice()
    }

    pub fn get_bytes<'a>(&self, m: &'a Message) -> &'a [u8] {
        self.accessor.get_bytes_generic(m)
    }

    pub fn get_rep_bytes<'a>(&self, m: &'a Message) -> &'a [Vec<u8>] {
        self.accessor.get_rep_bytes_generic(m)
    }

    pub fn get_rep_bytes_item<'a>(&self, m: &'a Message, index: uint) -> &'a [u8] {
        self.get_rep_bytes(m)[index].as_slice()
    }

    pub fn get_u32(&self, m: &Message) -> u32 {
        self.accessor.get_u32_generic(m)
    }

    pub fn get_rep_u32<'a>(&self, m: &'a Message) -> &'a [u32] {
        self.accessor.get_rep_u32_generic(m)
    }

    pub fn get_u64(&self, m: &Message) -> u64 {
        self.accessor.get_u64_generic(m)
    }

    pub fn get_rep_u64<'a>(&self, m: &'a Message) -> &'a [u64] {
        self.accessor.get_rep_u64_generic(m)
    }

    pub fn get_i32(&self, m: &Message) -> i32 {
        self.accessor.get_i32_generic(m)
    }

    pub fn get_rep_i32<'a>(&self, m: &'a Message) -> &'a [i32] {
        self.accessor.get_rep_i32_generic(m)
    }

    pub fn get_i64(&self, m: &Message) -> i64 {
        self.accessor.get_i64_generic(m)
    }

    pub fn get_rep_i64<'a>(&self, m: &'a Message) -> &'a [i64] {
        self.accessor.get_rep_i64_generic(m)
    }

    pub fn get_bool(&self, m: &Message) -> bool {
        self.accessor.get_bool_generic(m)
    }

    pub fn get_rep_bool<'a>(&self, m: &'a Message) -> &'a [bool] {
        self.accessor.get_rep_bool_generic(m)
    }

    pub fn get_f32(&self, m: &Message) -> f32 {
        self.accessor.get_f32_generic(m)
    }

    pub fn get_rep_f32<'a>(&self, m: &'a Message) -> &'a [f32] {
        self.accessor.get_rep_f32_generic(m)
    }

    pub fn get_f64(&self, m: &Message) -> f64 {
        self.accessor.get_f64_generic(m)
    }

    pub fn get_rep_f64<'a>(&self, m: &'a Message) -> &'a [f64] {
        self.accessor.get_rep_f64_generic(m)
    }
}


trait MessageFactory {
    fn new_instance(&self) -> Box<Message>;
}

struct MessageFactoryTyped<M> {
    _dummy: ()
}

impl<M> MessageFactoryTyped<M> {
    fn new() -> MessageFactoryTyped<M> {
        MessageFactoryTyped {
            _dummy: ()
        }
    }
}

impl<M : 'static + Message> MessageFactory for MessageFactoryTyped<M> {
    fn new_instance(&self) -> Box<Message> {
        let m: M = Default::default();
        box m as Box<Message>
    }
}

pub struct MessageDescriptor {
    proto: &'static DescriptorProto,
    factory: Box<MessageFactory + 'static>,
    fields: Vec<FieldDescriptor>,

    index_by_name: HashMap<String, uint>,
    index_by_number: HashMap<u32, uint>,
}

impl MessageDescriptor {
    pub fn for_type<M : Message>() -> &'static MessageDescriptor {
        Message::descriptor_static(None::<M>)
    }

    pub fn new<M : 'static + Message>(
            rust_name: &'static str,
            fields: Vec<&'static (FieldAccessor<M> + 'static)>,
            file: &'static FileDescriptorProto
        ) -> MessageDescriptor
    {
        let proto = find_message_by_rust_name(file, rust_name);

        let mut field_proto_by_name = HashMap::new();
        for field_proto in proto.message.get_field().iter() {
            field_proto_by_name.insert(field_proto.get_name(), field_proto);
        }

        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, f) in proto.message.get_field().iter().enumerate() {
            index_by_number.insert(f.get_number() as u32, i);
            index_by_name.insert(f.get_name().to_string(), i);
        }

        MessageDescriptor {
            proto: proto.message,
            factory: box MessageFactoryTyped::<M>::new() as Box<MessageFactory>,
            fields: fields.iter()
                    .map(|f| FieldDescriptor::new(*f, *field_proto_by_name.get(&f.name()).unwrap()))
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

    pub fn fields<'a>(&'a self) -> &'a [FieldDescriptor] {
        self.fields.as_slice()
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

pub struct EnumValueDescriptor {
    proto: &'static EnumValueDescriptorProto,
}

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

    index_by_name: HashMap<String, uint>,
    index_by_number: HashMap<i32, uint>,
}

impl EnumDescriptor {
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    pub fn for_type<E : ProtobufEnum>() -> &'static EnumDescriptor {
        ProtobufEnum::enum_descriptor_static(None::<E>)
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
            values: proto.en.get_value().iter().map(|v| EnumValueDescriptor { proto: v }).collect(),
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
