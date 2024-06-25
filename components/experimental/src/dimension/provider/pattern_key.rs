// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

use zerovec::{
    maps::ZeroMapKV,
    ule::{AsULE, ULE},
    ZeroVecError,
};

use crate::dimension::provider::units_essentials::CompoundCount;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::units_essentials)
)]
#[repr(u8)]
pub enum PowerValue {
    Two,
    Three,
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_experimental::dimension::provider::units_essentials)
)]
pub enum PatternKey {
    Binary(u8),
    Decimal(i8),
    Power(PowerValue, CompoundCount),
}

/// [`PatternKeyULE`] is a type optimized for efficient storage and
/// deserialization of [`PatternKey`] using the `ZeroVec` model.
///
/// The serialization model packages the pattern item in a single byte.
///
/// The first two bits (b7 & b6) determine the variant of the pattern key:
/// - `00`: `Binary`
/// - `01`: `Decimal`
/// - `10`: `Power`
/// - `11`: Forbidden
///
/// The next 6 bits (b5 to b0) determine the value of the pattern key:
/// - For `Binary`, the value is mapped directly to the pattern value.
/// - For `Decimal`:
///     - b5 is determining the sign of the value. if b5 is 0, the value is positive. if b5 is 1, the value is negative.
///     - b4 to b0 are determining the magnitude of the value.
/// - For `Power`:
///     - b5 and b4 represent the power value, which can be `10` to represent `Two` and `11` to represent `Three`.
///     - b3 to b0 represent the count value, which can be:
///         - `0000`: Zero
///         - `0001`: One
///         - `0010`: Two
///         - `0011`: Few
///         - `0100`: Many
///         - `0101`: Other
///     - Note: In the `Power` case, b3 is always 0, and when b2 is 1, b1 must be 0.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct PatternKeyULE(u8);

unsafe impl ULE for PatternKeyULE {
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
        if bytes.len() != 1 {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }

        let byte = &bytes[0];

        // Ensure the first two bits (b7 & b6) are not 11.
        if (byte & 0b1100_0000) == 0b1100_0000 {
            return Err(ZeroVecError::VarZeroVecFormatError);
        }

        // For the `Power` variant:
        //      b5 & b4 must be 10 or 11. (this means that b5 must be 1)
        //      b3 must be 0.
        //      When b2 is 1, b1 must be 0.
        if (byte & 0b1100_0000) == 0b1000_0000 {
            // b5 must be 1
            if (byte & 0b0010_0000) == 0 {
                return Err(ZeroVecError::VarZeroVecFormatError);
            }

            // b3 must be 0
            if (byte & 0b0000_1000) != 0 {
                return Err(ZeroVecError::VarZeroVecFormatError);
            }

            // If b2 is 1, b1 must be 0
            if (byte & 0b0000_0100) != 0 && (byte & 0b0000_0010) != 0 {
                return Err(ZeroVecError::VarZeroVecFormatError);
            }
        }

        Ok(())
    }
}

impl AsULE for PatternKey {
    type ULE = PatternKeyULE;

    fn to_unaligned(self) -> Self::ULE {
        let byte = match self {
            PatternKey::Binary(value) => value,
            PatternKey::Decimal(value) => {
                // TODO: shall I check the limits of the values?
                let sign = if value < 0 { 0b0010_0000 } else { 0 };
                0b0100_0000 + sign + (value as u8)
            }
            PatternKey::Power(power, count) => {
                let power_bits = {
                    match power {
                        PowerValue::Two => 0b0010_0000,
                        PowerValue::Three => 0b0100_0000,
                    }
                };
                0b1000_0000 + power_bits + count as u8
            }
        };

        PatternKeyULE(byte)
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let byte = unaligned.0;

        let variant = (byte & 0b11000000) >> 6;
        let value = byte & 0b00111111;

        match variant {
            0b00 => PatternKey::Binary(value),
            0b01 => match value & 0b00100000 {
                0b00000000 => PatternKey::Decimal(value as i8),
                0b00100000 => PatternKey::Decimal(-((value & 0b00011111) as i8)),
                _ => unreachable!(),
            },
            0b10 => {
                let power = match value & 0b00110000 {
                    0b00100000 => PowerValue::Two,
                    0b00110000 => PowerValue::Three,
                    _ => unreachable!(),
                };
                let count = value & 0b00001111;
                PatternKey::Power(power, count.into())
            }
            _ => unreachable!(),
        }
    }
}

impl<'a> ZeroMapKV<'a> for PatternKey {
    type Container = zerovec::ZeroVec<'a, PatternKey>;
    type Slice = zerovec::ZeroSlice<PatternKey>;
    type GetType = <PatternKey as AsULE>::ULE;
    type OwnedType = PatternKey;
}

impl From<u8> for CompoundCount {
    fn from(val: u8) -> Self {
        match val {
            0 => CompoundCount::Zero,
            1 => CompoundCount::One,
            2 => CompoundCount::Two,
            3 => CompoundCount::Few,
            4 => CompoundCount::Many,
            5 => CompoundCount::Other,
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_pattern_key_ule() {
    use PowerValue::Two;

    let binary = PatternKey::Binary(0b0000_1111);
    let binary_ule = binary.to_unaligned();
    PatternKeyULE::validate_byte_slice(&[binary_ule.0]).unwrap();
    assert_eq!(binary_ule.0, 0b0000_1111);

    let decimal = PatternKey::Decimal(0b0000_1111);
    let decimal_ule = decimal.to_unaligned();
    PatternKeyULE::validate_byte_slice(&[decimal_ule.0]).unwrap();
    assert_eq!(decimal_ule.0, 0b0100_1111);

    let power = PatternKey::Power(Two, CompoundCount::Two);
    let power_ule = power.to_unaligned();
    PatternKeyULE::validate_byte_slice(&[power_ule.0]).unwrap();
    assert_eq!(power_ule.0, 0b1010_0010);

    let binary = PatternKey::from_unaligned(binary_ule);
    assert_eq!(binary, PatternKey::Binary(0b0000_1111));

    let decimal = PatternKey::from_unaligned(decimal_ule);
    assert_eq!(decimal, PatternKey::Decimal(0b0000_1111));

    let power = PatternKey::from_unaligned(power_ule);
    assert_eq!(power, PatternKey::Power(Two, CompoundCount::Two));
}
