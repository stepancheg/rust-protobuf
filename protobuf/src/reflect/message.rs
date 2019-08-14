use descriptor::{DescriptorProto, FileDescriptorProto};
use descriptorx::find_message_by_rust_name;
use reflect::accessor::FieldAccessor;
use reflect::FieldDescriptor;
use std::collections::HashMap;
use std::marker;
use Message;

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

impl<M: 'static + Message + Default> MessageFactory for MessageFactoryTyped<M> {
    fn new_instance(&self) -> Box<Message> {
        let m: M = Default::default();
        Box::new(m)
    }
}

/// Dynamic message type
pub struct MessageDescriptor {
    full_name: String,
    proto: &'static DescriptorProto,
    factory: Box<MessageFactory + 'static>,
    fields: Vec<FieldDescriptor>,

    index_by_name: HashMap<String, usize>,
    index_by_number: HashMap<u32, usize>,
}

impl MessageDescriptor {
    /// Get underlying `DescriptorProto` object.
    pub fn get_proto(&self) -> &DescriptorProto {
        self.proto
    }

    /// Get message descriptor for given message type.
    pub fn for_type<M: Message>() -> &'static MessageDescriptor {
        M::descriptor_static()
    }

    /// Create new message descriptor.
    ///
    /// This function is called from generated code and rarely needed otherwise.
    pub fn new<M: 'static + Message + Default>(
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

    /// Create a new message of this type
    pub fn new_instance(&self) -> Box<Message> {
        self.factory.new_instance()
    }

    /// Protobuf message name
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    /// Full protobuf message name
    pub fn full_name(&self) -> &str {
        &self.full_name[..]
    }

    /// Get all fields
    pub fn fields<'a>(&'a self) -> &'a [FieldDescriptor] {
        &self.fields
    }

    /// Find field by name
    pub fn field_by_name<'a>(&'a self, name: &str) -> &'a FieldDescriptor {
        // TODO: clone is weird
        let &index = self.index_by_name.get(&name.to_string()).unwrap();
        &self.fields[index]
    }

    /// Find field by number
    pub fn field_by_number<'a>(&'a self, number: u32) -> &'a FieldDescriptor {
        let &index = self.index_by_number.get(&number).unwrap();
        &self.fields[index]
    }
}
