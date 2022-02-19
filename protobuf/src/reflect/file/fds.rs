use std::collections::HashMap;
use std::collections::HashSet;

use crate::descriptor::FileDescriptorProto;
use crate::reflect::FileDescriptor;

pub(crate) struct FdsBuilder {
    names: Vec<String>,
    unprocessed: HashMap<String, FileDescriptorProto>,
    processed: HashMap<String, FileDescriptor>,
}

impl FdsBuilder {
    fn process_one(&mut self) {
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
            .insert(n.clone(), FileDescriptor::new_dynamic(proto, deps));
    }

    pub fn build(protos: Vec<FileDescriptorProto>) -> Vec<FileDescriptor> {
        let mut builder = FdsBuilder {
            names: protos.iter().map(|p| p.name().to_owned()).collect(),
            unprocessed: protos
                .into_iter()
                .map(|p| (p.name().to_owned(), p))
                .collect(),
            processed: HashMap::new(),
        };

        while !builder.unprocessed.is_empty() {
            builder.process_one();
        }

        let mut processed = builder.processed;
        builder
            .names
            .iter()
            .map(|n| processed.remove(n).unwrap())
            .collect()
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
