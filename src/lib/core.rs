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
use stream::InputSource;
use stream::WithCodedInputStream;
use stream::WithCodedOutputStream;
use stream::CodedInputStream;
use stream::CodedOutputStream;
use stream::with_coded_output_stream_to_bytes;
use error::ProtobufError;
use error::ProtobufResult;


pub trait Message : fmt::Debug + Clear + Any + 'static {
    // All generated Message types also implement MessageStatic.
    // However, rust doesn't allow these types to be extended by
    // Message.

    fn descriptor(&self) -> &'static MessageDescriptor;

    // all required fields set
    fn is_initialized(&self) -> bool;

    // sizes of this messages (and nested messages) must be cached
    // by calling `compute_size` prior to this call
    fn write_to_with_cached_sizes(&self, os: &mut CodedOutputStream) -> ProtobufResult<()>;

    // compute and cache size of this message and all nested messages
    fn compute_size(&self) -> u32;

    // get size previously computed by `compute_size`
    fn get_cached_size(&self) -> u32;

    fn write_to(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        try!(self.check_initialized());

        // cache sizes
        self.compute_size();
        try!(self.write_to_with_cached_sizes(os));

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    fn write_length_delimited_to(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        let size = self.compute_size();
        try!(os.write_raw_varint32(size));
        try!(self.write_to_with_cached_sizes(os));

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    fn check_initialized(&self) -> ProtobufResult<()> {
        if !self.is_initialized() {
            Err((ProtobufError::message_not_initialized(self.descriptor().name())))
        } else {
            Ok(())
        }
    }

    fn write_to_writer(&self, w: &mut Write) -> ProtobufResult<()> {
        w.with_coded_output_stream(|os| {
            self.write_to(os)
        })
    }

    fn write_to_vec(&self, v: &mut Vec<u8>) -> ProtobufResult<()> {
        v.with_coded_output_stream(|os| {
            self.write_to(os)
        })
    }

    fn write_to_bytes(&self) -> ProtobufResult<Vec<u8>> {
        // TODO: compute message size and reserve that size
        let mut v = Vec::new();
        try!(self.write_to_vec(&mut v));
        Ok(v)
    }

    fn write_length_delimited_to_writer(&self, w: &mut Write) -> ProtobufResult<()> {
        w.with_coded_output_stream(|os| {
            self.write_length_delimited_to(os)
        })
    }

    fn write_length_delimited_to_bytes(&self) -> ProtobufResult<Vec<u8>> {
        with_coded_output_stream_to_bytes(|os| {
            self.write_length_delimited_to(os)
        })
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s UnknownFields;
    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut UnknownFields;

    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn as_any(&self) -> &Any;

    // Rust does not allow implementation of trait for trait:
    // impl<M : Message> fmt::Debug for M {
    // ...
    // }
}

pub trait CodedMessage: Message {
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ProtobufResult<()> {
        let mut is = CodedInputStream::from_bytes(bytes);
        self.merge_from(&mut is)
    }

    fn merge_from<I: InputSource>(&mut self, is: &mut CodedInputStream<I>) -> ProtobufResult<()>;
}

// For some reason Rust doesn't allow conversion of &Foo to &Message it
// Message contains static functions. So static functions must be placed
// into separate place.
//
// See https://github.com/rust-lang/rust/commit/cd31e6ff for details.
pub trait MessageStatic : Message + Clone + Default + PartialEq /* + ProtobufValue */ {
    fn new() -> Self;

    // http://stackoverflow.com/q/20342436/15018
    fn descriptor_static(_: Option<Self>) -> &'static MessageDescriptor {
        panic!("descriptor_static is not implemented for message, \
            LITE_RUNTIME must be used");
    }
}



pub fn message_down_cast<'a, M : Message + 'a>(m: &'a Message) -> &'a M {
    m.as_any().downcast_ref::<M>().unwrap()
}


pub trait ProtobufEnum : Eq + Sized + Copy + 'static /* + ProtobufValue */ {
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

pub fn parse_from<M, I>(is: &mut CodedInputStream<I>) -> ProtobufResult<M>
        where M: CodedMessage + MessageStatic, I: InputSource {
    let mut r: M = MessageStatic::new();
    try!(r.merge_from(is));
    try!(r.check_initialized());
    Ok(r)
}

pub fn parse_from_reader<M, R>(reader: R) -> ProtobufResult<M>
        where M: CodedMessage + MessageStatic, R: Read {
    let mut is = CodedInputStream::new(reader);
    parse_from(&mut is)
}

pub fn parse_from_bytes<M : CodedMessage + MessageStatic>(bytes: &[u8]) -> ProtobufResult<M> {
    let mut is = CodedInputStream::from_bytes(bytes);
    parse_from(&mut is)
}

pub fn parse_length_delimited_from<M, I>(is: &mut CodedInputStream<I>)
        -> ProtobufResult<M>
        where M: CodedMessage + MessageStatic, I: InputSource
{
    is.read_message::<M>()
}

pub fn parse_length_delimited_from_reader<M, R>(r: R) -> ProtobufResult<M>
        where M: CodedMessage + MessageStatic, R: Read {
    // TODO: wrong: we may read length first, and then read exact number of bytes needed
    r.with_coded_input_stream(|is| {
        is.read_message::<M>()
    })
}

pub fn parse_length_delimited_from_bytes<M : CodedMessage + MessageStatic>(bytes: &[u8]) -> ProtobufResult<M> {
    let mut is = CodedInputStream::from_bytes(bytes);
    is.read_message()
}


