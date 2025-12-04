// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::fractions::FractionInfo;
use zerovec::{
    maps::ZeroMapKV,
    ule::{AsULE, UleError, ULE},
};

/// Marker value indicating None for cash_digits and cash_rounding fields.
const NONE_MARKER: u8 = 255;

/// ULE type for FractionInfo - packed into 4 bytes.
///
/// Byte layout:
/// - Byte 0: digits
/// - Byte 1: rounding
/// - Byte 2: cash_digits (255 = None)
/// - Byte 3: cash_rounding (255 = None)
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct FractionInfoULE([u8; 4]);

// Safety (based on the safety checklist on the ULE trait):
//  1. FractionInfoULE does not include any uninitialized or padding bytes.
//     (achieved by `#[repr(transparent)]` on a ULE type)
//  2. FractionInfoULE is aligned to 1 byte.
//     (achieved by `#[repr(transparent)]` on a ULE type)
//  3. The impl of validate_bytes() returns an error if any byte is not valid.
//  4. The impl of validate_bytes() returns an error if there are extra bytes.
//  5. The other ULE methods use the default impl.
//  6. FractionInfoULE byte equality is semantic equality.
unsafe impl ULE for FractionInfoULE {
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        if bytes.len() % 4 != 0 {
            return Err(UleError::length::<Self>(bytes.len()));
        }
        Ok(())
    }
}

impl AsULE for FractionInfo {
    type ULE = FractionInfoULE;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        FractionInfoULE([
            self.digits,
            self.rounding,
            self.cash_digits.unwrap_or(NONE_MARKER),
            self.cash_rounding.unwrap_or(NONE_MARKER),
        ])
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let [digits, rounding, cash_digits, cash_rounding] = unaligned.0;
        FractionInfo {
            digits,
            rounding,
            cash_digits: if cash_digits == NONE_MARKER {
                None
            } else {
                Some(cash_digits)
            },
            cash_rounding: if cash_rounding == NONE_MARKER {
                None
            } else {
                Some(cash_rounding)
            },
        }
    }
}

impl<'a> ZeroMapKV<'a> for FractionInfo {
    type Container = zerovec::ZeroVec<'a, FractionInfo>;
    type Slice = zerovec::ZeroSlice<FractionInfo>;
    type GetType = <FractionInfo as AsULE>::ULE;
    type OwnedType = FractionInfo;
}
