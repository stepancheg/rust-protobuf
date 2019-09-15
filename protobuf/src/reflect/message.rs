use crate::core::message_down_cast;
use crate::descriptor::{DescriptorProto, FileDescriptorProto};
use crate::descriptorx::find_message_by_rust_name;
use crate::reflect::accessor::FieldAccessor;
use crate::reflect::reflect_eq::ReflectEq;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::FieldDescriptor;
use crate::Message;
use std::collections::HashMap;
use std::marker;

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
    file_descriptor_proto: &'static FileDescriptorProto,
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
        fields: Vec<FieldAccessor>,
        file_descriptor_proto: &'static FileDescriptorProto,
        factory: &'static dyn MessageFactory,
    ) -> MessageDescriptor {
        let proto = find_message_by_rust_name(file_descriptor_proto, rust_name);

        let mut field_proto_by_name = HashMap::new();
        for field_proto in &proto.message.field {
            field_proto_by_name.insert(field_proto.get_name(), field_proto);
        }

        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();

        let fields: Vec<_> = fields
            .into_iter()
            .map(|f| {
                let proto = *field_proto_by_name.get(&f.name).unwrap();
                FieldDescriptor::new(f, proto)
            })
            .collect();

        for (i, f) in proto.message.field.iter().enumerate() {
            assert!(index_by_number.insert(f.get_number() as u32, i).is_none());
            assert!(index_by_name.insert(f.get_name().to_owned(), i).is_none());
        }

        let mut full_name = file_descriptor_proto.get_package().to_string();
        if full_name.len() > 0 {
            full_name.push('.');
        }
        full_name.push_str(proto.message.get_name());

        MessageDescriptor {
            full_name,
            proto: proto.message,
            factory,
            fields,
            index_by_name,
            index_by_number,
            file_descriptor_proto,
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
        fields: Vec<FieldAccessor>,
        file_descriptor_proto: &'static FileDescriptorProto,
    ) -> MessageDescriptor {
        let factory = &MessageFactoryImpl(marker::PhantomData::<M>);
        MessageDescriptor::new_non_generic(rust_name, fields, file_descriptor_proto, factory)
    }

    /// `FileDescriptorProto` containg this message type
    pub fn file_descriptor_proto(&self) -> &FileDescriptorProto {
        self.file_descriptor_proto
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

    /// Similar to `eq`, but considers `NaN` values equal.
    ///
    /// # Panics
    ///
    /// Is any message has different type than this descriptor.
    pub(crate) fn reflect_eq(
        &self,
        a: &dyn Message,
        b: &dyn Message,
        mode: &ReflectEqMode,
    ) -> bool {
        // Explicitly force panic even if field list is empty
        assert_eq!(
            self as *const MessageDescriptor,
            a.descriptor() as *const MessageDescriptor
        );
        assert_eq!(
            self as *const MessageDescriptor,
            b.descriptor() as *const MessageDescriptor
        );

        for field in self.fields() {
            let af = field.get_reflect(a);
            let bf = field.get_reflect(b);
            if !af.reflect_eq(&bf, mode) {
                return false;
            }
        }
        true
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

    /// Find message field by protobuf field name
    ///
    /// Note: protobuf field name might be different for Rust field name.
    pub fn get_field_by_name<'a>(&'a self, name: &str) -> Option<&'a FieldDescriptor> {
        let &index = self.index_by_name.get(name)?;
        Some(&self.fields[index])
    }

    /// Find message field by field name
    pub fn get_field_by_number(&self, number: u32) -> Option<&FieldDescriptor> {
        let &index = self.index_by_number.get(&number)?;
        Some(&self.fields[index])
    }

    /// Find field by name
    #[deprecated]
    pub fn field_by_name<'a>(&'a self, name: &str) -> &'a FieldDescriptor {
        self.get_field_by_name(name).unwrap()
    }

    /// Find field by number
    #[deprecated]
    pub fn field_by_number(&self, number: u32) -> &FieldDescriptor {
        self.get_field_by_number(number).unwrap()
    }
}

/// Identity comparison: message descriptor are equal if their addresses are equal
impl PartialEq for MessageDescriptor {
    fn eq(&self, other: &MessageDescriptor) -> bool {
        self as *const MessageDescriptor == other as *const MessageDescriptor
    }
}
