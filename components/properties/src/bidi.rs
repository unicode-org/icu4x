// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::props::BidiClass;
use crate::CodePointMapDataBorrowed;
use unicode_bidi::data_source::BidiDataSource;

/// Implements [`unicode_bidi::BidiDataSource`] on [`CodePointMapDataBorrowed<BidiClass>`].
///
/// ✨ *Enabled with the `bidi` Cargo feature.*
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
impl BidiDataSource for CodePointMapDataBorrowed<'_, BidiClass> {
    fn bidi_class(&self, c: char) -> unicode_bidi::BidiClass {
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
