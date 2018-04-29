use std::any::Any;
use std::any::TypeId;
use std::fmt;
use std::io::Read;
use std::io::Write;

#[cfg(feature = "bytes")]
use bytes::Bytes;

use clear::Clear;
use reflect::MessageDescriptor;
use unknown::UnknownFields;
use stream::WithCodedInputStream;
use stream::WithCodedOutputStream;
use stream::CodedInputStream;
use stream::CodedOutputStream;
use stream::with_coded_output_stream_to_bytes;
use error::ProtobufError;
use error::ProtobufResult;


/// Trait implemented for all generated structs for protobuf messages.
/// Also, generated messages implement `Clone + Default + PartialEq`
pub trait Message: fmt::Debug + Clear + Any + Send + Sync {
    /// Message descriptor for this message, used for reflection.
    fn descriptor(&self) -> &'static MessageDescriptor;

    /// True iff all required fields are initialized.
    /// Always returns `true` for protobuf 3.
    fn is_initialized(&self) -> bool;

    /// Update this message object with fields read from given stream.
    fn merge_from(&mut self, is: &mut CodedInputStream) -> ProtobufResult<()>;

    /// Write message to the stream.
    ///
    /// Sizes of this messages and nested messages must be cached
    /// by calling `compute_size` prior to this call.
    fn write_to_with_cached_sizes(&self, os: &mut CodedOutputStream) -> ProtobufResult<()>;

    /// Compute and cache size of this message and all nested messages
    fn compute_size(&self) -> u32;

    /// Get size previously computed by `compute_size`.
    fn get_cached_size(&self) -> u32;

    /// Write the message to the stream.
    ///
    /// Results in error if message is not fully initialized.
    fn write_to(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        self.check_initialized()?;

        // cache sizes
        self.compute_size();
        // TODO: reserve additional
        self.write_to_with_cached_sizes(os)?;

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    /// Write the message to the stream prepending the message with message length
    /// encoded as varint.
    fn write_length_delimited_to(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        let size = self.compute_size();
        os.write_raw_varint32(size)?;
        self.write_to_with_cached_sizes(os)?;

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    /// Write the message to the vec, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_vec(&self, vec: &mut Vec<u8>) -> ProtobufResult<()> {
        let mut os = CodedOutputStream::vec(vec);
        self.write_length_delimited_to(&mut os)?;
        os.flush()?;
        Ok(())
    }

    /// Update this message object with fields read from given stream.
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ProtobufResult<()> {
        let mut is = CodedInputStream::from_bytes(bytes);
        self.merge_from(&mut is)
    }

    /// Check if all required fields of this object are initialized.
    fn check_initialized(&self) -> ProtobufResult<()> {
        if !self.is_initialized() {
            Err(ProtobufError::message_not_initialized(self.descriptor().name()))
        } else {
            Ok(())
        }
    }

    /// Write the message to the writer.
    fn write_to_writer(&self, w: &mut Write) -> ProtobufResult<()> {
        w.with_coded_output_stream(|os| self.write_to(os))
    }

    /// Write the message to bytes vec.
    fn write_to_vec(&self, v: &mut Vec<u8>) -> ProtobufResult<()> {
        v.with_coded_output_stream(|os| self.write_to(os))
    }

    /// Write the message to bytes vec.
    fn write_to_bytes(&self) -> ProtobufResult<Vec<u8>> {
        self.check_initialized()?;

        let size = self.compute_size() as usize;
        let mut v = Vec::with_capacity(size);
        // skip zerofill
        unsafe {
            v.set_len(size);
        }
        {
            let mut os = CodedOutputStream::bytes(&mut v);
            self.write_to_with_cached_sizes(&mut os)?;
            os.check_eof();
        }
        Ok(v)
    }

    /// Write the message to the writer, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_writer(&self, w: &mut Write) -> ProtobufResult<()> {
        w.with_coded_output_stream(|os| self.write_length_delimited_to(os))
    }

    /// Write the message to the bytes vec, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_bytes(&self) -> ProtobufResult<Vec<u8>> {
        with_coded_output_stream_to_bytes(|os| self.write_length_delimited_to(os))
    }

    /// Get a reference to unknown fields.
    fn get_unknown_fields<'s>(&'s self) -> &'s UnknownFields;
    /// Get a mutable reference to unknown fields.
    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut UnknownFields;

    /// Get type id for downcasting.
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    /// View self as `Any`.
    fn as_any(&self) -> &Any;

    /// View self as mutable `Any`.
    fn as_any_mut(&mut self) -> &mut Any {
        panic!()
    }

    /// Convert boxed self to boxed `Any`.
    fn into_any(self: Box<Self>) -> Box<Any> {
        panic!()
    }

    // Rust does not allow implementation of trait for trait:
    // impl<M : Message> fmt::Debug for M {
    // ...
    // }

    /// Create an empty message object.
    fn new() -> Self where Self : Sized;

    /// Get message descriptor for message type.
    fn descriptor_static() -> &'static MessageDescriptor
        where Self : Sized
    {
        panic!(
            "descriptor_static is not implemented for message, \
             LITE_RUNTIME must be used"
        );
    }

    /// Return a pointer to default immutable message.
    fn default_instance() -> &'static Self
        where Self : Sized;
}

pub fn message_down_cast<'a, M : Message + 'a>(m: &'a Message) -> &'a M {
    m.as_any().downcast_ref::<M>().unwrap()
}


/// Parse message from stream.
pub fn parse_from<M : Message>(is: &mut CodedInputStream) -> ProtobufResult<M> {
    let mut r: M = Message::new();
    r.merge_from(is)?;
    r.check_initialized()?;
    Ok(r)
}

/// Parse message from reader.
/// Parse stops on EOF or when error encountered.
pub fn parse_from_reader<M : Message>(reader: &mut Read) -> ProtobufResult<M> {
    reader.with_coded_input_stream(|is| parse_from::<M>(is))
}

/// Parse message from byte array.
pub fn parse_from_bytes<M : Message>(bytes: &[u8]) -> ProtobufResult<M> {
    bytes.with_coded_input_stream(|is| parse_from::<M>(is))
}

/// Parse message from `Bytes` object.
/// Resulting message may share references to the passed bytes object.
#[cfg(feature = "bytes")]
pub fn parse_from_carllerche_bytes<M : Message>(
    bytes: &Bytes,
) -> ProtobufResult<M> {
    // Call trait explicitly to avoid accidental construction from `&[u8]`
    WithCodedInputStream::with_coded_input_stream(bytes, |is| parse_from::<M>(is))
}

/// Parse length-delimited message from stream.
///
/// Read varint length first, and read messages of that length then.
pub fn parse_length_delimited_from<M : Message>(
    is: &mut CodedInputStream,
) -> ProtobufResult<M> {
    is.read_message::<M>()
}

/// Parse length-delimited message from `Read`.
pub fn parse_length_delimited_from_reader<M : Message>(
    r: &mut Read,
) -> ProtobufResult<M> {
    // TODO: wrong: we may read length first, and then read exact number of bytes needed
    r.with_coded_input_stream(|is| is.read_message::<M>())
}

/// Parse length-delimited message from bytes.
// TODO: currently it's not possible to know how many bytes read from slice.
pub fn parse_length_delimited_from_bytes<M : Message>(
    bytes: &[u8],
) -> ProtobufResult<M> {
    bytes.with_coded_input_stream(|is| is.read_message::<M>())
}
