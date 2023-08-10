// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ule::{AsULE, ZeroVecError, ULE};

use crate::provider::{CurrencyPatterns, PatternSelection};

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

        // #[allow(clippy::indexing_slicing)] // chunks
        // if !bytes
        //     .chunks(3)
        //     .all(|c| Self::bytes_in_range((&c[0], &c[1], &c[2])))
        // {
        //     return Err(ZeroVecError::parse::<Self>());
        // }
        Ok(())
    }
}

impl AsULE for CurrencyPatterns {
    type ULE = CurrencyPatternsULE;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        let mut first_byte: u8 = 0;

        if self.short_pattern_standard == PatternSelection::StandardAlphaNextToNumber {
            first_byte |= 0b0100_0000;
        }
        if self.narrow_pattern_standard == PatternSelection::StandardAlphaNextToNumber {
            first_byte |= 0b0001_0000;
        }

        // For short_place_holder_index
        let first_index_byte = self.short_place_holder_index.to_be_bytes()[0];
        let second_byte: u8 = self.short_place_holder_index.to_be_bytes()[1];
        if first_index_byte < 2_u8.pow(2) {
            first_byte |= first_index_byte << 2;
        } else {
            panic!(
                "short_place_holder_index is too large {}, {}",
                self.short_place_holder_index, first_index_byte
            )
        }

        // For narrow_place_holder_index
        let first_index_byte = self.narrow_place_holder_index.to_be_bytes()[0];
        let third_byte: u8 = self.narrow_place_holder_index.to_be_bytes()[1];
        if first_index_byte < 2_u8.pow(2) {
            first_byte |= first_index_byte;
        } else {
            panic!(
                "narrow_place_holder_index is too large {}, {}",
                self.narrow_place_holder_index, first_index_byte
            )
        }
        CurrencyPatternsULE([first_byte, second_byte, third_byte])
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let value = unaligned.0;
        let first_byte = value[0];
        let second_byte = value[1];
        let third_byte = value[2];

        let short_pattern_standard = if first_byte & 0b0100_0000 != 0 {
            PatternSelection::StandardAlphaNextToNumber
        } else {
            PatternSelection::Standard
        };

        let narrow_pattern_standard = if first_byte & 0b0001_0000 != 0 {
            PatternSelection::StandardAlphaNextToNumber
        } else {
            PatternSelection::Standard
        };
        let short_prefix = (first_byte & 0b0000_1100) >> 2;
        let narrow_prefix = first_byte & 0b0000_0011;

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
