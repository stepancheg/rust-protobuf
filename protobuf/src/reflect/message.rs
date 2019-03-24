use std::collections::HashMap;
use std::marker;

use Message;

use descriptor::DescriptorProto;
use descriptor::FileDescriptorProto;

use descriptorx::FileScope;

use reflect::accessor::FieldAccessor;
use reflect::reflect_deep_eq::ReflectDeepEq;
use reflect::FieldDescriptor;

use descriptorx::WithScope;
use json;

trait MessageFactory: Send + Sync + 'static {
    fn new_instance(&self) -> Box<Message>;
    fn default_instance(&self) -> &Message;
    fn clone(&self, message: &Message) -> Box<Message>;
    fn eq(&self, a: &Message, b: &Message) -> bool;
}

struct MessageFactoryImpl<M>(marker::PhantomData<M>);

impl<M> MessageFactory for MessageFactoryImpl<M>
where
    M: 'static + Message + Default + Clone + PartialEq,
{
    fn new_instance(&self) -> Box<Message> {
        let m: M = Default::default();
        Box::new(m)
    }

    fn default_instance(&self) -> &Message {
        M::default_instance() as &Message
    }

    fn clone(&self, message: &Message) -> Box<Message> {
        let m: &M = message.downcast_ref().expect("wrong message type");
        Box::new(m.clone())
    }

    fn eq(&self, a: &Message, b: &Message) -> bool {
        let a: &M = a.downcast_ref().expect("wrong message type");
        let b: &M = b.downcast_ref().expect("wrong message type");
        a == b
    }
}

pub struct MessageDescriptor {
    full_name: String,
    file_descriptor_proto: &'static FileDescriptorProto,
    proto: &'static DescriptorProto,
    factory: &'static MessageFactory,
    fields: Vec<FieldDescriptor>,

    index_by_name: HashMap<String, usize>,
    index_by_name_or_json_name: HashMap<String, usize>,
    index_by_number: HashMap<u32, usize>,
}

// find message by rust type name
fn find_message_by_rust_name<'a>(
    fd: &'a FileDescriptorProto,
    rust_name: &str,
) -> &'a DescriptorProto {
    FileScope {
        file_descriptor: fd,
    }.find_messages()
        .into_iter()
        .find(|m| m.rust_name() == rust_name)
        .unwrap()
        .message
}

impl MessageDescriptor {
    /// Get underlying `DescriptorProto` object.
    pub fn get_proto(&self) -> &DescriptorProto {
        self.proto
    }

    pub fn for_type<M: Message>() -> &'static MessageDescriptor {
        M::descriptor_static()
    }

    fn compute_full_name(package: &str, proto: &DescriptorProto) -> String {
        let mut full_name = package.to_owned();
        if full_name.len() > 0 {
            full_name.push('.');
        }
        full_name.push_str(&proto.name_to_package());
        full_name
    }

    // Non-generic part of `new` is a separate function
    // to reduce code bloat from multiple instantiations.
    fn new_non_generic(
        rust_name: &'static str,
        fields: Vec<FieldAccessor>,
        file_descriptor_proto: &'static FileDescriptorProto,
        factory: &'static MessageFactory,
    ) -> MessageDescriptor {
        let proto = find_message_by_rust_name(file_descriptor_proto, rust_name);

        let mut field_proto_by_name = HashMap::new();
        for field_proto in &proto.field {
            field_proto_by_name.insert(field_proto.get_name(), field_proto);
        }

        let mut index_by_name = HashMap::new();
        let mut index_by_name_or_json_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, f) in proto.field.iter().enumerate() {
            assert!(index_by_number.insert(f.get_number() as u32, i).is_none());
            assert!(index_by_name.insert(f.get_name().to_owned(), i).is_none());
            assert!(
                index_by_name_or_json_name
                    .insert(f.get_name().to_owned(), i)
                    .is_none()
            );

            let json_name = json::json_name(f.get_name());

            if !f.get_json_name().is_empty() {
                assert_eq!(json_name, f.get_json_name());
            }

            if json_name != f.get_name() {
                assert!(index_by_name_or_json_name.insert(json_name, i).is_none());
            }
        }

        MessageDescriptor {
            full_name: MessageDescriptor::compute_full_name(file_descriptor_proto.get_package(), &proto),
            proto,
            factory,
            fields: fields
                .into_iter()
                .map(|f| {
                    let proto = *field_proto_by_name.get(f.name).unwrap();
                    FieldDescriptor::new(f, proto)
                }).collect(),
            index_by_name,
            index_by_name_or_json_name,
            index_by_number,
            file_descriptor_proto,
        }
    }

    pub fn new<M: 'static + Message + Default + Clone + PartialEq>(
        rust_name: &'static str,
        fields: Vec<FieldAccessor>,
        file_descriptor_proto: &'static FileDescriptorProto,
    ) -> MessageDescriptor {
        let factory = &MessageFactoryImpl(marker::PhantomData::<M>);
        MessageDescriptor::new_non_generic(rust_name, fields, file_descriptor_proto, factory)
    }

    pub fn file_descriptor_proto(&self) -> &FileDescriptorProto {
        self.file_descriptor_proto
    }

    /// New empty message
    pub fn new_instance(&self) -> Box<Message> {
        self.factory.new_instance()
    }

    /// Shared immutable empty message
    pub fn default_instance(&self) -> &Message {
        self.factory.default_instance()
    }

    /// Clone a message
    pub fn clone(&self, message: &Message) -> Box<Message> {
        self.factory.clone(message)
    }

    /// Check if two messages equal.
    ///
    /// Panic is any message has different type than this descriptor.
    pub fn eq(&self, a: &Message, b: &Message) -> bool {
        self.factory.eq(a, b)
    }

    /// Similar to `eq`, but considers `NaN` values equal.
    ///
    /// Panics is any message has different type than this descriptor.
    pub fn deep_eq(&self, a: &Message, b: &Message) -> bool {
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
            if !af.reflect_deep_eq(&bf) {
                return false;
            }
        }
        true
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

    /// Find message field by field name
    pub fn field_by_name<'a>(&'a self, name: &str) -> Option<&'a FieldDescriptor> {
        let &index = self.index_by_name.get(name)?;
        Some(&self.fields[index])
    }

    /// Find message field by field name or field JSON name
    pub fn field_by_name_or_json_name<'a>(&'a self, name: &str) -> Option<&'a FieldDescriptor> {
        let &index = self.index_by_name_or_json_name.get(name)?;
        Some(&self.fields[index])
    }

    /// Find message field by field name
    pub fn field_by_number<'a>(&'a self, number: u32) -> Option<&'a FieldDescriptor> {
        let &index = self.index_by_number.get(&number)?;
        Some(&self.fields[index])
    }

    pub fn cast<M: 'static>(&self, message: Box<Message>) -> Result<M, Box<Message>> {
        message.downcast_box::<M>().map(|m| *m)
    }
}

/// Identity comparison: message descriptor are equal if their addresses are equal
impl PartialEq for MessageDescriptor {
    fn eq(&self, other: &MessageDescriptor) -> bool {
        self as *const MessageDescriptor == other as *const MessageDescriptor
    }
}
