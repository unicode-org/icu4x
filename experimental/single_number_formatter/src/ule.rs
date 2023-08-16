// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::{
    maps::ZeroMapKV,
    ule::{AsULE, ZeroVecError, ULE},
};

use crate::provider::{CurrencyPatterns, PatternSelection};

/// `CurrencyPatternsULE` is a type optimized for efficient storing and
/// deserialization of `CurrencyPatterns` using the `ZeroVec` model.
///
/// The serialization model packages the pattern item in three bytes.
///
/// The first two bits is used to determine the short_pattern_standard. If the bits are `00`, then, the value will be `Standard`.
/// If the bits are `01`, then, the value will be `StandardAlphaNextToNumber`.
///
/// The second two bits is used to determine the narrow_pattern_standard. If the bits are `00`, then, the value will be `Standard`.
/// If the bits are `01`, then, the value will be `StandardAlphaNextToNumber`.
///
/// The third two bits with the second byte is used to determine the short_place_holder_index.
/// The fourth two bits with the third byte is used to determine the narrow_place_holder_index.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct CurrencyPatternsULE([u8; 3]);

impl CurrencyPatternsULE {}

// Safety (based on the safety checklist on the ULE trait):
//  1. CurrencyPatternsULE does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(transparent)]` on a ULE type)
//  2. CurrencyPatternsULE is aligned to 1 byte.
//     (achieved by `#[repr(transparent)]` on a ULE type)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
//  5. The other ULE methods use the default impl.
//  6. CurrencyPatternsULE byte equality is semantic equality.
unsafe impl ULE for CurrencyPatternsULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        if bytes.len() % 3 != 0 {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }

        Ok(())
    }
}

const PATTERN_SHORT_POSITION: u8 = 6;
const PATTERN_NARROW_POSITION: u8 = 4;
const INDEX_SHORT_POSITION: u8 = 2;
const INDEX_NARROW_POSITION: u8 = 0;

impl AsULE for CurrencyPatterns {
    type ULE = CurrencyPatternsULE;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        let mut first_byte_ule: u8 = 0;

        if self.short_pattern_standard == PatternSelection::StandardAlphaNextToNumber {
            first_byte_ule |= 0b01 << PATTERN_SHORT_POSITION;
        }
        if self.narrow_pattern_standard == PatternSelection::StandardAlphaNextToNumber {
            first_byte_ule |= 0b01 << PATTERN_NARROW_POSITION;
        }

        // For short_place_holder_index
        let [first_index_byte, second_byte_ule] = self.short_place_holder_index.to_be_bytes();
        if first_index_byte < 0b0100 {
            first_byte_ule |= first_index_byte << INDEX_SHORT_POSITION;
        } else {
            panic!(
                "short_place_holder_index is too large {}, {}",
                self.short_place_holder_index, first_index_byte
            )
        }

        // For narrow_place_holder_index
        let [first_index_byte, third_byte_ule] = self.narrow_place_holder_index.to_be_bytes();
        if first_index_byte < 0b0100 {
            first_byte_ule |= first_index_byte << INDEX_NARROW_POSITION;
        } else {
            panic!(
                "narrow_place_holder_index is too large {}, {}",
                self.narrow_place_holder_index, first_index_byte
            )
        }
        CurrencyPatternsULE([first_byte_ule, second_byte_ule, third_byte_ule])
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let [first_byte, second_byte, third_byte] = unaligned.0;

        let short_pattern_standard =
            if (first_byte & 0b11 << PATTERN_SHORT_POSITION) >> PATTERN_SHORT_POSITION == 1 {
                PatternSelection::StandardAlphaNextToNumber
            } else {
                PatternSelection::Standard
            };

        let narrow_pattern_standard =
            if (first_byte & 0b11 << PATTERN_NARROW_POSITION) >> PATTERN_NARROW_POSITION == 1 {
                PatternSelection::StandardAlphaNextToNumber
            } else {
                PatternSelection::Standard
            };
        let short_prefix = (first_byte & 0b11 << INDEX_SHORT_POSITION) >> INDEX_SHORT_POSITION;
        let narrow_prefix = (first_byte & 0b11 << INDEX_NARROW_POSITION) >> INDEX_NARROW_POSITION;

        let short_place_holder_index = ((short_prefix as u16) << 8) | second_byte as u16;
        let narrow_place_holder_index = ((narrow_prefix as u16) << 8) | third_byte as u16;

        CurrencyPatterns {
            short_pattern_standard,
            narrow_pattern_standard,
            short_place_holder_index,
            narrow_place_holder_index,
        }
    }
}

impl<'a> ZeroMapKV<'a> for CurrencyPatterns {
    type Container = zerovec::ZeroVec<'a, CurrencyPatterns>;
    type Slice = zerovec::ZeroSlice<CurrencyPatterns>;
    type GetType = <CurrencyPatterns as AsULE>::ULE;
    type OwnedType = CurrencyPatterns;
}
