use std::slice;

use crate::reflect::ProtobufValue;
use crate::reflect::ReflectValueRef;

trait ReflectRepeatedIterDyn<'a> {
    fn next(&mut self) -> Option<ReflectValueRef<'a>>;
}

struct ReflectRepeatedIterImplSlice<'a, V: ProtobufValue + 'static> {
    pub(crate) iter: slice::Iter<'a, V>,
}

impl<'a, V: ProtobufValue> ReflectRepeatedIterDyn<'a> for ReflectRepeatedIterImplSlice<'a, V> {
    fn next(&mut self) -> Option<ReflectValueRef<'a>> {
        self.iter.next().map(ProtobufValue::as_ref)
    }
}

pub(crate) struct ReflectRepeatedIter<'a> {
    imp: Box<dyn ReflectRepeatedIterDyn<'a> + 'a>,
}

impl<'a> ReflectRepeatedIter<'a> {
    pub(crate) fn new_slice(slice: &'a [impl ProtobufValue]) -> ReflectRepeatedIter<'a> {
        ReflectRepeatedIter {
            imp: Box::new(ReflectRepeatedIterImplSlice { iter: slice.iter() }),
        }
    }
}

impl<'a> Iterator for ReflectRepeatedIter<'a> {
    type Item = ReflectValueRef<'a>;

    fn next(&mut self) -> Option<ReflectValueRef<'a>> {
        self.imp.next()
    }
}
