// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{props::EnumeratedProperty, provider::PropertyEnumBidiMirroringGlyphV1};
use icu_collections::codepointtrie::TrieValue;
use zerovec::ule::{AsULE, RawBytesULE};

/// This is a bitpacked combination of the `Bidi_Mirroring_Glyph`,
/// `Bidi_Mirrored`, and `Bidi_Paired_Bracket_Type` properties.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::props))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // needed for baked construction
pub struct BidiMirroringGlyph {
    /// The mirroring glyph
    pub mirroring_glyph: Option<char>,
    /// Whether the glyph is mirrored
    pub mirrored: bool,
    /// The paired bracket type
    pub paired_bracket_type: BidiPairedBracketType,
}

impl EnumeratedProperty for BidiMirroringGlyph {
    type DataMarker = PropertyEnumBidiMirroringGlyphV1;
    #[cfg(feature = "compiled_data")]
    const SINGLETON: &'static crate::provider::PropertyCodePointMap<'static, Self> =
        crate::provider::Baked::SINGLETON_PROPERTY_ENUM_BIDI_MIRRORING_GLYPH_V1;
    const NAME: &'static [u8] = b"Bidi_Mirroring_Glyph";
    const SHORT_NAME: &'static [u8] = b"bmg";
}

impl crate::private::Sealed for BidiMirroringGlyph {}

impl AsULE for BidiMirroringGlyph {
    type ULE = zerovec::ule::RawBytesULE<3>;

    fn to_unaligned(self) -> Self::ULE {
        let [a, b, c, _] = TrieValue::to_u32(self).to_le_bytes();
        RawBytesULE([a, b, c])
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let [a, b, c] = unaligned.0;
        TrieValue::try_from_u32(u32::from_le_bytes([a, b, c, 0])).unwrap_or_default()
    }
}

/// The enum represents `Bidi_Paired_Bracket_Type`.
///
/// It does not implement [`EnumeratedProperty`], instead it can be obtained
/// through the bitpacked [`BidiMirroringGlyph`] property.
///
/// If you have a use case this property without also needing the [`BidiMirroringGlyph`]
/// property, and need to optimize data size, please file an issue.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::props))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum BidiPairedBracketType {
    /// Represents `Bidi_Paired_Bracket_Type=Open`.
    Open,
    /// Represents `Bidi_Paired_Bracket_Type=Close`.
    Close,
    /// Represents `Bidi_Paired_Bracket_Type=None`.
    #[default]
    None,
}

#[cfg(feature = "unicode_bidi")]
use crate::props::BidiClass;

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
impl unicode_bidi::data_source::BidiDataSource for crate::CodePointMapDataBorrowed<'_, BidiClass> {
    fn bidi_class(&self, c: char) -> unicode_bidi::BidiClass {
        self.get(c).into()
    }
}

#[cfg(feature = "unicode_bidi")]
impl From<BidiClass> for unicode_bidi::BidiClass {
    fn from(value: BidiClass) -> Self {
        match value {
            BidiClass::LeftToRight => Self::L,
            BidiClass::RightToLeft => Self::R,
            BidiClass::EuropeanNumber => Self::EN,
            BidiClass::EuropeanSeparator => Self::ES,
            BidiClass::EuropeanTerminator => Self::ET,
            BidiClass::ArabicNumber => Self::AN,
            BidiClass::CommonSeparator => Self::CS,
            BidiClass::ParagraphSeparator => Self::B,
            BidiClass::SegmentSeparator => Self::S,
            BidiClass::WhiteSpace => Self::WS,
            BidiClass::OtherNeutral => Self::ON,
            BidiClass::LeftToRightEmbedding => Self::LRE,
            BidiClass::LeftToRightOverride => Self::LRO,
            BidiClass::ArabicLetter => Self::AL,
            BidiClass::RightToLeftEmbedding => Self::RLE,
            BidiClass::RightToLeftOverride => Self::RLO,
            BidiClass::PopDirectionalFormat => Self::PDF,
            BidiClass::NonspacingMark => Self::NSM,
            BidiClass::BoundaryNeutral => Self::BN,
            BidiClass::FirstStrongIsolate => Self::FSI,
            BidiClass::LeftToRightIsolate => Self::LRI,
            BidiClass::RightToLeftIsolate => Self::RLI,
            BidiClass::PopDirectionalIsolate => Self::PDI,
            // This must not happen.
            _ => Self::ON,
        }
    }
}

#[cfg(feature = "unicode_bidi")]
impl From<unicode_bidi::BidiClass> for BidiClass {
    fn from(value: unicode_bidi::BidiClass) -> Self {
        match value {
            unicode_bidi::BidiClass::L => Self::LeftToRight,
            unicode_bidi::BidiClass::R => Self::RightToLeft,
            unicode_bidi::BidiClass::EN => Self::EuropeanNumber,
            unicode_bidi::BidiClass::ES => Self::EuropeanSeparator,
            unicode_bidi::BidiClass::ET => Self::EuropeanTerminator,
            unicode_bidi::BidiClass::AN => Self::ArabicNumber,
            unicode_bidi::BidiClass::CS => Self::CommonSeparator,
            unicode_bidi::BidiClass::B => Self::ParagraphSeparator,
            unicode_bidi::BidiClass::S => Self::SegmentSeparator,
            unicode_bidi::BidiClass::WS => Self::WhiteSpace,
            unicode_bidi::BidiClass::ON => Self::OtherNeutral,
            unicode_bidi::BidiClass::LRE => Self::LeftToRightEmbedding,
            unicode_bidi::BidiClass::LRO => Self::LeftToRightOverride,
            unicode_bidi::BidiClass::AL => Self::ArabicLetter,
            unicode_bidi::BidiClass::RLE => Self::RightToLeftEmbedding,
            unicode_bidi::BidiClass::RLO => Self::RightToLeftOverride,
            unicode_bidi::BidiClass::PDF => Self::PopDirectionalFormat,
            unicode_bidi::BidiClass::NSM => Self::NonspacingMark,
            unicode_bidi::BidiClass::BN => Self::BoundaryNeutral,
            unicode_bidi::BidiClass::FSI => Self::FirstStrongIsolate,
            unicode_bidi::BidiClass::LRI => Self::LeftToRightIsolate,
            unicode_bidi::BidiClass::RLI => Self::RightToLeftIsolate,
            unicode_bidi::BidiClass::PDI => Self::PopDirectionalIsolate,
        }
    }
}
