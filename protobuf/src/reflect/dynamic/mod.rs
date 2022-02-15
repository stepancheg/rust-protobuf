use std::fmt;

use crate::cached_size::CachedSize;
use crate::descriptor::field_descriptor_proto::Type;
use crate::message_dyn::MessageDyn;
use crate::reflect::dynamic::map::DynamicMap;
use crate::reflect::dynamic::optional::DynamicOptional;
use crate::reflect::dynamic::repeated::DynamicRepeated;
use crate::reflect::map::ReflectMap;
use crate::reflect::protobuf_type_box::ProtobufTypeBox;
use crate::reflect::repeated::ReflectRepeated;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::FieldDescriptor;
use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectFieldRef;
use crate::reflect::ReflectMapMut;
use crate::reflect::ReflectMapRef;
use crate::reflect::ReflectRepeatedMut;
use crate::reflect::ReflectRepeatedRef;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::reflect::RuntimeFieldType;
use crate::reflect::Syntax;
use crate::rt::bytes_size;
use crate::rt::compute_raw_varint32_size;
use crate::rt::read_map_template;
use crate::rt::read_unknown_or_skip_group;
use crate::rt::string_size;
use crate::rt::tag_size;
use crate::rt::unknown_fields_size;
use crate::rt::value_size;
use crate::rt::value_varint_zigzag_size;
use crate::rt::vec_packed_fixed_size;
use crate::rt::vec_packed_varint_size;
use crate::rt::vec_packed_varint_zigzag_size;
use crate::text_format;
use crate::wire_format::WireType;
use crate::Clear;
use crate::CodedInputStream;
use crate::CodedOutputStream;
use crate::Message;
use crate::Result;
use crate::UnknownFields;

pub(crate) mod map;
pub(crate) mod optional;
pub(crate) mod repeated;

#[derive(Debug, Clone)]
enum DynamicFieldValue {
    Singular(DynamicOptional),
    Repeated(DynamicRepeated),
    Map(DynamicMap),
}

impl DynamicFieldValue {
    fn as_ref(&self) -> ReflectFieldRef {
        match self {
            DynamicFieldValue::Singular(v) => ReflectFieldRef::Optional(v.get()),
            DynamicFieldValue::Repeated(r) => ReflectFieldRef::Repeated(ReflectRepeatedRef::new(r)),
            DynamicFieldValue::Map(m) => ReflectFieldRef::Map(ReflectMapRef::new(m)),
        }
    }

    fn clear(&mut self) {
        match self {
            DynamicFieldValue::Singular(o) => o.clear(),
            DynamicFieldValue::Repeated(r) => r.clear(),
            DynamicFieldValue::Map(m) => m.clear(),
        }
    }
}

impl DynamicFieldValue {
    fn default_for_field(field: &FieldDescriptor) -> DynamicFieldValue {
        match field.runtime_field_type() {
            RuntimeFieldType::Singular(s) => DynamicFieldValue::Singular(DynamicOptional::none(s)),
            RuntimeFieldType::Repeated(r) => DynamicFieldValue::Repeated(DynamicRepeated::new(r)),
            RuntimeFieldType::Map(k, v) => DynamicFieldValue::Map(DynamicMap::new(k, v)),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DynamicMessage {
    pub(crate) descriptor: MessageDescriptor,
    /// Fields by index in the description.
    /// This field is lazy-init: it is empty when created.
    fields: Box<[DynamicFieldValue]>,
    unknown_fields: UnknownFields,
    cached_size: CachedSize,
}

impl DynamicMessage {
    pub(crate) fn new(descriptor: MessageDescriptor) -> DynamicMessage {
        DynamicMessage {
            descriptor,
            fields: Vec::new().into_boxed_slice(),
            unknown_fields: UnknownFields::new(),
            cached_size: CachedSize::new(),
        }
    }

    fn init_fields(&mut self) {
        if self.fields.is_empty() {
            self.fields = self
                .descriptor
                .fields()
                .map(|f| DynamicFieldValue::default_for_field(&f))
                .collect();
        }
    }

    pub(crate) fn get_reflect<'a>(&'a self, field: &FieldDescriptor) -> ReflectFieldRef<'a> {
        let (descriptor, index) = field.regular();
        assert_eq!(&self.descriptor, descriptor);
        if self.fields.is_empty() {
            ReflectFieldRef::default_for_field(field)
        } else {
            self.fields[index].as_ref()
        }
    }

    pub fn clear_field(&mut self, field: &FieldDescriptor) {
        let (descriptor, index) = field.regular();
        assert_eq!(&self.descriptor, descriptor);
        if self.fields.is_empty() {
            return;
        }

        self.fields[index].clear();
    }

    fn clear_oneof_group_fields_except(&mut self, field: &FieldDescriptor) {
        if let Some(oneof) = field.containing_oneof() {
            for next in oneof.fields() {
                if &next == field {
                    continue;
                }
                self.clear_field(&next);
            }
        }
    }

    pub(crate) fn mut_singular_field_or_default<'a>(
        &'a mut self,
        field: &FieldDescriptor,
    ) -> ReflectValueMut<'a> {
        let (descriptor, index) = field.regular();
        assert_eq!(&self.descriptor, descriptor);
        self.init_fields();
        self.clear_oneof_group_fields_except(field);
        // TODO: reset oneof group fields
        match &mut self.fields[index] {
            DynamicFieldValue::Singular(f) => f.mut_or_default(),
            _ => panic!("Not a singular field"),
        }
    }

    pub(crate) fn mut_repeated<'a>(
        &'a mut self,
        field: &FieldDescriptor,
    ) -> ReflectRepeatedMut<'a> {
        let (descriptor, index) = field.regular();
        assert_eq!(&self.descriptor, descriptor);
        self.init_fields();
        // TODO: reset oneof group fields
        match &mut self.fields[index] {
            DynamicFieldValue::Repeated(r) => ReflectRepeatedMut::new(r),
            _ => panic!("Not a repeated field: {}", field),
        }
    }

    pub(crate) fn mut_map<'a>(&'a mut self, field: &FieldDescriptor) -> ReflectMapMut<'a> {
        let (descriptor, index) = field.regular();
        assert_eq!(&self.descriptor, descriptor);
        self.init_fields();
        // TODO: reset oneof group fields
        match &mut self.fields[index] {
            DynamicFieldValue::Map(m) => ReflectMapMut::new(m),
            _ => panic!("Not a map field: {}", field),
        }
    }

    pub(crate) fn set_field(&mut self, field: &FieldDescriptor, value: ReflectValueBox) {
        let (descriptor, index) = field.regular();
        assert_eq!(&self.descriptor, descriptor);
        self.init_fields();
        // TODO: reset oneof group fields
        match &mut self.fields[index] {
            DynamicFieldValue::Singular(s) => s.set(value),
            _ => panic!("Not a singular field: {}", field),
        }
    }

    pub fn downcast_ref(message: &dyn MessageDyn) -> &DynamicMessage {
        <dyn MessageDyn>::downcast_ref(message).unwrap()
    }

    pub fn downcast_mut(message: &mut dyn MessageDyn) -> &mut DynamicMessage {
        <dyn MessageDyn>::downcast_mut(message).unwrap()
    }

    fn for_each_field_to_write(
        &self,
        handler: &mut impl ForEachSingularFieldToWrite,
    ) -> Result<()> {
        let is_proto3 = self.descriptor.file_descriptor().syntax() == Syntax::Proto3;
        for field_desc in self.descriptor.fields() {
            let field_number = field_desc.get_proto().get_number() as u32;
            match field_desc.runtime_field_type() {
                RuntimeFieldType::Singular(..) => {
                    if let Some(v) = field_desc.get_singular(self) {
                        // Ignore default value for proto3.
                        if !is_proto3 || v.is_non_zero() {
                            handler.field(
                                field_desc.get_proto().get_field_type(),
                                field_number,
                                &v,
                            )?;
                        }
                    }
                }
                RuntimeFieldType::Repeated(..) => {
                    let repeated = field_desc.get_repeated(self);
                    if field_desc.get_proto().options.get_or_default().get_packed() {
                        handler.repeated_packed(
                            field_desc.get_proto().get_field_type(),
                            field_number,
                            &repeated,
                        )?;
                    } else {
                        for i in 0..repeated.len() {
                            let v = repeated.get(i);
                            handler.field(
                                field_desc.get_proto().get_field_type(),
                                field_number,
                                &v,
                            )?;
                        }
                    }
                }
                RuntimeFieldType::Map(_, _) => {
                    let map = field_desc.get_map(self);
                    let (key_type, value_type) = field_desc.map_proto_type();
                    for (k, v) in &map {
                        handler.map_field_entry(
                            field_number,
                            &k,
                            key_type.t(),
                            &v,
                            value_type.t(),
                        )?;
                    }
                }
            }
        }

        handler.unknown_fields(&self.unknown_fields)?;
        Ok(())
    }
}

trait ForEachSingularFieldToWrite {
    fn field(&mut self, t: Type, number: u32, value: &ReflectValueRef) -> Result<()>;
    fn repeated_packed(&mut self, t: Type, number: u32, value: &ReflectRepeatedRef) -> Result<()>;
    fn map_field_entry(
        &mut self,
        number: u32,
        key: &ReflectValueRef,
        kt: Type,
        value: &ReflectValueRef,
        vt: Type,
    ) -> Result<()>;
    fn unknown_fields(&mut self, unknown_fields: &UnknownFields) -> Result<()>;
}

impl Clear for DynamicMessage {
    fn clear(&mut self) {
        self.fields = Vec::new().into_boxed_slice();
    }
}

impl fmt::Display for DynamicMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        text_format::fmt(self, f)
    }
}

// TODO: implement PartialEq, Default
impl Message for DynamicMessage {
    fn descriptor_by_instance(&self) -> MessageDescriptor {
        self.descriptor.clone()
    }

    fn is_initialized(&self) -> bool {
        // TODO: this check can be much faster for proto3 without contained proto2 messages.
        for f in self.descriptor.fields() {
            let fv = self.get_reflect(&f);
            match fv {
                ReflectFieldRef::Optional(s) => match s {
                    None => {
                        if f.is_required() {
                            return false;
                        }
                    }
                    Some(v) => {
                        if !v.is_initialized() {
                            return false;
                        }
                    }
                },
                ReflectFieldRef::Repeated(r) => {
                    for v in &r {
                        if !v.is_initialized() {
                            return false;
                        }
                    }
                }
                ReflectFieldRef::Map(m) => {
                    for (_k, v) in &m {
                        // Keys cannot be messages, so only check values.
                        if !v.is_initialized() {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) -> Result<()> {
        while !is.eof()? {
            let (field, wire_type) = is.read_tag_unpack()?;
            let field_desc = match self.descriptor.get_field_by_number(field) {
                Some(f) => f,
                None => {
                    read_unknown_or_skip_group(field, wire_type, is, &mut self.unknown_fields)?;
                    continue;
                }
            };
            match field_desc.runtime_field_type() {
                RuntimeFieldType::Singular(rtb) => {
                    let pt = ProtobufTypeBox::new(rtb, field_desc.get_proto().get_field_type())?;
                    let value = pt.read(is, wire_type)?;
                    self.set_field(&field_desc, value);
                }
                RuntimeFieldType::Repeated(rtb) => {
                    let pt = ProtobufTypeBox::new(rtb, field_desc.get_proto().get_field_type())?;
                    let mut repeated = self.mut_repeated(&field_desc);
                    pt.read_repeated_into(is, wire_type, &mut repeated)?;
                }
                RuntimeFieldType::Map(..) => {
                    let (key_type, value_type) = field_desc.map_proto_type();
                    let mut map = self.mut_map(&field_desc);
                    let mut key = key_type.runtime().default_value_box();
                    let mut value = value_type.runtime().default_value_box();
                    read_map_template(
                        wire_type,
                        is,
                        |wire_type, is| {
                            key = key_type.read(is, wire_type)?;
                            Ok(())
                        },
                        |wire_type, is| {
                            value = value_type.read(is, wire_type)?;
                            Ok(())
                        },
                    )?;
                    map.insert(key, value);
                }
            }
        }
        Ok(())
    }

    fn write_to_with_cached_sizes(&self, os: &mut CodedOutputStream) -> Result<()> {
        struct Handler<'a, 'o> {
            os: &'a mut CodedOutputStream<'o>,
        }

        impl<'a, 'o> ForEachSingularFieldToWrite for Handler<'a, 'o> {
            fn field(&mut self, t: Type, number: u32, value: &ReflectValueRef) -> Result<()> {
                singular_write_to(t, number, value, self.os)
            }

            fn repeated_packed(
                &mut self,
                t: Type,
                number: u32,
                value: &ReflectRepeatedRef,
            ) -> Result<()> {
                repeated_write_to(t, number, value, self.os)
            }

            fn map_field_entry(
                &mut self,
                number: u32,
                key: &ReflectValueRef,
                kt: Type,
                value: &ReflectValueRef,
                vt: Type,
            ) -> Result<()> {
                let entry_data_size = compute_map_entry_field_data_size(key, kt, value, vt);
                self.os.write_tag(number, WireType::LengthDelimited)?;
                self.os.write_raw_varint32(entry_data_size)?;
                singular_write_to(kt, 1, key, self.os)?;
                singular_write_to(vt, 2, value, self.os)?;
                Ok(())
            }

            fn unknown_fields(&mut self, unknown_fields: &UnknownFields) -> Result<()> {
                self.os.write_unknown_fields(unknown_fields)
            }
        }

        let mut handler = Handler { os };

        self.for_each_field_to_write(&mut handler)
    }

    fn compute_size(&self) -> u32 {
        struct Handler {
            m_size: u32,
        }

        impl ForEachSingularFieldToWrite for Handler {
            fn field(&mut self, t: Type, number: u32, value: &ReflectValueRef) -> Result<()> {
                self.m_size += compute_singular_size(t, number, value);
                Ok(())
            }

            fn repeated_packed(
                &mut self,
                t: Type,
                number: u32,
                value: &ReflectRepeatedRef,
            ) -> Result<()> {
                self.m_size += compute_repeated_packed_size(t, number, value);
                Ok(())
            }

            fn map_field_entry(
                &mut self,
                number: u32,
                key: &ReflectValueRef,
                kt: Type,
                value: &ReflectValueRef,
                vt: Type,
            ) -> Result<()> {
                let entry_data_size = compute_map_entry_field_data_size(key, kt, value, vt);
                self.m_size += tag_size(number)
                    + compute_raw_varint32_size(entry_data_size as u32)
                    + entry_data_size;
                Ok(())
            }

            fn unknown_fields(&mut self, unknown_fields: &UnknownFields) -> Result<()> {
                self.m_size += unknown_fields_size(unknown_fields);
                Ok(())
            }
        }

        let mut handler = Handler { m_size: 0 };

        self.for_each_field_to_write(&mut handler)
            .expect("compute_size should not fail");

        handler.m_size
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut UnknownFields {
        &mut self.unknown_fields
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        panic!("DynamicMessage cannot be constructed directly")
    }

    fn default_instance() -> &'static Self
    where
        Self: Sized,
    {
        panic!("There's no default instance for dynamic message")
    }
}

/// Write singular field to output stream
fn singular_write_to(
    proto_type: Type,
    field_number: u32,
    v: &ReflectValueRef,
    os: &mut CodedOutputStream,
) -> Result<()> {
    match proto_type {
        Type::TYPE_ENUM => {
            let enum_v = v.to_enum_value().unwrap();
            os.write_enum(field_number, enum_v)
        }
        Type::TYPE_MESSAGE => {
            let msg_v = v.to_message().unwrap();
            os.write_message_dyn(field_number, &*msg_v)
        }
        Type::TYPE_GROUP => {
            unimplemented!()
        }
        Type::TYPE_UINT32 => os.write_uint32(field_number, v.to_u32().unwrap()),
        Type::TYPE_UINT64 => os.write_uint64(field_number, v.to_u64().unwrap()),
        Type::TYPE_INT32 => os.write_int32(field_number, v.to_i32().unwrap()),
        Type::TYPE_INT64 => os.write_int64(field_number, v.to_i64().unwrap()),
        Type::TYPE_SINT32 => os.write_sint32(field_number, v.to_i32().unwrap()),
        Type::TYPE_SINT64 => os.write_sint64(field_number, v.to_i64().unwrap()),
        Type::TYPE_FIXED32 => os.write_fixed32(field_number, v.to_u32().unwrap()),
        Type::TYPE_FIXED64 => os.write_fixed64(field_number, v.to_u64().unwrap()),
        Type::TYPE_SFIXED64 => os.write_sfixed64(field_number, v.to_i64().unwrap()),
        Type::TYPE_SFIXED32 => os.write_sfixed32(field_number, v.to_i32().unwrap()),
        Type::TYPE_BOOL => os.write_bool(field_number, v.to_bool().unwrap()),
        Type::TYPE_STRING => os.write_string(field_number, v.to_str().unwrap()),
        Type::TYPE_BYTES => os.write_bytes(field_number, v.to_bytes().unwrap()),
        Type::TYPE_FLOAT => os.write_float(field_number, v.to_f32().unwrap()),
        Type::TYPE_DOUBLE => os.write_double(field_number, v.to_f64().unwrap()),
    }
}

/// Compute singular field size
fn compute_singular_size(proto_type: Type, field_number: u32, v: &ReflectValueRef) -> u32 {
    match proto_type {
        Type::TYPE_ENUM => {
            let enum_v = v.to_enum_value().unwrap();
            value_size(field_number, enum_v, WireType::Varint)
        }
        Type::TYPE_MESSAGE => {
            let msg_v = v.to_message().unwrap();
            let len = msg_v.compute_size_dyn();
            tag_size(field_number) + compute_raw_varint32_size(len) + len
        }
        Type::TYPE_GROUP => {
            unimplemented!()
        }
        Type::TYPE_UINT32 => {
            let typed_v = v.to_u32().unwrap();
            value_size(field_number, typed_v, WireType::Varint)
        }
        Type::TYPE_UINT64 => {
            let typed_v = v.to_u64().unwrap();
            value_size(field_number, typed_v, WireType::Varint)
        }
        Type::TYPE_INT32 => {
            let typed_v = v.to_i32().unwrap();
            value_size(field_number, typed_v, WireType::Varint)
        }
        Type::TYPE_INT64 => {
            let typed_v = v.to_i64().unwrap();
            value_size(field_number, typed_v, WireType::Varint)
        }
        Type::TYPE_SINT32 => {
            let typed_v = v.to_i32().unwrap();
            value_varint_zigzag_size(field_number, typed_v)
        }
        Type::TYPE_SINT64 => {
            let typed_v = v.to_i64().unwrap();
            value_varint_zigzag_size(field_number, typed_v)
        }
        Type::TYPE_FIXED32 => tag_size(field_number) + 4,
        Type::TYPE_FIXED64 => tag_size(field_number) + 8,
        Type::TYPE_SFIXED32 => tag_size(field_number) + 4,
        Type::TYPE_SFIXED64 => tag_size(field_number) + 8,
        Type::TYPE_BOOL => {
            let typed_v = v.to_bool().unwrap();
            value_size(field_number, typed_v, WireType::Varint)
        }
        Type::TYPE_STRING => {
            let typed_v = v.to_str().unwrap();
            string_size(field_number, typed_v)
        }
        Type::TYPE_BYTES => {
            let typed_v = v.to_bytes().unwrap();
            bytes_size(field_number, typed_v)
        }
        Type::TYPE_FLOAT => tag_size(field_number) + 4,
        Type::TYPE_DOUBLE => tag_size(field_number) + 8,
    }
}

fn compute_repeated_packed_size(
    proto_type: Type,
    field_number: u32,
    v: &ReflectRepeatedRef,
) -> u32 {
    match proto_type {
        Type::TYPE_INT32 => vec_packed_varint_size(field_number, v.data_i32()),
        Type::TYPE_INT64 => vec_packed_varint_size(field_number, v.data_i64()),
        Type::TYPE_UINT32 => vec_packed_varint_size(field_number, v.data_u32()),
        Type::TYPE_UINT64 => vec_packed_varint_size(field_number, v.data_u64()),
        Type::TYPE_SINT32 => vec_packed_varint_zigzag_size(field_number, v.data_i32()),
        Type::TYPE_SINT64 => vec_packed_varint_zigzag_size(field_number, v.data_i64()),
        Type::TYPE_FIXED32 => vec_packed_fixed_size(field_number, v.data_u32()),
        Type::TYPE_FIXED64 => vec_packed_fixed_size(field_number, v.data_u64()),
        Type::TYPE_SFIXED32 => vec_packed_fixed_size(field_number, v.data_i32()),
        Type::TYPE_SFIXED64 => vec_packed_fixed_size(field_number, v.data_i64()),
        Type::TYPE_FLOAT => vec_packed_fixed_size(field_number, v.data_f32()),
        Type::TYPE_DOUBLE => vec_packed_fixed_size(field_number, v.data_f64()),
        Type::TYPE_BOOL => vec_packed_fixed_size(field_number, v.data_bool()),
        Type::TYPE_STRING => panic!("strings cannot be packed"),
        Type::TYPE_BYTES => panic!("bytes cannot be packed"),
        Type::TYPE_ENUM => vec_packed_varint_size(field_number, v.data_enum_values()),
        Type::TYPE_MESSAGE => panic!("messages cannot be packed"),
        Type::TYPE_GROUP => panic!("groups cannot be packed"),
    }
}

fn repeated_write_to(
    proto_type: Type,
    field_number: u32,
    v: &ReflectRepeatedRef,
    os: &mut CodedOutputStream,
) -> Result<()> {
    match proto_type {
        Type::TYPE_INT32 => os.write_repeated_packed_int32(field_number, v.data_i32()),
        Type::TYPE_INT64 => os.write_repeated_packed_int64(field_number, v.data_i64()),
        Type::TYPE_UINT64 => os.write_repeated_packed_uint64(field_number, v.data_u64()),
        Type::TYPE_FIXED64 => os.write_repeated_packed_fixed64(field_number, v.data_u64()),
        Type::TYPE_FIXED32 => os.write_repeated_packed_fixed32(field_number, v.data_u32()),
        Type::TYPE_UINT32 => os.write_repeated_packed_uint32(field_number, v.data_u32()),
        Type::TYPE_SINT32 => os.write_repeated_packed_sint32(field_number, v.data_i32()),
        Type::TYPE_SINT64 => os.write_repeated_packed_sint64(field_number, v.data_i64()),
        Type::TYPE_SFIXED32 => os.write_repeated_packed_sfixed32(field_number, v.data_i32()),
        Type::TYPE_SFIXED64 => os.write_repeated_packed_sfixed64(field_number, v.data_i64()),
        Type::TYPE_BOOL => os.write_repeated_packed_bool(field_number, v.data_bool()),
        Type::TYPE_FLOAT => os.write_repeated_packed_float(field_number, v.data_f32()),
        Type::TYPE_DOUBLE => os.write_repeated_packed_double(field_number, v.data_f64()),
        Type::TYPE_ENUM => os.write_repeated_packed_int32(field_number, v.data_enum_values()),
        Type::TYPE_STRING => panic!("strings cannot be packed"),
        Type::TYPE_BYTES => panic!("bytes cannot be packed"),
        Type::TYPE_GROUP => panic!("groups cannot be packed"),
        Type::TYPE_MESSAGE => panic!("messages cannot be packed"),
    }
}

fn compute_map_entry_field_data_size(
    key: &ReflectValueRef,
    kt: Type,
    value: &ReflectValueRef,
    vt: Type,
) -> u32 {
    let key_size = compute_singular_size(kt, 1, key);
    let value_size = compute_singular_size(vt, 2, value);
    key_size + value_size
}
