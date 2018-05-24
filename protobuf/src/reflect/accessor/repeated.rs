use std::marker;

use Message;
use RepeatedField;

use reflect::repeated::ReflectRepeated;
use reflect::runtime_types::RuntimeType;
use reflect::accessor::AccessorKind;
use reflect::accessor::FieldAccessor;
use reflect::types::ProtobufType;
use reflect::ProtobufValue;
use reflect::repeated::ReflectRepeatedRef;
use core::message_down_cast;
use reflect::repeated::ReflectRepeatedMut;
use core::message_down_cast_mut;
use std::fmt;
use reflect::type_dynamic::ProtobufTypeDynamic;


pub(crate) trait RepeatedFieldAccessor : Send + Sync + 'static {
    fn len_field_generic(&self, m: &Message) -> usize;

    fn get_reflect<'a>(&self, m: &'a Message) -> ReflectRepeatedRef<'a>;
    fn mut_reflect<'a>(&self, m: &'a mut Message) -> ReflectRepeatedMut<'a>;

    fn element_protobuf_type(&self) -> &ProtobufTypeDynamic;
}


trait RepeatedFieldGetMut<M, R : ?Sized> : Send + Sync + 'static
    where
        M : Message + 'static,
{
    fn get_field<'a>(&self, message: &'a M) -> &'a R;
    fn mut_field<'a>(&self, message: &'a mut M) -> &'a mut R;
}

struct RepeatedFieldGetMutImpl<M, L>
    where
        M : Message + 'static,
{
    get_field: for<'a> fn(&'a M) -> &'a L,
    mut_field: for<'a> fn(&'a mut M) -> &'a mut L,
}

impl<M, V> RepeatedFieldGetMut<M, ReflectRepeated> for RepeatedFieldGetMutImpl<M, Vec<V>>
    where
        M : Message + 'static,
        V : ProtobufValue + fmt::Debug + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a ReflectRepeated {
        (self.get_field)(m) as &ReflectRepeated
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut ReflectRepeated {
        (self.mut_field)(m) as &mut ReflectRepeated
    }
}

impl<M, V> RepeatedFieldGetMut<M, ReflectRepeated> for RepeatedFieldGetMutImpl<M, RepeatedField<V>>
    where
        M : Message + 'static,
        V : ProtobufValue + fmt::Debug + 'static,
{
    fn get_field<'a>(&self, m: &'a M) -> &'a ReflectRepeated {
        (self.get_field)(m) as &ReflectRepeated
    }

    fn mut_field<'a>(&self, m: &'a mut M) -> &'a mut ReflectRepeated {
        (self.mut_field)(m) as &mut ReflectRepeated
    }
}


struct RepeatedFieldAccessorImpl<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fns: Box<RepeatedFieldGetMut<M, ReflectRepeated>>,
    _marker: marker::PhantomData<V>,
}

impl<M, V> RepeatedFieldAccessor for RepeatedFieldAccessorImpl<M, V>
    where
        M : Message,
        V : ProtobufType,
{
    fn len_field_generic(&self, m: &Message) -> usize {
        let m = message_down_cast(m);
        self.fns.get_field(m).len()
    }

    fn get_reflect<'a>(&self, m: &'a Message) -> ReflectRepeatedRef<'a> {
        let m = message_down_cast(m);
        let repeated = self.fns.get_field(m);
        ReflectRepeatedRef {
            repeated,
            dynamic: V::RuntimeType::dynamic(),
        }
    }

    fn mut_reflect<'a>(&self, m: &'a mut Message) -> ReflectRepeatedMut<'a> {
        let m = message_down_cast_mut(m);
        let repeated = self.fns.mut_field(m);
        ReflectRepeatedMut {
            repeated,
            dynamic: V::RuntimeType::dynamic(),
        }
    }

    fn element_protobuf_type(&self) -> &ProtobufTypeDynamic {
        V::dynamic()
    }
}



pub fn make_vec_accessor<M, V>(
    name: &'static str,
    get_vec: for<'a> fn(&'a M) -> &'a Vec<<V::RuntimeType as RuntimeType>::Value>,
    mut_vec: for<'a> fn(&'a mut M) -> &'a mut Vec<<V::RuntimeType as RuntimeType>::Value>,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Repeated(Box::new(RepeatedFieldAccessorImpl::<M, V> {
            fns: Box::new(RepeatedFieldGetMutImpl::<M, Vec<<V::RuntimeType as RuntimeType>::Value>> {
                get_field: get_vec,
                mut_field: mut_vec,
            }),
            _marker: marker::PhantomData::<V>,
        }))
    }
}


pub fn make_repeated_field_accessor<M, V>(
    name: &'static str,
    get_vec: for<'a> fn(&'a M) -> &'a RepeatedField<<V::RuntimeType as RuntimeType>::Value>,
    mut_vec: for<'a> fn(&'a mut M)
        -> &'a mut RepeatedField<<V::RuntimeType as RuntimeType>::Value>,
) -> FieldAccessor
    where
        M : Message + 'static,
        V : ProtobufType + 'static,
{
    FieldAccessor {
        name,
        accessor: AccessorKind::Repeated(Box::new(RepeatedFieldAccessorImpl::<M, V> {
            fns: Box::new(RepeatedFieldGetMutImpl::<M, RepeatedField<<V::RuntimeType as RuntimeType>::Value>> {
                get_field: get_vec,
                mut_field: mut_vec,
            }),
            _marker: marker::PhantomData::<V>,
        }))
    }
}
