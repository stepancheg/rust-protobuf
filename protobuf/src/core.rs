// TODO: drop all panic!

use std::any::Any;
use std::any::TypeId;
use std::default::Default;
use std::fmt;
use std::io::Read;
use std::io::Write;

#[cfg(feature = "bytes")]
use bytes::Bytes;

use clear::Clear;
use reflect::MessageDescriptor;
use reflect::EnumDescriptor;
use reflect::EnumValueDescriptor;
use unknown::UnknownFields;
use stream::WithCodedInputStream;
use stream::WithCodedOutputStream;
use stream::CodedInputStream;
use stream::CodedOutputStream;
use stream::with_coded_output_stream_to_bytes;
use error::ProtobufError;
use error::ProtobufResult;


pub trait Message: fmt::Debug + Clear + Any + Send + Sync {
    // All generated Message types also implement MessageStatic.
    // However, rust doesn't allow these types to be extended by
    // Message.

    fn descriptor(&self) -> &'static MessageDescriptor;

    // all required fields set
    fn is_initialized(&self) -> bool;
    fn merge_from(&mut self, is: &mut CodedInputStream) -> ProtobufResult<()>;

    // sizes of this messages (and nested messages) must be cached
    // by calling `compute_size` prior to this call
    fn write_to_with_cached_sizes(&self, os: &mut CodedOutputStream) -> ProtobufResult<()>;

    // compute and cache size of this message and all nested messages
    fn compute_size(&self) -> u32;

    // get size previously computed by `compute_size`
    fn get_cached_size(&self) -> u32;

    fn write_to(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        self.check_initialized()?;

        // cache sizes
        self.compute_size();
        // TODO: reserve additional
        self.write_to_with_cached_sizes(os)?;

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    fn write_length_delimited_to(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        let size = self.compute_size();
        os.write_raw_varint32(size)?;
        self.write_to_with_cached_sizes(os)?;

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    fn write_length_delimited_to_vec(&self, vec: &mut Vec<u8>) -> ProtobufResult<()> {
        let mut os = CodedOutputStream::vec(vec);
        self.write_length_delimited_to(&mut os)?;
        os.flush()?;
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
        self.check_initialized()?;

        let size = self.compute_size() as usize;
        let mut v = Vec::with_capacity(size);
        // skip zerofill
        unsafe { v.set_len(size); }
        {
            let mut os = CodedOutputStream::bytes(&mut v);
            self.write_to_with_cached_sizes(&mut os)?;
            os.check_eof();
        }
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

    fn as_any_mut(&mut self) -> &mut Any {
        panic!()
    }

    fn into_any(self: Box<Self>) -> Box<Any> {
        panic!()
    }

    // Rust does not allow implementation of trait for trait:
    // impl<M : Message> fmt::Debug for M {
    // ...
    // }
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

pub fn parse_from<M : Message + MessageStatic>(is: &mut CodedInputStream) -> ProtobufResult<M> {
    let mut r: M = MessageStatic::new();
    r.merge_from(is)?;
    r.check_initialized()?;
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

#[cfg(feature = "bytes")]
pub fn parse_from_carllerche_bytes<M : Message + MessageStatic>(bytes: &Bytes) -> ProtobufResult<M> {
    // Call trait explicitly to avoid accidental construction from `&[u8]`
    WithCodedInputStream::with_coded_input_stream(bytes, |is| {
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


