use std::fmt;


// Represent subset of rust types used in generated code
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum RustType {
    // integer: signed?, size in bits
    Int(bool, u32),
    // param is size in bits
    Float(u32),
    Bool,
    Vec(Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    String,
    // [T], not &[T]
    Slice(Box<RustType>),
    // str, not &str
    Str,
    Option(Box<RustType>),
    SingularField(Box<RustType>),
    SingularPtrField(Box<RustType>),
    RepeatedField(Box<RustType>),
    // Box<T>
    Uniq(Box<RustType>),
    // &T
    Ref(Box<RustType>),
    // protobuf message
    Message(String),
    // protobuf enum, not any enum
    Enum(String, String),
    // oneof enum
    Oneof(String),
    // group
    Group,
}

impl fmt::Display for RustType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RustType::Int(true, bits)    => write!(f, "i{}", bits),
            RustType::Int(false, bits)   => write!(f, "u{}", bits),
            RustType::Float(bits)        => write!(f, "f{}", bits),
            RustType::Bool               => write!(f, "bool"),
            RustType::Vec(ref param)     => write!(f, "::std::vec::Vec<{}>", **param),
            RustType::HashMap(ref key, ref value) =>
                write!(f, "::std::collections::HashMap<{}, {}>", **key, **value),
            RustType::String             => write!(f, "::std::string::String"),
            RustType::Slice(ref param)   => write!(f, "[{}]", **param),
            RustType::Str                => write!(f, "str"),
            RustType::Option(ref param)           => write!(f, "::std::option::Option<{}>", **param),
            RustType::SingularField(ref param)    => write!(f, "::protobuf::SingularField<{}>", **param),
            RustType::SingularPtrField(ref param) => write!(f, "::protobuf::SingularPtrField<{}>", **param),
            RustType::RepeatedField(ref param)    => write!(f, "::protobuf::RepeatedField<{}>", **param),
            RustType::Uniq(ref param)             => write!(f, "::std::boxed::Box<{}>", **param),
            RustType::Ref(ref param)              => write!(f, "&{}", **param),
            RustType::Message(ref name) |
            RustType::Enum(ref name, _)    |
            RustType::Oneof(ref name)   => write!(f, "{}", name),
            RustType::Group             => write!(f, "<group>"),
        }
    }
}

impl RustType {
    pub fn u8() -> RustType {
        RustType::Int(false, 8)
    }

    /// Type is rust primitive?
    pub fn is_primitive(&self) -> bool {
        match *self {
            RustType::Int(..)      |
            RustType::Float(..)    |
            RustType::Bool         => true,
            _                      => false,
        }
    }

    fn is_str(&self) -> bool {
        match *self {
            RustType::Str => true,
            _ => false
        }
    }

    fn is_string(&self) -> bool {
        match *self {
            RustType::String => true,
            _ => false
        }
    }

    fn is_slice(&self) -> bool {
        match *self {
            RustType::Slice(..) => true,
            _ => false
        }
    }

    fn is_message(&self) -> bool {
        match *self {
            RustType::Message(..) => true,
            _ => false
        }
    }

    fn is_enum(&self) -> bool {
        match *self {
            RustType::Enum(..) => true,
            _ => false
        }
    }

    pub fn is_u8(&self) -> bool {
        match *self {
            RustType::Int(false, 8) => true,
            _ => false
        }
    }

    pub fn is_ref(&self) -> bool {
        match *self {
            RustType::Ref(..) => true,
            _           => false,
        }
    }

    // default value for type
    pub fn default_value(&self) -> String {
        match *self {
            RustType::Ref(ref t) if t.is_str()       => "\"\"".to_string(),
            RustType::Ref(ref t) if t.is_slice()     => "&[]".to_string(),
            RustType::Int(..)                        => "0".to_string(),
            RustType::Float(..)                      => "0.".to_string(),
            RustType::Bool                           => "false".to_string(),
            RustType::Vec(..)                        => "::std::vec::Vec::new()".to_string(),
            RustType::HashMap(..)                    => "::std::collections::HashMap::new()".to_string(),
            RustType::String                         => "::std::string::String::new()".to_string(),
            RustType::Option(..)                     => "::std::option::Option::None".to_string(),
            RustType::SingularField(..)              => "::protobuf::SingularField::none()".to_string(),
            RustType::SingularPtrField(..)           => "::protobuf::SingularPtrField::none()".to_string(),
            RustType::RepeatedField(..)              => "::protobuf::RepeatedField::new()".to_string(),
            RustType::Message(ref name)              => format!("{}::new()", name),
            RustType::Ref(ref m) if m.is_message()   => match **m {
                RustType::Message(ref name) => format!("{}::default_instance()", name),
                _ => unreachable!()
            },
            // Note: default value of enum type may not be equal to default value of field
            RustType::Enum(ref name, ref default)    => format!("{}::{}", name, default),
            _ => panic!("cannot create default value for: {}", *self),
        }
    }

    pub fn default_value_typed(self) -> RustValueTyped {
        RustValueTyped {
            value: self.default_value(),
            rust_type: self,
        }
    }

    /// Emit a code to clear a variable `v`
    pub fn clear(&self, v: &str) -> String {
        match *self {
            RustType::Option(..) => format!("{} = ::std::option::Option::None", v),
            RustType::Vec(..) |
            RustType::String |
            RustType::RepeatedField(..)    |
            RustType::SingularField(..)    |
            RustType::SingularPtrField(..) |
            RustType::HashMap(..)          => format!("{}.clear()", v),
            RustType::Bool      |
            RustType::Float(..) |
            RustType::Int(..)   |
            RustType::Enum(..)  => format!("{} = {}", v, self.default_value()),
            ref ty => panic!("cannot clear type: {:?}", ty),
        }
    }

    // wrap value in storage type
    pub fn wrap_value(&self, value: &str) -> String {
        match *self {
            RustType::Option(..)           => format!("::std::option::Option::Some({})", value),
            RustType::SingularField(..)    => format!("::protobuf::SingularField::some({})", value),
            RustType::SingularPtrField(..) => format!("::protobuf::SingularPtrField::some({})", value),
            _ => panic!("not a wrapper type: {}", *self),
        }
    }

    // expression to convert `v` of type `self` to type `target`
    pub fn into_target(&self, target: &RustType, v: &str) -> String {
        match (self, target) {
            (x, y) if x == y                        =>
                    format!("{}", v),
            (&RustType::Ref(ref x), y) if **x == *y =>
                    format!("*{}", v),
            (x, &RustType::Uniq(ref y)) if *x == **y =>
                    format!("::std::boxed::Box::new({})", v),
            (&RustType::Uniq(ref x), y) if **x == *y =>
                    format!("*{}", v),
            (&RustType::String, &RustType::Ref(ref t)) if t.is_str() =>
                    format!("&{}", v),
            (&RustType::Ref(ref t1), &RustType::Ref(ref t2)) if t1.is_string() && t2.is_str() =>
                    format!("&{}", v),
            (&RustType::Ref(ref t1), &RustType::String)
                if match **t1 { RustType::Str => true, _ => false } =>
                    format!("{}.to_owned()", v),
            (&RustType::Ref(ref t1), &RustType::Vec(ref t2))
                if match (&**t1, &**t2) {
                    (&RustType::Slice(ref x), ref y) => **x == **y,
                    _ => false
                } => format!("{}.to_vec()", v),
            (&RustType::Vec(ref x), &RustType::Ref(ref t))
                if match **t { RustType::Slice(ref y) => x == y, _ => false } =>
                    format!("&{}", v),
            (&RustType::Ref(ref t1), &RustType::Ref(ref t2))
                if match (&**t1, &**t2) {
                    (&RustType::Vec(ref x), &RustType::Slice(ref y)) => x == y,
                    _ => false
                } => format!("&{}", v),
            (&RustType::Enum(..), &RustType::Int(true, 32)) =>
                    format!("{}.value()", v),
            (&RustType::Ref(ref t), &RustType::Int(true, 32)) if t.is_enum() =>
                    format!("{}.value()", v),
            _ => panic!("cannot convert {} to {}", self, target),
        }
    }

    /// Type to view data of this type
    pub fn ref_type(&self) -> RustType {
        RustType::Ref(Box::new(match self {
            &RustType::String               => RustType::Str,
            &RustType::Vec(ref p)           |
            &RustType::RepeatedField(ref p) => RustType::Slice(p.clone()),
            &RustType::Message(ref p)       => RustType::Message(p.clone()),
            x => panic!("no ref type for {}", x),
        }))
    }

    pub fn elem_type(&self) -> RustType {
        match self {
            &RustType::Option(ref ty) => (**ty).clone(),
            x => panic!("cannot get elem type of {}", x),
        }
    }

    // type of `v` in `for v in xxx`
    pub fn iter_elem_type(&self) -> RustType {
        match self {
            &RustType::Vec(ref ty)              |
            &RustType::Option(ref ty)           |
            &RustType::RepeatedField(ref ty)    |
            &RustType::SingularField(ref ty)    |
            &RustType::SingularPtrField(ref ty) => RustType::Ref(ty.clone()),
            x => panic!("cannot iterate {}", x),
        }
    }

    pub fn value(self, value: String) -> RustValueTyped {
        RustValueTyped {
            value: value,
            rust_type: self,
        }
    }
}


/// Representation of an expression in code generator: text and type
pub struct RustValueTyped {
    pub value: String,
    pub rust_type: RustType,
}

impl RustValueTyped {
    pub fn into_type(&self, target: RustType) -> RustValueTyped {
        let target_value = self.rust_type.into_target(&target, &self.value);
        RustValueTyped {
            value: target_value,
            rust_type: target,
        }
    }

    pub fn boxed(self) -> RustValueTyped {
        self.into_type(RustType::Uniq(Box::new(self.rust_type.clone())))
    }
}
