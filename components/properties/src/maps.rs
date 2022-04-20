// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The functions in this module return a [`CodePointTrie`] representing, for
//! each code point in the entire range of code points, the property values
//! for a particular Unicode property.
//!
//! The descriptions of most properties are taken from [`TR44`], the documentation for the
//! Unicode Character Database.
//!
//! [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
//! [`TR44`]: https://www.unicode.org/reports/tr44

use crate::error::PropertiesError;
use crate::provider::*;
use crate::*;
use BidiClass;
use icu_codepointtrie::TrieValue;
use icu_provider::prelude::*;

/// TODO(#1239): Finalize this API.
pub type CodePointMapResult<T> =
    Result<DataPayload<UnicodePropertyMapV1Marker<T>>, PropertiesError>;

fn get_cp_map<D, T>(provider: &D, key: ResourceKey) -> CodePointMapResult<T>
where
    D: DynProvider<UnicodePropertyMapV1Marker<T>> + ?Sized,
    T: TrieValue,
{
    let resp: DataResponse<UnicodePropertyMapV1Marker<T>> =
        provider.load_payload(key, &Default::default())?;

    let property_payload: DataPayload<UnicodePropertyMapV1Marker<T>> = resp.take_payload()?;
    Ok(property_payload)
}

macro_rules! make_map_property {
    (
        // currently unused
        property: $prop_name:expr;
        // currently unused
        marker: $marker_name:ident;
        value: $value_ty:path;
        key: $key_name:expr;
        func:
        $(#[$attr:meta])*
        pub fn $funcname:ident();
    ) => {
        $(#[$attr])*
        pub fn $funcname<D>(provider: &D) -> CodePointMapResult<$value_ty>
        where
            D: DynProvider<UnicodePropertyMapV1Marker<$value_ty>> + ?Sized,
        {
            get_cp_map(provider, $key_name)
        }
    }
}

make_map_property! {
    property: "General_Category";
    marker: GeneralCategoryProperty;
    value: GeneralCategory;
    key: crate::provider::key::GENERAL_CATEGORY_V1;
    func:
    /// Return a [`CodePointTrie`] for the General_Category Unicode enumerated property. See [`GeneralCategory`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{maps, GeneralCategory};
    /// use icu_codepointtrie::CodePointTrie;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let payload =
    ///     maps::get_general_category(&provider)
    ///         .expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let gc = &data_struct.code_point_trie;
    /// assert_eq!(gc.get('木' as u32), GeneralCategory::OtherLetter);  // U+6728
    /// assert_eq!(gc.get('🎃' as u32), GeneralCategory::OtherSymbol);  // U+1F383 JACK-O-LANTERN
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_general_category();
}

make_map_property! {
    property: "Bidi_Class";
    marker: BidiClassProperty;
    value: BidiClass;
    key: crate::provider::key::BIDI_CLASS;
    func:
    /// Return a [`CodePointTrie`] for the Bidi_Class Unicode enumerated property. See [`BidiClass`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{maps, BidiClass};
    /// use icu_codepointtrie::CodePointTrie;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let payload =
    ///     maps::get_bidi_class(&provider)
    ///         .expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let bc = &data_struct.code_point_trie;
    /// assert_eq!(bc.get('y' as u32), BidiClass::LeftToRight);  // U+0079
    /// assert_eq!(bc.get('ع' as u32), BidiClass::ArabicLetter);  // U+0639
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_bidi_class();
}

make_map_property! {
    property: "Script";
    marker: ScriptProperty;
    value: Script;
    key: crate::provider::key::SCRIPT_V1;
    func:
    /// Return a [`CodePointTrie`] for the Script Unicode enumerated property. See [`Script`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{maps, Script};
    /// use icu_codepointtrie::CodePointTrie;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let payload =
    ///     maps::get_script(&provider)
    ///         .expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let script = &data_struct.code_point_trie;
    /// assert_eq!(script.get('木' as u32), Script::Han);  // U+6728
    /// assert_eq!(script.get('🎃' as u32), Script::Common);  // U+1F383 JACK-O-LANTERN
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_script();
}

make_map_property! {
    property: "East_Asian_Width";
    marker: EastAsianWidthProperty;
    value: EastAsianWidth;
    key: crate::provider::key::EAST_ASIAN_WIDTH_V1;
    func:
    /// Return a [`CodePointTrie`] for the East_Asian_Width Unicode enumerated
    /// property. See [`EastAsianWidth`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{maps, EastAsianWidth};
    ///
    /// let provider = icu_testdata::get_provider();
    /// let payload = maps::get_east_asian_width(&provider).expect("The data should be valid!");
    /// let eaw = &payload.get().code_point_trie;
    ///
    /// assert_eq!(eaw.get('ｱ' as u32), EastAsianWidth::Halfwidth); // U+FF71: Halfwidth Katakana Letter A
    /// assert_eq!(eaw.get('ア' as u32), EastAsianWidth::Wide); //U+30A2: Katakana Letter A
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_east_asian_width();
}

make_map_property! {
    property: "Line_Break";
    marker: LineBreakProperty;
    value: LineBreak;
    key: crate::provider::key::LINE_BREAK_V1;
    func:
    /// Return a [`CodePointTrie`] for the Line_Break Unicode enumerated
    /// property. See [`LineBreak`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{maps, LineBreak};
    ///
    /// let provider = icu_testdata::get_provider();
    /// let payload = maps::get_line_break(&provider).expect("The data should be valid!");
    /// let lb = &payload.get().code_point_trie;
    ///
    /// assert_eq!(lb.get(')' as u32), LineBreak::CloseParenthesis); // U+0029: Right Parenthesis
    /// assert_eq!(lb.get('ぁ' as u32), LineBreak::ConditionalJapaneseStarter); //U+3041: Hiragana Letter Small A
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_line_break();
}

make_map_property! {
    property: "Grapheme_Cluster_Break";
    marker: GraphemeClusterBreakProperty;
    value: GraphemeClusterBreak;
    key: crate::provider::key::GRAPHEME_CLUSTER_BREAK_V1;
    func:
    /// Return a [`CodePointTrie`] for the Grapheme_Cluster_Break Unicode enumerated
    /// property. See [`GraphemeClusterBreak`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{maps, GraphemeClusterBreak};
    ///
    /// let provider = icu_testdata::get_provider();
    /// let payload = maps::get_grapheme_cluster_break(&provider).expect("The data should be valid!");
    /// let gcb = &payload.get().code_point_trie;
    ///
    /// assert_eq!(gcb.get('🇦' as u32), GraphemeClusterBreak::RegionalIndicator); // U+1F1E6: Regional Indicator Symbol Letter A
    /// assert_eq!(gcb.get('ำ' as u32), GraphemeClusterBreak::SpacingMark); //U+0E33: Thai Character Sara Am
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_grapheme_cluster_break();
}

make_map_property! {
    property: "Word_Break";
    marker: WordBreakProperty;
    value: WordBreak;
    key: crate::provider::key::WORD_BREAK_V1;
    func:
    /// Return a [`CodePointTrie`] for the Word_Break Unicode enumerated
    /// property. See [`WordBreak`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{maps, WordBreak};
    ///
    /// let provider = icu_testdata::get_provider();
    /// let payload = maps::get_word_break(&provider).expect("The data should be valid!");
    /// let wb = &payload.get().code_point_trie;
    ///
    /// assert_eq!(wb.get('.' as u32), WordBreak::MidNumLet); // U+002E: Full Stop
    /// assert_eq!(wb.get('，' as u32), WordBreak::MidNum); // U+FF0C: Fullwidth Comma
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_word_break();
}

make_map_property! {
    property: "Sentence_Break";
    marker: SentenceBreakProperty;
    value: SentenceBreak;
    key: crate::provider::key::SENTENCE_BREAK_V1;
    func:
    /// Return a [`CodePointTrie`] for the Sentence_Break Unicode enumerated
    /// property. See [`SentenceBreak`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{maps, SentenceBreak};
    ///
    /// let provider = icu_testdata::get_provider();
    /// let payload = maps::get_sentence_break(&provider).expect("The data should be valid!");
    /// let sb = &payload.get().code_point_trie;
    ///
    /// assert_eq!(sb.get('９' as u32), SentenceBreak::Numeric); // U+FF19: Fullwidth Digit Nine
    /// assert_eq!(sb.get(',' as u32), SentenceBreak::SContinue); // U+002C: Comma
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_sentence_break();
}

make_map_property! {
    property: "Canonical_Combining_Class";
    marker: CanonicalCombiningClassProperty;
    value: CanonicalCombiningClass;
    key: crate::provider::key::CANONICAL_COMBINING_CLASS_V1;
    func:
    /// Return a [`CodePointTrie`] for the Canonical_Combining_Class Unicode property. See
    /// [`CanonicalCombiningClass`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{maps, CanonicalCombiningClass};
    ///
    /// let provider = icu_testdata::get_provider();
    /// let payload = maps::get_canonical_combining_class(&provider).expect("The data should be valid!");
    /// let sb = &payload.get().code_point_trie;
    ///
    /// assert_eq!(sb.get('a' as u32), CanonicalCombiningClass::NotReordered); // U+0061: LATIN SMALL LETTER A
    /// assert_eq!(sb.get(0x0301), CanonicalCombiningClass::Above); // U+0301: COMBINING ACUTE ACCENT
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_canonical_combining_class();
}
