use std::fmt;
use std::marker;

use crate::enums::ProtobufEnum;
use crate::enums::ProtobufEnumOrUnknown;
use crate::message::Message;
use crate::message_dyn::MessageDyn;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::runtime_types::RuntimeTypeWithDeref;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::MessageRef;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::MessageField;

pub(crate) mod oneof;

/// Option-like objects
#[doc(hidden)]
trait OptionLike<T> {
    fn as_option_ref(&self) -> Option<&T>;
    fn as_option_mut(&mut self) -> Option<&mut T>;
    fn set_value(&mut self, value: T);
}

impl<T> OptionLike<T> for Option<T> {
    fn as_option_ref(&self) -> Option<&T> {
        self.as_ref()
    }

    fn as_option_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }

    fn set_value(&mut self, value: T) {
        *self = Some(value);
    }
}

impl<T> OptionLike<T> for MessageField<T> {
    fn as_option_ref(&self) -> Option<&T> {
        self.as_ref()
    }

    fn as_option_mut(&mut self) -> Option<&mut T> {
        self.as_mut()
    }

    fn set_value(&mut self, value: T) {
        *self = MessageField::some(value);
    }
}

/// This trait should not be used directly, use `FieldDescriptor` instead
pub(crate) trait SingularFieldAccessor: Send + Sync + 'static {
    fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>>;
    fn get_field_or_default<'a>(&self, m: &'a dyn MessageDyn) -> ReflectValueRef<'a>;
    fn mut_field_or_default<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a>;
    fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox);
}

pub(crate) struct SingularFieldAccessorHolder {
    pub accessor: Box<dyn SingularFieldAccessor>,
}

impl<'a> fmt::Debug for SingularFieldAccessorHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SingularFieldAccessorHolder").finish()
    }
}

trait GetOptionImpl<M>: Send + Sync + 'static {
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>>;
}

trait GetOrDefaultImpl<M>: Send + Sync + 'static {
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a>;
}

trait MutOrDefaultImpl<M>: Send + Sync + 'static {
    fn mut_singular_field_or_default_impl<'a>(&self, m: &'a mut M) -> ReflectValueMut<'a>;
}

trait SetImpl<M>: Send + Sync + 'static {
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox);
}

struct MutOrDefaultUnmplemented<M>
where
    M: Message,
{
    _marker: marker::PhantomData<M>,
}

impl<M> MutOrDefaultUnmplemented<M>
where
    M: Message,
{
    fn new() -> MutOrDefaultUnmplemented<M> {
        MutOrDefaultUnmplemented {
            _marker: marker::PhantomData,
        }
    }
}

impl<M> MutOrDefaultImpl<M> for MutOrDefaultUnmplemented<M>
where
    M: Message,
{
    fn mut_singular_field_or_default_impl<'a>(&self, _m: &'a mut M) -> ReflectValueMut<'a> {
        unimplemented!()
    }
}

struct SingularFieldAccessorImpl<M, V, G, D, E, S>
where
    M: Message,
    V: ProtobufValue,
    G: GetOptionImpl<M>,
    D: GetOrDefaultImpl<M>,
    E: MutOrDefaultImpl<M>,
    S: SetImpl<M>,
{
    get_option_impl: G,
    get_or_default_impl: D,
    mut_or_default_impl: E,
    set_impl: S,
    _marker: marker::PhantomData<(M, V)>,
}

impl<M, V, G, D, E, S> SingularFieldAccessor for SingularFieldAccessorImpl<M, V, G, D, E, S>
where
    M: Message,
    V: ProtobufValue,
    G: GetOptionImpl<M>,
    D: GetOrDefaultImpl<M>,
    E: MutOrDefaultImpl<M>,
    S: SetImpl<M>,
{
    fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
        let m = m.downcast_ref().unwrap();
        self.get_option_impl.get_reflect_impl(m)
    }

    fn get_field_or_default<'a>(&self, m: &'a dyn MessageDyn) -> ReflectValueRef<'a> {
        let m = m.downcast_ref().unwrap();
        self.get_or_default_impl
            .get_singular_field_or_default_impl(m)
    }

    fn mut_field_or_default<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
        let m = m.downcast_mut().unwrap();
        self.mut_or_default_impl
            .mut_singular_field_or_default_impl(m)
    }

    fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
        let m = m.downcast_mut().unwrap();
        self.set_impl.set_singular_field(m, value)
    }
}

struct GetOptionImplFieldPointer<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    get_field: for<'a> fn(&'a M) -> &'a V,
}

struct GetOptionImplOptionFieldPointer<M, V, O>
where
    M: Message,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a O,
    _marker: marker::PhantomData<V>,
}

struct GetOptionImplHasGetRef<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> &'a V,
}

struct GetOptionImplHasGetRefDeref<M, V>
where
    M: Message,
    V: ProtobufValue,
    V::RuntimeType: RuntimeTypeWithDeref,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
}

struct GetOptionImplHasGetCopy<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> V,
}

impl<M, V> GetOptionImpl<M> for GetOptionImplFieldPointer<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        let v = (self.get_field)(m);
        if V::is_non_zero(v) {
            Some(V::as_ref(v))
        } else {
            None
        }
    }
}

impl<M, V, O> GetOptionImpl<M> for GetOptionImplOptionFieldPointer<M, V, O>
where
    M: Message,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        (self.get_field)(m).as_option_ref().map(V::as_ref)
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetRef<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        if (self.has)(m) {
            Some(V::as_ref((self.get)(m)))
        } else {
            None
        }
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetRefDeref<M, V>
where
    M: Message,
    V: ProtobufValue,
    V::RuntimeType: RuntimeTypeWithDeref,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        if (self.has)(m) {
            Some(<V::RuntimeType as RuntimeTypeWithDeref>::defef_as_ref(
                (self.get)(m),
            ))
        } else {
            None
        }
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetCopy<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        if (self.has)(m) {
            Some(V::into_static_value_ref((self.get)(m)))
        } else {
            None
        }
    }
}

struct GetOrDefaultGetRef<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    get_field: for<'a> fn(&'a M) -> &'a V,
}

struct GetOrDefaultGetRefDeref<M, V>
where
    M: Message,
    V: ProtobufValue,
    V::RuntimeType: RuntimeTypeWithDeref,
{
    get_field:
        for<'a> fn(
            &'a M,
        )
            -> &'a <<V as ProtobufValue>::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
}

struct GetOrDefaultOptionRefTypeDefault<M, V, O>
where
    M: Message,
    V: Message,
    O: OptionLike<V> + Sync + Send + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a O,
    _marker: marker::PhantomData<V>,
}

struct GetOrDefaultGetCopy<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    get_field: for<'a> fn(&'a M) -> V,
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetRef<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::as_ref((self.get_field)(m))
    }
}

impl<M, V, O> GetOrDefaultImpl<M> for GetOrDefaultOptionRefTypeDefault<M, V, O>
where
    M: Message,
    V: Message,
    O: OptionLike<V> + Sync + Send + 'static,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        ReflectValueRef::Message(MessageRef::from(
            match (self.get_field)(m).as_option_ref() {
                Some(v) => v,
                None => V::default_instance(),
            },
        ))
    }
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetRefDeref<M, V>
where
    M: Message,
    V: ProtobufValue,
    V::RuntimeType: RuntimeTypeWithDeref,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        <V::RuntimeType as RuntimeTypeWithDeref>::defef_as_ref((self.get_field)(m))
    }
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetCopy<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::into_static_value_ref((self.get_field)(m))
    }
}

struct MutOrDefaultGetMut<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut V,
}

impl<M, V> MutOrDefaultImpl<M> for MutOrDefaultGetMut<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    fn mut_singular_field_or_default_impl<'a>(&self, m: &'a mut M) -> ReflectValueMut<'a> {
        V::as_mut((self.mut_field)(m))
    }
}

struct MutOrDefaultOptionMut<M, V, O>
where
    M: Message,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    _marker: marker::PhantomData<V>,
}

impl<M, V, O> MutOrDefaultImpl<M> for MutOrDefaultOptionMut<M, V, O>
where
    M: Message,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    fn mut_singular_field_or_default_impl<'a>(&self, m: &'a mut M) -> ReflectValueMut<'a> {
        let option = (self.mut_field)(m);
        if option.as_option_ref().is_none() {
            option.set_value(V::default());
        }
        V::as_mut(option.as_option_mut().unwrap())
    }
}

struct SetImplFieldPointer<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut V,
}

struct SetImplOptionFieldPointer<M, V, O>
where
    M: Message,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    _marker: marker::PhantomData<V>,
}

struct SetImplSetField<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    set_field: for<'a> fn(&'a mut M, V),
}

impl<M, V> SetImpl<M> for SetImplFieldPointer<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        V::set_from_value_box((self.mut_field)(m), value);
    }
}

impl<M, V, O> SetImpl<M> for SetImplOptionFieldPointer<M, V, O>
where
    M: Message,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        (self.mut_field)(m).set_value(V::from_value_box(value).expect("wrong type"));
    }
}

impl<M, V> SetImpl<M> for SetImplSetField<M, V>
where
    M: Message,
    V: ProtobufValue,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        let value = value.downcast::<V>().expect("message");
        (self.set_field)(m, value)
    }
}

/// Make accessor for `SingularPtrField`
pub fn make_message_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a MessageField<V>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut MessageField<V>,
) -> FieldAccessor
where
    M: Message + 'static,
    V: Message + ProtobufValue + 'static,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplOptionFieldPointer::<M, V, _> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultOptionRefTypeDefault::<M, V, _> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                mut_or_default_impl: MutOrDefaultOptionMut::<M, V, _> {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                set_impl: SetImplOptionFieldPointer::<M, V, _> {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                _marker: marker::PhantomData,
            }),
        }),
    )
}

/// Make accessor for `Option<C>` field
pub fn make_option_get_copy_simpler_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a Option<V>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<V>,
    get_value: fn(&M) -> V,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufValue + 'static,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplOptionFieldPointer::<M, V, _> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultGetCopy::<M, V> {
                    get_field: get_value,
                },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplOptionFieldPointer::<M, V, _> {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                _marker: marker::PhantomData,
            }),
        }),
    )
}

struct GetOrDefaultEnum<M, E: ProtobufEnum> {
    get_field: for<'a> fn(&'a M) -> &'a Option<ProtobufEnumOrUnknown<E>>,
    default_value: E,
}

impl<M: Message, E: ProtobufEnum> GetOrDefaultImpl<M> for GetOrDefaultEnum<M, E> {
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        ReflectValueRef::Enum(
            E::enum_descriptor_static(),
            match (self.get_field)(m) {
                Some(e) => e.value(),
                None => self.default_value.value(),
            },
        )
    }
}

/// Make accessor for enum field
pub fn make_option_enum_accessor<M, E>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a Option<ProtobufEnumOrUnknown<E>>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<ProtobufEnumOrUnknown<E>>,
    default_value: E,
) -> FieldAccessor
where
    M: Message + 'static,
    E: ProtobufEnum + ProtobufValue,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<
                M,
                ProtobufEnumOrUnknown<E>,
                _,
                _,
                _,
                _,
            > {
                get_option_impl: GetOptionImplOptionFieldPointer::<
                    M,
                    ProtobufEnumOrUnknown<E>,
                    Option<ProtobufEnumOrUnknown<E>>,
                > {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultEnum::<M, E> {
                    get_field,
                    default_value,
                },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplOptionFieldPointer::<
                    M,
                    ProtobufEnumOrUnknown<E>,
                    Option<ProtobufEnumOrUnknown<E>>,
                > {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                _marker: marker::PhantomData,
            }),
        }),
    )
}

/// String or bytes field
pub fn make_option_get_ref_simpler_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a Option<V>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<V>,
    get_value: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufValue + 'static,
    V::RuntimeType: RuntimeTypeWithDeref,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplOptionFieldPointer::<M, V, _> {
                    get_field,
                    _marker: marker::PhantomData,
                },
                get_or_default_impl: GetOrDefaultGetRefDeref::<M, V> {
                    get_field: get_value,
                },
                mut_or_default_impl: MutOrDefaultUnmplemented::new(),
                set_impl: SetImplOptionFieldPointer::<M, V, _> {
                    mut_field,
                    _marker: marker::PhantomData,
                },
                _marker: marker::PhantomData,
            }),
        }),
    )
}

/// Make accessor for simple field
pub fn make_simpler_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a V,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut V,
) -> FieldAccessor
where
    M: Message + 'static,
    V: ProtobufValue,
{
    FieldAccessor::new_v2(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder {
            accessor: Box::new(SingularFieldAccessorImpl::<M, V, _, _, _, _> {
                get_option_impl: GetOptionImplFieldPointer::<M, V> { get_field },
                get_or_default_impl: GetOrDefaultGetRef::<M, V> { get_field },
                mut_or_default_impl: MutOrDefaultGetMut::<M, V> { mut_field },
                set_impl: SetImplFieldPointer::<M, V> { mut_field },
                _marker: marker::PhantomData,
            }),
        }),
    )
}
