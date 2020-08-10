use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::ReflectFieldRef;
use crate::reflect::RuntimeFieldType;
use crate::Message;

pub(crate) mod v2;

pub(crate) enum GeneratedFieldAccessor {
    V2(AccessorV2),
}

/// Accessor object is constructed in generated code.
/// Should not be used directly.
pub struct FieldAccessor {
    pub(crate) name: &'static str,
    pub(crate) accessor: GeneratedFieldAccessor,
}

impl GeneratedFieldAccessor {
    pub(crate) fn get_reflect<'a>(&self, m: &'a dyn Message) -> ReflectFieldRef<'a> {
        match self {
            GeneratedFieldAccessor::V2(AccessorV2::Singular(ref a)) => {
                ReflectFieldRef::Optional(a.accessor.get_field(m))
            }
            GeneratedFieldAccessor::V2(AccessorV2::Repeated(ref a)) => {
                ReflectFieldRef::Repeated(a.accessor.get_repeated(m))
            }
            GeneratedFieldAccessor::V2(AccessorV2::Map(ref a)) => {
                ReflectFieldRef::Map(a.accessor.get_reflect(m))
            }
        }
    }

    pub(crate) fn runtime_field_type(&self) -> RuntimeFieldType {
        match self {
            GeneratedFieldAccessor::V2(AccessorV2::Singular(ref a)) => {
                RuntimeFieldType::Singular(a.element_type.to_box())
            }
            GeneratedFieldAccessor::V2(AccessorV2::Repeated(ref a)) => {
                RuntimeFieldType::Repeated(a.element_type.to_box())
            }
            GeneratedFieldAccessor::V2(AccessorV2::Map(ref a)) => {
                RuntimeFieldType::Map(a.key_type.to_box(), a.value_type.to_box())
            }
        }
    }
}

impl FieldAccessor {
    pub(crate) fn new_v2(name: &'static str, accessor: AccessorV2) -> FieldAccessor {
        FieldAccessor {
            name,
            accessor: GeneratedFieldAccessor::V2(accessor),
        }
    }
}
