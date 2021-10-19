pub const STATE64_REMAINING_SHIFT: u32 = 59;
pub const STATE64_POS_MASK: u64 = (1 << STATE64_REMAINING_SHIFT) - 1;

// 10..1f: Linear-match node, match 1..16 bytes and continue reading the next node.
pub const MIN_LINEAR_MATCH: u8 = 0x10;
pub const MAX_LINEAR_MATCH_LENGTH: usize = 0x10;

// 20..ff: Variable-length value node.
// If odd, the value is final. (Otherwise, intermediate value or jump delta.)
// Then shift-right by 1 bit.
// The remaining lead byte value indicates the number of following bytes (0..4)
// and contains the value's top bits.
pub const MIN_VALUE_LEAD: u8 = MIN_LINEAR_MATCH + MAX_LINEAR_MATCH_LENGTH as u8; // 0x20

// It is a final value if bit 0 is set.
pub const VALUE_IS_FINAL: i32 = 1;

// Compact value: After testing bit 0, shift right by 1 and then use the following thresholds.
pub const MIN_ONE_BYTE_VALUE_LEAD: u8 = MIN_VALUE_LEAD / 2; // 0x10
pub const MAX_ONE_BYTE_VALUE: u8 = 0x40;

pub const FIVE_BYTE_VALUE_LEAD: u8 = 0x7f;

pub const MIN_TWO_BYTE_VALUE_LEAD: u8 = MIN_ONE_BYTE_VALUE_LEAD + MAX_ONE_BYTE_VALUE + 1; // 0x51
pub const MAX_TWO_BYTE_VALUE: i32 = 0x1aff;

pub const MIN_THREE_BYTE_VALUE_LEAD: i32 = MIN_TWO_BYTE_VALUE_LEAD as i32 + (MAX_TWO_BYTE_VALUE >> 8)  + 1; // 0x6c
pub const FOUR_BYTE_VALUE_LEAD: i32 = 0x7e;

// A little more than Unicode code points. (0x11ffff)
pub const MAX_THREE_BYTE_VALUE: i32 = ((FOUR_BYTE_VALUE_LEAD - MIN_THREE_BYTE_VALUE_LEAD) << 16) - 1;
