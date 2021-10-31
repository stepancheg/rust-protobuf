use std::any::Any;
use std::any::TypeId;
use std::fmt;
use std::io::Write;

use crate::coded_output_stream::WithCodedOutputStream;
use crate::reflect::MessageDescriptor;
use crate::reflect::ReflectEqMode;
use crate::CodedInputStream;
use crate::CodedOutputStream;
use crate::Message;
use crate::ProtobufError;
use crate::ProtobufResult;
use crate::UnknownFields;

/// Dynamic-dispatch version of [`Message`].
pub trait MessageDyn: Any + fmt::Debug + Send + Sync + 'static {
    /// Message descriptor for this message, used for reflection.
    fn descriptor_dyn(&self) -> MessageDescriptor;

    /// Update this message fields with contents of given stream.
    fn merge_from_dyn(&mut self, is: &mut CodedInputStream) -> ProtobufResult<()>;

    /// Write the message.
    fn write_to_with_cached_sizes_dyn(&self, os: &mut CodedOutputStream) -> ProtobufResult<()>;

    /// Compute (and cache) the message size.
    fn compute_size_dyn(&self) -> u32;

    /// True iff all required fields are initialized.
    /// Always returns `true` for protobuf 3.
    fn is_initialized_dyn(&self) -> bool;

    /// Get a reference to unknown fields.
    fn get_unknown_fields_dyn(&self) -> &UnknownFields;
    /// Get a mutable reference to unknown fields.
    fn mut_unknown_fields_dyn(&mut self) -> &mut UnknownFields;
}

impl<M: Message> MessageDyn for M {
    fn descriptor_dyn(&self) -> MessageDescriptor {
        self.descriptor_by_instance()
    }

    fn merge_from_dyn(&mut self, is: &mut CodedInputStream) -> ProtobufResult<()> {
        self.merge_from(is)
    }

    fn write_to_with_cached_sizes_dyn(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        self.write_to_with_cached_sizes(os)
    }

    fn compute_size_dyn(&self) -> u32 {
        self.compute_size()
    }

    fn is_initialized_dyn(&self) -> bool {
        self.is_initialized()
    }

    fn get_unknown_fields_dyn(&self) -> &UnknownFields {
        self.get_unknown_fields()
    }

    fn mut_unknown_fields_dyn(&mut self) -> &mut UnknownFields {
        self.mut_unknown_fields()
    }
}

impl dyn MessageDyn {
    /// Check if all required fields of this object are initialized.
    pub fn check_initialized_dyn(&self) -> ProtobufResult<()> {
        if !self.is_initialized_dyn() {
            Err(ProtobufError::MessageNotInitialized(
                self.descriptor_dyn().name().to_owned(),
            ))
        } else {
            Ok(())
        }
    }

    /// Write the message to the writer.
    pub fn write_to_writer_dyn(&self, w: &mut dyn Write) -> ProtobufResult<()> {
        w.with_coded_output_stream(|os| self.write_to_dyn(os))
    }

    /// Write the message to bytes vec.
    pub fn write_to_vec_dyn(&self, v: &mut Vec<u8>) -> ProtobufResult<()> {
        v.with_coded_output_stream(|os| self.write_to_dyn(os))
    }

    /// Write the message to the stream.
    ///
    /// Results in error if message is not fully initialized.
    pub fn write_to_dyn(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        self.check_initialized_dyn()?;

        // cache sizes
        self.compute_size_dyn();
        // TODO: reserve additional
        self.write_to_with_cached_sizes_dyn(os)?;

        Ok(())
    }

    /// Write the message to the vec, prepend the message with message length
    /// encoded as varint.
    pub fn write_length_delimited_to_vec_dyn(&self, vec: &mut Vec<u8>) -> ProtobufResult<()> {
        let mut os = CodedOutputStream::vec(vec);
        self.write_length_delimited_to_dyn(&mut os)?;
        os.flush()?;
        Ok(())
    }

    /// Update this message object with fields read from given stream.
    pub fn merge_from_bytes_dyn(&mut self, bytes: &[u8]) -> ProtobufResult<()> {
        let mut is = CodedInputStream::from_bytes(bytes);
        self.merge_from_dyn(&mut is)
    }

    /// Write the message to bytes vec.
    ///
    /// > **Note**: You can use [`Message::parse_from_bytes`]
    /// to do the reverse.
    pub fn write_to_bytes_dyn(&self) -> ProtobufResult<Vec<u8>> {
        self.check_initialized_dyn()?;

        let size = self.compute_size_dyn() as usize;
        let mut v = Vec::with_capacity(size);
        // skip zerofill
        unsafe {
            v.set_len(size);
        }
        {
            let mut os = CodedOutputStream::bytes(&mut v);
            self.write_to_with_cached_sizes_dyn(&mut os)?;
            os.check_eof();
        }
        Ok(v)
    }

    /// Write the message to the stream prepending the message with message length
    /// encoded as varint.
    pub fn write_length_delimited_to_dyn(&self, os: &mut CodedOutputStream) -> ProtobufResult<()> {
        let size = self.compute_size_dyn();
        os.write_raw_varint32(size)?;
        self.write_to_with_cached_sizes_dyn(os)?;

        // TODO: assert we've written same number of bytes as computed

        Ok(())
    }

    /// Write the message to the writer, prepend the message with message length
    /// encoded as varint.
    pub fn write_length_delimited_to_writer_dyn(&self, w: &mut dyn Write) -> ProtobufResult<()> {
        w.with_coded_output_stream(|os| self.write_length_delimited_to_dyn(os))
    }

    /// Write the message to the bytes vec, prepend the message with message length
    /// encoded as varint.
    pub fn write_length_delimited_to_bytes_dyn(&self) -> ProtobufResult<Vec<u8>> {
        let mut v = Vec::new();
        v.with_coded_output_stream(|os| self.write_length_delimited_to_dyn(os))?;
        Ok(v)
    }

    /// Downcast `Box<dyn Message>` to specific message type.
    ///
    /// ```
    /// # use protobuf::{Message, MessageDyn};
    /// # fn foo<MyMessage: Message>(message: Box<dyn MessageDyn>) {
    /// let m: Box<dyn MessageDyn> = message;
    /// let m: Box<MyMessage> = MessageDyn::downcast_box(m).unwrap();
    /// # }
    /// ```
    pub fn downcast_box<T: Any>(self: Box<dyn MessageDyn>) -> Result<Box<T>, Box<dyn MessageDyn>> {
        if Any::type_id(&*self) == TypeId::of::<T>() {
            unsafe {
                let raw: *mut dyn MessageDyn = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }

    /// Downcast `&dyn Message` to specific message type.
    ///
    /// ```
    /// # use protobuf::{Message, MessageDyn};
    /// # fn foo<MyMessage: Message>(message: &dyn MessageDyn) {
    /// let m: &dyn MessageDyn = message;
    /// let m: &MyMessage = MessageDyn::downcast_ref(m).unwrap();
    /// # }
    /// ```
    pub fn downcast_ref<'a, M: Message + 'a>(&'a self) -> Option<&'a M> {
        if Any::type_id(&*self) == TypeId::of::<M>() {
            unsafe { Some(&*(self as *const dyn MessageDyn as *const M)) }
        } else {
            None
        }
    }

    /// Downcast `&mut dyn Message` to specific message type.
    ///
    /// ```
    /// # use protobuf::{Message, MessageDyn};
    /// # fn foo<MyMessage: Message>(message: &mut dyn MessageDyn) {
    /// let m: &mut dyn MessageDyn = message;
    /// let m: &mut MyMessage = MessageDyn::downcast_mut(m).unwrap();
    /// # }
    /// ```
    pub fn downcast_mut<'a, M: Message + 'a>(&'a mut self) -> Option<&'a mut M> {
        if Any::type_id(&*self) == TypeId::of::<M>() {
            unsafe { Some(&mut *(self as *mut dyn MessageDyn as *mut M)) }
        } else {
            None
        }
    }

    /// Clone from a `dyn Message` reference.
    pub fn clone_box(&self) -> Box<dyn MessageDyn> {
        self.descriptor_dyn().clone_message(self)
    }

    /// Reflectively compare the messages.
    pub fn reflect_eq_dyn(&self, other: &dyn MessageDyn, mode: &ReflectEqMode) -> bool {
        MessageDescriptor::reflect_eq_maybe_unrelated(self, other, mode)
    }
}

impl Clone for Box<dyn MessageDyn> {
    fn clone(&self) -> Self {
        (*self).clone_box()
    }
}

impl PartialEq for Box<dyn MessageDyn> {
    fn eq(&self, other: &Box<dyn MessageDyn>) -> bool {
        MessageDescriptor::reflect_eq_maybe_unrelated(&**self, &**other, &ReflectEqMode::default())
    }
}

#[cfg(test)]
mod test {
    use crate::descriptor::FileDescriptorProto;
    use crate::MessageDyn;

    #[test]
    fn downcast_ref() {
        let m = FileDescriptorProto::new();
        let d = &m as &dyn MessageDyn;
        let c: &FileDescriptorProto = d.downcast_ref().unwrap();
        assert_eq!(
            c as *const FileDescriptorProto,
            &m as *const FileDescriptorProto
        );
    }

    #[test]
    fn downcast_mut() {
        let mut m = FileDescriptorProto::new();
        let d = &mut m as &mut dyn MessageDyn;
        let c: &mut FileDescriptorProto = d.downcast_mut().unwrap();
        assert_eq!(
            c as *const FileDescriptorProto,
            &m as *const FileDescriptorProto
        );
    }

    #[test]
    fn downcast_box() {
        let m = FileDescriptorProto::new();
        let d: Box<dyn MessageDyn> = Box::new(m);
        let mut _c: Box<FileDescriptorProto> = d.downcast_box().unwrap();
    }
}
