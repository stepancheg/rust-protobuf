use crate::reflect::acc::v2::singular::GetOptionImplHasGetCopy;
use crate::reflect::acc::v2::singular::GetOptionImplHasGetRef;
use crate::reflect::acc::v2::singular::GetOptionImplHasGetRefDeref;
use crate::reflect::acc::v2::singular::GetOrDefaultGetCopy;
use crate::reflect::acc::v2::singular::GetOrDefaultGetRef;
use crate::reflect::acc::v2::singular::GetOrDefaultGetRefDeref;
use crate::reflect::acc::v2::singular::MutOrDefaultGetMut;
use crate::reflect::acc::v2::singular::MutOrDefaultUnmplemented;
use crate::reflect::acc::v2::singular::SetImplSetField;
use crate::reflect::acc::v2::singular::SingularFieldAccessorHolder;
use crate::reflect::acc::v2::singular::SingularFieldAccessorImpl;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::runtime_types::RuntimeTypeWithDeref;
use crate::reflect::types::ProtobufType;
use crate::reflect::ProtobufValueSized;
use crate::Message;
use std::marker;

/// Make accessor for `Copy` field
pub fn make_oneof_copy_has_get_set_accessors<M, V>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: fn(&M) -> V::ProtobufValue,
    set: fn(&mut M, V::ProtobufValue),
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufType + 'static,
    V::ProtobufValue: Copy,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(
                SingularFieldAccessorImpl::<M, V::ProtobufValue, _, _, _, _> {
                    get_option_impl: GetOptionImplHasGetCopy::<M, V::ProtobufValue> { has, get },
                    get_or_default_impl: GetOrDefaultGetCopy::<M, V::ProtobufValue> {
                        get_field: get,
                    },
                    mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                    set_impl: SetImplSetField::<M, V::ProtobufValue> { set_field: set },
                    _marker: marker::PhantomData,
                },
            ),
            element_type: V::ProtobufValue::dynamic(),
        }),
    )
}

/// Make accessor for `oneof` field
pub fn make_oneof_deref_has_get_set_accessor<M, F>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a <<F::ProtobufValue as ProtobufValueSized>::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
    set: fn(&mut M, F::ProtobufValue),
) -> FieldAccessor
where
    M: Message + 'static,
    F: ProtobufType,
    <F::ProtobufValue as ProtobufValueSized>::RuntimeType: RuntimeTypeWithDeref,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(
                SingularFieldAccessorImpl::<M, F::ProtobufValue, _, _, _, _> {
                    get_option_impl: GetOptionImplHasGetRefDeref::<M, F::ProtobufValue> {
                        has,
                        get,
                    },
                    get_or_default_impl: GetOrDefaultGetRefDeref::<M, F::ProtobufValue> {
                        get_field: get,
                    },
                    mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                    set_impl: SetImplSetField::<M, F::ProtobufValue> { set_field: set },
                    _marker: marker::PhantomData,
                },
            ),
            element_type: F::ProtobufValue::dynamic(),
        }),
    )
}

/// Make accessor for `oneof` `message` field
pub fn make_oneof_message_has_get_mut_set_accessor<M, F>(
    name: &'static str,
    has_field: fn(&M) -> bool,
    get_field: for<'a> fn(&'a M) -> &'a F,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
    set_field: fn(&mut M, F),
) -> FieldAccessor
where
    M: Message + 'static,
    F: Message + ProtobufValueSized,
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
            element_type: F::dynamic(),
        }),
    )
}
