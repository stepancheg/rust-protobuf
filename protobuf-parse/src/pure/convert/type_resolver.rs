use crate::model;
use crate::pure::convert::WithFullName;
use crate::ProtobufAbsPath;
use crate::ProtobufIdent;
use crate::ProtobufIdentRef;
use crate::ProtobufRelPath;
use crate::ProtobufRelPathRef;

pub(crate) enum MessageOrEnum<'a> {
    Message(&'a model::Message),
    Enum(&'a model::Enumeration),
}

impl MessageOrEnum<'_> {
    fn _descriptor_type(&self) -> protobuf::descriptor::field_descriptor_proto::Type {
        match *self {
            MessageOrEnum::Message(..) => {
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_MESSAGE
            }
            MessageOrEnum::Enum(..) => {
                protobuf::descriptor::field_descriptor_proto::Type::TYPE_ENUM
            }
        }
    }
}

#[derive(Clone)]
pub(crate) enum LookupScope<'a> {
    File(&'a model::FileDescriptor),
    Message(&'a model::Message, ProtobufAbsPath),
}

impl<'a> LookupScope<'a> {
    fn current_path(&self) -> ProtobufAbsPath {
        match self {
            LookupScope::File(f) => f.package.clone(),
            LookupScope::Message(_, p) => p.clone(),
        }
    }

    fn messages(&self) -> &'a [model::WithLoc<model::Message>] {
        match self {
            &LookupScope::File(file) => &file.messages,
            &LookupScope::Message(messasge, _) => &messasge.messages,
        }
    }

    fn find_message(&self, simple_name: &ProtobufIdentRef) -> Option<&'a model::Message> {
        self.messages()
            .into_iter()
            .find(|m| m.t.name == simple_name.as_str())
            .map(|m| &m.t)
    }

    fn enums(&self) -> &'a [model::Enumeration] {
        match self {
            &LookupScope::File(file) => &file.enums,
            &LookupScope::Message(messasge, _) => &messasge.enums,
        }
    }

    fn members(&self) -> Vec<(ProtobufIdent, MessageOrEnum<'a>)> {
        let mut r = Vec::new();
        r.extend(
            self.enums()
                .into_iter()
                .map(|e| (ProtobufIdent::from(&e.name[..]), MessageOrEnum::Enum(e))),
        );
        r.extend(self.messages().into_iter().map(|m| {
            (
                ProtobufIdent::from(&m.t.name[..]),
                MessageOrEnum::Message(&m.t),
            )
        }));
        r
    }

    fn find_member(&self, simple_name: &ProtobufIdentRef) -> Option<MessageOrEnum<'a>> {
        self.members()
            .into_iter()
            .filter_map(|(member_name, message_or_enum)| {
                if member_name.as_ref() == simple_name {
                    Some(message_or_enum)
                } else {
                    None
                }
            })
            .next()
    }

    fn down(&self, name: &ProtobufIdentRef) -> Option<LookupScope<'a>> {
        match self.find_member(name)? {
            MessageOrEnum::Enum(_) => return None,
            MessageOrEnum::Message(m) => {
                let mut path = self.current_path();
                path.push_simple(name.clone());
                Some(LookupScope::Message(m, path))
            }
        }
    }

    pub(crate) fn find_message_or_enum(
        &self,
        path: &ProtobufRelPathRef,
    ) -> Option<WithFullName<MessageOrEnum<'a>>> {
        let current_path = self.current_path();
        let (first, rem) = match path.split_first_rem() {
            Some(x) => x,
            None => return None,
        };

        if rem.is_empty() {
            match self.find_member(first) {
                Some(message_or_enum) => {
                    let mut result_path = current_path.clone();
                    result_path.push_simple(first);
                    Some(WithFullName {
                        full_name: result_path,
                        t: message_or_enum,
                    })
                }
                None => None,
            }
        } else {
            match self.find_message(first) {
                Some(message) => {
                    let mut message_path = current_path.clone();
                    message_path.push_simple(ProtobufIdentRef::new(&message.name));
                    let message_scope = LookupScope::Message(message, message_path);
                    message_scope.find_message_or_enum(rem)
                }
                None => None,
            }
        }
    }

    fn extensions(&self) -> Vec<&'a model::Extension> {
        match self {
            LookupScope::File(f) => f.extensions.iter().map(|e| &e.t).collect(),
            LookupScope::Message(m, _) => m.extensions.iter().map(|e| &e.t).collect(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct LookupScopeUnion<'a> {
    pub(crate) path: ProtobufAbsPath,
    pub(crate) scopes: Vec<LookupScope<'a>>,
    pub(crate) partial_scopes: Vec<&'a model::FileDescriptor>,
}

impl<'a> LookupScopeUnion<'a> {
    fn down(&self, name: &ProtobufIdentRef) -> LookupScopeUnion<'a> {
        let mut path: ProtobufAbsPath = self.path.clone();
        path.push_simple(name);

        let mut scopes: Vec<_> = self.scopes.iter().flat_map(|f| f.down(name)).collect();
        let mut partial_scopes = Vec::new();

        for &partial_scope in &self.partial_scopes {
            if partial_scope.package == path {
                scopes.push(LookupScope::File(partial_scope));
            } else if partial_scope.package.starts_with(&path) {
                partial_scopes.push(partial_scope);
            }
        }
        LookupScopeUnion {
            path,
            scopes,
            partial_scopes,
        }
    }

    pub(crate) fn lookup(&self, path: &ProtobufRelPath) -> LookupScopeUnion<'a> {
        let mut scope = self.clone();
        for c in path.components() {
            scope = scope.down(c);
        }
        scope
    }

    pub(crate) fn extensions(&self) -> Vec<&'a model::Extension> {
        self.scopes.iter().flat_map(|s| s.extensions()).collect()
    }
}
