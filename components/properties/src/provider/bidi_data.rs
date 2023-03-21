// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]
//!
//! This module provides an efficient storage of data serving the following
//! properties:
//! - `Bidi_Paired_Bracket`
//! - `Bidi_Paired_Bracket_Type`
//! - `Bidi_Mirrored`
//! - `Bidi_Mirroring_Glyph`

use crate::props::BidiPairedBracketType;

use icu_collections::codepointtrie::CodePointTrie;
use icu_provider::prelude::*;
use zerovec::ule::{AsULE, CharULE, ULE};
use zerovec::ZeroVecError;

/// A data provider struct for properties related to Bidi algorithms, including
/// mirroring and bracket pairing.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(BidiAuxiliaryPropertiesV1Marker = "props/bidiauxiliaryprops@1")]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider::bidi_data),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct BidiAuxiliaryPropertiesV1<'data> {
    /// A `CodePointTrie` efficiently storing the data from which property values
    /// can be extracted or derived for the supported Bidi properties.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, MirroredPairedBracketData>,
}

impl<'data> BidiAuxiliaryPropertiesV1<'data> {
    #[doc(hidden)]
    pub fn new(
        trie: CodePointTrie<'data, MirroredPairedBracketData>,
    ) -> BidiAuxiliaryPropertiesV1<'data> {
        BidiAuxiliaryPropertiesV1 { trie }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::bidi_data))]
#[doc(hidden)] // needed for datagen but not intended for users
pub struct MirroredPairedBracketData {
    pub mirroring_glyph: char,
    pub mirrored: bool,
    pub paired_bracket_type: BidiPairedBracketType,
}

impl Default for MirroredPairedBracketData {
    fn default() -> Self {
        Self {
            mirroring_glyph: 0 as char,
            mirrored: false,
            paired_bracket_type: BidiPairedBracketType::None,
        }
    }
}

impl From<MirroredPairedBracketData> for u32 {
    fn from(mpbd: MirroredPairedBracketData) -> u32 {
        let ule = mpbd.to_unaligned();
        let ule_slice = &[ule];
        let bytes = ULE::as_byte_slice(ule_slice);
        match <[u8; 3]>::try_from(bytes) {
            Ok(array) => u32::from_le_bytes([array[0], array[1], array[2], 0]),
            Err(_) => u32::MAX,
        }
    }
}

impl TryFrom<u32> for MirroredPairedBracketData {
    type Error = ZeroVecError;

    fn try_from(x: u32) -> Result<Self, ZeroVecError> {
        let[byte0, byte1, byte2, byte3] = u32::to_le_bytes(x);
        if byte3 != 0 {
            return Err(ZeroVecError::parse::<Self>());
        }
        let ule_byte_array = [byte0, byte1, byte2];
        let ule_struct_slice = MirroredPairedBracketDataULE::parse_byte_slice(&ule_byte_array)?;
        let ule_struct = ule_struct_slice
            .first()
            .ok_or(ZeroVecError::parse::<Self>())?;
        let data = <Self as AsULE>::from_unaligned(*ule_struct);
        Ok(data)
    }
}

/// Bit layout for the 24 bits (0..=23) of the `[u8; 3]` ULE raw type.
/// LE means first byte is 0..=7, second byte 8..=15, third byte is 16..=23
///  0..=20  Code point return value for Bidi_Mirroring_Glyph value
///    extracted with: mask = 0x1FFFFF <=> [bytes[0], bytes[1], bytes[2] & 0x1F]
///  21..=21 Boolean for Bidi_Mirrored
///    extracted with: bitshift right by 21 followed by mask = 0x1 <=> (bytes[2] >> 5) & 0x1
///  22..=23 Enum discriminant value for Bidi_Paired_Bracket_Type
///    extracted with: bitshift right by 22 followed by mask = 0x3 <=> (bytes[2] >> 6) & 0x3
///                    <=> (bytes[2] >> 6) b/c we left fill with 0s on bitshift right for unsigned
///                         numbers and a byte has 8 bits
#[doc(hidden)]
/// needed for datagen but not intended for users
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
#[repr(packed)]
pub struct MirroredPairedBracketDataULE([u8; 3]);

// Safety (based on the safety checklist on the ULE trait):
//  1. MirroredPairedBracketDataULE does not include any uninitialized or padding bytes
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  2. MirroredPairedBracketDataULE is aligned to 1 byte.
//     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
//  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
//  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
//  5. The other ULE methods use the default impl.
//  6. MirroredPairedBracketDataULE byte equality is semantic equality because all bits
//     are used, so no unused bits requires no extra work to zero out unused bits
unsafe impl ULE for MirroredPairedBracketDataULE {
    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        if bytes.len() % 3 != 0 {
            return Err(ZeroVecError::length::<Self>(bytes.len()));
        }
        // Validate the bytes
        #[allow(clippy::indexing_slicing)] // Won't panic because the chunks are always 3 bytes long
        for byte_triple in bytes.chunks_exact(3) {
            // Bidi_Mirroring_Glyph validation
            #[allow(clippy::unwrap_used)] // chunks_exact returns slices of length 3
            let [byte0, byte1, byte2] = *<&[u8; 3]>::try_from(byte_triple).unwrap();
            let mut mirroring_glyph_code_point: u32 = (byte2 & 0x1F) as u32;
            mirroring_glyph_code_point = (mirroring_glyph_code_point << 8) | (byte1 as u32);
            mirroring_glyph_code_point = (mirroring_glyph_code_point << 8) | (byte0 as u32);
            let _mirroring_glyph = char::from_u32(mirroring_glyph_code_point)
                .ok_or(ZeroVecError::VarZeroVecFormatError)?;

            // skip validating the Bidi_Mirrored boolean since it is always valid

            // assert that Bidi_Paired_Bracket_Type cannot have a 4th value because it only
            // has 3 values: Open, Close, None
            let paired_bracket_type_discriminant = byte2 >> 6;
            if paired_bracket_type_discriminant == 0x3 {
                return Err(ZeroVecError::ParseError {
                    ty:
                        "Unrecognized value for paired_bracket_type in MirroredPairedBracketDataULE",
                });
            }
        }

        Ok(())
    }
}

impl AsULE for MirroredPairedBracketData {
    type ULE = MirroredPairedBracketDataULE;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        let mirr_glyph_char_ule_ref_slice = &[self.mirroring_glyph.to_unaligned()];
        let byte_slice = CharULE::as_byte_slice(mirr_glyph_char_ule_ref_slice);
        let mut byte2 = byte_slice.get(2).copied().unwrap_or_default();
        byte2 |= (self.mirrored as u8) << 5;
        byte2 |= self.paired_bracket_type.0 << 6;

        MirroredPairedBracketDataULE([
            byte_slice.first().copied().unwrap_or_default(),
            byte_slice.get(1).copied().unwrap_or_default(),
            byte2,
        ])
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        // Safe because the lower bits 20..0 of MirroredPairedBracketDataULE bytes are the CharULE bytes,
        // and CharULE::from_unaligned is safe because bytes are defined to represent a valid Unicode code point.
        let mirroring_glyph = unsafe {
            let mirroring_glyph_ule_bytes =
                &[unaligned.0[0], unaligned.0[1], unaligned.0[2] & 0x1F];
            let mirroring_glyph_ule = CharULE::from_byte_slice_unchecked(mirroring_glyph_ule_bytes);
            mirroring_glyph_ule
                .first()
                .map(|ule| char::from_unaligned(*ule))
                .unwrap_or(char::REPLACEMENT_CHARACTER)
        };
        let mirrored = (unaligned.0[2] >> 5) & 0x1 == 1;
        let paired_bracket_type = BidiPairedBracketType(unaligned.0[2] >> 6);

        Self {
            mirroring_glyph,
            mirrored,
            paired_bracket_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // data for U+007B LEFT CURLY BRACKET

        // serialize to ULE bytes
        let data = MirroredPairedBracketData {
            mirroring_glyph: '}',
            mirrored: true,
            paired_bracket_type: BidiPairedBracketType::Open,
        };
        let expected_bytes = &[0x7D, 0x0, 0x60];
        assert_eq!(
            expected_bytes,
            MirroredPairedBracketDataULE::as_byte_slice(&[data.to_unaligned()])
        );

        // deserialize from ULE bytes
        let ule = MirroredPairedBracketDataULE::parse_byte_slice(expected_bytes).unwrap();
        let parsed_data = MirroredPairedBracketData::from_unaligned(*ule.first().unwrap());
        assert_eq!(data, parsed_data);
    }

    #[test]
    fn test_parse_error() {
        // data for U+007B LEFT CURLY BRACKET
        let ule_bytes = &mut [0x7D, 0x0, 0x60];

        // Set discriminant value for the BidiPairedBracketType enum to be invalid.
        // BidiPairedBracketType only has 3 values (discriminants => 0..=2), so the 4th
        // expressible value from the 2 bits (3) should not parse successfully.
        ule_bytes[2] = ule_bytes[2] | 0xC0;

        // deserialize from ULE bytes
        let ule_parse_result = MirroredPairedBracketDataULE::parse_byte_slice(ule_bytes);
        assert!(matches!(ule_parse_result, Err(_)));
    }
}
