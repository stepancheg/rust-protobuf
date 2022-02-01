//! Constants used in serializations.

use crate::descriptor::field_descriptor_proto;

/// Tag occupies three bits.
pub const TAG_TYPE_BITS: u32 = 3;
/// Apply this mask to varint value to obtain a tag.
pub const TAG_TYPE_MASK: u32 = (1u32 << TAG_TYPE_BITS as usize) - 1;
/// Max possible field number
pub const FIELD_NUMBER_MAX: u32 = 0x1fffffff;

/// All supported "wire types" are listed in this enum.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WireType {
    /// Variable-length integer
    WireTypeVarint = 0,
    /// 32-bit field (e. g. `fixed64` or `double`)
    WireTypeFixed64 = 1,
    /// Length-delimited field
    WireTypeLengthDelimited = 2,
    /// Groups are not supported in rust-protobuf
    WireTypeStartGroup = 3,
    /// Groups are not supported in rust-protobuf
    WireTypeEndGroup = 4,
    /// 32-bit field (e. g. `fixed32` or `float`)
    WireTypeFixed32 = 5,
}

impl WireType {
    /// Construct `WireType` from number, or return `None` if type is unknown.
    pub fn new(n: u32) -> Option<WireType> {
        match n {
            0 => Some(WireType::WireTypeVarint),
            1 => Some(WireType::WireTypeFixed64),
            2 => Some(WireType::WireTypeLengthDelimited),
            3 => Some(WireType::WireTypeStartGroup),
            4 => Some(WireType::WireTypeEndGroup),
            5 => Some(WireType::WireTypeFixed32),
            _ => None,
        }
    }

    #[doc(hidden)]
    pub fn for_type(field_type: field_descriptor_proto::Type) -> WireType {
        use field_descriptor_proto::Type;
        match field_type {
            Type::TYPE_INT32 => WireType::WireTypeVarint,
            Type::TYPE_INT64 => WireType::WireTypeVarint,
            Type::TYPE_UINT32 => WireType::WireTypeVarint,
            Type::TYPE_UINT64 => WireType::WireTypeVarint,
            Type::TYPE_SINT32 => WireType::WireTypeVarint,
            Type::TYPE_SINT64 => WireType::WireTypeVarint,
            Type::TYPE_BOOL => WireType::WireTypeVarint,
            Type::TYPE_ENUM => WireType::WireTypeVarint,
            Type::TYPE_FIXED32 => WireType::WireTypeFixed32,
            Type::TYPE_FIXED64 => WireType::WireTypeFixed64,
            Type::TYPE_SFIXED32 => WireType::WireTypeFixed32,
            Type::TYPE_SFIXED64 => WireType::WireTypeFixed64,
            Type::TYPE_FLOAT => WireType::WireTypeFixed32,
            Type::TYPE_DOUBLE => WireType::WireTypeFixed64,
            Type::TYPE_STRING => WireType::WireTypeLengthDelimited,
            Type::TYPE_BYTES => WireType::WireTypeLengthDelimited,
            Type::TYPE_MESSAGE => WireType::WireTypeLengthDelimited,
            Type::TYPE_GROUP => WireType::WireTypeLengthDelimited, // not true
        }
    }
}

/// Parsed field tag (a pair of field number and wire type)
#[derive(Clone, Copy)]
pub struct Tag {
    field_number: u32,
    wire_type: WireType,
}

impl Tag {
    /// Fold tag to a number to be serialized.
    pub fn value(self) -> u32 {
        (self.field_number << TAG_TYPE_BITS) | (self.wire_type as u32)
    }

    /// Extract wire type and field number from integer tag
    // TODO: should return Result instead of Option
    pub fn new(value: u32) -> Option<Tag> {
        let wire_type = WireType::new(value & TAG_TYPE_MASK);
        if wire_type.is_none() {
            return None;
        }
        let field_number = value >> TAG_TYPE_BITS;
        if field_number == 0 {
            return None;
        }
        Some(Tag {
            field_number,
            wire_type: wire_type.unwrap(),
        })
    }

    /// Construct a tag from a field number and wire type.
    ///
    /// # Panics
    ///
    /// If field number is outside of valid range.
    pub fn make(field_number: u32, wire_type: WireType) -> Tag {
        assert!(field_number > 0 && field_number <= FIELD_NUMBER_MAX);
        Tag {
            field_number,
            wire_type,
        }
    }

    /// Get field number and wire type
    pub fn unpack(self) -> (u32, WireType) {
        (self.field_number(), self.wire_type())
    }

    /// Get wire type
    fn wire_type(self) -> WireType {
        self.wire_type
    }

    /// Get field number
    pub fn field_number(self) -> u32 {
        self.field_number
    }
}
