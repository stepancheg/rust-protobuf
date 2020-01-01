use std::collections::HashMap;
use std::marker;

use crate::core::Message;

use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;

use crate::reflect::accessor::FieldAccessor;
use crate::reflect::find_message_or_enum::find_message_or_enum;
use crate::reflect::find_message_or_enum::MessageOrEnum;
use crate::reflect::reflect_eq::{ReflectEq, ReflectEqMode};
use crate::reflect::FieldDescriptor;

trait MessageFactory: Send + Sync + 'static {
    fn new_instance(&self) -> Box<dyn Message>;
    fn default_instance(&self) -> &dyn Message;
    fn clone(&self, message: &dyn Message) -> Box<dyn Message>;
    fn eq(&self, a: &dyn Message, b: &dyn Message) -> bool;
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
        let m: &M = message.downcast_ref().expect("wrong message type");
        Box::new(m.clone())
    }

    fn eq(&self, a: &dyn Message, b: &dyn Message) -> bool {
        let a: &M = a.downcast_ref().expect("wrong message type");
        let b: &M = b.downcast_ref().expect("wrong message type");
        a == b
    }
}

/// Dynamic representation of message type.
///
/// Used for reflection.
pub struct MessageDescriptor {
    full_name: String,
    file_descriptor_proto: &'static FileDescriptorProto,
    proto: &'static DescriptorProto,
    factory: &'static dyn MessageFactory,
    fields: Vec<FieldDescriptor>,

    index_by_name: HashMap<String, usize>,
    index_by_name_or_json_name: HashMap<String, usize>,
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

    fn compute_full_name(package: &str, path_to_package: &str, proto: &DescriptorProto) -> String {
        let mut full_name = package.to_owned();
        if path_to_package.len() != 0 {
            if full_name.len() != 0 {
                full_name.push('.');
            }
            full_name.push_str(path_to_package);
        }
        if full_name.len() != 0 {
            full_name.push('.');
        }
        full_name.push_str(proto.get_name());
        full_name
    }

    // Non-generic part of `new` is a separate function
    // to reduce code bloat from multiple instantiations.
    fn new_non_generic(
        protobuf_name_to_package: &'static str,
        fields: Vec<FieldAccessor>,
        file_descriptor_proto: &'static FileDescriptorProto,
        factory: &'static dyn MessageFactory,
    ) -> MessageDescriptor {
        let (path_to_package, proto) =
            match find_message_or_enum(file_descriptor_proto, protobuf_name_to_package) {
                (path_to_package, MessageOrEnum::Message(m)) => (path_to_package, m),
                (_, MessageOrEnum::Enum(_)) => panic!("not a message"),
            };

        let mut field_proto_by_name = HashMap::new();
        for field_proto in &proto.field {
            field_proto_by_name.insert(field_proto.get_name(), field_proto);
        }

        let mut index_by_name = HashMap::new();
        let mut index_by_name_or_json_name = HashMap::new();
        let mut index_by_number = HashMap::new();

        let fields: Vec<_> = fields
            .into_iter()
            .map(|f| {
                let proto = *field_proto_by_name.get(f.name).unwrap();
                FieldDescriptor::new(f, proto)
            })
            .collect();

        for (i, f) in fields.iter().enumerate() {
            assert!(index_by_number
                .insert(f.proto().get_number() as u32, i)
                .is_none());
            assert!(index_by_name
                .insert(f.proto().get_name().to_owned(), i)
                .is_none());
            assert!(index_by_name_or_json_name
                .insert(f.proto().get_name().to_owned(), i)
                .is_none());

            let json_name = f.json_name().to_owned();

            if json_name != f.proto().get_name() {
                assert!(index_by_name_or_json_name.insert(json_name, i).is_none());
            }
        }
        MessageDescriptor {
            full_name: MessageDescriptor::compute_full_name(
                file_descriptor_proto.get_package(),
                &path_to_package,
                &proto,
            ),
            proto,
            factory,
            fields,
            index_by_name,
            index_by_name_or_json_name,
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
        protobuf_name_to_package: &'static str,
        fields: Vec<FieldAccessor>,
        file_descriptor_proto: &'static FileDescriptorProto,
    ) -> MessageDescriptor {
        let factory = &MessageFactoryImpl(marker::PhantomData::<M>);
        MessageDescriptor::new_non_generic(
            protobuf_name_to_package,
            fields,
            file_descriptor_proto,
            factory,
        )
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
    pub(crate) fn clone(&self, message: &dyn Message) -> Box<dyn Message> {
        self.factory.clone(message)
    }

    /// Check if two messages equal.
    ///
    /// # Panics
    ///
    /// Is any message has different type than this descriptor.
    pub fn eq(&self, a: &dyn Message, b: &dyn Message) -> bool {
        self.factory.eq(a, b)
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

    /// Find message field by field name or field JSON name
    pub fn get_field_by_name_or_json_name<'a>(&'a self, name: &str) -> Option<&'a FieldDescriptor> {
        let &index = self.index_by_name_or_json_name.get(name)?;
        Some(&self.fields[index])
    }

    /// Find message field by field name
    pub fn get_field_by_number(&self, number: u32) -> Option<&FieldDescriptor> {
        let &index = self.index_by_number.get(&number)?;
        Some(&self.fields[index])
    }
}

/// Identity comparison: message descriptor are equal if their addresses are equal
impl PartialEq for MessageDescriptor {
    fn eq(&self, other: &MessageDescriptor) -> bool {
        self as *const MessageDescriptor == other as *const MessageDescriptor
    }
}
