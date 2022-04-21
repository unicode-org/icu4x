// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::props::BidiClass;
use icu_codepointtrie::CodePointTrie;
use unicode_bidi::data_source::BidiDataSource;
use unicode_bidi::BidiClass as DataSourceBidiClass;

/// An adapter to convert from icu4x `BidiClass` to `unicode_bidi::BidiClass`.
pub struct BidiClassAdapter<'a> {
    bidi_trie: &'a CodePointTrie<'a, BidiClass>,
}

#[allow(dead_code)]
impl<'a> BidiClassAdapter<'a> {
    pub fn new(bidi_trie: &'a CodePointTrie<'a, BidiClass>) -> BidiClassAdapter<'a> {
        BidiClassAdapter { bidi_trie }
    }
}

impl<'a> BidiDataSource for BidiClassAdapter<'a> {
    /// Returns a [`DataSourceBidiClass`] given a unicode character.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::BidiClassAdapter;
    /// use icu::properties::{maps, BidiClass};
    /// use icu_codepointtrie::CodePointTrie;
    /// use unicode_bidi::BidiClass as DataSourceBidiClass;
    /// use unicode_bidi::BidiDataSource;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let payload =
    ///     maps::get_bidi_class(&provider)
    ///         .expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let bc = &data_struct.code_point_trie;
    ///
    /// let adapter = BidiClassAdapter::new(&bc);
    /// assert_eq!(adapter.bidi_class('a'), DataSourceBidiClass::L);
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    fn bidi_class(&self, c: char) -> DataSourceBidiClass {
        let bidi_class = self.bidi_trie.get(c as u32);
        match bidi_class {
            BidiClass::LeftToRight => DataSourceBidiClass::L,
            BidiClass::RightToLeft => DataSourceBidiClass::R,
            BidiClass::EuropeanNumber => DataSourceBidiClass::EN,
            BidiClass::EuropeanSeparator => DataSourceBidiClass::ES,
            BidiClass::EuropeanTerminator => DataSourceBidiClass::ET,
            BidiClass::ArabicNumber => DataSourceBidiClass::AN,
            BidiClass::CommonSeparator => DataSourceBidiClass::CS,
            BidiClass::ParagraphSeparator => DataSourceBidiClass::B,
            BidiClass::SegmentSeparator => DataSourceBidiClass::S,
            BidiClass::WhiteSpace => DataSourceBidiClass::WS,
            BidiClass::OtherNeutral => DataSourceBidiClass::ON,
            BidiClass::LeftToRightEmbedding => DataSourceBidiClass::LRE,
            BidiClass::LeftToRightOverride => DataSourceBidiClass::LRO,
            BidiClass::ArabicLetter => DataSourceBidiClass::AL,
            BidiClass::RightToLeftEmbedding => DataSourceBidiClass::RLE,
            BidiClass::RightToLeftOverride => DataSourceBidiClass::RLO,
            BidiClass::PopDirectionalFormat => DataSourceBidiClass::PDF,
            BidiClass::NonspacingMark => DataSourceBidiClass::NSM,
            BidiClass::BoundaryNeutral => DataSourceBidiClass::BN,
            BidiClass::FirstStrongIsolate => DataSourceBidiClass::FSI,
            BidiClass::LeftToRightIsolate => DataSourceBidiClass::LRI,
            BidiClass::RightToLeftIsolate => DataSourceBidiClass::RLI,
            BidiClass::PopDirectionalIsolate => DataSourceBidiClass::PDI,
            _ => {
                // we need to log
                //eprintln!(
                //     "this must not happen, this means this value: {:?} is not supported",
                //     bidi_class
                // );
                DataSourceBidiClass::L
            }
        }
    }
}
