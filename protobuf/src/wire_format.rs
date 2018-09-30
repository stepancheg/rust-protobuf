// TODO: temporary
pub use self::WireType::*;

pub const TAG_TYPE_BITS: u32 = 3;
pub const TAG_TYPE_MASK: u32 = (1u32 << TAG_TYPE_BITS as usize) - 1;
// max possible tag number
pub const FIELD_NUMBER_MAX: u32 = 0x1fffffff;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum WireType {
    WireTypeVarint = 0,
    WireTypeFixed64 = 1,
    WireTypeLengthDelimited = 2,
    WireTypeStartGroup = 3,
    WireTypeEndGroup = 4,
    WireTypeFixed32 = 5,
}

impl Copy for WireType {}

impl WireType {
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

#[derive(Clone)]
pub struct Tag {
    field_number: u32,
    wire_type: WireType,
}

impl Copy for Tag {}

impl Tag {
    pub fn value(self) -> u32 {
        (self.field_number << TAG_TYPE_BITS) | (self.wire_type as u32)
    }

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
            field_number: field_number,
            wire_type: wire_type.unwrap(),
        })
    }

    pub fn make(field_number: u32, wire_type: WireType) -> Tag {
        assert!(field_number > 0 && field_number <= FIELD_NUMBER_MAX);
        Tag {
            field_number: field_number,
            wire_type: wire_type,
        }
    }

    pub fn unpack(self) -> (u32, WireType) {
        (self.field_number(), self.wire_type())
    }

    fn wire_type(self) -> WireType {
        self.wire_type
    }

    pub fn field_number(self) -> u32 {
        self.field_number
    }
}
