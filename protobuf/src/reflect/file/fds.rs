use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use crate::descriptor::FileDescriptorProto;
use crate::reflect::FileDescriptor;

pub(crate) struct FdsBuilder {
    names: Vec<String>,
    unprocessed: HashMap<String, FileDescriptorProto>,
    processed: HashMap<String, FileDescriptor>,
}

impl FdsBuilder {
    fn process_one(&mut self) -> crate::Result<()> {
        let (n, deps) = self
            .unprocessed
            .iter()
            .find_map(|(n, p)| {
                let deps: Result<Vec<FileDescriptor>, ()> = p
                    .dependency
                    .iter()
                    .map(|n| match self.processed.get(n) {
                        Some(d) => Ok(d.clone()),
                        None => {
                            assert!(
                                self.unprocessed.get(n).is_some(),
                                "unsatisfied dependency: {}",
                                n
                            );
                            Err(())
                        }
                    })
                    .collect();
                deps.ok().map(|deps| (n, deps))
            })
            .unwrap();
        let n = n.clone();
        let proto = self.unprocessed.remove(&n).unwrap();
        self.processed
            .insert(n.clone(), FileDescriptor::new_dynamic(proto, deps)?);
        Ok(())
    }

    pub(crate) fn build(protos: Vec<FileDescriptorProto>) -> crate::Result<Vec<FileDescriptor>> {
        let mut builder = FdsBuilder {
            names: protos.iter().map(|p| p.name().to_owned()).collect(),
            unprocessed: protos
                .into_iter()
                .map(|p| (p.name().to_owned(), p))
                .collect(),
            processed: HashMap::new(),
        };

        while !builder.unprocessed.is_empty() {
            builder.process_one()?;
        }

        let mut processed = builder.processed;
        Ok(builder
            .names
            .iter()
            .map(|n| processed.remove(n).unwrap())
            .collect())
    }
}

pub(crate) fn fds_extend_with_public(file_descriptors: Vec<FileDescriptor>) -> Vec<FileDescriptor> {
    let mut visited = HashSet::new();

    let mut r = Vec::new();
    let mut stack = file_descriptors;
    stack.reverse();

    while let Some(f) = stack.pop() {
        if !visited.insert(f.proto().name().to_owned()) {
            continue;
        }

        stack.extend(f.public_deps());

        r.push(f);
    }
    r
}

fn all_deps_impl<T: Clone + Eq + Hash>(input: &[T], deps: impl Fn(&T) -> Vec<T>) -> Vec<T> {
    let mut visited = HashSet::new();

    let mut r = Vec::new();
    let mut stack: Vec<T> = input.iter().rev().cloned().collect();

    while let Some(f) = stack.pop() {
        if !visited.insert(f.clone()) {
            continue;
        }

        stack.extend(deps(&f).into_iter());

        r.push(f);
    }
    r
}

pub(crate) fn all_deps(file_descriptor: &[FileDescriptor]) -> Vec<FileDescriptor> {
    all_deps_impl(file_descriptor, |f| f.deps().to_vec())
}

#[cfg(test)]
mod test {
    use crate::reflect::FileDescriptor;

    #[test]
    fn all_deps() {
        assert_eq!(Vec::<FileDescriptor>::new(), super::all_deps(&[]));
    }

    #[test]
    fn all_deps_impl() {
        let mut all_deps = super::all_deps_impl(&["A", "B"], |&x| match x {
            "A" => vec!["B", "C"],
            "C" => vec!["D"],
            _ => vec![],
        });
        all_deps.sort();
        assert_eq!(vec!["A", "B", "C", "D"], all_deps);
    }
}
