use descriptor::{DescriptorProto, FileDescriptorProto};
use descriptorx::find_message_by_rust_name;
use reflect::accessor::FieldAccessor;
use reflect::FieldDescriptor;
use std::collections::HashMap;
use std::marker;
use Message;
use core::message_down_cast;

trait MessageFactory: Send + Sync + 'static {
    fn new_instance(&self) -> Box<dyn Message>;
    fn default_instance(&self) -> &dyn Message;
    fn clone(&self, message: &dyn Message) -> Box<dyn Message>;
}

struct MessageFactoryImpl<M>(marker::PhantomData<M>);

impl<M> MessageFactory for MessageFactoryImpl<M>
where
    M: 'static + Message + Default + Clone + PartialEq,
{
    fn new_instance(&self) -> Box<dyn Message> {
        let m: M = Default::default();
        Box::new(m)
    }

    fn default_instance(&self) -> &dyn Message {
        M::default_instance() as &dyn Message
    }

    fn clone(&self, message: &dyn Message) -> Box<dyn Message> {
        let m: &M = message_down_cast(message);
        Box::new(m.clone())
    }
}

/// Dynamic message type
pub struct MessageDescriptor {
    full_name: String,
    proto: &'static DescriptorProto,
    factory: &'static dyn MessageFactory,
    fields: Vec<FieldDescriptor>,

    index_by_name: HashMap<String, usize>,
    index_by_number: HashMap<u32, usize>,
}

impl MessageDescriptor {
    /// Get underlying `DescriptorProto` object.
    pub fn get_proto(&self) -> &DescriptorProto {
        self.proto
    }

    /// Get a message descriptor for given message type
    pub fn for_type<M: Message>() -> &'static MessageDescriptor {
        M::descriptor_static()
    }

    // Non-generic part of `new` is a separate function
    // to reduce code bloat from multiple instantiations.
    fn new_non_generic(
        rust_name: &'static str,
        fields: Vec<Box<FieldAccessor + 'static>>,
        file: &'static FileDescriptorProto,
        factory: &'static dyn MessageFactory,
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
            factory,
            fields: fields
                .into_iter()
                .map(|f| {
                    let proto = *field_proto_by_name.get(&f.name_generic()).unwrap();
                    FieldDescriptor::new(f, proto)
                })
                .collect(),
            index_by_name,
            index_by_number,
        }
    }

    /// Construct a new message descriptor.
    ///
    /// This operation is called from generated code and rarely
    /// need to be called directly.
    ///
    /// This function is not a part of public API.
    #[doc(hidden)]
    pub fn new<M: 'static + Message + Default + Clone + PartialEq>(
        rust_name: &'static str,
        fields: Vec<Box<FieldAccessor + 'static>>,
        file: &'static FileDescriptorProto,
    ) -> MessageDescriptor {
        let factory = &MessageFactoryImpl(marker::PhantomData::<M>);
        MessageDescriptor::new_non_generic(
            rust_name,
            fields,
            file,
            factory,
        )
    }

    /// New empty message
    pub fn new_instance(&self) -> Box<dyn Message> {
        self.factory.new_instance()
    }

    /// Shared immutable empty message
    pub fn default_instance(&self) -> &dyn Message {
        self.factory.default_instance()
    }

    /// Clone a message
    pub fn clone(&self, message: &dyn Message) -> Box<dyn Message> {
        self.factory.clone(message)
    }

    /// Message name as given in `.proto` file
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    /// Fully qualified protobuf message name
    pub fn full_name(&self) -> &str {
        &self.full_name[..]
    }

    /// Message field descriptors.
    pub fn fields(&self) -> &[FieldDescriptor] {
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
