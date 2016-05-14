// TODO: drop all panic!

use std::any::Any;
use std::any::TypeId;
use std::default::Default;
use std::fmt;
use std::io::Read;
use std::io::Write;

use clear::Clear;
use reflect::MessageDescriptor;
use reflect::EnumDescriptor;
use reflect::EnumValueDescriptor;
use unknown::UnknownFields;
use stream::WithCodedInputStream;
use stream::CodedInputStream;
use stream::CodedOutputStream;
use error::ProtobufError;
use error::ProtobufResult;


// For some reason Rust doesn't allow conversion of &Foo to &Message it
// Message contains static functions. So static functions must be placed
// into separate place. For me it looks like unnecessary complication.
//
// See https://github.com/rust-lang/rust/commit/cd31e6ff for details.
pub trait MessageStatic : Message + Clone + Default + PartialEq {
    fn new() -> Self;

    // http://stackoverflow.com/q/20342436/15018
    fn descriptor_static(_: Option<Self>) -> &'static MessageDescriptor {
        panic!("descriptor_static is not implemented for message, \
            LITE_RUNTIME must be used");
    }
}

pub trait Message : fmt::Debug + Clear + Any {
    // All generated Message types also implement MessageStatic.
    // However, rust doesn't allow these types to be extended by
    // Message.

    fn descriptor(&self) -> &'static MessageDescriptor;

    // all required fields set
    fn is_initialized(&self) -> bool;
    fn merge_from(&mut self, is: &mut CodedInputStream) -> ProtobufResult<()>;

    // sizes of this messages (and nested messages) must be cached
    // by calling `compute_size` prior to this call
    fn write_to_with_cached_sizes<W>(&self, w: &mut W) -> ProtobufResult<()> where W: Write, Self: Sized;

    // compute and cache size of this message and all nested messages
    fn compute_size(&self) -> u32;

    // get size previously computed by `compute_size`
    fn get_cached_size(&self) -> u32;

    fn write_to<W>(&self, w: &mut W) -> ProtobufResult<()> where W: Write, Self: Sized {
        try!(self.check_initialized());

        // cache sizes
        self.compute_size();
        try!(self.write_to_with_cached_sizes(w));

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    fn write_length_delimited_to<W>(&self, w: &mut W) -> ProtobufResult<()> where W: Write, Self: Sized {
        let size = self.compute_size();
        try!(w.write_raw_varint32(size));
        try!(self.write_to_with_cached_sizes(w));

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ProtobufResult<()> {
        let mut is = CodedInputStream::from_bytes(bytes);
        self.merge_from(&mut is)
    }

    fn check_initialized(&self) -> ProtobufResult<()> {
        if !self.is_initialized() {
            Err((ProtobufError::message_not_initialized(self.descriptor().name())))
        } else {
            Ok(())
        }
    }

    fn write_to_bytes(&self) -> ProtobufResult<Vec<u8>> where Self: Sized {
        // TODO: compute message size and reserve that size
        let mut v = Vec::new();
        try!(self.write_to(&mut v));
        Ok(v)
    }

    fn write_length_delimited_to_bytes(&self) -> ProtobufResult<Vec<u8>> where Self: Sized {
        // TODO: compute message size and reserve that size
        let mut v = Vec::new();
        try!(self.write_length_delimited_to(&mut v));
        Ok(v)
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s UnknownFields;
    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut UnknownFields;

    fn type_id(&self) -> TypeId;
    fn as_any(&self) -> &Any;

    // Rust does not allow implementation of trait for trait:
    // impl<M : Message> fmt::Debug for M {
    // ...
    // }
}

pub fn message_is<M : Message>(m: &Message) -> bool {
    TypeId::of::<M>() == m.type_id()
}

pub fn message_down_cast<'a, M : Message + 'a>(m: &'a Message) -> &'a M {
    assert!(message_is::<M>(m));
    m.as_any().downcast_ref::<M>().unwrap()
}


pub trait ProtobufEnum : Eq + Sized {
    fn value(&self) -> i32;

    fn from_i32(v: i32) -> Option<Self>;

    fn values() -> &'static [Self] {
        panic!();
    }

    fn descriptor(&self) -> &'static EnumValueDescriptor {
        self.enum_descriptor().value_by_number(self.value())
    }

    fn enum_descriptor(&self) -> &'static EnumDescriptor {
        ProtobufEnum::enum_descriptor_static(None::<Self>)
    }

    // http://stackoverflow.com/q/20342436/15018
    fn enum_descriptor_static(_: Option<Self>) -> &'static EnumDescriptor {
        panic!();
    }
}

pub fn parse_from<M : Message + MessageStatic>(is: &mut CodedInputStream) -> ProtobufResult<M> {
    let mut r: M = MessageStatic::new();
    try!(r.merge_from(is));
    try!(r.check_initialized());
    Ok(r)
}

pub fn parse_from_reader<M : Message + MessageStatic>(reader: &mut Read) -> ProtobufResult<M> {
    reader.with_coded_input_stream(|is| {
        parse_from::<M>(is)
    })
}

pub fn parse_from_bytes<M : Message + MessageStatic>(bytes: &[u8]) -> ProtobufResult<M> {
    bytes.with_coded_input_stream(|is| {
        parse_from::<M>(is)
    })
}

pub fn parse_length_delimited_from<M : Message + MessageStatic>(is: &mut CodedInputStream)
        -> ProtobufResult<M>
{
    is.read_message::<M>()
}

pub fn parse_length_delimited_from_reader<M : Message + MessageStatic>(r: &mut Read) -> ProtobufResult<M> {
    // TODO: wrong: we may read length first, and then read exact number of bytes needed
    r.with_coded_input_stream(|is| {
        is.read_message::<M>()
    })
}

pub fn parse_length_delimited_from_bytes<M : Message + MessageStatic>(bytes: &[u8]) -> ProtobufResult<M> {
    bytes.with_coded_input_stream(|is| {
        is.read_message::<M>()
    })
}


