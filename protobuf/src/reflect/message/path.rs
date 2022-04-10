use crate::descriptor::DescriptorProto;
use crate::descriptor::FileDescriptorProto;

#[derive(Clone, Debug)]
pub(crate) struct MessagePath(pub Vec<usize>);

impl MessagePath {
    pub(crate) fn push(&mut self, index: usize) {
        self.0.push(index);
    }

    pub(crate) fn pop(&mut self) -> Option<usize> {
        self.0.pop()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn eval_path<'a>(&self, file: &'a FileDescriptorProto) -> Vec<&'a DescriptorProto> {
        let mut r = Vec::new();
        if self.0.len() != 0 {
            let mut m = &file.message_type[self.0[0]];
            r.push(m);
            for &p in &self.0[1..] {
                m = &m.nested_type[p];
                r.push(m);
            }
        }
        r
    }

    pub(crate) fn eval<'a>(&self, file: &'a FileDescriptorProto) -> Option<&'a DescriptorProto> {
        self.eval_path(file).last().cloned()
    }
}
