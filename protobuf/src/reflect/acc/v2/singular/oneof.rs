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
use crate::reflect::ProtobufValue;
use crate::Message;
use std::marker;

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
    F: Message + ProtobufValue,
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
    M: Message + 'static,
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

/// Make accessor for `oneof` field
pub fn make_oneof_deref_has_get_set_simpler_accessor<M, F>(
    name: &'static str,
    has: fn(&M) -> bool,
    get: for<'a> fn(&'a M) -> &'a <F::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
    set: fn(&mut M, F),
) -> FieldAccessor
where
    M: Message + 'static,
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
