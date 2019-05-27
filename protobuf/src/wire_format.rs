//! Constants used in serializations.

// TODO: temporary
pub use self::WireType::*;

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
            0 => Some(WireTypeVarint),
            1 => Some(WireTypeFixed64),
            2 => Some(WireTypeLengthDelimited),
            3 => Some(WireTypeStartGroup),
            4 => Some(WireTypeEndGroup),
            5 => Some(WireTypeFixed32),
            _ => None,
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
