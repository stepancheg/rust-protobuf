use std::marker;

use crate::reflect::acc::v2::singular::GetOptionImplHasGetRef;
use crate::reflect::acc::v2::singular::MutOrDefaultGetMut;
use crate::reflect::acc::v2::singular::SetImplSetField;
use crate::reflect::acc::v2::singular::SingularFieldAccessorHolder;
use crate::reflect::acc::v2::singular::SingularFieldAccessorImpl;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::runtime_types::RuntimeTypeWithDeref;
use crate::reflect::ProtobufValue;
use crate::EnumFull;
use crate::EnumOrUnknown;
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
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, F, _, _, _> {
                get_option_impl: GetOptionImplHasGetRef::<M, F> {
                    get: get_field,
                    has: has_field,
                },
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
    M: MessageFull,
    V: ProtobufValue + Copy,
{
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_has_get_set(has, get, set)),
    )
}

/// Make accessor for `Copy` field
pub fn make_oneof_enum_accessors<M, E>(
    name: &'static str,
    get: fn(&M) -> Option<EnumOrUnknown<E>>,
    set: fn(&mut M, EnumOrUnknown<E>),
    // TODO: remove this
    _default_value: E,
) -> FieldAccessor
where
    M: MessageFull,
    E: EnumFull,
{
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_get_option_set_enum(
            get, set,
        )),
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
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_has_get_set_deref(
            has, get, set,
        )),
    )
}
