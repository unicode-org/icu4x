// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::PatternItem;
use crate::fields;
use core::convert::TryFrom;
use zerovec::ule::{AsULE, ULE};

/// `PatternItemULE` is a type optimized for efficent storing and
/// deserialization of `DateTimeFormat` `PatternItem` elements using
/// `ZeroVec` model.
///
/// The serialization model packages the pattern item in three bytes.
///
/// The first bit is used to disriminate the item variant. If the bit is
/// set, then the value is the `PatternItem::Field` variant. Otherwise,
/// the `PatternItem::Literal` is used.
///
/// In case the discriminant is set:
///
/// 1) The rest of the first byte remains usnused.
/// 2) The second byte encodes `FieldSymbol` encoded as (Type: 4 bits, Symbol: 4 bits).
/// 3) The third byte encodes the field length.
///
/// If the discriminant is not set, the bottom three bits of the first byte,
/// together with the next two bytes, contain all 21 bits required to encode
/// any [`Unicode Code Point`].
///
/// # Diagram
///
/// <pre>
/// ┌───────────────┬───────────────┬───────────────┐
/// │       u8      │       u8      │       u8      │
/// ├─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┤
/// ├─┴─┴─┼─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┤
/// │     │          Unicode Code Point             │ Literal
/// ├─┬───┴─────────┬───────────────┬───────────────┤
/// │X│             │  FieldSymbol  │  FieldLength  │ Field
/// └─┴─────────────┴───────────────┴───────────────┘
///  ▲
///  │
///  Variant Discriminant
/// </pre>
///
/// # Optimization
///
/// This model is optimized for efficient packaging of the `PatternItem` elements
/// and performant deserialization from the `PatternItemULE` to `PatternItem` type.
///
/// # Constraints
///
/// The model leaves at most 8 `PatternItem` variants, limits the number of possible
/// field types and symbols to 16 each and limits the number of length variants to 256.
///
/// [`Unicode Code Point`]: http://www.unicode.org/versions/latest/
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PatternItemULE([u8; 3]);

impl PatternItemULE {
    #[inline]
    fn determine_field_from_u8(byte: u8) -> bool {
        byte & 0b1000_0000 != 0
    }

    #[inline]
    fn bytes_in_range(value: (&u8, &u8, &u8)) -> bool {
        match Self::determine_field_from_u8(*value.0) {
            true => fields::Field::bytes_in_range(value.1, value.2),
            false => {
                let u = u32::from_be_bytes([0x00, *value.0, *value.1, *value.2]);
                char::try_from(u).is_ok()
            }
        }
    }
}

// This impl is safe because (1) validate_byte_slice rejects all invalid byte slices, including
// those that are the wrong length, and (2) byte equality is semantic equality.
unsafe impl ULE for PatternItemULE {
    type Error = &'static str;

    fn validate_byte_slice(bytes: &[u8]) -> Result<(), Self::Error> {
        let mut chunks = bytes.chunks_exact(3);

        if !chunks.all(|c| Self::bytes_in_range((&c[0], &c[1], &c[2])))
            || !chunks.remainder().is_empty()
        {
            return Err("Invalid byte sequence");
        }
        Ok(())
    }
}

impl AsULE for PatternItem {
    type ULE = PatternItemULE;

    #[inline]
    fn as_unaligned(&self) -> Self::ULE {
        match self {
            Self::Field(field) => {
                PatternItemULE([0b1000_0000, field.symbol.idx(), field.length.idx()])
            }
            Self::Literal(ch) => {
                let u = *ch as u32;
                let bytes = u.to_be_bytes();
                PatternItemULE([bytes[1], bytes[2], bytes[3]])
            }
        }
    }

    #[inline]
    fn from_unaligned(unaligned: &Self::ULE) -> Self {
        let value = unaligned.0;
        match PatternItemULE::determine_field_from_u8(value[0]) {
            false => {
                let u = u32::from_be_bytes([0x00, value[0], value[1], value[2]]);
                PatternItem::Literal(char::try_from(u).unwrap())
            }
            true => {
                let symbol = fields::FieldSymbol::from_idx(value[1]).unwrap();
                let length = fields::FieldLength::from_idx(value[2]).unwrap();
                let field = fields::Field { symbol, length };
                PatternItem::Field(field)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{PatternItem, PatternItemULE};
    use crate::fields::{FieldLength, FieldSymbol, Second, Year};
    use zerovec::ule::{AsULE, ULE};

    #[test]
    fn test_pattern_item_as_ule() {
        let samples = &[
            (
                PatternItem::from((FieldSymbol::Minute, FieldLength::TwoDigit)),
                &[0x80, FieldSymbol::Minute.idx(), FieldLength::TwoDigit.idx()],
            ),
            (
                PatternItem::from((FieldSymbol::Year(Year::Calendar), FieldLength::Wide)),
                &[
                    0x80,
                    FieldSymbol::Year(Year::Calendar).idx(),
                    FieldLength::Wide.idx(),
                ],
            ),
            (
                PatternItem::from((FieldSymbol::Year(Year::WeekOf), FieldLength::Wide)),
                &[
                    0x80,
                    FieldSymbol::Year(Year::WeekOf).idx(),
                    FieldLength::Wide.idx(),
                ],
            ),
            (
                PatternItem::from((FieldSymbol::Second(Second::Millisecond), FieldLength::One)),
                &[
                    0x80,
                    FieldSymbol::Second(Second::Millisecond).idx(),
                    FieldLength::One.idx(),
                ],
            ),
            (PatternItem::from('z'), &[0x00, 0x00, 0x7a]),
        ];

        for (ref_pattern, ref_bytes) in samples {
            let ule = ref_pattern.as_unaligned();
            assert_eq!(ULE::as_byte_slice(&[ule]), *ref_bytes);
            let pattern = PatternItem::from_unaligned(&ule);
            assert_eq!(pattern, *ref_pattern);
        }
    }

    #[test]
    fn test_pattern_item_ule() {
        let samples = &[(
            &[
                PatternItem::from((FieldSymbol::Year(Year::Calendar), FieldLength::Wide)),
                PatternItem::from('z'),
                PatternItem::from((FieldSymbol::Second(Second::Millisecond), FieldLength::One)),
            ],
            &[
                &[
                    0x80,
                    FieldSymbol::Year(Year::Calendar).idx(),
                    FieldLength::Wide.idx(),
                ],
                &[0x00, 0x00, 0x7a],
                &[
                    0x80,
                    FieldSymbol::Second(Second::Millisecond).idx(),
                    FieldLength::One.idx(),
                ],
            ],
        )];

        for (ref_pattern, ref_bytes) in samples {
            let mut bytes: Vec<u8> = vec![];
            for item in ref_pattern.iter() {
                let ule = item.as_unaligned();
                bytes.extend(ULE::as_byte_slice(&[ule]));
            }

            let mut bytes2: Vec<u8> = vec![];
            for seq in ref_bytes.iter() {
                bytes2.extend_from_slice(*seq);
            }

            assert!(PatternItemULE::validate_byte_slice(&bytes).is_ok());
            assert_eq!(bytes, bytes2);
        }
    }
}
