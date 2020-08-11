use crate::reflect::MessageDescriptor;
use crate::stream::WithCodedOutputStream;
use crate::CodedInputStream;
use crate::CodedOutputStream;
use crate::Message;
use crate::ProtobufError;
use crate::ProtobufResult;
use crate::UnknownFields;
use std::io::Write;

pub trait MessageDyn {
    fn descriptor(&self) -> MessageDescriptor;

    fn merge_from(&mut self, is: &mut CodedInputStream) -> ProtobufResult<()>;

    fn write_to_with_cached_sizes(&self, os: &mut CodedOutputStream) -> ProtobufResult<()>;

    fn compute_size(&self) -> u32;

    /// True iff all required fields are initialized.
    /// Always returns `true` for protobuf 3.
    fn is_initialized(&self) -> bool;

    /// Get a reference to unknown fields.
    fn get_unknown_fields(&self) -> &UnknownFields;
    /// Get a mutable reference to unknown fields.
    fn mut_unknown_fields(&mut self) -> &mut UnknownFields;

    /// Temporary for migration
    fn as_message_todo(&self) -> &dyn Message;
}

impl<M: Message> MessageDyn for M {
    fn descriptor(&self) -> MessageDescriptor {
        self.descriptor()
    }

    fn merge_from(&mut self, is: &mut CodedInputStream) -> ProtobufResult<()> {
        self.merge_from(is)
    }

    fn write_to_with_cached_sizes(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        self.write_to_with_cached_sizes(os)
    }

    fn compute_size(&self) -> u32 {
        self.compute_size()
    }

    fn is_initialized(&self) -> bool {
        self.is_initialized()
    }

    fn get_unknown_fields(&self) -> &UnknownFields {
        self.get_unknown_fields()
    }

    fn mut_unknown_fields(&mut self) -> &mut UnknownFields {
        self.mut_unknown_fields()
    }

    fn as_message_todo(&self) -> &dyn Message {
        self
    }
}

impl dyn MessageDyn {
    /// Check if all required fields of this object are initialized.
    fn check_initialized(&self) -> ProtobufResult<()> {
        if !self.is_initialized() {
            Err(ProtobufError::MessageNotInitialized(
                self.descriptor().name().to_owned(),
            ))
        } else {
            Ok(())
        }
    }

    /// Write the message to the writer.
    fn write_to_writer(&self, w: &mut dyn Write) -> ProtobufResult<()> {
        w.with_coded_output_stream(|os| self.write_to(os))
    }

    /// Write the message to bytes vec.
    fn write_to_vec(&self, v: &mut Vec<u8>) -> ProtobufResult<()> {
        v.with_coded_output_stream(|os| self.write_to(os))
    }

    /// Write the message to the stream.
    ///
    /// Results in error if message is not fully initialized.
    fn write_to(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        self.check_initialized()?;

        // cache sizes
        self.compute_size();
        // TODO: reserve additional
        self.write_to_with_cached_sizes(os)?;

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

    /// Write the message to bytes vec.
    ///
    /// > **Note**: You can use [`Message::parse_from_bytes`]
    /// to do the reverse.
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

    /// Write the message to the stream prepending the message with message length
    /// encoded as varint.
    pub fn write_length_delimited_to(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        let size = self.compute_size();
        os.write_raw_varint32(size)?;
        self.write_to_with_cached_sizes(os)?;

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    /// Write the message to the writer, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_writer(&self, w: &mut dyn Write) -> ProtobufResult<()> {
        w.with_coded_output_stream(|os| self.write_length_delimited_to(os))
    }

    /// Write the message to the bytes vec, prepend the message with message length
    /// encoded as varint.
    fn write_length_delimited_to_bytes(&self) -> ProtobufResult<Vec<u8>> {
        let mut v = Vec::new();
        v.with_coded_output_stream(|os| self.write_length_delimited_to(os))?;
        Ok(v)
    }
}
