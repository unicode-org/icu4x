// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// `ExtendedCurrencyPatternConfigULE` is a type optimized for efficient storing and
/// deserialization of `CurrencyPatterns` using the `ZeroVec` model.
///
/// The serialization model packages the pattern item in three bytes.
///
/// The first bit (b7) is used to determine the short_pattern_selection. If the bit is `0`, then, the value will be `Standard`.
/// If the bit is `1`, then, the value will be `StandardAlphaNextToNumber`.
///
/// The second bit (b6) is used to determine the narrow_pattern_selection. If the bit is `0`, then, the value will be `Standard`.
/// If the bit is `1`, then, the value will be `StandardAlphaNextToNumber`.
///
/// The next three bits (b5, b4 & b3) with the second byte is used to determine the short_placeholder_value.
/// The next three bits (b2, b1 & b0) with the third byte is used to determine the narrow_placeholder_value.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct ExtendedCurrencyPatternConfigULE([u8; 3]);