// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerovec::ule::{AsULE, CharULE, ULE};
use zerovec::ZeroVecError;

use crate::props::{BidiPairedBracketType, BidiPairedBracketTypeULE};

/// TODO: This module's description
///
/// This module provides an efficient storage of data serving the following
/// properties:
/// - `Bidi_Paired_Bracket`
/// - `Bidi_Paired_Bracket_Type`
/// - `Bidi_Mirrored`
/// - `Bidi_Mirroring_Glyph`
///


/// 20..0  Code point return value for Bidi_Mirroring_Glyph value
/// 21..21 Boolean for Bidi_Mirrored
/// 23..22 Enum value for Bidi_Paired_Bracket_Type
#[doc(hidden)] // needed for datagen but not intended for users
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct MirroredPairedBracketData {
    mirroring_glyph: char,
    is_mirrored: bool,
    paired_bracket_type: BidiPairedBracketType,
}

#[doc(hidden)] // needed for datagen but not intended for users
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
//  6. CharULE byte equality is semantic equality
unsafe impl ULE for MirroredPairedBracketDataULE {
    #[inline]
    fn validate_byte_slice(bytes: &[u8]) -> Result<(), ZeroVecError> {
        if bytes.len() % 3 != 0 {
            return Err(ZeroVecError::InvalidLength {
                ty: "MirroredPairedBracketDataULE",
                len: bytes.len(),
            });
        }
        // Validate the bytes
        for byte_triple in bytes.chunks_exact(3) {
            // Bidi_Mirroring_Glyph validation
            #[allow(clippy::indexing_slicing)]
            // Won't panic because the chunks are always 3 bytes long
            let byte0 = byte_triple[0];
            let byte1 = byte_triple[1];
            let byte2 = byte_triple[2];
            let mut mirroring_glyph_code_point: u32 = (byte2 | 0x1F) as u32;
            mirroring_glyph_code_point = (mirroring_glyph_code_point << 8) | (byte1 as u32);
            mirroring_glyph_code_point = (mirroring_glyph_code_point << 8) | (byte0 as u32);
            let _mirroring_glyph = char::from_u32(mirroring_glyph_code_point)
                .ok_or(ZeroVecError::VarZeroVecFormatError)?;

            // skip validating the Bidi_Mirrored boolean since it is always valid

            // skip validating Bidi_Paired_Bracket_Type because enum is open
            //
            // TODO: is the above statement correct and okay?
            // TODO: or do we have a way to validate the enum discriminant against the enum for
            //       newtype idiom enums?
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
        let mut byte2 = byte_slice[2];
        byte2 |= (self.is_mirrored as u8) << 5;
        byte2 |= self.paired_bracket_type.0 << 6;

        MirroredPairedBracketDataULE([byte_slice[0], byte_slice[1], byte2])
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let mirroring_glyph_ule_bytes = &[unaligned.0[0], unaligned.0[1], unaligned.0[2] | 0x1F];
        // Safe because the lower bits 20..0 of MirroredPairedBracketDataULE bytes are the CharULE bytes,
        // and CharULE::from_unaligned is safe because bytes are defined to represent a valid Unicode code point.
        let mirroring_glyph = unsafe {
            let mirroring_glyph_ule = CharULE::from_byte_slice_unchecked(mirroring_glyph_ule_bytes);
            char::from_unaligned(mirroring_glyph_ule[0])
        };
        let is_mirrored = unaligned.0[2] | 0x20 == 1;
        let paired_bracket_type =
            BidiPairedBracketType::from_unaligned(BidiPairedBracketTypeULE(unaligned.0[2] | 0xC0));

        Self {
            mirroring_glyph,
            is_mirrored,
            paired_bracket_type,
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_parse() {
        // data for U+007B LEFT CURLY BRACKET 
        
        let data1 = MirroredPairedBracketData {
            mirroring_glyph: '}',
            is_mirrored: true,
            paired_bracket_type: BidiPairedBracketType::Open,
        };
        let expected_bytes1 = &[0x7D, 0x0, 0x60];
        assert_eq!(expected_bytes1, MirroredPairedBracketDataULE::as_byte_slice(&[data1.to_unaligned()]));


        let ule1 = MirroredPairedBracketDataULE::parse_byte_slice(expected_bytes1).unwrap();
        let parsed_data1 = MirroredPairedBracketData::from_unaligned(ule1.first().unwrap().clone());
        assert_eq!(data1, parsed_data1);
    }
}
