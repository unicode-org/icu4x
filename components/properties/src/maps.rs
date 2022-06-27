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
use crate::sets::{CodePointSetData, ErasedSetlikeMarker};
#[cfg(doc)]
use crate::*;
use core::marker::PhantomData;
use core::ops::RangeInclusive;
use icu_codepointtrie::{CodePointMapRange, TrieValue};
use icu_provider::prelude::*;

/// A wrapper around code point set data, returned by property getters for
/// unicode sets.
pub struct CodePointMapData<T: TrieValue> {
    data: DataPayload<ErasedMaplikeMarker<T>>,
}

/// Private marker type for CodePointMapData
/// to work for all same-value map properties at once
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct ErasedMaplikeMarker<T>(PhantomData<T>);
impl<T: TrieValue> DataMarker for ErasedMaplikeMarker<T> {
    type Yokeable = UnicodePropertyMapV1<'static, T>;
}

impl<T: TrieValue> CodePointMapData<T> {
    /// Construct a borrowed version of this type that can be queried
    ///
    /// This avoids a potential small cost per [`Self::get()`] call by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> CodePointMapDataBorrowed<'_, T> {
        CodePointMapDataBorrowed {
            map: self.data.get(),
        }
    }

    /// Get the value this map has associated with code point `ch`
    pub fn get(&self, ch: char) -> T {
        self.data.get().code_point_trie.get(ch as u32)
    }

    /// Get the value this map has associated with code point `ch`
    pub fn get_u32(&self, ch: u32) -> T {
        self.data.get().code_point_trie.get(ch)
    }

    /// Returns an iterator over ranges of code points with the same value
    pub fn iter_ranges(&self) -> impl Iterator<Item = CodePointMapRange<T>> + '_ {
        self.data.get().code_point_trie.iter_ranges()
    }

    /// Get an iterator over all map ranges for a particular value
    pub fn get_ranges_for_value(&self, value: T) -> impl Iterator<Item = RangeInclusive<u32>> + '_ {
        self.data.get().code_point_trie.get_ranges_for_value(value)
    }

    /// Get a [`CodePointSetData`] for all elements corresponding to a particular value
    pub fn get_set_for_value(&self, value: T) -> CodePointSetData {
        let set = self.data.get().code_point_trie.get_set_for_value(value);
        let set = UnicodePropertyV1 { inv_list: set };
        CodePointSetData::from_data(DataPayload::<ErasedSetlikeMarker>::from_owned(set))
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters like [`get_general_category()`] instead
    pub fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DataMarker<Yokeable = UnicodePropertyMapV1<'static, T>>,
    {
        Self {
            data: data.map_project(|m, _| m),
        }
    }
}

/// A borrowed wrapper around code point set data, returned by
/// [`CodePointSetData::as_borrowed()`]. More efficient to query.
#[derive(Clone, Copy)]
pub struct CodePointMapDataBorrowed<'a, T: TrieValue> {
    map: &'a UnicodePropertyMapV1<'a, T>,
}

impl<'a, T: TrieValue> CodePointMapDataBorrowed<'a, T> {
    /// Get the value this map has associated with code point `ch`
    pub fn get(&self, ch: char) -> T {
        self.map.code_point_trie.get(ch as u32)
    }

    /// Get the value this map has associated with code point `ch`
    pub fn get_u32(&self, ch: u32) -> T {
        self.map.code_point_trie.get(ch)
    }

    /// Returns an iterator over ranges of code points with the same value
    pub fn iter_ranges(&self) -> impl Iterator<Item = CodePointMapRange<T>> + '_ {
        self.map.code_point_trie.iter_ranges()
    }

    /// Get an iterator over all map ranges for a particular value
    pub fn get_ranges_for_value(&self, value: T) -> impl Iterator<Item = RangeInclusive<u32>> + '_ {
        self.map.code_point_trie.get_ranges_for_value(value)
    }

    /// Get a [`CodePointSetData`] for all elements corresponding to a particular value
    pub fn get_set_for_value(&self, value: T) -> CodePointSetData {
        let set = self.map.code_point_trie.get_set_for_value(value);
        let set = UnicodePropertyV1 { inv_list: set };
        CodePointSetData::from_data(DataPayload::<ErasedSetlikeMarker>::from_owned(set))
    }
}

macro_rules! make_map_property {
    (
        // currently unused
        property: $prop_name:expr;
        // currently unused
        marker: $marker_name:ident;
        value: $value_ty:path;
        resource_marker: $resource_marker:ty;
        func:
        $(#[$attr:meta])*
        $vis:vis fn $name:ident();
    ) => {
        $(#[$attr])*
        $vis fn $name(
            provider: &(impl ResourceProvider<$resource_marker> + ?Sized)
        ) -> Result<CodePointMapData<$value_ty>, PropertiesError> {
            Ok(provider.load_resource(&Default::default()).and_then(DataResponse::take_payload).map(CodePointMapData::from_data)?)
        }
    }
}

make_map_property! {
    property: "General_Category";
    marker: GeneralCategoryProperty;
    value: crate::GeneralCategory;
    resource_marker: GeneralCategoryV1Marker;
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
    /// let data =
    ///     maps::get_general_category(&provider)
    ///         .expect("The data should be valid");
    /// let gc = data.as_borrowed();
    /// assert_eq!(gc.get('木'), GeneralCategory::OtherLetter);  // U+6728
    /// assert_eq!(gc.get('🎃'), GeneralCategory::OtherSymbol);  // U+1F383 JACK-O-LANTERN
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_general_category();
}

make_map_property! {
    property: "Bidi_Class";
    marker: BidiClassProperty;
    value: crate::BidiClass;
    resource_marker: BidiClassV1Marker;
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
    /// let data =
    ///     maps::get_bidi_class(&provider)
    ///         .expect("The data should be valid");
    /// let bc = data.as_borrowed();
    /// assert_eq!(bc.get('y'), BidiClass::LeftToRight);  // U+0079
    /// assert_eq!(bc.get('ع'), BidiClass::ArabicLetter);  // U+0639
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_bidi_class();
}

make_map_property! {
    property: "Script";
    marker: ScriptProperty;
    value: crate::Script;
    resource_marker: ScriptV1Marker;
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
    /// let data =
    ///     maps::get_script(&provider)
    ///         .expect("The data should be valid");
    /// let script = data.as_borrowed();
    /// assert_eq!(script.get('木'), Script::Han);  // U+6728
    /// assert_eq!(script.get('🎃'), Script::Common);  // U+1F383 JACK-O-LANTERN
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_script();
}

make_map_property! {
    property: "East_Asian_Width";
    marker: EastAsianWidthProperty;
    value: crate::EastAsianWidth;
    resource_marker: EastAsianWidthV1Marker;
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
    /// let data = maps::get_east_asian_width(&provider).expect("The data should be valid!");
    /// let eaw = data.as_borrowed();;
    ///
    /// assert_eq!(eaw.get('ｱ'), EastAsianWidth::Halfwidth); // U+FF71: Halfwidth Katakana Letter A
    /// assert_eq!(eaw.get('ア'), EastAsianWidth::Wide); //U+30A2: Katakana Letter A
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_east_asian_width();
}

make_map_property! {
    property: "Line_Break";
    marker: LineBreakProperty;
    value: crate::LineBreak;
    resource_marker: LineBreakV1Marker;
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
    /// let data = maps::get_line_break(&provider).expect("The data should be valid!");
    /// let lb = data.as_borrowed();
    ///
    /// assert_eq!(lb.get(')'), LineBreak::CloseParenthesis); // U+0029: Right Parenthesis
    /// assert_eq!(lb.get('ぁ'), LineBreak::ConditionalJapaneseStarter); //U+3041: Hiragana Letter Small A
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_line_break();
}

make_map_property! {
    property: "Grapheme_Cluster_Break";
    marker: GraphemeClusterBreakProperty;
    value: crate::GraphemeClusterBreak;
    resource_marker: GraphemeClusterBreakV1Marker;
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
    /// let data = maps::get_grapheme_cluster_break(&provider).expect("The data should be valid!");
    /// let gcb = data.as_borrowed();
    ///
    /// assert_eq!(gcb.get('🇦'), GraphemeClusterBreak::RegionalIndicator); // U+1F1E6: Regional Indicator Symbol Letter A
    /// assert_eq!(gcb.get('ำ'), GraphemeClusterBreak::SpacingMark); //U+0E33: Thai Character Sara Am
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_grapheme_cluster_break();
}

make_map_property! {
    property: "Word_Break";
    marker: WordBreakProperty;
    value: crate::WordBreak;
    resource_marker: WordBreakV1Marker;
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
    /// let data = maps::get_word_break(&provider).expect("The data should be valid!");
    /// let wb = data.as_borrowed();
    ///
    /// assert_eq!(wb.get('.'), WordBreak::MidNumLet); // U+002E: Full Stop
    /// assert_eq!(wb.get('，'), WordBreak::MidNum); // U+FF0C: Fullwidth Comma
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_word_break();
}

make_map_property! {
    property: "Sentence_Break";
    marker: SentenceBreakProperty;
    value: crate::SentenceBreak;
    resource_marker: SentenceBreakV1Marker;
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
    /// let data = maps::get_sentence_break(&provider).expect("The data should be valid!");
    /// let sb = data.as_borrowed();;
    ///
    /// assert_eq!(sb.get('９'), SentenceBreak::Numeric); // U+FF19: Fullwidth Digit Nine
    /// assert_eq!(sb.get(','), SentenceBreak::SContinue); // U+002C: Comma
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_sentence_break();
}

make_map_property! {
    property: "Canonical_Combining_Class";
    marker: CanonicalCombiningClassProperty;
    value: crate::CanonicalCombiningClass;
    resource_marker: CanonicalCombiningClassV1Marker;
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
    /// let data = maps::get_canonical_combining_class(&provider).expect("The data should be valid!");
    /// let sb = data.as_borrowed();;
    ///
    /// assert_eq!(sb.get('a'), CanonicalCombiningClass::NotReordered); // U+0061: LATIN SMALL LETTER A
    /// assert_eq!(sb.get_u32(0x0301), CanonicalCombiningClass::Above); // U+0301: COMBINING ACUTE ACCENT
    /// ```
    ///
    /// [`CodePointTrie`]: icu_codepointtrie::CodePointTrie
    pub fn get_canonical_combining_class();
}
