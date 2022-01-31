use crate::descriptor::field_descriptor_proto::Type;
use crate::reflect::ReflectRepeatedMut;
use crate::reflect::ReflectValueBox;
use crate::reflect::RuntimeTypeBox;
use crate::rt;
use crate::wire_format::WireType;
use crate::CodedInputStream;
use crate::ProtobufError;
use crate::ProtobufResult;

/// Runtime type and protobuf type.
pub(crate) struct ProtobufTypeBox {
    runtime: RuntimeTypeBox,
    t: Type,
}

impl ProtobufTypeBox {
    pub(crate) fn new(runtime: RuntimeTypeBox, t: Type) -> ProtobufResult<ProtobufTypeBox> {
        match (t, &runtime) {
            (Type::TYPE_INT32, RuntimeTypeBox::I32) => {}
            (Type::TYPE_INT64, RuntimeTypeBox::I64) => {}
            (Type::TYPE_UINT32, RuntimeTypeBox::U32) => {}
            (Type::TYPE_UINT64, RuntimeTypeBox::U64) => {}
            (Type::TYPE_SINT32, RuntimeTypeBox::I32) => {}
            (Type::TYPE_SINT64, RuntimeTypeBox::I64) => {}
            (Type::TYPE_FIXED32, RuntimeTypeBox::U32) => {}
            (Type::TYPE_FIXED64, RuntimeTypeBox::U64) => {}
            (Type::TYPE_SFIXED32, RuntimeTypeBox::I32) => {}
            (Type::TYPE_SFIXED64, RuntimeTypeBox::I64) => {}
            (Type::TYPE_FLOAT, RuntimeTypeBox::F32) => {}
            (Type::TYPE_DOUBLE, RuntimeTypeBox::F64) => {}
            (Type::TYPE_BOOL, RuntimeTypeBox::Bool) => {}
            (Type::TYPE_STRING, RuntimeTypeBox::String) => {}
            (Type::TYPE_BYTES, RuntimeTypeBox::VecU8) => {}
            (Type::TYPE_MESSAGE, RuntimeTypeBox::Message(..)) => {}
            (Type::TYPE_ENUM, RuntimeTypeBox::Enum(..)) => {}
            (Type::TYPE_GROUP, ..) => return Err(ProtobufError::GroupIsNotImplemented),
            _ => return Err(ProtobufError::IncompatibleProtobufTypeAndRuntimeType),
        }
        Ok(ProtobufTypeBox { runtime, t })
    }

    pub(crate) fn read(
        &self,
        is: &mut CodedInputStream,
        wire_type: WireType,
    ) -> ProtobufResult<ReflectValueBox> {
        if wire_type != WireType::for_type(self.t) {
            return Err(rt::unexpected_wire_type(wire_type));
        }
        Ok(match self.t {
            Type::TYPE_DOUBLE => ReflectValueBox::F64(is.read_double()?),
            Type::TYPE_FLOAT => ReflectValueBox::F32(is.read_float()?),
            Type::TYPE_INT64 => ReflectValueBox::I64(is.read_int64()?),
            Type::TYPE_UINT64 => ReflectValueBox::U64(is.read_uint64()?),
            Type::TYPE_INT32 => ReflectValueBox::I32(is.read_int32()?),
            Type::TYPE_FIXED64 => ReflectValueBox::U64(is.read_fixed64()?),
            Type::TYPE_FIXED32 => ReflectValueBox::U32(is.read_fixed32()?),
            Type::TYPE_BOOL => ReflectValueBox::Bool(is.read_bool()?),
            Type::TYPE_UINT32 => ReflectValueBox::U32(is.read_uint32()?),
            Type::TYPE_SFIXED32 => ReflectValueBox::I32(is.read_sfixed32()?),
            Type::TYPE_SFIXED64 => ReflectValueBox::I64(is.read_sfixed64()?),
            Type::TYPE_SINT32 => ReflectValueBox::I32(is.read_sint32()?),
            Type::TYPE_SINT64 => ReflectValueBox::I64(is.read_sint64()?),
            Type::TYPE_STRING => ReflectValueBox::String(is.read_string()?),
            Type::TYPE_BYTES => ReflectValueBox::Bytes(is.read_bytes()?),
            Type::TYPE_ENUM => match &self.runtime {
                RuntimeTypeBox::Enum(e) => {
                    let v = is.read_enum_value()?;
                    ReflectValueBox::Enum(e.clone(), v)
                }
                _ => unreachable!(),
            },
            Type::TYPE_GROUP => return Err(ProtobufError::GroupIsNotImplemented),
            Type::TYPE_MESSAGE => match &self.runtime {
                RuntimeTypeBox::Message(m) => ReflectValueBox::Message(is.read_message_dyn(m)?),
                _ => unreachable!(),
            },
        })
    }

    pub(crate) fn read_repeated_into(
        &self,
        is: &mut CodedInputStream,
        wire_type: WireType,
        repeated: &mut ReflectRepeatedMut,
    ) -> ProtobufResult<()> {
        if wire_type == WireType::for_type(self.t) {
            let value = self.read(is, wire_type)?;
            repeated.push(value);
            Ok(())
        } else if wire_type == WireType::WireTypeLengthDelimited {
            fn extend(repeated: &mut ReflectRepeatedMut, v: Vec<impl Into<ReflectValueBox>>) {
                for v in v {
                    // TODO: inefficient
                    repeated.push(v.into());
                }
            }

            match self.t {
                Type::TYPE_INT32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_int32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_INT64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_int64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_UINT32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_uint32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_UINT64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_uint64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_SINT32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_sint32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_SINT64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_sint64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_FIXED32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_fixed32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_FIXED64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_fixed64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_SFIXED32 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_sfixed32_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_SFIXED64 => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_sfixed64_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_FLOAT => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_float_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_DOUBLE => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_double_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_BOOL => {
                    let mut v = Vec::new();
                    is.read_repeated_packed_bool_into(&mut v)?;
                    extend(repeated, v);
                    Ok(())
                }
                Type::TYPE_ENUM => match &self.runtime {
                    RuntimeTypeBox::Enum(e) => {
                        let mut v = Vec::new();
                        is.read_repeated_packed_enum_values_into(&mut v)?;
                        for e_v in v {
                            repeated.push(ReflectValueBox::Enum(e.clone(), e_v));
                        }
                        Ok(())
                    }
                    _ => unreachable!(),
                },
                Type::TYPE_GROUP => Err(ProtobufError::GroupIsNotImplemented),
                Type::TYPE_MESSAGE | Type::TYPE_STRING | Type::TYPE_BYTES => {
                    Err(rt::unexpected_wire_type(wire_type))
                }
            }
        } else {
            Err(rt::unexpected_wire_type(wire_type))
        }
    }
}
