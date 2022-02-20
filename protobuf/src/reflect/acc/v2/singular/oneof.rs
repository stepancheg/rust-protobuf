use std::marker;

use crate::reflect::acc::v2::singular::GetOptionImplHasGetCopy;
use crate::reflect::acc::v2::singular::GetOptionImplHasGetRef;
use crate::reflect::acc::v2::singular::GetOptionImplHasGetRefDeref;
use crate::reflect::acc::v2::singular::GetOrDefaultGetCopy;
use crate::reflect::acc::v2::singular::GetOrDefaultGetRef;
use crate::reflect::acc::v2::singular::GetOrDefaultGetRefDeref;
use crate::reflect::acc::v2::singular::MutOrDefaultGetMut;
use crate::reflect::acc::v2::singular::MutOrDefaultUnmplemented;
use crate::reflect::acc::v2::singular::SetImplSetField;
use crate::reflect::acc::v2::singular::SingularFieldAccessor;
use crate::reflect::acc::v2::singular::SingularFieldAccessorHolder;
use crate::reflect::acc::v2::singular::SingularFieldAccessorImpl;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::runtime_types::RuntimeTypeWithDeref;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::EnumFull;
use crate::EnumOrUnknown;
use crate::MessageDyn;
use crate::MessageFull;

/// Make accessor for `oneof` `message` field
pub fn make_oneof_message_has_get_mut_set_accessor<M, F>(
    name: &'static str,
    has_field: fn(&M) -> bool,
    get_field: for<'a> fn(&'a M) -> &'a F,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
    set_field: fn(&mut M, F),
) -> FieldAccessor
where
    M: MessageFull + 'static,
    F: MessageFull + ProtobufValue,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, F, _, _, _, _> {
                get_option_impl: GetOptionImplHasGetRef::<M, F> {
                    get: get_field,
                    has: has_field,
                },
                get_or_default_impl: GetOrDefaultGetRef::<M, F> { get_field },
                mut_or_default_impl: MutOrDefaultGetMut::<M, F> { mut_field },
                set_impl: SetImplSetField::<M, F> { set_field },
                _marker: marker::PhantomData,
            }),
        }),
    )
}

/// Make accessor for `Copy` field
pub fn make_oneof_copy_has_get_set_simpler_accessors<M, V>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> V,
    set: fn(&mut M, V),
) -> FieldAccessor
where
    M: MessageFull + 'static,
    V: ProtobufValue + Copy,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplHasGetCopy::<M, V> { has, get },
                get_or_default_impl: GetOrDefaultGetCopy::<M, V> { get_field: get },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplSetField::<M, V> { set_field: set },
                _marker: marker::PhantomData,
            }),
        }),
    )
}

struct OneofEnumAccessor<M: MessageFull, E: EnumFull> {
    get: fn(&M) -> Option<EnumOrUnknown<E>>,
    set: fn(&mut M, EnumOrUnknown<E>),
    default_value: E,
}

impl<M: MessageFull, E: EnumFull> SingularFieldAccessor for OneofEnumAccessor<M, E> {
    fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
        let m = m.downcast_ref().unwrap();
        let value = (self.get)(m);
        value.map(|v| ReflectValueRef::Enum(E::enum_descriptor_static(), v.value()))
    }

    fn get_field_or_default<'a>(&self, m: &'a dyn MessageDyn) -> ReflectValueRef<'a> {
        let m = m.downcast_ref().unwrap();
        let value = (self.get)(m);
        let value = value.unwrap_or(EnumOrUnknown::new(self.default_value));
        ReflectValueRef::Enum(E::enum_descriptor_static(), value.value())
    }

    fn mut_field_or_default<'a>(&self, _m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
        panic!("cannot get mutable pointer")
    }

    fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
        let m = m.downcast_mut().unwrap();
        match value {
            ReflectValueBox::Enum(e, v) => {
                assert_eq!(E::enum_descriptor_static(), e);
                (self.set)(m, EnumOrUnknown::from_i32(v));
            }
            _ => panic!("expecting enum value"),
        }
    }
}

/// Make accessor for `Copy` field
pub fn make_oneof_enum_accessors<M, E>(
    name: &'static str,
    get: fn(&M) -> Option<EnumOrUnknown<E>>,
    set: fn(&mut M, EnumOrUnknown<E>),
    default_value: E,
) -> FieldAccessor
where
    M: MessageFull,
    E: EnumFull,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(OneofEnumAccessor {
                get,
                set,
                default_value,
            }),
        }),
    )
}

/// Make accessor for `oneof` field
pub fn make_oneof_deref_has_get_set_simpler_accessor<M, F>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a <F::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
    set: fn(&mut M, F),
) -> FieldAccessor
where
    M: MessageFull + 'static,
    F: ProtobufValue,
    F::RuntimeType: RuntimeTypeWithDeref,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, F, _, _, _, _> {
                get_option_impl: GetOptionImplHasGetRefDeref::<M, F> { has, get },
                get_or_default_impl: GetOrDefaultGetRefDeref::<M, F> { get_field: get },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplSetField::<M, F> { set_field: set },
                _marker: marker::PhantomData,
            }),
        }),
    )
}
