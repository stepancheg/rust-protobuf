use std::slice;

use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueRef;

pub(crate) trait ReflectRepeatedIterTrait<'a> {
    fn next(&mut self) -> Option<ReflectValueRef<'a>>;
}

pub(crate) struct ReflectRepeatedIterImplSlice<'a, V: ProtobufValue + 'static> {
    pub(crate) iter: slice::Iter<'a, V>,
}

impl<'a, V: ProtobufValue + 'static> ReflectRepeatedIterTrait<'a>
    for ReflectRepeatedIterImplSlice<'a, V>
{
    fn next(&mut self) -> Option<ReflectValueRef<'a>> {
        self.iter.next().map(ProtobufValue::as_ref)
    }
}

pub(crate) struct ReflectRepeatedIter<'a> {
    imp: Box<dyn ReflectRepeatedIterTrait<'a> + 'a>,
}

impl<'a> ReflectRepeatedIter<'a> {
    pub(crate) fn new(imp: impl ReflectRepeatedIterTrait<'a> + 'a) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter { imp: Box::new(imp) }
    }
}

impl<'a> Iterator for ReflectRepeatedIter<'a> {
    type Item = ReflectValueRef<'a>;

    fn next(&mut self) -> Option<ReflectValueRef<'a>> {
        self.imp.next()
    }
}
