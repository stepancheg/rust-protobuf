use std::collections::HashMap;
use std::{fmt, ptr};

use descriptor::EnumDescriptorProto;
use descriptor::EnumValueDescriptorProto;
use descriptor::FileDescriptorProto;
use reflect::ProtobufValue;
use std::any::TypeId;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker;
use std::mem;
use ::{ProtobufEnum, ProtobufEnumOrUnknown};
use reflect::find_message_or_enum::find_message_or_enum;
use reflect::find_message_or_enum::MessageOrEnum;


#[derive(Clone)]
pub struct EnumValueDescriptor {
    proto: &'static EnumValueDescriptorProto,
    protobuf_value: &'static ProtobufValue,
    get_descriptor: &'static GetEnumDescriptor,
}

impl PartialEq for EnumValueDescriptor {
    fn eq(&self, other: &EnumValueDescriptor) -> bool {
        self.enum_descriptor() == other.enum_descriptor() && self.value() == other.value()
    }
}

impl Hash for EnumValueDescriptor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.value(), state)
    }
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

    pub fn enum_descriptor(&self) -> &EnumDescriptor {
        self.get_descriptor.descriptor()
    }

    pub fn cast<E: 'static>(&self) -> Option<E> {
        self.enum_descriptor().cast(self.value())
    }
}

trait GetEnumDescriptor: Send + Sync + 'static {
    fn descriptor(&self) -> &EnumDescriptor;
    unsafe fn copy_to(&self, value: i32, dest: *mut ());
}

struct GetDescriptorImpl<E: ProtobufEnum>(marker::PhantomData<E>);

impl<E: ProtobufEnum> GetEnumDescriptor for GetDescriptorImpl<E> {
    fn descriptor(&self) -> &EnumDescriptor {
        E::enum_descriptor_static()
    }

    unsafe fn copy_to(&self, value: i32, dest: *mut ()) {
        let e = E::from_i32(value).expect("unknown value");
        (&e as *const E).copy_to(dest as *mut E, 1);
    }
}

pub struct EnumDescriptor {
    full_name: String,
    proto: &'static EnumDescriptorProto,
    values: Vec<EnumValueDescriptor>,
    /// Type id of `<E>`
    type_id: TypeId,
    /// Type id of `<ProtobufEnumOrUnknown<E>>`
    enum_or_unknown_type_id: TypeId,

    get_descriptor: &'static GetEnumDescriptor,

    index_by_name: HashMap<String, usize>,
    index_by_number: HashMap<i32, usize>,
}

/// Identity comparison: message descriptor are equal if their addresses are equal
impl PartialEq for EnumDescriptor {
    fn eq(&self, other: &EnumDescriptor) -> bool {
        self as *const EnumDescriptor == other as *const EnumDescriptor
    }
}

impl EnumDescriptor {
    pub fn name(&self) -> &'static str {
        self.proto.get_name()
    }

    pub fn full_name(&self) -> &str {
        &self.full_name[..]
    }

    pub fn for_type<E: ProtobufEnum>() -> &'static EnumDescriptor {
        E::enum_descriptor_static()
    }

    fn compute_full_name(package: &str, path_to_package: &str, proto: &EnumDescriptorProto) -> String {
        let mut full_name = package.to_owned();
        if path_to_package.len() != 0 {
            if full_name.len()!= 0 {
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

    pub fn new<E>(name_in_file: &'static str, file: &'static FileDescriptorProto) -> EnumDescriptor
    where
        E: ProtobufEnum,
    {
        let (path_to_package, proto) = match find_message_or_enum(file, name_in_file) {
            (path_to_package, MessageOrEnum::Enum(e)) => (path_to_package, e),
            (_, MessageOrEnum::Message(_)) => panic!("not an enum"),
        };

        let mut index_by_name = HashMap::new();
        let mut index_by_number = HashMap::new();
        for (i, v) in proto.value.iter().enumerate() {
            index_by_number.insert(v.get_number(), i);
            index_by_name.insert(v.get_name().to_string(), i);
        }

        let proto_values = &proto.value;
        let code_values = E::values();
        assert_eq!(proto_values.len(), code_values.len());

        let get_descriptor = &GetDescriptorImpl(marker::PhantomData::<E>);

        let values = proto_values
            .iter()
            .zip(code_values)
            .map(|(p, c)| EnumValueDescriptor {
                proto: p,
                protobuf_value: c,
                get_descriptor,
            }).collect();

        EnumDescriptor {
            full_name: EnumDescriptor::compute_full_name(
                file.get_package(), &path_to_package, &proto),
            proto,
            values,
            type_id: TypeId::of::<E>(),
            enum_or_unknown_type_id: TypeId::of::<ProtobufEnumOrUnknown<E>>(),
            get_descriptor,
            index_by_name,
            index_by_number,
        }
    }

    pub fn values(&self) -> &[EnumValueDescriptor] {
        &self.values
    }

    pub fn value_by_name(&self, name: &str) -> Option<&EnumValueDescriptor> {
        let &index = self.index_by_name.get(name)?;
        Some(&self.values[index])
    }

    pub fn value_by_number(&self, number: i32) -> Option<&EnumValueDescriptor> {
        let &index = self.index_by_number.get(&number)?;
        Some(&self.values[index])
    }

    pub fn is<E: 'static>(&self) -> bool {
        TypeId::of::<E>() == self.type_id
    }

    pub fn cast<E: 'static>(&self, value: i32) -> Option<E> {
        if TypeId::of::<E>() == self.type_id {
            unsafe {
                let mut r = mem::uninitialized();
                self.get_descriptor
                    .copy_to(value, &mut r as *mut E as *mut ());
                Some(r)
            }
        } else if TypeId::of::<E>() == self.enum_or_unknown_type_id {
            debug_assert_eq!(mem::size_of::<E>(), mem::size_of::<i32>());
            unsafe {
                // This works because `ProtobufEnumOrUnknown<E>` is `#[repr(transparent)]`
                let mut r = mem::uninitialized();
                ptr::copy(&value, &mut r as *mut E as *mut i32, 1);
                Some(r)
            }
        } else {
            None
        }
    }
}
