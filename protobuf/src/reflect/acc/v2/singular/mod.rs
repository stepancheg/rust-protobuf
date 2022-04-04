use std::fmt;
use std::marker;

use crate::message::Message;
use crate::message_dyn::MessageDyn;
use crate::message_field::MessageField;
use crate::message_full::MessageFull;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::runtime_types::RuntimeTypeWithDeref;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::MessageRef;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;

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
    fn mut_field_or_default<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a>;
    fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox);
}

pub(crate) struct SingularFieldAccessorHolder {
    pub accessor: Box<dyn SingularFieldAccessor>,
}

impl SingularFieldAccessorHolder {
    fn new_get_mut<M, V>(
        get_field: for<'a> fn(&'a M) -> &'a V,
        mut_field: for<'a> fn(&'a mut M) -> &'a mut V,
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        V: ProtobufValue,
    {
        struct Impl<M, V> {
            get_field: for<'a> fn(&'a M) -> &'a V,
            mut_field: for<'a> fn(&'a mut M) -> &'a mut V,
        }

        impl<M, V> SingularFieldAccessor for Impl<M, V>
        where
            M: MessageFull,
            V: ProtobufValue,
        {
            fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
                let m = m.downcast_ref::<M>().unwrap();
                let v = (self.get_field)(m);
                if V::is_non_zero(v) {
                    Some(V::as_ref(v))
                } else {
                    None
                }
            }

            fn mut_field_or_default<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
                let m = m.downcast_mut::<M>().unwrap();
                V::as_mut((self.mut_field)(m))
            }

            fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
                let m = m.downcast_mut::<M>().unwrap();
                V::set_from_value_box((self.mut_field)(m), value);
            }
        }

        SingularFieldAccessorHolder {
            accessor: Box::new(Impl {
                get_field,
                mut_field,
            }),
        }
    }

    fn new_get_option_mut_option<M, V>(
        get_field: for<'a> fn(&'a M) -> &'a Option<V>,
        mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<V>,
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        V: ProtobufValue,
    {
        struct Impl<M, V> {
            get_field: for<'a> fn(&'a M) -> &'a Option<V>,
            mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<V>,
        }

        impl<M, V> SingularFieldAccessor for Impl<M, V>
        where
            M: MessageFull,
            V: ProtobufValue,
        {
            fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
                let m = m.downcast_ref::<M>().unwrap();
                (self.get_field)(m).as_option_ref().map(V::as_ref)
            }

            fn mut_field_or_default<'a>(&self, _m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
                unimplemented!()
            }

            fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
                let m = m.downcast_mut().unwrap();
                (self.mut_field)(m).set_value(V::from_value_box(value).expect("wrong type"));
            }
        }

        SingularFieldAccessorHolder {
            accessor: Box::new(Impl {
                get_field,
                mut_field,
            }),
        }
    }

    fn new_get_mut_message<M, V>(
        get_field: for<'a> fn(&'a M) -> &'a MessageField<V>,
        mut_field: for<'a> fn(&'a mut M) -> &'a mut MessageField<V>,
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        V: MessageFull,
    {
        struct Impl<M, V> {
            get_field: for<'a> fn(&'a M) -> &'a MessageField<V>,
            mut_field: for<'a> fn(&'a mut M) -> &'a mut MessageField<V>,
        }

        impl<M, V> SingularFieldAccessor for Impl<M, V>
        where
            M: MessageFull,
            V: MessageFull,
        {
            fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
                let m = m.downcast_ref().unwrap();
                (self.get_field)(m).as_option_ref().map(V::as_ref)
            }

            fn mut_field_or_default<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
                let m = m.downcast_mut().unwrap();
                let option = (self.mut_field)(m);
                if option.as_option_ref().is_none() {
                    option.set_value(V::default());
                }
                V::as_mut(option.as_option_mut().unwrap())
            }

            fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
                let m = m.downcast_mut().unwrap();
                (self.mut_field)(m).set_value(V::from_value_box(value).expect("wrong type"));
            }
        }

        SingularFieldAccessorHolder {
            accessor: Box::new(Impl {
                get_field,
                mut_field,
            }),
        }
    }
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
    M: MessageFull,
{
    _marker: marker::PhantomData<M>,
}

impl<M> MutOrDefaultUnmplemented<M>
where
    M: MessageFull,
{
    fn new() -> MutOrDefaultUnmplemented<M> {
        MutOrDefaultUnmplemented {
            _marker: marker::PhantomData,
        }
    }
}

impl<M> MutOrDefaultImpl<M> for MutOrDefaultUnmplemented<M>
where
    M: MessageFull,
{
    fn mut_singular_field_or_default_impl<'a>(&self, _m: &'a mut M) -> ReflectValueMut<'a> {
        unimplemented!()
    }
}

struct SingularFieldAccessorImpl<M, V, G, E, S>
where
    M: MessageFull,
    V: ProtobufValue,
    G: GetOptionImpl<M>,
    E: MutOrDefaultImpl<M>,
    S: SetImpl<M>,
{
    get_option_impl: G,
    mut_or_default_impl: E,
    set_impl: S,
    _marker: marker::PhantomData<(M, V)>,
}

impl<M, V, G, E, S> SingularFieldAccessor for SingularFieldAccessorImpl<M, V, G, E, S>
where
    M: MessageFull,
    V: ProtobufValue,
    G: GetOptionImpl<M>,
    E: MutOrDefaultImpl<M>,
    S: SetImpl<M>,
{
    fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
        let m = m.downcast_ref().unwrap();
        self.get_option_impl.get_reflect_impl(m)
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
    M: MessageFull,
    V: ProtobufValue,
{
    get_field: for<'a> fn(&'a M) -> &'a V,
}

struct GetOptionImplOptionFieldPointer<M, V, O>
where
    M: MessageFull,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a O,
    _marker: marker::PhantomData<V>,
}

struct GetOptionImplHasGetRef<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> &'a V,
}

struct GetOptionImplHasGetRefDeref<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
    V::RuntimeType: RuntimeTypeWithDeref,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
}

struct GetOptionImplHasGetCopy<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    has: for<'a> fn(&'a M) -> bool,
    get: for<'a> fn(&'a M) -> V,
}

impl<M, V> GetOptionImpl<M> for GetOptionImplFieldPointer<M, V>
where
    M: MessageFull,
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
    M: MessageFull,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    fn get_reflect_impl<'a>(&self, m: &'a M) -> Option<ReflectValueRef<'a>> {
        (self.get_field)(m).as_option_ref().map(V::as_ref)
    }
}

impl<M, V> GetOptionImpl<M> for GetOptionImplHasGetRef<M, V>
where
    M: MessageFull,
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
    M: MessageFull,
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
    M: MessageFull,
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
    M: MessageFull,
    V: ProtobufValue,
{
    get_field: for<'a> fn(&'a M) -> &'a V,
}

struct GetOrDefaultGetRefDeref<M, V>
where
    M: MessageFull,
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
    M: MessageFull,
    V: MessageFull,
    O: OptionLike<V> + Sync + Send + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a O,
    _marker: marker::PhantomData<V>,
}

struct GetOrDefaultGetCopy<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    get_field: for<'a> fn(&'a M) -> V,
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetRef<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::as_ref((self.get_field)(m))
    }
}

impl<M, V, O> GetOrDefaultImpl<M> for GetOrDefaultOptionRefTypeDefault<M, V, O>
where
    M: MessageFull,
    V: MessageFull,
    O: OptionLike<V> + Sync + Send + 'static,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        ReflectValueRef::Message(MessageRef::from(
            match (self.get_field)(m).as_option_ref() {
                Some(v) => v,
                None => <V as Message>::default_instance(),
            },
        ))
    }
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetRefDeref<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
    V::RuntimeType: RuntimeTypeWithDeref,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        <V::RuntimeType as RuntimeTypeWithDeref>::defef_as_ref((self.get_field)(m))
    }
}

impl<M, V> GetOrDefaultImpl<M> for GetOrDefaultGetCopy<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    fn get_singular_field_or_default_impl<'a>(&self, m: &'a M) -> ReflectValueRef<'a> {
        V::into_static_value_ref((self.get_field)(m))
    }
}

struct MutOrDefaultGetMut<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut V,
}

impl<M, V> MutOrDefaultImpl<M> for MutOrDefaultGetMut<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    fn mut_singular_field_or_default_impl<'a>(&self, m: &'a mut M) -> ReflectValueMut<'a> {
        V::as_mut((self.mut_field)(m))
    }
}

struct MutOrDefaultOptionMut<M, V, O>
where
    M: MessageFull,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    _marker: marker::PhantomData<V>,
}

impl<M, V, O> MutOrDefaultImpl<M> for MutOrDefaultOptionMut<M, V, O>
where
    M: MessageFull,
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
    M: MessageFull,
    V: ProtobufValue,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut V,
}

struct SetImplOptionFieldPointer<M, V, O>
where
    M: MessageFull,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    mut_field: for<'a> fn(&'a mut M) -> &'a mut O,
    _marker: marker::PhantomData<V>,
}

struct SetImplSetField<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    set_field: for<'a> fn(&'a mut M, V),
}

impl<M, V> SetImpl<M> for SetImplFieldPointer<M, V>
where
    M: MessageFull,
    V: ProtobufValue,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        V::set_from_value_box((self.mut_field)(m), value);
    }
}

impl<M, V, O> SetImpl<M> for SetImplOptionFieldPointer<M, V, O>
where
    M: MessageFull,
    V: ProtobufValue,
    O: OptionLike<V> + Sync + Send + 'static,
{
    fn set_singular_field(&self, m: &mut M, value: ReflectValueBox) {
        (self.mut_field)(m).set_value(V::from_value_box(value).expect("wrong type"));
    }
}

impl<M, V> SetImpl<M> for SetImplSetField<M, V>
where
    M: MessageFull,
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
    M: MessageFull,
    V: MessageFull,
{
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_get_mut_message(
            get_field, mut_field,
        )),
    )
}

/// Make accessor for `Option<C>` field
pub fn make_option_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a Option<V>,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<V>,
) -> FieldAccessor
where
    M: MessageFull,
    V: ProtobufValue,
{
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_get_option_mut_option(
            get_field, mut_field,
        )),
    )
}

/// Make accessor for simple field
pub fn make_simpler_field_accessor<M, V>(
    name: &'static str,
    get_field: for<'a> fn(&'a M) -> &'a V,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut V,
) -> FieldAccessor
where
    M: MessageFull,
    V: ProtobufValue,
{
    FieldAccessor::new(
        name,
        AccessorV2::Singular(SingularFieldAccessorHolder::new_get_mut(
            get_field, mut_field,
        )),
    )
}
