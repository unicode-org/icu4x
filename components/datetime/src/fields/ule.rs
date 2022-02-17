// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields;
use zerovec::ule::{AsULE, ZeroVecError, ULE};

/// `FieldULE` is a type optimized for efficent storing and
/// deserialization of `DateTimeFormat` `Field` elements using
/// `ZeroVec` model.
///
/// The serialization model packages the field in two bytes.
///
/// 2) The first byte encodes `FieldSymbol` encoded as (Type: 4 bits, Symbol: 4 bits).
/// 3) The second byte encodes the field length.
///
/// # Diagram
///
/// ```text
/// ┌───────────────┬───────────────┐
/// │       u8      │       u8      │
/// ├─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┤
/// ├─┴─┴─┴─┴─┴─┴─┴─┼─┴─┴─┴─┴─┴─┴─┴─┤
/// ├───────────────┼───────────────┤
/// │  FieldSymbol  │  FieldLength  │
/// └───────────────┴───────────────┘
/// ```
///
/// # Optimization
///
/// This model is optimized for efficient packaging of the `Field` elements
/// and performant deserialization from the `FieldULE` to `Field` type.
/// It remains aligned with `PatternItem` and `PatternItemULE` and is used
/// directly in `runtime::Skeleton` and indirectly in `runtime::Pattern`.
///
/// # Constraints
///
/// The model leaves limits the number of possible
/// field types and symbols to 16 each and limits the number of length variants to 256.
///
/// [`Unicode Code Point`]: http://www.unicode.org/versions/latest/
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct FieldULE([u8; 2]);

impl AsULE for fields::Field {
    type ULE = FieldULE;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        FieldULE([self.symbol.idx(), self.length.idx()])
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let value = unaligned.0;
        let symbol = fields::FieldSymbol::from_idx(value[0]).unwrap();
        let length = fields::FieldLength::from_idx(value[1]).unwrap();
        fields::Field { symbol, length }
    }
}

// Safety (based on the safety checklist on the ULE trait):
//  1. FieldULE does not include any uninitialized or padding bytes
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
/// 2. FieldULE is aligned to 1 byte
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
//  5 The other ULE methods use the default impl.
//  6. FieldULE byte equality is semantic equality.
unsafe impl ULE for FieldULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        let mut chunks = bytes.chunks_exact(2);

        if !chunks.all(|c| fields::Field::bytes_in_range(&c[0], &c[1])) {
            return Err(ZeroVecError::parse::<Self>());
        }

        if !chunks.remainder().is_empty() {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fields::{Field, FieldLength, FieldSymbol, Second, Year};
    use zerovec::ule::{AsULE, ULE};

    #[test]
    fn test_field_as_ule() {
        let samples = &[
            (
                Field::from((FieldSymbol::Minute, FieldLength::TwoDigit)),
                &[FieldSymbol::Minute.idx(), FieldLength::TwoDigit.idx()],
            ),
            (
                Field::from((FieldSymbol::Year(Year::Calendar), FieldLength::Wide)),
                &[
                    FieldSymbol::Year(Year::Calendar).idx(),
                    FieldLength::Wide.idx(),
                ],
            ),
            (
                Field::from((FieldSymbol::Year(Year::WeekOf), FieldLength::Wide)),
                &[
                    FieldSymbol::Year(Year::WeekOf).idx(),
                    FieldLength::Wide.idx(),
                ],
            ),
            (
                Field::from((FieldSymbol::Second(Second::Millisecond), FieldLength::One)),
                &[
                    FieldSymbol::Second(Second::Millisecond).idx(),
                    FieldLength::One.idx(),
                ],
            ),
        ];

        for (ref_field, ref_bytes) in samples {
            let ule = ref_field.to_unaligned();
            assert_eq!(ULE::as_byte_slice(&[ule]), *ref_bytes);
            let field = Field::from_unaligned(ule);
            assert_eq!(field, *ref_field);
        }
    }

    #[test]
    fn test_field_ule() {
        let samples = &[(
            &[
                Field::from((FieldSymbol::Year(Year::Calendar), FieldLength::Wide)),
                Field::from((FieldSymbol::Second(Second::Millisecond), FieldLength::One)),
            ],
            &[
                &[
                    FieldSymbol::Year(Year::Calendar).idx(),
                    FieldLength::Wide.idx(),
                ],
                &[
                    FieldSymbol::Second(Second::Millisecond).idx(),
                    FieldLength::One.idx(),
                ],
            ],
        )];

        for (ref_field, ref_bytes) in samples {
            let mut bytes: Vec<u8> = vec![];
            for item in ref_field.iter() {
                let ule = item.to_unaligned();
                bytes.extend(ULE::as_byte_slice(&[ule]));
            }

            let mut bytes2: Vec<u8> = vec![];
            for seq in ref_bytes.iter() {
                bytes2.extend_from_slice(*seq);
            }

            assert!(FieldULE::validate_byte_slice(&bytes).is_ok());
            assert_eq!(bytes, bytes2);
        }
    }
}
