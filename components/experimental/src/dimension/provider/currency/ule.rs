// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::{
    maps::ZeroMapKV,
    ule::{AsULE, ULE, UleError},
};

use crate::dimension::provider::currency::symbols::{CurrencyPatternConfig, CurrencySymbolIndex};

const NO_SYMBOL: u16 = 0b0111_1111_1111; // decimal: 2047
const USE_ISO_CODE: u16 = 0b0111_1111_1110; // decimal: 2046

// TODO(#4013): Remove this constant once we have an invariant that the injecting text index is always less than 2046.
pub const MAX_SYMBOL_INDEX: u16 = 0b0111_1111_1101; // decimal: 2045

/// [`CurrencyPatternConfigULE`] is a type optimized for efficient storing and
/// deserialization of [`CurrencyPatternConfig`] using the `ZeroVec` model.
///
/// The serialization model packages the pattern item in three bytes.
///
/// The first bit (b7) is unused.
///
/// The second bit (b6) is unused.
///
/// The next three bits (b5, b4 & b3) with the second byte is used to determine the `short_symbol`.
/// The next three bits (b2, b1 & b0) with the third byte is used to determine the `narrow_symbol`.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct CurrencyPatternConfigULE([u8; 3]);

// Safety (based on the safety checklist on the ULE trait):
//  1. CurrencyPatternConfigULE does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(transparent)]` on a ULE type)
//  2. CurrencyPatternConfigULE is aligned to 1 byte.
//     (achieved by `#[repr(transparent)]` on a ULE type)
//  3. The impl of validate_bytes() returns an error if any byte is not valid.
//  4. The impl of validate_bytes() returns an error if there are extra bytes.
//  5. The other ULE methods use the default impl.
//  6. CurrencyPatternConfigULE byte equality is semantic equality.
unsafe impl ULE for CurrencyPatternConfigULE {
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        if !bytes.len().is_multiple_of(3) {
            return Err(UleError::length::<Self>(bytes.len()));
        }

        Ok(())
    }
}

const INDEX_SHORT_SHIFT: u8 = 3;
const INDEX_NARROW_SHIFT: u8 = 0;

impl AsULE for CurrencyPatternConfig {
    type ULE = CurrencyPatternConfigULE;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        let mut first_byte_ule: u8 = 0;

        // For short_symbol
        let [
            short_most_significant_byte,
            short_least_significant_byte_ule,
        ] = match self.short_symbol {
            Some(CurrencySymbolIndex::Index(index)) => index.to_be_bytes(),
            Some(CurrencySymbolIndex::ISO) => USE_ISO_CODE.to_be_bytes(),
            None => NO_SYMBOL.to_be_bytes(),
        };
        if short_most_significant_byte & 0b1111_1000 != 0 {
            panic!(
                "short_symbol is too large {short_most_significant_byte}, {short_least_significant_byte_ule}"
            )
        }
        first_byte_ule |= short_most_significant_byte << INDEX_SHORT_SHIFT;

        // For narrow_symbol
        let [
            narrow_most_significant_byte,
            narrow_least_significant_byte_ule,
        ] = match self.narrow_symbol {
            Some(CurrencySymbolIndex::Index(index)) => index.to_be_bytes(),
            Some(CurrencySymbolIndex::ISO) => USE_ISO_CODE.to_be_bytes(),
            None => NO_SYMBOL.to_be_bytes(),
        };
        if narrow_most_significant_byte & 0b1111_1000 != 0 {
            panic!(
                "narrow_symbol is too large {narrow_most_significant_byte}, {narrow_least_significant_byte_ule}"
            )
        }
        first_byte_ule |= narrow_most_significant_byte << INDEX_NARROW_SHIFT;

        CurrencyPatternConfigULE([
            first_byte_ule,
            short_least_significant_byte_ule,
            narrow_least_significant_byte_ule,
        ])
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let [first_byte, second_byte, third_byte] = unaligned.0;

        let short_prefix = (first_byte & (0b111 << INDEX_SHORT_SHIFT)) >> INDEX_SHORT_SHIFT;
        let narrow_prefix = (first_byte & (0b111 << INDEX_NARROW_SHIFT)) >> INDEX_NARROW_SHIFT;

        let short_symbol = ((short_prefix as u16) << 8) | second_byte as u16;
        let narrow_symbol = ((narrow_prefix as u16) << 8) | third_byte as u16;

        let short_symbol = match short_symbol {
            NO_SYMBOL => None,
            USE_ISO_CODE => Some(CurrencySymbolIndex::ISO),
            index => {
                debug_assert!(index <= MAX_SYMBOL_INDEX);
                Some(CurrencySymbolIndex::Index(index))
            }
        };

        let narrow_symbol = match narrow_symbol {
            NO_SYMBOL => None,
            USE_ISO_CODE => Some(CurrencySymbolIndex::ISO),
            index => {
                debug_assert!(index <= MAX_SYMBOL_INDEX);
                Some(CurrencySymbolIndex::Index(index))
            }
        };

        CurrencyPatternConfig {
            short_symbol,
            narrow_symbol,
        }
    }
}

impl<'a> ZeroMapKV<'a> for CurrencyPatternConfig {
    type Container = zerovec::ZeroVec<'a, CurrencyPatternConfig>;
    type Slice = zerovec::ZeroSlice<CurrencyPatternConfig>;
    type GetType = <CurrencyPatternConfig as AsULE>::ULE;
    type OwnedType = CurrencyPatternConfig;
}
