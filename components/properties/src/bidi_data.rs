// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::bidi_data::{BidiAuxiliaryPropertiesV1, BidiAuxiliaryPropertiesV1Marker};
use crate::BidiPairedBracketType;
use crate::PropertiesError;

use icu_provider::prelude::*;

#[derive(Debug)]
pub struct BidiAuxiliaryProperties {
    data: DataPayload<BidiAuxiliaryPropertiesV1Marker>,
}

impl BidiAuxiliaryProperties {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> BidiAuxiliaryPropertiesBorrowed<'_> {
        BidiAuxiliaryPropertiesBorrowed {
            data: self.data.get(),
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters like [`load_script_with_extensions_unstable()`] instead
    pub(crate) fn from_data(data: DataPayload<BidiAuxiliaryPropertiesV1Marker>) -> Self {
        Self { data }
    }
}

#[derive(Debug)]
pub struct BidiMirroringProperties {
    pub mirroring_glyph: Option<char>,
    pub mirrored: bool,
}

/// The enum represents Bidi_Paired_Bracket_Type, the char represents Bidi_Paired_Bracket.
/// Bidi_Paired_Bracket has a value of `None` when Bidi_Paired_Bracket_Type is `None`.
#[derive(Debug)]
pub enum BidiPairingProperties {
    Open(char),
    Close(char),
    None,
}

#[derive(Debug)]
pub struct BidiAuxiliaryPropertiesBorrowed<'a> {
    data: &'a BidiAuxiliaryPropertiesV1<'a>,
}

impl<'a> BidiAuxiliaryPropertiesBorrowed<'a> {
    fn convert_mirroring_glyph_data(trie_data_char: char) -> Option<char> {
        if trie_data_char as u32 == 0 {
            None
        } else {
            Some(trie_data_char)
        }
    }

    pub fn get_mirroring_props(self, code_point: u32) -> BidiMirroringProperties {
        let bidi_aux_props = self.data.trie.get32(code_point);
        let mirroring_glyph_opt =
            Self::convert_mirroring_glyph_data(bidi_aux_props.mirroring_glyph);
        BidiMirroringProperties {
            mirroring_glyph: mirroring_glyph_opt,
            mirrored: bidi_aux_props.mirrored,
        }
    }

    pub fn get_pairing_props(self, code_point: u32) -> BidiPairingProperties {
        let bidi_aux_props = self.data.trie.get32(code_point);
        let mirroring_glyph = bidi_aux_props.mirroring_glyph;
        let paired_bracket_type = bidi_aux_props.paired_bracket_type;
        match paired_bracket_type {
            BidiPairedBracketType::Open => BidiPairingProperties::Open(mirroring_glyph),
            BidiPairedBracketType::Close => BidiPairingProperties::Close(mirroring_glyph),
            _ => BidiPairingProperties::None,
        }
    }
}

pub fn load_bidi_mirroring_properties_unstable(
    provider: &(impl DataProvider<BidiAuxiliaryPropertiesV1Marker> + ?Sized),
) -> Result<BidiAuxiliaryProperties, PropertiesError> {
    Ok(provider
        .load(Default::default())
        .and_then(DataResponse::take_payload)
        .map(BidiAuxiliaryProperties::from_data)?)
}

icu_provider::gen_any_buffer_constructors!(
    locale: skip,
    options: skip,
    result: Result<BidiAuxiliaryProperties, PropertiesError>,
    functions: [
        load_bidi_mirroring_properties_unstable,
        load_bidi_mirroring_properties_with_any_provider,
        load_bidi_mirroring_properties_with_buffer_provider
    ]
);
