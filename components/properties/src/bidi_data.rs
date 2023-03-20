// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data and APIs for supporting specific Bidi properties data in an efficient structure.
//!
//! Supported properties are:
//! - `Bidi_Paired_Bracket`
//! - `Bidi_Paired_Bracket_Type`
//! - `Bidi_Mirrored`
//! - `Bidi_Mirroring_Glyph`

use crate::provider::bidi_data::{BidiAuxiliaryPropertiesV1, BidiAuxiliaryPropertiesV1Marker};
use crate::BidiPairedBracketType;
use crate::PropertiesError;

use icu_provider::prelude::*;

/// A wrapper around certain Bidi properties data. Can be obtained via [`load_bidi_auxiliary_properties_unstable()`] and
/// related getters.
///
/// Most useful methods are on [`BidiAuxiliaryPropertiesBorrowed`] obtained by calling [`BidiAuxiliaryProperties::as_borrowed()`]
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
    /// Typically it is preferable to use getters like [`load_bidi_auxiliary_properties_unstable()`] instead
    pub(crate) fn from_data(data: DataPayload<BidiAuxiliaryPropertiesV1Marker>) -> Self {
        Self { data }
    }
}

/// This struct represents the properties Bidi_Mirrored and Bidi_Mirroring_Glyph.
/// If Bidi_Mirroring_Glyph is not defined for a code point, then the value in the
/// struct is `None`.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct BidiMirroringProperties {
    /// Represents the Bidi_Mirroring_Glyph property value
    pub mirroring_glyph: Option<char>,
    /// Represents the Bidi_Mirrored property value
    pub mirrored: bool,
}

/// The enum represents Bidi_Paired_Bracket_Type, the char represents Bidi_Paired_Bracket.
/// Bidi_Paired_Bracket has a value of `None` when Bidi_Paired_Bracket_Type is `None`.
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum BidiPairingProperties {
    /// Represents Bidi_Paired_Bracket_Type=Open, and the Bidi_Paired_Bracket value for that code point.
    Open(char),
    /// Represents Bidi_Paired_Bracket_Type=Close, and the Bidi_Paired_Bracket value for that code point.
    Close(char),
    /// Represents Bidi_Paired_Bracket_Type=None, which cooccurs with Bidi_Paired_Bracket
    /// being undefined for that code point.
    None,
}

/// A borrowed wrapper around Bidi properties data, returned by
/// [`BidiAuxiliaryProperties::as_borrowed()`]. More efficient to query.
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

    /// Return a struct for the given code point representing Bidi mirroring-related
    /// property values. See [`BidiMirroringProperties`].
    ///
    /// # Examples
    /// ```
    /// use icu_properties::{bidi_data, bidi_data::BidiMirroringProperties};
    ///
    /// let data =
    ///     bidi_data::load_bidi_auxiliary_properties_unstable(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let bidi_data = data.as_borrowed();
    ///
    /// let open_paren = bidi_data.get_mirroring_props('(' as u32);
    /// assert_eq!(open_paren, BidiMirroringProperties{ mirroring_glyph: Some(')'), mirrored: true });
    /// let close_paren = bidi_data.get_mirroring_props(')' as u32);
    /// assert_eq!(close_paren, BidiMirroringProperties{ mirroring_glyph: Some('('), mirrored: true });
    /// let open_angle_bracket = bidi_data.get_mirroring_props('<' as u32);
    /// assert_eq!(open_angle_bracket, BidiMirroringProperties{ mirroring_glyph: Some('>'), mirrored: true });
    /// let close_angle_bracket = bidi_data.get_mirroring_props('>' as u32);
    /// assert_eq!(close_angle_bracket, BidiMirroringProperties{ mirroring_glyph: Some('<'), mirrored: true });
    /// let three = bidi_data.get_mirroring_props('3' as u32);
    /// assert_eq!(three, BidiMirroringProperties{ mirroring_glyph: None, mirrored: false });
    /// ```
    pub fn get_mirroring_props(&self, code_point: u32) -> BidiMirroringProperties {
        let bidi_aux_props = self.data.trie.get32(code_point);
        let mirroring_glyph_opt =
            Self::convert_mirroring_glyph_data(bidi_aux_props.mirroring_glyph);
        BidiMirroringProperties {
            mirroring_glyph: mirroring_glyph_opt,
            mirrored: bidi_aux_props.mirrored,
        }
    }

    /// Return a struct for the given code point representing Bidi bracket
    /// pairing-related property values. See [`BidiPairingProperties`]
    ///
    /// # Examples
    /// ```
    /// use icu_properties::BidiPairedBracketType;
    /// use icu_properties::{bidi_data, bidi_data::BidiPairingProperties};
    ///
    /// let data =
    ///     bidi_data::load_bidi_auxiliary_properties_unstable(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let bidi_data = data.as_borrowed();
    ///
    /// let open_paren = bidi_data.get_pairing_props('(' as u32);
    /// assert_eq!(open_paren, BidiPairingProperties::Open(')'));
    /// let close_paren = bidi_data.get_pairing_props(')' as u32);
    /// assert_eq!(close_paren, BidiPairingProperties::Close('('));
    /// let open_angle_bracket = bidi_data.get_pairing_props('<' as u32);
    /// assert_eq!(open_angle_bracket, BidiPairingProperties::None);
    /// let close_angle_bracket = bidi_data.get_pairing_props('>' as u32);
    /// assert_eq!(close_angle_bracket, BidiPairingProperties::None);
    /// let three = bidi_data.get_pairing_props('3' as u32);
    /// assert_eq!(three, BidiPairingProperties::None);
    /// ```
    pub fn get_pairing_props(&self, code_point: u32) -> BidiPairingProperties {
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

/// Returns a [`BidiAuxiliaryPropertiesV1`] struct that represents the data for certain
/// Bidi properties.
///
/// [📚 Help choosing a constructor](icu_provider::constructors)
/// <div class="stab unstable">
/// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
/// </div>
///
/// # Examples
/// ```
/// use icu_properties::{bidi_data, bidi_data::BidiMirroringProperties};
///
/// let data =
///     bidi_data::load_bidi_auxiliary_properties_unstable(&icu_testdata::unstable())
///         .expect("The data should be valid");
/// let bidi_data = data.as_borrowed();
///
/// let open_paren = bidi_data.get_mirroring_props('(' as u32);
/// assert_eq!(open_paren, BidiMirroringProperties{ mirroring_glyph: Some(')'), mirrored: true });
/// ```
pub fn load_bidi_auxiliary_properties_unstable(
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
        load_bidi_auxiliary_properties_unstable,
        load_bidi_auxiliary_properties_with_any_provider,
        load_bidi_auxiliary_properties_with_buffer_provider
    ]
);
