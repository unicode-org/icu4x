// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::MixedPatternItem;
use crate::fields;
use core::convert::TryFrom;
use zerovec::ule::{AsULE, ZeroVecError, ULE};

/// # Diagram
///
/// ```text
/// ┌───────────────┬───────────────┬───────────────┐
/// │       u8      │       u8      │       u8      │
/// ├─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┤
/// ├─┴─┴─┼─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┤
/// │     │          Unicode Code Point             │ Literal
/// ├─┬─┬─┴─────────┬───────────────┬───────────────┤
/// │0│1│           │  FieldSymbol  │  FieldLength  │ Field
/// ├─┼─┼───────────┴───────────────┼───────────────┤
/// │1│0│                           │  Placeholder  │ Placeholder
/// └─┴─┴───────────────────────────┴───────────────┘
///  ▲▲▲
///  │││
///  Variant Discriminant
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct MixedPatternItemULE([u8; 3]);

#[repr(u8)]
enum Tag {
    Literal,
    Field,
    Placeholder,
}

impl MixedPatternItemULE {
    #[inline]
    fn tag_from_byte(byte: u8) -> Option<Tag> {
        match (byte & 0b1100_0000) >> 6 {
            0 => Some(Tag::Literal),
            1 => Some(Tag::Field),
            2 => Some(Tag::Placeholder),
            _ => None,
        }
    }

    #[inline]
    fn bytes_in_range(value: (u8, u8, u8)) -> bool {
        match Self::tag_from_byte(value.0) {
            Some(Tag::Literal) => {
                char::try_from(u32::from_be_bytes([0x00, value.0, value.1, value.2])).is_ok()
            }
            Some(Tag::Field) => {
                fields::FieldULE::validate_bytes((value.1, value.2)).is_ok()
                    && value.0 == 0b0100_0000
            }
            Some(Tag::Placeholder) => value.0 == 0b1000_0000 && value.1 == 0 && value.2 < 10,
            None => false
        }
    }
}

unsafe impl ULE for MixedPatternItemULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        if bytes.len() % 3 != 0 {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }

        #[allow(clippy::indexing_slicing)] // chunks
        if !bytes
            .chunks(3)
            .all(|c| Self::bytes_in_range((c[0], c[1], c[2])))
        {
            return Err(ZeroVecError::parse::<Self>());
        }
        Ok(())
    }
}

impl AsULE for MixedPatternItem {
    type ULE = MixedPatternItemULE;

    fn to_unaligned(self) -> Self::ULE {
        match self {
            Self::Literal(ch) => {
                let u = ch as u32;
                let bytes = u.to_be_bytes();
                MixedPatternItemULE([bytes[1], bytes[2], bytes[3]])
            }
            Self::Field(field) => {
                MixedPatternItemULE([0b0100_0000, field.symbol.idx(), field.length.idx()])
            }
            Self::Placeholder(idx) => MixedPatternItemULE([0b1000_0000, 0x00, idx]),
        }
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let value = unaligned.0;
        #[allow(clippy::unwrap_used)] // validated
        match MixedPatternItemULE::tag_from_byte(value[0]).unwrap() {
            Tag::Literal => Self::Literal(
                char::try_from(u32::from_be_bytes([0x00, value[0], value[1], value[2]])).unwrap(),
            ),
            Tag::Field => {
                let symbol = fields::FieldSymbol::from_idx(value[1]).unwrap();
                let length = fields::FieldLength::from_idx(value[2]).unwrap();
                Self::Field(fields::Field { symbol, length })
            }
            Tag::Placeholder => Self::Placeholder(value[2]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fields::{FieldLength, FieldSymbol, Second, Year};
    use zerovec::ule::{AsULE, ULE};

    #[test]
    fn test_mixed_pattern_item_as_ule() {
        let samples = &[
            (MixedPatternItem::Placeholder(4), &[0x80, 0x00, 4]),
            (MixedPatternItem::from('z'), &[0x00, 0x00, 0x7a]),
            (MixedPatternItem::Placeholder(0), &[0x80, 0x00, 0]),
            (
                MixedPatternItem::from((FieldSymbol::Year(Year::Calendar), FieldLength::Wide)),
                &[
                    0x40,
                    FieldSymbol::Year(Year::Calendar).idx(),
                    FieldLength::Wide.idx(),
                ],
            ),
            (MixedPatternItem::from('秒'), &[0, 0x79, 0xd2]),
            (
                MixedPatternItem::from((
                    FieldSymbol::Second(Second::Millisecond),
                    FieldLength::One,
                )),
                &[
                    0x40,
                    FieldSymbol::Second(Second::Millisecond).idx(),
                    FieldLength::One.idx(),
                ],
            ),
        ];

        for (ref_pattern, ref_bytes) in samples {
            let ule = ref_pattern.to_unaligned();
            assert_eq!(ULE::as_byte_slice(&[ule]), *ref_bytes);
            let pattern = MixedPatternItem::from_unaligned(ule);
            assert_eq!(pattern, *ref_pattern);
        }
    }
}
