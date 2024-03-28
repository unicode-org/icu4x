// // This file is part of ICU4X. For terms of use, please see the file
// // called LICENSE at the top level of the ICU4X source tree
// // (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// /// `ExtendedCurrencyPatternConfigULE` is a type optimized for efficient storing and
// /// deserialization of `CurrencyPatterns` using the `ZeroVec` model.
// ///
// /// The serialization model packages the pattern item in three bytes.
// ///
// /// The first bit (b7) is used to determine the short_pattern_selection. If the bit is `0`, then, the value will be `Standard`.
// /// If the bit is `1`, then, the value will be `StandardAlphaNextToNumber`.
// ///
// /// The second bit (b6) is used to determine the narrow_pattern_selection. If the bit is `0`, then, the value will be `Standard`.
// /// If the bit is `1`, then, the value will be `StandardAlphaNextToNumber`.
// ///
// /// The next three bits (b5, b4 & b3) with the second byte is used to determine the short_placeholder_value.
// /// The next three bits (b2, b1 & b0) with the third byte is used to determine the narrow_placeholder_value.
// #[derive(Copy, Clone, Debug, PartialEq)]
// #[repr(transparent)]
// pub struct ExtendedCurrencyPatternConfigULE([u8; 3]);


// // Safety (based on the safety checklist on the ULE trait):
// //  1. ExtendedCurrencyPatternConfigULE does not include any uninitialized or padding bytes.
// //     (achieved by `#[repr(transparent)]` on a ULE type)
// //  2. ExtendedCurrencyPatternConfigULE is aligned to 1 byte.
// //     (achieved by `#[repr(transparent)]` on a ULE type)
// //  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
// //  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
// //  5. The other ULE methods use the default impl.
// //  6. ExtendedCurrencyPatternConfigULE byte equality is semantic equality.
// unsafe impl ULE for ExtendedCurrencyPatternConfigULE {
//     fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
//         if bytes.len() % 3 != 0 {
//             return Err(ZeroVecError::length::<Self>(bytes.len()));
//         }

//         Ok(())
//     }
// }