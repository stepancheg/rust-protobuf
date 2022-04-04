use std::fmt;
use std::marker;

use crate::message_dyn::MessageDyn;
use crate::message_field::MessageField;
use crate::message_full::MessageFull;
use crate::reflect::acc::v2::AccessorV2;
use crate::reflect::acc::FieldAccessor;
use crate::reflect::runtime_types::RuntimeTypeWithDeref;
use crate::reflect::value::value_ref::ReflectValueMut;
use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueBox;
use crate::reflect::ReflectValueRef;
use crate::EnumFull;
use crate::EnumOrUnknown;

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
    fn new<M>(
        get_field: impl for<'a> Fn(&'a M) -> Option<ReflectValueRef<'a>> + Send + Sync + 'static,
        mut_field_or_default: impl for<'a> Fn(&'a mut M) -> ReflectValueMut<'a> + Send + Sync + 'static,
        set_field: impl Fn(&mut M, ReflectValueBox) + Send + Sync + 'static,
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
    {
        struct Impl<M, G, H, S> {
            get_field: G,
            mut_field_or_default: H,
            set_field: S,
            _marker: marker::PhantomData<M>,
        }

        impl<M, G, H, S> SingularFieldAccessor for Impl<M, G, H, S>
        where
            M: MessageFull,
            G: for<'a> Fn(&'a M) -> Option<ReflectValueRef<'a>> + Send + Sync + 'static,
            H: for<'a> Fn(&'a mut M) -> ReflectValueMut<'a> + Send + Sync + 'static,
            S: Fn(&mut M, ReflectValueBox) + Send + Sync + 'static,
        {
            fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
                (self.get_field)(m.downcast_ref::<M>().unwrap())
            }

            fn mut_field_or_default<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
                (self.mut_field_or_default)(m.downcast_mut::<M>().unwrap())
            }

            fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
                (self.set_field)(m.downcast_mut::<M>().unwrap(), value);
            }
        }

        SingularFieldAccessorHolder {
            accessor: Box::new(Impl {
                get_field,
                mut_field_or_default,
                set_field,
                _marker: marker::PhantomData,
            }),
        }
    }

    fn new_get_mut<M, V>(
        get_field: for<'a> fn(&'a M) -> &'a V,
        mut_field: for<'a> fn(&'a mut M) -> &'a mut V,
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        V: ProtobufValue,
    {
        Self::new(
            move |m| {
                let v = (get_field)(m);
                if V::is_non_zero(v) {
                    Some(V::as_ref(v))
                } else {
                    None
                }
            },
            move |m| V::as_mut((mut_field)(m)),
            move |m, value| V::set_from_value_box((mut_field)(m), value),
        )
    }

    fn new_get_option_mut_option<M, V>(
        get_field: for<'a> fn(&'a M) -> &'a Option<V>,
        mut_field: for<'a> fn(&'a mut M) -> &'a mut Option<V>,
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        V: ProtobufValue,
    {
        Self::new(
            move |m| (get_field)(m).as_option_ref().map(V::as_ref),
            move |_m| unimplemented!(),
            move |m, value| (mut_field)(m).set_value(V::from_value_box(value).expect("wrong type")),
        )
    }

    fn new_get_mut_message<M, V>(
        get_field: for<'a> fn(&'a M) -> &'a MessageField<V>,
        mut_field: for<'a> fn(&'a mut M) -> &'a mut MessageField<V>,
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        V: MessageFull,
    {
        Self::new(
            move |m| (get_field)(m).as_option_ref().map(V::as_ref),
            move |m| {
                let option = (mut_field)(m);
                if option.as_option_ref().is_none() {
                    option.set_value(V::default());
                }
                V::as_mut(option.as_option_mut().unwrap())
            },
            move |m, value| (mut_field)(m).set_value(V::from_value_box(value).expect("wrong type")),
        )
    }

    pub(crate) fn new_get_option_set_enum<M, E>(
        get: fn(&M) -> Option<EnumOrUnknown<E>>,
        set: fn(&mut M, EnumOrUnknown<E>),
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        E: EnumFull,
    {
        struct Impl<M, E> {
            get: fn(&M) -> Option<EnumOrUnknown<E>>,
            set: fn(&mut M, EnumOrUnknown<E>),
        }

        impl<M: MessageFull, E: EnumFull> SingularFieldAccessor for Impl<M, E> {
            fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
                let m = m.downcast_ref().unwrap();
                let value = (self.get)(m);
                value.map(|v| ReflectValueRef::Enum(E::enum_descriptor_static(), v.value()))
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

        SingularFieldAccessorHolder {
            accessor: Box::new(Impl { get, set }),
        }
    }

    pub(crate) fn new_has_get_set<M, V>(
        has: fn(&M) -> bool,
        get: fn(&M) -> V,
        set: fn(&mut M, V),
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        V: ProtobufValue + Copy,
    {
        struct Impl<M, V> {
            has: fn(&M) -> bool,
            get: fn(&M) -> V,
            set: fn(&mut M, V),
        }

        impl<M, V> SingularFieldAccessor for Impl<M, V>
        where
            M: MessageFull,
            V: ProtobufValue + Copy,
        {
            fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
                let m = m.downcast_ref().unwrap();
                if (self.has)(m) {
                    Some(V::into_static_value_ref((self.get)(m)))
                } else {
                    None
                }
            }

            fn mut_field_or_default<'a>(&self, _m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
                unimplemented!()
            }

            fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
                let m = m.downcast_mut().unwrap();
                (self.set)(m, value.downcast::<V>().expect("wrong type"))
            }
        }

        SingularFieldAccessorHolder {
            accessor: Box::new(Impl { has, get, set }),
        }
    }

    pub(crate) fn new_has_get_set_deref<M, V>(
        has: fn(&M) -> bool,
        get: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
        set: fn(&mut M, V),
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        V: ProtobufValue,
        V::RuntimeType: RuntimeTypeWithDeref,
    {
        struct Impl<M, V>
        where
            M: MessageFull,
            V: ProtobufValue,
            V::RuntimeType: RuntimeTypeWithDeref,
        {
            has: fn(&M) -> bool,
            get: for<'a> fn(&'a M) -> &'a <V::RuntimeType as RuntimeTypeWithDeref>::DerefTarget,
            set: fn(&mut M, V),
        }

        impl<M, V> SingularFieldAccessor for Impl<M, V>
        where
            M: MessageFull,
            V: ProtobufValue,
            V::RuntimeType: RuntimeTypeWithDeref,
        {
            fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
                let m = m.downcast_ref().unwrap();
                if (self.has)(m) {
                    Some(<V::RuntimeType as RuntimeTypeWithDeref>::defef_as_ref(
                        (self.get)(m),
                    ))
                } else {
                    None
                }
            }

            fn mut_field_or_default<'a>(&self, _m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
                unimplemented!()
            }

            fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
                let m = m.downcast_mut().unwrap();
                (self.set)(m, value.downcast::<V>().expect("message"))
            }
        }

        SingularFieldAccessorHolder {
            accessor: Box::new(Impl { has, get, set }),
        }
    }

    pub(crate) fn new_has_get_mut_set<M, F>(
        has_field: fn(&M) -> bool,
        get_field: for<'a> fn(&'a M) -> &'a F,
        mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
        set_field: fn(&mut M, F),
    ) -> SingularFieldAccessorHolder
    where
        M: MessageFull,
        F: MessageFull,
    {
        struct Impl<M, F> {
            has_field: fn(&M) -> bool,
            get_field: for<'a> fn(&'a M) -> &'a F,
            mut_field: for<'a> fn(&'a mut M) -> &'a mut F,
            set_field: fn(&mut M, F),
        }

        impl<M, F> SingularFieldAccessor for Impl<M, F>
        where
            M: MessageFull,
            F: MessageFull,
        {
            fn get_field<'a>(&self, m: &'a dyn MessageDyn) -> Option<ReflectValueRef<'a>> {
                let m = m.downcast_ref().unwrap();
                if (self.has_field)(m) {
                    Some(F::as_ref((self.get_field)(m)))
                } else {
                    None
                }
            }

            fn mut_field_or_default<'a>(&self, m: &'a mut dyn MessageDyn) -> ReflectValueMut<'a> {
                let m = m.downcast_mut().unwrap();
                F::as_mut((self.mut_field)(m))
            }

            fn set_field(&self, m: &mut dyn MessageDyn, value: ReflectValueBox) {
                let m = m.downcast_mut().unwrap();
                (self.set_field)(m, value.downcast::<F>().expect("message"))
            }
        }

        SingularFieldAccessorHolder {
            accessor: Box::new(Impl {
                has_field,
                get_field,
                mut_field,
                set_field,
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
