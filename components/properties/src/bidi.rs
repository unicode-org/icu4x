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

use crate::provider::bidi::{
    BidiAuxiliaryPropertiesV1, BidiAuxiliaryPropertiesV1Marker, CheckedBidiPairedBracketType,
};

use icu_provider::prelude::*;

/// A wrapper around certain Bidi properties data.
///
/// Most useful methods are on [`BidiAuxiliaryPropertiesBorrowed`] obtained by calling [`BidiAuxiliaryProperties::as_borrowed()`]
#[derive(Debug)]
pub struct BidiAuxiliaryProperties {
    data: DataPayload<BidiAuxiliaryPropertiesV1Marker>,
}

impl BidiAuxiliaryProperties {
    /// Creates a [`BidiAuxiliaryPropertiesV1`] struct that represents the data for certain
    /// Bidi properties.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// # Examples
    /// ```
    /// use icu::properties::bidi::BidiAuxiliaryProperties;
    ///
    /// let bidi_data = BidiAuxiliaryProperties::new();
    ///
    /// let open_paren = bidi_data.get32_mirroring_props('(' as u32);
    /// assert_eq!(open_paren.mirroring_glyph, Some(')'));
    /// assert_eq!(open_paren.mirrored, true);
    /// ```
    #[cfg(feature = "compiled_data")]
    #[allow(clippy::new_ret_no_self)]
    pub const fn new() -> BidiAuxiliaryPropertiesBorrowed<'static> {
        BidiAuxiliaryPropertiesBorrowed {
            data: crate::provider::Baked::SINGLETON_BIDI_AUXILIARY_PROPERTIES_V1_MARKER,
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(
        () -> result: Result<BidiAuxiliaryProperties, DataError>,
        functions: [
            new: skip,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable(
        provider: &(impl DataProvider<BidiAuxiliaryPropertiesV1Marker> + ?Sized),
    ) -> Result<BidiAuxiliaryProperties, DataError> {
        Ok(BidiAuxiliaryProperties::from_data(
            provider.load(Default::default())?.payload,
        ))
    }

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
    /// Typically it is preferable to use getters like [`bidi_auxiliary_properties()`] instead
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
    // The source data coming from icuexportdata will use 0 to represent the
    // property value in cases for which the Bidi_Mirroring_Glyph property value
    // of a code point is undefined. Since Rust types can be more expressive, we
    // should represent these cases as None.
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
    /// use icu::properties::bidi::BidiAuxiliaryProperties;
    ///
    /// let bidi_data = BidiAuxiliaryProperties::new();
    ///
    /// let open_paren = bidi_data.get32_mirroring_props('(' as u32);
    /// assert_eq!(open_paren.mirroring_glyph, Some(')'));
    /// assert_eq!(open_paren.mirrored, true);
    /// let close_paren = bidi_data.get32_mirroring_props(')' as u32);
    /// assert_eq!(close_paren.mirroring_glyph, Some('('));
    /// assert_eq!(close_paren.mirrored, true);
    /// let open_angle_bracket = bidi_data.get32_mirroring_props('<' as u32);
    /// assert_eq!(open_angle_bracket.mirroring_glyph, Some('>'));
    /// assert_eq!(open_angle_bracket.mirrored, true);
    /// let close_angle_bracket = bidi_data.get32_mirroring_props('>' as u32);
    /// assert_eq!(close_angle_bracket.mirroring_glyph, Some('<'));
    /// assert_eq!(close_angle_bracket.mirrored, true);
    /// let three = bidi_data.get32_mirroring_props('3' as u32);
    /// assert_eq!(three.mirroring_glyph, None);
    /// assert_eq!(three.mirrored, false);
    /// ```
    pub fn get32_mirroring_props(&self, code_point: u32) -> BidiMirroringProperties {
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
    /// use icu::properties::bidi::{BidiAuxiliaryProperties, BidiPairingProperties};
    ///
    /// let bidi_data = BidiAuxiliaryProperties::new();
    ///
    /// let open_paren = bidi_data.get32_pairing_props('(' as u32);
    /// assert_eq!(open_paren, BidiPairingProperties::Open(')'));
    /// let close_paren = bidi_data.get32_pairing_props(')' as u32);
    /// assert_eq!(close_paren, BidiPairingProperties::Close('('));
    /// let open_angle_bracket = bidi_data.get32_pairing_props('<' as u32);
    /// assert_eq!(open_angle_bracket, BidiPairingProperties::None);
    /// let close_angle_bracket = bidi_data.get32_pairing_props('>' as u32);
    /// assert_eq!(close_angle_bracket, BidiPairingProperties::None);
    /// let three = bidi_data.get32_pairing_props('3' as u32);
    /// assert_eq!(three, BidiPairingProperties::None);
    /// ```
    pub fn get32_pairing_props(&self, code_point: u32) -> BidiPairingProperties {
        let bidi_aux_props = self.data.trie.get32(code_point);
        let mirroring_glyph = bidi_aux_props.mirroring_glyph;
        let paired_bracket_type = bidi_aux_props.paired_bracket_type;
        match paired_bracket_type {
            CheckedBidiPairedBracketType::Open => BidiPairingProperties::Open(mirroring_glyph),
            CheckedBidiPairedBracketType::Close => BidiPairingProperties::Close(mirroring_glyph),
            _ => BidiPairingProperties::None,
        }
    }
}

impl BidiAuxiliaryPropertiesBorrowed<'static> {
    /// Cheaply converts a [`BidiAuxiliaryPropertiesBorrowed<'static>`] into a [`BidiAuxiliaryProperties`].
    ///
    /// Note: Due to branching and indirection, using [`BidiAuxiliaryProperties`] might inhibit some
    /// compile-time optimizations that are possible with [`BidiAuxiliaryPropertiesBorrowed`].
    pub const fn static_to_owned(self) -> BidiAuxiliaryProperties {
        BidiAuxiliaryProperties {
            data: DataPayload::from_static_ref(self.data),
        }
    }
}

/// Implements [`unicode_bidi::BidiDataSource`] on [`CodePointMapDataBorrowed<BidiClass>`].
///
/// ✨ *Enabled with the `unicode_bidi` Cargo feature.*
///
/// # Examples
///
///```
/// use icu::properties::CodePointMapData;
/// use icu::properties::props::BidiClass;
/// use unicode_bidi::BidiInfo;
///
/// // This example text is defined using `concat!` because some browsers
/// // and text editors have trouble displaying bidi strings.
/// let text =  concat!["א", // RTL#1
///                     "ב", // RTL#2
///                     "ג", // RTL#3
///                     "a", // LTR#1
///                     "b", // LTR#2
///                     "c", // LTR#3
///                     ]; //
///
///
/// let bidi_map = CodePointMapData::<BidiClass>::new();
///
/// // Resolve embedding levels within the text.  Pass `None` to detect the
/// // paragraph level automatically.
/// let bidi_info = BidiInfo::new_with_data_source(&bidi_map, text, None);
///
/// // This paragraph has embedding level 1 because its first strong character is RTL.
/// assert_eq!(bidi_info.paragraphs.len(), 1);
/// let para = &bidi_info.paragraphs[0];
/// assert_eq!(para.level.number(), 1);
/// assert!(para.level.is_rtl());
///
/// // Re-ordering is done after wrapping each paragraph into a sequence of
/// // lines. For this example, I'll just use a single line that spans the
/// // entire paragraph.
/// let line = para.range.clone();
///
/// let display = bidi_info.reorder_line(para, line);
/// assert_eq!(display, concat!["a", // LTR#1
///                             "b", // LTR#2
///                             "c", // LTR#3
///                             "ג", // RTL#3
///                             "ב", // RTL#2
///                             "א", // RTL#1
///                             ]);
/// ```
#[cfg(feature = "unicode_bidi")]
impl unicode_bidi::data_source::BidiDataSource
    for crate::CodePointMapDataBorrowed<'_, crate::props::BidiClass>
{
    fn bidi_class(&self, c: char) -> unicode_bidi::BidiClass {
        use crate::props::BidiClass;
        match self.get(c) {
            BidiClass::LeftToRight => unicode_bidi::BidiClass::L,
            BidiClass::RightToLeft => unicode_bidi::BidiClass::R,
            BidiClass::EuropeanNumber => unicode_bidi::BidiClass::EN,
            BidiClass::EuropeanSeparator => unicode_bidi::BidiClass::ES,
            BidiClass::EuropeanTerminator => unicode_bidi::BidiClass::ET,
            BidiClass::ArabicNumber => unicode_bidi::BidiClass::AN,
            BidiClass::CommonSeparator => unicode_bidi::BidiClass::CS,
            BidiClass::ParagraphSeparator => unicode_bidi::BidiClass::B,
            BidiClass::SegmentSeparator => unicode_bidi::BidiClass::S,
            BidiClass::WhiteSpace => unicode_bidi::BidiClass::WS,
            BidiClass::OtherNeutral => unicode_bidi::BidiClass::ON,
            BidiClass::LeftToRightEmbedding => unicode_bidi::BidiClass::LRE,
            BidiClass::LeftToRightOverride => unicode_bidi::BidiClass::LRO,
            BidiClass::ArabicLetter => unicode_bidi::BidiClass::AL,
            BidiClass::RightToLeftEmbedding => unicode_bidi::BidiClass::RLE,
            BidiClass::RightToLeftOverride => unicode_bidi::BidiClass::RLO,
            BidiClass::PopDirectionalFormat => unicode_bidi::BidiClass::PDF,
            BidiClass::NonspacingMark => unicode_bidi::BidiClass::NSM,
            BidiClass::BoundaryNeutral => unicode_bidi::BidiClass::BN,
            BidiClass::FirstStrongIsolate => unicode_bidi::BidiClass::FSI,
            BidiClass::LeftToRightIsolate => unicode_bidi::BidiClass::LRI,
            BidiClass::RightToLeftIsolate => unicode_bidi::BidiClass::RLI,
            BidiClass::PopDirectionalIsolate => unicode_bidi::BidiClass::PDI,
            // This must not happen.
            _ => unicode_bidi::BidiClass::ON,
        }
    }
}
