// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ule::{AsULE, ZeroVecError, ULE};

use super::{count::Count, extended_currency::CurrencyDisplayNameCount};

/// [`CurrencyDisplayNameCountULE`] is a type optimized for efficient storing and
/// deserialization of [`CurrencyDisplayNameCount`] using the `ZeroVec` model.
///
/// The serialization model packages the pattern item in one byte.
///
/// The last three bits (b2, b1 & b0), are used to determine the count:
///     000 -> PluralRules(Count::Zero)
///     001 -> PluralRules(Count::One)
///     010 -> PluralRules(Count::Two)
///     011 -> PluralRules(Count::Few)
///     100 -> PluralRules(Count::Many)
///     101 -> PluralRules(Count::Other)
///     110 -> DisplayName,
///
/// The other bits (b7, b6, b5, b4, b3) must always be zeros.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct CurrencyDisplayNameCountULE(u8);

// Safety (based on the safety checklist on the ULE trait):
//  1. CurrencyDisplayNameCountULE does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(transparent)]` on a ULE type)
//  2. CurrencyDisplayNameCountULE is aligned to 1 byte.
//     (achieved by `#[repr(transparent)]` on a ULE type)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
//  5. The other ULE methods use the default impl.
//  6. CurrencyDisplayNameCountULE byte equality is semantic equality.
unsafe impl ULE for CurrencyDisplayNameCountULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        for byte in bytes {
            if byte & 0b1111_1000 != 0 {
                return Err(ZeroVecError::parse::<Self>());
            }

            // 0000 0111 represent the seventh element of the enum which is not exist.
            if (byte & 0b0000_0111) == 0b0000_0111 {
                return Err(ZeroVecError::parse::<Self>());
            }
        }

        Ok(())
    }
}

impl AsULE for CurrencyDisplayNameCount {
    type ULE = CurrencyDisplayNameCountULE;
    fn to_unaligned(self) -> Self::ULE {
        CurrencyDisplayNameCountULE(match self {
            CurrencyDisplayNameCount::PluralRules(count) => count as u8,
            CurrencyDisplayNameCount::DisplayName => 0b0000_0110,
        })
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        match unaligned.0 & 0b0000_0111 {
            0 => CurrencyDisplayNameCount::PluralRules(Count::Zero),
            1 => CurrencyDisplayNameCount::PluralRules(Count::One),
            2 => CurrencyDisplayNameCount::PluralRules(Count::Two),
            3 => CurrencyDisplayNameCount::PluralRules(Count::Few),
            4 => CurrencyDisplayNameCount::PluralRules(Count::Many),
            5 => CurrencyDisplayNameCount::PluralRules(Count::Other),
            6 => CurrencyDisplayNameCount::DisplayName,
            _ => unreachable!(),
        }
    }
}
