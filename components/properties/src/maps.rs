// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The functions in this module return a [`CodePointMapData`] representing, for
//! each code point in the entire range of code points, the property values
//! for a particular Unicode property.
//!
//! The descriptions of most properties are taken from [`TR44`], the documentation for the
//! Unicode Character Database.
//!
//! [`TR44`]: https://www.unicode.org/reports/tr44

use crate::error::PropertiesError;
use crate::provider::*;
use crate::sets::CodePointSetData;
#[cfg(doc)]
use crate::*;
use core::marker::PhantomData;
use icu_codepointtrie::{CodePointMapRange, CodePointTrie, TrieValue};
use icu_provider::prelude::*;

/// A wrapper around code point map data. It is returned by APIs that return Unicode
/// property data in a map-like form, ex: enumerated property value data keyed
/// by code point. Access its data via the borrowed version,
/// [`CodePointMapDataBorrowed`].
pub struct CodePointMapData<T: TrieValue> {
    data: DataPayload<ErasedMaplikeMarker<T>>,
}

/// Private marker type for CodePointMapData
/// to work for all same-value map properties at once
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct ErasedMaplikeMarker<T>(PhantomData<T>);
impl<T: TrieValue> DataMarker for ErasedMaplikeMarker<T> {
    type Yokeable = PropertyCodePointMapV1<'static, T>;
}

impl<T: TrieValue> CodePointMapData<T> {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (like `get()`) by consolidating it
    /// up front.
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
    ///
    /// let gc = data.as_borrowed();
    ///
    /// assert_eq!(gc.get('木'), GeneralCategory::OtherLetter);  // U+6728
    /// assert_eq!(gc.get('🎃'), GeneralCategory::OtherSymbol);  // U+1F383 JACK-O-LANTERN
    /// ```
    #[inline]
    pub fn as_borrowed(&self) -> CodePointMapDataBorrowed<'_, T> {
        CodePointMapDataBorrowed {
            map: self.data.get(),
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters like [`get_general_category()`] instead
    pub fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DataMarker<Yokeable = PropertyCodePointMapV1<'static, T>>,
    {
        Self { data: data.cast() }
    }

    /// Construct a new one an owned [`CodePointTrie`]
    pub fn from_code_point_trie(trie: CodePointTrie<'static, T>) -> Self {
        let set = PropertyCodePointMapV1::from_code_point_trie(trie);
        CodePointMapData::from_data(DataPayload::<ErasedMaplikeMarker<T>>::from_owned(set))
    }

    /// Convert this type to a [`CodePointTrie`] as a borrowed value.
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointTrie`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// This method returns an `Option` in order to return `None` when the backing data provider
    /// cannot return a [`CodePointTrie`], or cannot do so within the expected constant time
    /// constraint.
    pub fn as_code_point_trie(&self) -> Option<&CodePointTrie<'_, T>> {
        self.data.get().as_code_point_trie()
    }

    /// Convert this type to a [`CodePointTrie`], borrowing if possible,
    /// otherwise allocating a new [`CodePointTrie`].
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointTrie`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// The performance of the conversion to this specific return type will vary
    /// depending on the data structure that is backing `self`.
    pub fn to_code_point_trie(&self) -> CodePointTrie<'_, T> {
        self.data.get().to_code_point_trie()
    }
}

/// A borrowed wrapper around code point set data, returned by
/// [`CodePointSetData::as_borrowed()`]. More efficient to query.
#[derive(Clone, Copy)]
pub struct CodePointMapDataBorrowed<'a, T: TrieValue> {
    map: &'a PropertyCodePointMapV1<'a, T>,
}

impl<'a, T: TrieValue> CodePointMapDataBorrowed<'a, T> {
    /// Get the value this map has associated with code point `ch`
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
    ///
    /// assert_eq!(gc.get('木'), GeneralCategory::OtherLetter);  // U+6728
    /// assert_eq!(gc.get('🎃'), GeneralCategory::OtherSymbol);  // U+1F383 JACK-O-LANTERN
    /// ```
    pub fn get(&self, ch: char) -> T {
        self.map.get_u32(ch as u32)
    }

    /// Get the value this map has associated with code point `ch`
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
    ///
    /// assert_eq!(gc.get_u32(0x6728), GeneralCategory::OtherLetter);  // U+6728 (木)
    /// assert_eq!(gc.get_u32(0x1F383), GeneralCategory::OtherSymbol);  // U+1F383 JACK-O-LANTERN
    /// ```
    pub fn get_u32(&self, ch: u32) -> T {
        self.map.get_u32(ch)
    }

    /// Get a [`CodePointSetData`] for all elements corresponding to a particular value
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
    ///
    /// let other_letter_set_data = gc.get_set_for_value(GeneralCategory::OtherLetter);
    /// let other_letter_set = other_letter_set_data.as_borrowed();
    ///
    /// assert!(other_letter_set.contains('木'));  // U+6728
    /// assert!(!other_letter_set.contains('🎃'));  // U+1F383 JACK-O-LANTERN
    /// ```
    pub fn get_set_for_value(&self, value: T) -> CodePointSetData {
        let set = self.map.get_set_for_value(value);
        CodePointSetData::from_code_point_inversion_list(set)
    }

    /// Yields an [`Iterator`] returning ranges of consecutive code points that
    /// share the same value in the [`CodePointMapData`].
    ///
    /// # Examples
    ///
    /// ```
    /// use core::ops::RangeInclusive;
    /// use icu::properties::maps::CodePointMapData;
    /// use icu_codepointtrie::CodePointMapRange;
    /// use icu_codepointtrie::planes;
    ///
    /// let planes_trie = planes::get_planes_trie();
    /// let cp_map_data = CodePointMapData::from_code_point_trie(planes_trie);
    /// let cp_map = cp_map_data.as_borrowed();
    ///
    /// let mut ranges = cp_map.iter_ranges();
    ///
    /// for plane in 0..=16 {
    ///     let exp_start = plane * 0x1_0000;
    ///     let exp_end = exp_start + 0xffff;
    ///     assert_eq!(
    ///         ranges.next(),
    ///         Some(CodePointMapRange {
    ///             range: RangeInclusive::new(exp_start, exp_end),
    ///             value: plane as u8
    ///         })
    ///     );
    /// }
    ///
    /// // Hitting the end of the iterator returns `None`, as will subsequent
    /// // calls to .next().
    /// assert_eq!(ranges.next(), None);
    /// assert_eq!(ranges.next(), None);
    /// ```
    pub fn iter_ranges(&self) -> impl Iterator<Item = CodePointMapRange<T>> + '_ {
        self.map.iter_ranges()
    }
}

macro_rules! make_map_property {
    (
        // currently unused
        property: $prop_name:expr;
        // currently unused
        marker: $marker_name:ident;
        value: $value_ty:path;
        keyed_data_marker: $keyed_data_marker:ty;
        func:
        $(#[$attr:meta])*
        $vis:vis fn $name:ident();
    ) => {
        $(#[$attr])*
        $vis fn $name(
            provider: &(impl DataProvider<$keyed_data_marker> + ?Sized)
        ) -> Result<CodePointMapData<$value_ty>, PropertiesError> {
            Ok(provider.load(Default::default()).and_then(DataResponse::take_payload).map(CodePointMapData::from_data)?)
        }
    }
}

make_map_property! {
    property: "General_Category";
    marker: GeneralCategoryProperty;
    value: crate::GeneralCategory;
    keyed_data_marker: GeneralCategoryV1Marker;
    func:
    /// Return a [`CodePointMapData`] for the General_Category Unicode enumerated property. See [`GeneralCategory`].
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
    keyed_data_marker: BidiClassV1Marker;
    func:
    /// Return a [`CodePointMapData`] for the Bidi_Class Unicode enumerated property. See [`BidiClass`].
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
    keyed_data_marker: ScriptV1Marker;
    func:
    /// Return a [`CodePointMapData`] for the Script Unicode enumerated property. See [`Script`].
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
    keyed_data_marker: EastAsianWidthV1Marker;
    func:
    /// Return a [`CodePointMapData`] for the East_Asian_Width Unicode enumerated
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
    keyed_data_marker: LineBreakV1Marker;
    func:
    /// Return a [`CodePointMapData`] for the Line_Break Unicode enumerated
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
    keyed_data_marker: GraphemeClusterBreakV1Marker;
    func:
    /// Return a [`CodePointMapData`] for the Grapheme_Cluster_Break Unicode enumerated
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
    keyed_data_marker: WordBreakV1Marker;
    func:
    /// Return a [`CodePointMapData`] for the Word_Break Unicode enumerated
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
    keyed_data_marker: SentenceBreakV1Marker;
    func:
    /// Return a [`CodePointMapData`] for the Sentence_Break Unicode enumerated
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
    keyed_data_marker: CanonicalCombiningClassV1Marker;
    func:
    /// Return a [`CodePointMapData`] for the Canonical_Combining_Class Unicode property. See
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
