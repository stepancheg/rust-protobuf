use std::fmt;
use std::io::Read;
use std::io::Write;

#[cfg(feature = "bytes")]
use bytes::Bytes;

use crate::clear::Clear;
use crate::coded_input_stream::CodedInputStream;
use crate::coded_output_stream::CodedOutputStream;
use crate::coded_output_stream::WithCodedOutputStream;
use crate::error::ProtobufError;
use crate::error::Result;
use crate::message_dyn::MessageDyn;
use crate::reflect::reflect_eq::ReflectEqMode;
use crate::reflect::MessageDescriptor;
use crate::unknown::UnknownFields;

/// Trait implemented for all the messages (generated and dynamic).
///
/// * Generated messages are generated from `.proto` files
/// * Dynamic messages can be created without code generation using only parsed proto files
///   (see [FileDescriptor::new_dynamic](crate::reflect::FileDescriptor::new_dynamic)).
///
///
/// Also, generated messages implement `Default + PartialEq`
///
/// This trait is sized, there's accompanying [`MessageDyn`](crate::MessageDyn) trait
/// which is implemented for all messages which can be used in functions
/// without making message a function type parameter.
///
/// ## `Display`
///
/// [`Display`](fmt::Display) implementation for messages does protobuf text format.
/// See [`text_format`](crate::text_format) for more details.
pub trait Message:
    fmt::Debug + fmt::Display + Clear + Clone + Send + Sync + Sized + 'static
{
    /// Message descriptor for this message, used for reflection.
    ///
    /// This function is rarely needed to be called directly, use
    /// [`Message::descriptor_static()`] instead.
    fn descriptor_by_instance(&self) -> MessageDescriptor {
        Self::descriptor_static()
    }

    /// Get message descriptor for message type.
    ///
    /// ```
    /// # use protobuf::Message;
    /// # fn foo<MyMessage: Message>() {
    /// let descriptor = MyMessage::descriptor_static();
    /// assert_eq!("MyMessage", descriptor.name());
    /// # }
    /// ```
    fn descriptor_static() -> MessageDescriptor {
        panic!(
            "descriptor_static is not implemented for message, \
             LITE_RUNTIME must be used"
        );
    }

    /// True iff all required fields are initialized.
    /// Always returns `true` for protobuf 3.
    fn is_initialized(&self) -> bool;

    /// Update this message object with fields read from given stream.
    fn merge_from(&mut self, is: &mut CodedInputStream) -> Result<()>;

    /// Parse message from stream.
    fn parse_from(is: &mut CodedInputStream) -> Result<Self> {
        let mut r: Self = Message::new();
        r.merge_from(is)?;
        r.check_initialized()?;
        Ok(r)
    }

    /// Write message to the stream.
    ///
    /// Sizes of this messages and nested messages must be cached
    /// by calling `compute_size` prior to this call.
    fn write_to_with_cached_sizes(&self, os: &mut CodedOutputStream) -> Result<()>;

    /// Compute and cache size of this message and all nested messages
    fn compute_size(&self) -> u32;

    /// Get size previously computed by `compute_size`.
    fn cached_size(&self) -> u32;

    /// Write the message to the stream.
    ///
    /// Results in error if message is not fully initialized.
    fn write_to(&self, os: &mut CodedOutputStream) -> Result<()> {
        self.check_initialized()?;

        // cache sizes
        self.compute_size();
        // TODO: reserve additional
        self.write_to_with_cached_sizes(os)?;

        Ok(())
    }

    /// Write the message to the stream prepending the message with message length
    /// encoded as varint.
    fn write_length_delimited_to(&self, os: &mut CodedOutputStream) -> Result<()> {
        let size = self.compute_size();
        os.write_raw_varint32(size)?;

        let written = os.total_bytes_written();

        self.write_to_with_cached_sizes(os)?;

        // Self-check.
        assert_eq!(written + size as u64, os.total_bytes_written());

        Ok(())
    }

    /// Write the message to the vec, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_vec(&self, vec: &mut Vec<u8>) -> Result<()> {
        let mut os = CodedOutputStream::vec(vec);
        self.write_length_delimited_to(&mut os)?;
        os.flush()?;
        Ok(())
    }

    /// Update this message object with fields read from given stream.
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let mut is = CodedInputStream::from_bytes(bytes);
        self.merge_from(&mut is)
    }

    /// Parse message from reader.
    /// Parse stops on EOF or when error encountered.
    fn parse_from_reader(reader: &mut dyn Read) -> Result<Self>
    where
        Self: Sized,
    {
        let mut is = CodedInputStream::new(reader);
        let r = Message::parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }

    /// Parse message from byte array.
    fn parse_from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let mut is = CodedInputStream::from_bytes(bytes);
        let r = Message::parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }

    /// Parse message from `Bytes` object.
    /// Resulting message may share references to the passed bytes object.
    #[cfg(feature = "bytes")]
    fn parse_from_tokio_bytes(bytes: &Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        let mut is = CodedInputStream::from_tokio_bytes(bytes);
        let r = Self::parse_from(&mut is)?;
        is.check_eof()?;
        Ok(r)
    }

    /// Check if all required fields of this object are initialized.
    fn check_initialized(&self) -> Result<()> {
        if !self.is_initialized() {
            Err(ProtobufError::MessageNotInitialized(
                self.descriptor_by_instance().name().to_owned(),
            )
            .into())
        } else {
            Ok(())
        }
    }

    /// Write the message to the writer.
    fn write_to_writer(&self, w: &mut dyn Write) -> Result<()> {
        w.with_coded_output_stream(|os| self.write_to(os))
    }

    /// Write the message to bytes vec.
    fn write_to_vec(&self, v: &mut Vec<u8>) -> Result<()> {
        v.with_coded_output_stream(|os| self.write_to(os))
    }

    /// Write the message to bytes vec.
    ///    
    /// > **Note**: You can use [`Message::parse_from_bytes`]
    /// to do the reverse.
    fn write_to_bytes(&self) -> Result<Vec<u8>> {
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
    fn write_length_delimited_to_writer(&self, w: &mut dyn Write) -> Result<()> {
        w.with_coded_output_stream(|os| self.write_length_delimited_to(os))
    }

    /// Write the message to the bytes vec, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_bytes(&self) -> Result<Vec<u8>> {
        let mut v = Vec::new();
        v.with_coded_output_stream(|os| self.write_length_delimited_to(os))?;
        Ok(v)
    }

    /// Get a reference to unknown fields.
    fn unknown_fields(&self) -> &UnknownFields;
    /// Get a mutable reference to unknown fields.
    fn mut_unknown_fields(&mut self) -> &mut UnknownFields;

    /// Create an empty message object.
    ///
    /// ```
    /// # use protobuf::Message;
    /// # fn foo<MyMessage: Message>() {
    /// let m = MyMessage::new();
    /// # }
    /// ```
    fn new() -> Self;

    /// Return a pointer to default immutable message with static lifetime.
    ///
    /// ```
    /// # use protobuf::Message;
    /// # fn foo<MyMessage: Message>() {
    /// let m: &MyMessage = MyMessage::default_instance();
    /// # }
    /// ```
    fn default_instance() -> &'static Self;

    /// Reflective equality.
    ///
    /// # See also
    ///
    /// [`dyn Message::reflect_eq_dyn()`], `dyn` version of this function.
    fn reflect_eq(&self, other: &Self, mode: &ReflectEqMode) -> bool {
        <dyn MessageDyn>::reflect_eq_dyn(self, other, mode)
    }
}
