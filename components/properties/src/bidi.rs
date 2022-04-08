// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use unicode_bidi::data_source::BidiDataSource;
use unicode_bidi::BidiClass as DataSourceBidiClass;
use crate::maps;
use icu_codepointtrie::CodePointTrie;
use crate::props::BidiClass;


pub struct BidiClassAdapter {
	bidi_trie: CodePointTrie,
}

impl BidiClassAdapter {
	pub fn new() -> BidiClassAdapter {
		 let provider = icu_testdata::get_provider();
		 let payload =
		     maps::get_bidi_class(&provider)
		         .expect("The data should be valid");
		 let code_point_trie = payload.get().code_point_trie;
		 BidiClassAdapter{ code_point_trie }
	}
}


impl BidiDataSource for BidiClassAdapter {

    /// Returns a [`DataSourceBidiClass`] given a unicode character.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::BidiClassAdapter;
    /// use crate::BidiClass as DataSourceBidiClass
    ///
    /// let Adapter = BidiClassAdapter::new();
    /// assert_eq!(Adapter.bidi_class('a' as u32), DataSourceBidiClass::R);  // U
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
		}
	}

}