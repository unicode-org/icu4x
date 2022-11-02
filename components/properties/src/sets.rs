// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The functions in this module return a [`CodePointSetData`] containing
//! the set of characters with a particular Unicode property.
//!
//! The descriptions of most properties are taken from [`TR44`], the documentation for the
//! Unicode Character Database.  Some properties are instead defined in [`TR18`], the
//! documentation for Unicode regular expressions. In particular, Annex C of this document
//! defines properties for POSIX compatibility.
//!
//! [`CodePointSetData`]: crate::sets::CodePointSetData
//! [`TR44`]: https://www.unicode.org/reports/tr44
//! [`TR18`]: https://www.unicode.org/reports/tr18

use crate::error::PropertiesError;
use crate::provider::*;
use crate::*;
use core::iter::FromIterator;
use core::ops::RangeInclusive;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_collections::codepointinvliststringlist::CodePointInversionListStringList;
use icu_provider::prelude::*;

//
// CodePointSet* structs, impls, & macros
// (a set with only code points)
//

/// A wrapper around code point set data. It is returned by APIs that return Unicode
/// property data in a set-like form, ex: a set of code points sharing the same
/// value for a Unicode property. Access its data via the borrowed version,
/// [`CodePointSetDataBorrowed`].
pub struct CodePointSetData {
    data: DataPayload<ErasedSetlikeMarker>,
}

/// Private marker type for CodePointSetData
/// to work for all set properties at once
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct ErasedSetlikeMarker;
impl DataMarker for ErasedSetlikeMarker {
    type Yokeable = PropertyCodePointSetV1<'static>;
}

impl CodePointSetData {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (ex: `contains()`) by consolidating it
    /// up front.
    ///
    /// ```rust
    /// use icu_properties::sets;
    ///
    /// let data = sets::load_alphabetic(&icu_testdata::unstable())
    ///     .expect("The data should be valid");
    ///
    /// let alphabetic = data.as_borrowed();
    ///
    /// assert!(!alphabetic.contains('3'));
    /// assert!(alphabetic.contains('A'));
    /// ```
    #[inline]
    pub fn as_borrowed(&self) -> CodePointSetDataBorrowed<'_> {
        CodePointSetDataBorrowed {
            set: self.data.get(),
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters like [`load_ascii_hex_digit()`] instead
    pub fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DataMarker<Yokeable = PropertyCodePointSetV1<'static>>,
    {
        Self {
            data: data.map_project(|m, _| m),
        }
    }

    /// Construct a new owned [`CodePointInversionList`]
    pub fn from_code_point_inversion_list(set: CodePointInversionList<'static>) -> Self {
        let set = PropertyCodePointSetV1::from_code_point_inversion_list(set);
        CodePointSetData::from_data(DataPayload::<ErasedSetlikeMarker>::from_owned(set))
    }

    /// Convert this type to a [`CodePointInversionList`] as a borrowed value.
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointInversionList`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// This method returns an `Option` in order to return `None` when the backing data provider
    /// cannot return a [`CodePointInversionList`], or cannot do so within the expected constant time
    /// constraint.
    pub fn as_code_point_inversion_list(&self) -> Option<&CodePointInversionList<'_>> {
        self.data.get().as_code_point_inversion_list()
    }

    /// Convert this type to a [`CodePointInversionList`], borrowing if possible,
    /// otherwise allocating a new [`CodePointInversionList`].
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointInversionList`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// The performance of the conversion to this specific return type will vary
    /// depending on the data structure that is backing `self`.
    pub fn to_code_point_inversion_list(&self) -> CodePointInversionList<'_> {
        self.data.get().to_code_point_inversion_list()
    }
}

/// A borrowed wrapper around code point set data, returned by
/// [`CodePointSetData::as_borrowed()`]. More efficient to query.
#[derive(Clone, Copy)]
pub struct CodePointSetDataBorrowed<'a> {
    set: &'a PropertyCodePointSetV1<'a>,
}

impl<'a> CodePointSetDataBorrowed<'a> {
    /// Check if the set contains a character
    ///
    /// ```rust
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_alphabetic(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let alphabetic = data.as_borrowed();
    ///
    /// assert!(!alphabetic.contains('3'));
    /// assert!(!alphabetic.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(alphabetic.contains('A'));
    /// assert!(alphabetic.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```
    #[inline]
    pub fn contains(self, ch: char) -> bool {
        self.set.contains(ch)
    }

    /// Check if the set contains a character as a UTF32 code unit
    ///
    /// ```rust
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_alphabetic(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let alphabetic = data.as_borrowed();
    ///
    /// assert!(!alphabetic.contains32(0x0A69));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(alphabetic.contains32(0x00C4));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```
    #[inline]
    pub fn contains32(self, ch: u32) -> bool {
        self.set.contains32(ch)
    }

    // Yields an [`Iterator`] returning the ranges of the code points that are
    /// included in the [`CodePointSetData`]
    ///
    /// Ranges are returned as [`RangeInclusive`], which is inclusive of its
    /// `end` bound value. An end-inclusive behavior matches the ICU4C/J
    /// behavior of ranges, ex: `UnicodeSet::contains(UChar32 start, UChar32 end)`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data = sets::load_alphabetic(&icu_testdata::unstable())
    ///     .expect("The data should be valid");
    /// let alphabetic = data.as_borrowed();
    /// let mut ranges = alphabetic.iter_ranges();
    ///
    /// assert_eq!(Some(0x0041..=0x005A), ranges.next()); // 'A'..'Z'
    /// assert_eq!(Some(0x0061..=0x007A), ranges.next()); // 'a'..'z'
    /// ```
    #[inline]
    pub fn iter_ranges(self) -> impl Iterator<Item = RangeInclusive<u32>> + 'a {
        self.set.iter_ranges()
    }
}

//
// UnicodeSet* structs, impls, & macros
// (a set with code points + strings)
//

/// A wrapper around `UnicodeSet` data (characters and strings)
pub struct UnicodeSetData {
    data: DataPayload<ErasedUnicodeSetlikeMarker>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct ErasedUnicodeSetlikeMarker;
impl DataMarker for ErasedUnicodeSetlikeMarker {
    type Yokeable = PropertyUnicodeSetV1<'static>;
}

impl UnicodeSetData {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (ex: `contains()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> UnicodeSetDataBorrowed<'_> {
        UnicodeSetDataBorrowed {
            set: self.data.get(),
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters instead
    pub fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DataMarker<Yokeable = PropertyUnicodeSetV1<'static>>,
    {
        Self {
            data: data.map_project(|m, _| m),
        }
    }

    /// Construct a new owned [`CodePointInversionListStringList`]
    pub fn from_code_point_inversion_list_string_list(
        set: CodePointInversionListStringList<'static>,
    ) -> Self {
        let set = PropertyUnicodeSetV1::from_code_point_inversion_list_string_list(set);
        UnicodeSetData::from_data(DataPayload::<ErasedUnicodeSetlikeMarker>::from_owned(set))
    }

    /// Convert this type to a [`CodePointInversionListStringList`] as a borrowed value.
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointInversionListStringList`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// This method returns an `Option` in order to return `None` when the backing data provider
    /// cannot return a [`CodePointInversionListStringList`], or cannot do so within the expected constant time
    /// constraint.
    pub fn as_code_point_inversion_list_string_list(
        &self,
    ) -> Option<&CodePointInversionListStringList<'_>> {
        self.data.get().as_code_point_inversion_list_string_list()
    }

    /// Convert this type to a [`CodePointInversionListStringList`], borrowing if possible,
    /// otherwise allocating a new [`CodePointInversionListStringList`].
    ///
    /// The data backing this is extensible and supports multiple implementations.
    /// Currently it is always [`CodePointInversionListStringList`]; however in the future more backends may be
    /// added, and users may select which at data generation time.
    ///
    /// The performance of the conversion to this specific return type will vary
    /// depending on the data structure that is backing `self`.
    pub fn to_code_point_inversion_list_string_list(&self) -> CodePointInversionListStringList<'_> {
        self.data.get().to_code_point_inversion_list_string_list()
    }
}

/// A borrowed wrapper around code point set data, returned by
/// [`UnicodeSetData::as_borrowed()`]. More efficient to query.
pub struct UnicodeSetDataBorrowed<'a> {
    set: &'a PropertyUnicodeSetV1<'a>,
}

impl<'a> UnicodeSetDataBorrowed<'a> {
    /// Check if the set contains the string. Strings consisting of one character
    /// are treated as a character/code point.
    ///
    /// This matches ICU behavior for ICU's `UnicodeSet`.
    #[inline]
    pub fn contains(self, s: &str) -> bool {
        self.set.contains(s)
    }

    /// Check if the set contains a character as a UTF32 code unit
    #[inline]
    pub fn contains32(&self, cp: u32) -> bool {
        self.set.contains32(cp)
    }

    /// Check if the set contains the code point corresponding to the Rust character.
    #[inline]
    pub fn contains_char(&self, ch: char) -> bool {
        self.set.contains_char(ch)
    }
}

//
// Binary property getter fns
//

macro_rules! make_set_property {
    (
        // currently unused
        property: $property:expr;
        // currently unused
        marker: $marker_name:ident;
        keyed_data_marker: $keyed_data_marker:ty;
        func:
        $(#[$attr:meta])*
        $vis:vis fn $funcname:ident();
    ) => {
        $(#[$attr])*
        $vis fn $funcname(
            provider: &(impl DataProvider<$keyed_data_marker> + ?Sized)
        ) -> Result<CodePointSetData, PropertiesError> {
            Ok(provider.load(Default::default()).and_then(DataResponse::take_payload).map(CodePointSetData::from_data)?)
        }
    }
}

make_set_property! {
    property: "ASCII_Hex_Digit";
    marker: AsciiHexDigitProperty;
    keyed_data_marker: AsciiHexDigitV1Marker;
    func:
    /// ASCII characters commonly used for the representation of hexadecimal numbers
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_ascii_hex_digit(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let ascii_hex_digit = data.as_borrowed();;
    ///
    /// assert!(ascii_hex_digit.contains('3'));
    /// assert!(!ascii_hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(ascii_hex_digit.contains('A'));
    /// assert!(!ascii_hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```
    pub fn load_ascii_hex_digit();
}

make_set_property! {
    property: "Alnum";
    marker: AlnumProperty;
    keyed_data_marker: AlnumV1Marker;
    func:
    /// Characters with the Alphabetic or Decimal_Number property
    /// This is defined for POSIX compatibility.

    pub fn load_alnum();
}

make_set_property! {
    property: "Alphabetic";
    marker: AlphabeticProperty;
    keyed_data_marker: AlphabeticV1Marker;
    func:
    /// Alphabetic characters
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_alphabetic(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let alphabetic = data.as_borrowed();;
    ///
    /// assert!(!alphabetic.contains('3'));
    /// assert!(!alphabetic.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(alphabetic.contains('A'));
    /// assert!(alphabetic.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```

    pub fn load_alphabetic();
}

make_set_property! {
    property: "Bidi_Control";
    marker: BidiControlProperty;
    keyed_data_marker: BidiControlV1Marker;
    func:
    /// Format control characters which have specific functions in the Unicode Bidirectional
    /// Algorithm
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_bidi_control(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let bidi_control = data.as_borrowed();;
    ///
    /// assert!(bidi_control.contains32(0x200F));  // RIGHT-TO-LEFT MARK
    /// assert!(!bidi_control.contains('ش'));  // U+0634 ARABIC LETTER SHEEN
    /// ```

    pub fn load_bidi_control();
}

make_set_property! {
    property: "Bidi_Mirrored";
    marker: BidiMirroredProperty;
    keyed_data_marker: BidiMirroredV1Marker;
    func:
    /// Characters that are mirrored in bidirectional text
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_bidi_mirrored(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let bidi_mirrored = data.as_borrowed();;
    ///
    /// assert!(bidi_mirrored.contains('['));
    /// assert!(bidi_mirrored.contains(']'));
    /// assert!(bidi_mirrored.contains('∑'));  // U+2211 N-ARY SUMMATION
    /// assert!(!bidi_mirrored.contains('ཉ'));  // U+0F49 TIBETAN LETTER NYA
    /// ```

    pub fn load_bidi_mirrored();
}

make_set_property! {
    property: "Blank";
    marker: BlankProperty;
    keyed_data_marker: BlankV1Marker;
    func:
    /// Horizontal whitespace characters

    pub fn load_blank();
}

make_set_property! {
    property: "Cased";
    marker: CasedProperty;
    keyed_data_marker: CasedV1Marker;
    func:
    /// Uppercase, lowercase, and titlecase characters
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_cased(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let cased = data.as_borrowed();;
    ///
    /// assert!(cased.contains('Ꙡ'));  // U+A660 CYRILLIC CAPITAL LETTER REVERSED TSE
    /// assert!(!cased.contains('ދ'));  // U+078B THAANA LETTER DHAALU
    /// ```

    pub fn load_cased();
}

make_set_property! {
    property: "Case_Ignorable";
    marker: CaseIgnorableProperty;
    keyed_data_marker: CaseIgnorableV1Marker;
    func:
    /// Characters which are ignored for casing purposes
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_case_ignorable(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let case_ignorable = data.as_borrowed();;
    ///
    /// assert!(case_ignorable.contains(':'));
    /// assert!(!case_ignorable.contains('λ'));  // U+03BB GREEK SMALL LETTER LAMDA
    /// ```

    pub fn load_case_ignorable();
}

make_set_property! {
    property: "Full_Composition_Exclusion";
    marker: FullCompositionExclusionProperty;
    keyed_data_marker: FullCompositionExclusionV1Marker;
    func:
    /// Characters that are excluded from composition
    /// See <https://unicode.org/Public/UNIDATA/CompositionExclusions.txt>

    pub fn load_full_composition_exclusion();
}

make_set_property! {
    property: "Changes_When_Casefolded";
    marker: ChangesWhenCasefoldedProperty;
    keyed_data_marker: ChangesWhenCasefoldedV1Marker;
    func:
    /// Characters whose normalized forms are not stable under case folding
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_changes_when_casefolded(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let changes_when_casefolded = data.as_borrowed();;
    ///
    /// assert!(changes_when_casefolded.contains('ß'));  // U+00DF LATIN SMALL LETTER SHARP S
    /// assert!(!changes_when_casefolded.contains('ᜉ'));  // U+1709 TAGALOG LETTER PA
    /// ```

    pub fn load_changes_when_casefolded();
}

make_set_property! {
    property: "Changes_When_Casemapped";
    marker: ChangesWhenCasemappedProperty;
    keyed_data_marker: ChangesWhenCasemappedV1Marker;
    func:
    /// Characters which may change when they undergo case mapping

    pub fn load_changes_when_casemapped();
}

make_set_property! {
    property: "Changes_When_NFKC_Casefolded";
    marker: ChangesWhenNfkcCasefoldedProperty;
    keyed_data_marker: ChangesWhenNfkcCasefoldedV1Marker;
    func:
    /// Characters which are not identical to their NFKC_Casefold mapping
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_changes_when_nfkc_casefolded(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let changes_when_nfkc_casefolded = data.as_borrowed();;
    ///
    /// assert!(changes_when_nfkc_casefolded.contains('🄵'));  // U+1F135 SQUARED LATIN CAPITAL LETTER F
    /// assert!(!changes_when_nfkc_casefolded.contains('f'));
    /// ```

    pub fn load_changes_when_nfkc_casefolded();
}

make_set_property! {
    property: "Changes_When_Lowercased";
    marker: ChangesWhenLowercasedProperty;
    keyed_data_marker: ChangesWhenLowercasedV1Marker;
    func:
    /// Characters whose normalized forms are not stable under a toLowercase mapping
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_changes_when_lowercased(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let changes_when_lowercased = data.as_borrowed();;
    ///
    /// assert!(changes_when_lowercased.contains('Ⴔ'));  // U+10B4 GEORGIAN CAPITAL LETTER PHAR
    /// assert!(!changes_when_lowercased.contains('ფ'));  // U+10E4 GEORGIAN LETTER PHAR
    /// ```

    pub fn load_changes_when_lowercased();
}

make_set_property! {
    property: "Changes_When_Titlecased";
    marker: ChangesWhenTitlecasedProperty;
    keyed_data_marker: ChangesWhenTitlecasedV1Marker;
    func:
    /// Characters whose normalized forms are not stable under a toTitlecase mapping
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_changes_when_titlecased(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let changes_when_titlecased = data.as_borrowed();;
    ///
    /// assert!(changes_when_titlecased.contains('æ'));  // U+00E6 LATIN SMALL LETTER AE
    /// assert!(!changes_when_titlecased.contains('Æ'));  // U+00E6 LATIN CAPITAL LETTER AE
    /// ```

    pub fn load_changes_when_titlecased();
}

make_set_property! {
    property: "Changes_When_Uppercased";
    marker: ChangesWhenUppercasedProperty;
    keyed_data_marker: ChangesWhenUppercasedV1Marker;
    func:
    /// Characters whose normalized forms are not stable under a toUppercase mapping
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_changes_when_uppercased(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let changes_when_uppercased = data.as_borrowed();;
    ///
    /// assert!(changes_when_uppercased.contains('ւ'));  // U+0582 ARMENIAN SMALL LETTER YIWN
    /// assert!(!changes_when_uppercased.contains('Ւ'));  // U+0552 ARMENIAN CAPITAL LETTER YIWN
    /// ```

    pub fn load_changes_when_uppercased();
}

make_set_property! {
    property: "Dash";
    marker: DashProperty;
    keyed_data_marker: DashV1Marker;
    func:
    /// Punctuation characters explicitly called out as dashes in the Unicode Standard, plus
    /// their compatibility equivalents
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_dash(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let dash = data.as_borrowed();;
    ///
    /// assert!(dash.contains('⸺'));  // U+2E3A TWO-EM DASH
    /// assert!(dash.contains('-'));  // U+002D
    /// assert!(!dash.contains('='));  // U+003D
    /// ```

    pub fn load_dash();
}

make_set_property! {
    property: "Deprecated";
    marker: DeprecatedProperty;
    keyed_data_marker: DeprecatedV1Marker;
    func:
    /// Deprecated characters. No characters will ever be removed from the standard, but the
    /// usage of deprecated characters is strongly discouraged.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_deprecated(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let deprecated = data.as_borrowed();;
    ///
    /// assert!(deprecated.contains('ឣ'));  // U+17A3 KHMER INDEPENDENT VOWEL QAQ
    /// assert!(!deprecated.contains('A'));
    /// ```

    pub fn load_deprecated();
}

make_set_property! {
    property: "Default_Ignorable_Code_Point";
    marker: DefaultIgnorableCodePointProperty;
    keyed_data_marker: DefaultIgnorableCodePointV1Marker;
    func:
    /// For programmatic determination of default ignorable code points.  New characters that
    /// should be ignored in rendering (unless explicitly supported) will be assigned in these
    /// ranges, permitting programs to correctly handle the default rendering of such
    /// characters when not otherwise supported.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_default_ignorable_code_point(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let default_ignorable_code_point = data.as_borrowed();;
    ///
    /// assert!(default_ignorable_code_point.contains32(0x180B));  // MONGOLIAN FREE VARIATION SELECTOR ONE
    /// assert!(!default_ignorable_code_point.contains('E'));
    /// ```

    pub fn load_default_ignorable_code_point();
}

make_set_property! {
    property: "Diacritic";
    marker: DiacriticProperty;
    keyed_data_marker: DiacriticV1Marker;
    func:
    /// Characters that linguistically modify the meaning of another character to which they apply
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_diacritic(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let diacritic = data.as_borrowed();;
    ///
    /// assert!(diacritic.contains('\u{05B3}'));  // HEBREW POINT HATAF QAMATS
    /// assert!(!diacritic.contains('א'));  // U+05D0 HEBREW LETTER ALEF
    /// ```

    pub fn load_diacritic();
}

make_set_property! {
    property: "Emoji_Modifier_Base";
    marker: EmojiModifierBaseProperty;
    keyed_data_marker: EmojiModifierBaseV1Marker;
    func:
    /// Characters that can serve as a base for emoji modifiers
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_emoji_modifier_base(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let emoji_modifier_base = data.as_borrowed();;
    ///
    /// assert!(emoji_modifier_base.contains('✊'));  // U+270A RAISED FIST
    /// assert!(!emoji_modifier_base.contains('⛰'));  // U+26F0 MOUNTAIN
    /// ```

    pub fn load_emoji_modifier_base();
}

make_set_property! {
    property: "Emoji_Component";
    marker: EmojiComponentProperty;
    keyed_data_marker: EmojiComponentV1Marker;
    func:
    /// Characters used in emoji sequences that normally do not appear on emoji keyboards as
    /// separate choices, such as base characters for emoji keycaps
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_emoji_component(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let emoji_component = data.as_borrowed();;
    ///
    /// assert!(emoji_component.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T
    /// assert!(emoji_component.contains32(0x20E3));  // COMBINING ENCLOSING KEYCAP
    /// assert!(emoji_component.contains('7'));
    /// assert!(!emoji_component.contains('T'));
    /// ```

    pub fn load_emoji_component();
}

make_set_property! {
    property: "Emoji_Modifier";
    marker: EmojiModifierProperty;
    keyed_data_marker: EmojiModifierV1Marker;
    func:
    /// Characters that are emoji modifiers
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_emoji_modifier(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let emoji_modifier = data.as_borrowed();;
    ///
    /// assert!(emoji_modifier.contains32(0x1F3FD));  // EMOJI MODIFIER FITZPATRICK TYPE-4
    /// assert!(!emoji_modifier.contains32(0x200C));  // ZERO WIDTH NON-JOINER
    /// ```

    pub fn load_emoji_modifier();
}

make_set_property! {
    property: "Emoji";
    marker: EmojiProperty;
    keyed_data_marker: EmojiV1Marker;
    func:
    /// Characters that are emoji
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_emoji(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let emoji = data.as_borrowed();;
    ///
    /// assert!(emoji.contains('🔥'));  // U+1F525 FIRE
    /// assert!(!emoji.contains('V'));
    /// ```

    pub fn load_emoji();
}

make_set_property! {
    property: "Emoji_Presentation";
    marker: EmojiPresentationProperty;
    keyed_data_marker: EmojiPresentationV1Marker;
    func:
    /// Characters that have emoji presentation by default
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_emoji_presentation(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let emoji_presentation = data.as_borrowed();;
    ///
    /// assert!(emoji_presentation.contains('🦬')); // U+1F9AC BISON
    /// assert!(!emoji_presentation.contains('♻'));  // U+267B BLACK UNIVERSAL RECYCLING SYMBOL
    /// ```

    pub fn load_emoji_presentation();
}

make_set_property! {
    property: "Extender";
    marker: ExtenderProperty;
    keyed_data_marker: ExtenderV1Marker;
    func:
    /// Characters whose principal function is to extend the value of a preceding alphabetic
    /// character or to extend the shape of adjacent characters.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_extender(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let extender = data.as_borrowed();;
    ///
    /// assert!(extender.contains('ヾ'));  // U+30FE KATAKANA VOICED ITERATION MARK
    /// assert!(extender.contains('ー'));  // U+30FC KATAKANA-HIRAGANA PROLONGED SOUND MARK
    /// assert!(!extender.contains('・'));  // U+30FB KATAKANA MIDDLE DOT
    /// ```

    pub fn load_extender();
}

make_set_property! {
    property: "Extended_Pictographic";
    marker: ExtendedPictographicProperty;
    keyed_data_marker: ExtendedPictographicV1Marker;
    func:
    /// Pictographic symbols, as well as reserved ranges in blocks largely associated with
    /// emoji characters
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_extended_pictographic(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let extended_pictographic = data.as_borrowed();;
    ///
    /// assert!(extended_pictographic.contains('🥳')); // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
    /// assert!(!extended_pictographic.contains('🇪'));  // U+1F1EA REGIONAL INDICATOR SYMBOL LETTER E
    /// ```

    pub fn load_extended_pictographic();
}

make_set_property! {
    property: "Graph";
    marker: GraphProperty;
    keyed_data_marker: GraphV1Marker;
    func:
    /// Visible characters.
    /// This is defined for POSIX compatibility.

    pub fn load_graph();
}

make_set_property! {
    property: "Grapheme_Base";
    marker: GraphemeBaseProperty;
    keyed_data_marker: GraphemeBaseV1Marker;
    func:
    /// Property used together with the definition of Standard Korean Syllable Block to define
    /// "Grapheme base". See D58 in Chapter 3, Conformance in the Unicode Standard.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_grapheme_base(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let grapheme_base = data.as_borrowed();;
    ///
    /// assert!(grapheme_base.contains('ക'));  // U+0D15 MALAYALAM LETTER KA
    /// assert!(grapheme_base.contains('\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I
    /// assert!(!grapheme_base.contains('\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA
    /// ```

    pub fn load_grapheme_base();
}

make_set_property! {
    property: "Grapheme_Extend";
    marker: GraphemeExtendProperty;
    keyed_data_marker: GraphemeExtendV1Marker;
    func:
    /// Property used to define "Grapheme extender". See D59 in Chapter 3, Conformance in the
    /// Unicode Standard.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_grapheme_extend(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let grapheme_extend = data.as_borrowed();;
    ///
    /// assert!(!grapheme_extend.contains('ക'));  // U+0D15 MALAYALAM LETTER KA
    /// assert!(!grapheme_extend.contains('\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I
    /// assert!(grapheme_extend.contains('\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA
    /// ```

    pub fn load_grapheme_extend();
}

make_set_property! {
    property: "Grapheme_Link";
    marker: GraphemeLinkProperty;
    keyed_data_marker: GraphemeLinkV1Marker;
    func:
    /// Deprecated property. Formerly proposed for programmatic determination of grapheme
    /// cluster boundaries.

    pub fn load_grapheme_link();
}

make_set_property! {
    property: "Hex_Digit";
    marker: HexDigitProperty;
    keyed_data_marker: HexDigitV1Marker;
    func:
    /// Characters commonly used for the representation of hexadecimal numbers, plus their
    /// compatibility equivalents
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_hex_digit(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let hex_digit = data.as_borrowed();;
    ///
    /// assert!(hex_digit.contains('0'));
    /// assert!(!hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(hex_digit.contains('f'));
    /// assert!(hex_digit.contains('ｆ'));  // U+FF46 FULLWIDTH LATIN SMALL LETTER F
    /// assert!(hex_digit.contains('Ｆ'));  // U+FF26 FULLWIDTH LATIN CAPITAL LETTER F
    /// assert!(!hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```

    pub fn load_hex_digit();
}

make_set_property! {
    property: "Hyphen";
    marker: HyphenProperty;
    keyed_data_marker: HyphenV1Marker;
    func:
    /// Deprecated property. Dashes which are used to mark connections between pieces of
    /// words, plus the Katakana middle dot.

    pub fn load_hyphen();
}

make_set_property! {
    property: "Id_Continue";
    marker: IdContinueProperty;
    keyed_data_marker: IdContinueV1Marker;
    func:
    /// Characters that can come after the first character in an identifier. If using NFKC to
    /// fold differences between characters, use [`load_xid_continue`] instead.  See
    /// [`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
    /// more details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_id_continue(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let id_continue = data.as_borrowed();;
    ///
    /// assert!(id_continue.contains('x'));
    /// assert!(id_continue.contains('1'));
    /// assert!(id_continue.contains('_'));
    /// assert!(id_continue.contains('ߝ'));  // U+07DD NKO LETTER FA
    /// assert!(!id_continue.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
    /// assert!(id_continue.contains32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
    /// ```

    pub fn load_id_continue();
}

make_set_property! {
    property: "Ideographic";
    marker: IdeographicProperty;
    keyed_data_marker: IdeographicV1Marker;
    func:
    /// Characters considered to be CJKV (Chinese, Japanese, Korean, and Vietnamese)
    /// ideographs, or related siniform ideographs
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_ideographic(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let ideographic = data.as_borrowed();;
    ///
    /// assert!(ideographic.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD
    /// assert!(!ideographic.contains('밥'));  // U+BC25 HANGUL SYLLABLE BAB
    /// ```

    pub fn load_ideographic();
}

make_set_property! {
    property: "Id_Start";
    marker: IdStartProperty;
    keyed_data_marker: IdStartV1Marker;
    func:
    /// Characters that can begin an identifier. If using NFKC to fold differences between
    /// characters, use [`load_xid_start`] instead.  See [`Unicode Standard Annex
    /// #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_id_start(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let id_start = data.as_borrowed();;
    ///
    /// assert!(id_start.contains('x'));
    /// assert!(!id_start.contains('1'));
    /// assert!(!id_start.contains('_'));
    /// assert!(id_start.contains('ߝ'));  // U+07DD NKO LETTER FA
    /// assert!(!id_start.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
    /// assert!(id_start.contains32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
    /// ```

    pub fn load_id_start();
}

make_set_property! {
    property: "Ids_Binary_Operator";
    marker: IdsBinaryOperatorProperty;
    keyed_data_marker: IdsBinaryOperatorV1Marker;
    func:
    /// Characters used in Ideographic Description Sequences
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_ids_binary_operator(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let ids_binary_operator = data.as_borrowed();;
    ///
    /// assert!(ids_binary_operator.contains32(0x2FF5));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE
    /// assert!(!ids_binary_operator.contains32(0x3006));  // IDEOGRAPHIC CLOSING MARK
    /// ```

    pub fn load_ids_binary_operator();
}

make_set_property! {
    property: "Ids_Trinary_Operator";
    marker: IdsTrinaryOperatorProperty;
    keyed_data_marker: IdsTrinaryOperatorV1Marker;
    func:
    /// Characters used in Ideographic Description Sequences
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_ids_trinary_operator(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let ids_trinary_operator = data.as_borrowed();;
    ///
    /// assert!(ids_trinary_operator.contains32(0x2FF2));  // IDEOGRAPHIC DESCRIPTION CHARACTER LEFT TO MIDDLE AND RIGHT
    /// assert!(ids_trinary_operator.contains32(0x2FF3));  // IDEOGRAPHIC DESCRIPTION CHARACTER ABOVE TO MIDDLE AND BELOW
    /// assert!(!ids_trinary_operator.contains32(0x2FF4));
    /// assert!(!ids_trinary_operator.contains32(0x2FF5));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE
    /// assert!(!ids_trinary_operator.contains32(0x3006));  // IDEOGRAPHIC CLOSING MARK
    /// ```

    pub fn load_ids_trinary_operator();
}

make_set_property! {
    property: "Join_Control";
    marker: JoinControlProperty;
    keyed_data_marker: JoinControlV1Marker;
    func:
    /// Format control characters which have specific functions for control of cursive joining
    /// and ligation
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_join_control(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let join_control = data.as_borrowed();;
    ///
    /// assert!(join_control.contains32(0x200C));  // ZERO WIDTH NON-JOINER
    /// assert!(join_control.contains32(0x200D));  // ZERO WIDTH JOINER
    /// assert!(!join_control.contains32(0x200E));
    /// ```

    pub fn load_join_control();
}

make_set_property! {
    property: "Logical_Order_Exception";
    marker: LogicalOrderExceptionProperty;
    keyed_data_marker: LogicalOrderExceptionV1Marker;
    func:
    /// A small number of spacing vowel letters occurring in certain Southeast Asian scripts such as Thai and Lao
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_logical_order_exception(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let logical_order_exception = data.as_borrowed();;
    ///
    /// assert!(logical_order_exception.contains('ແ'));  // U+0EC1 LAO VOWEL SIGN EI
    /// assert!(!logical_order_exception.contains('ະ'));  // U+0EB0 LAO VOWEL SIGN A
    /// ```

    pub fn load_logical_order_exception();
}

make_set_property! {
    property: "Lowercase";
    marker: LowercaseProperty;
    keyed_data_marker: LowercaseV1Marker;
    func:
    /// Lowercase characters
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_lowercase(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let lowercase = data.as_borrowed();;
    ///
    /// assert!(lowercase.contains('a'));
    /// assert!(!lowercase.contains('A'));
    /// ```

    pub fn load_lowercase();
}

make_set_property! {
    property: "Math";
    marker: MathProperty;
    keyed_data_marker: MathV1Marker;
    func:
    /// Characters used in mathematical notation
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_math(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let math = data.as_borrowed();;
    ///
    /// assert!(math.contains('='));
    /// assert!(math.contains('+'));
    /// assert!(!math.contains('-'));
    /// assert!(math.contains('−'));  // U+2212 MINUS SIGN
    /// assert!(!math.contains('/'));
    /// assert!(math.contains('∕'));  // U+2215 DIVISION SLASH
    /// ```

    pub fn load_math();
}

make_set_property! {
    property: "Noncharacter_Code_Point";
    marker: NoncharacterCodePointProperty;
    keyed_data_marker: NoncharacterCodePointV1Marker;
    func:
    /// Code points permanently reserved for internal use
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_noncharacter_code_point(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let noncharacter_code_point = data.as_borrowed();;
    ///
    /// assert!(noncharacter_code_point.contains32(0xFDD0));
    /// assert!(noncharacter_code_point.contains32(0xFFFF));
    /// assert!(!noncharacter_code_point.contains32(0x10000));
    /// ```

    pub fn load_noncharacter_code_point();
}

make_set_property! {
    property: "NFC_Inert";
    marker: NfcInertProperty;
    keyed_data_marker: NfcInertV1Marker;
    func:
    /// Characters that are inert under NFC, i.e., they do not interact with adjacent characters

    pub fn load_nfc_inert();
}

make_set_property! {
    property: "NFD_Inert";
    marker: NfdInertProperty;
    keyed_data_marker: NfdInertV1Marker;
    func:
    /// Characters that are inert under NFD, i.e., they do not interact with adjacent characters

    pub fn load_nfd_inert();
}

make_set_property! {
    property: "NFKC_Inert";
    marker: NfkcInertProperty;
    keyed_data_marker: NfkcInertV1Marker;
    func:
    /// Characters that are inert under NFKC, i.e., they do not interact with adjacent characters

    pub fn load_nfkc_inert();
}

make_set_property! {
    property: "NFKD_Inert";
    marker: NfkdInertProperty;
    keyed_data_marker: NfkdInertV1Marker;
    func:
    /// Characters that are inert under NFKD, i.e., they do not interact with adjacent characters

    pub fn load_nfkd_inert();
}

make_set_property! {
    property: "Pattern_Syntax";
    marker: PatternSyntaxProperty;
    keyed_data_marker: PatternSyntaxV1Marker;
    func:
    /// Characters used as syntax in patterns (such as regular expressions). See [`Unicode
    /// Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
    /// details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_pattern_syntax(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let pattern_syntax = data.as_borrowed();;
    ///
    /// assert!(pattern_syntax.contains('{'));
    /// assert!(pattern_syntax.contains('⇒'));  // U+21D2 RIGHTWARDS DOUBLE ARROW
    /// assert!(!pattern_syntax.contains('0'));
    /// ```

    pub fn load_pattern_syntax();
}

make_set_property! {
    property: "Pattern_White_Space";
    marker: PatternWhiteSpaceProperty;
    keyed_data_marker: PatternWhiteSpaceV1Marker;
    func:
    /// Characters used as whitespace in patterns (such as regular expressions).  See
    /// [`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
    /// more details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_pattern_white_space(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let pattern_white_space = data.as_borrowed();;
    ///
    /// assert!(pattern_white_space.contains(' '));
    /// assert!(pattern_white_space.contains32(0x2029));  // PARAGRAPH SEPARATOR
    /// assert!(pattern_white_space.contains32(0x000A));  // NEW LINE
    /// assert!(!pattern_white_space.contains32(0x00A0));  // NO-BREAK SPACE
    /// ```

    pub fn load_pattern_white_space();
}

make_set_property! {
    property: "Prepended_Concatenation_Mark";
    marker: PrependedConcatenationMarkProperty;
    keyed_data_marker: PrependedConcatenationMarkV1Marker;
    func:
    /// A small class of visible format controls, which precede and then span a sequence of
    /// other characters, usually digits.

    pub fn load_prepended_concatenation_mark();
}

make_set_property! {
    property: "Print";
    marker: PrintProperty;
    keyed_data_marker: PrintV1Marker;
    func:
    /// Printable characters (visible characters and whitespace).
    /// This is defined for POSIX compatibility.

    pub fn load_print();
}

make_set_property! {
    property: "Quotation_Mark";
    marker: QuotationMarkProperty;
    keyed_data_marker: QuotationMarkV1Marker;
    func:
    /// Punctuation characters that function as quotation marks.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_quotation_mark(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let quotation_mark = data.as_borrowed();;
    ///
    /// assert!(quotation_mark.contains('\''));
    /// assert!(quotation_mark.contains('„'));  // U+201E DOUBLE LOW-9 QUOTATION MARK
    /// assert!(!quotation_mark.contains('<'));
    /// ```

    pub fn load_quotation_mark();
}

make_set_property! {
    property: "Radical";
    marker: RadicalProperty;
    keyed_data_marker: RadicalV1Marker;
    func:
    /// Characters used in the definition of Ideographic Description Sequences
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_radical(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let radical = data.as_borrowed();;
    ///
    /// assert!(radical.contains('⺆'));  // U+2E86 CJK RADICAL BOX
    /// assert!(!radical.contains('丹'));  // U+F95E CJK COMPATIBILITY IDEOGRAPH-F95E
    /// ```

    pub fn load_radical();
}

make_set_property! {
    property: "Regional_Indicator";
    marker: RegionalIndicatorProperty;
    keyed_data_marker: RegionalIndicatorV1Marker;
    func:
    /// Regional indicator characters, U+1F1E6..U+1F1FF
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_regional_indicator(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let regional_indicator = data.as_borrowed();;
    ///
    /// assert!(regional_indicator.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T
    /// assert!(!regional_indicator.contains('Ⓣ'));  // U+24C9 CIRCLED LATIN CAPITAL LETTER T
    /// assert!(!regional_indicator.contains('T'));
    /// ```

    pub fn load_regional_indicator();
}

make_set_property! {
    property: "Soft_Dotted";
    marker: SoftDottedProperty;
    keyed_data_marker: SoftDottedV1Marker;
    func:
    /// Characters with a "soft dot", like i or j. An accent placed on these characters causes
    /// the dot to disappear.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_soft_dotted(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let soft_dotted = data.as_borrowed();;
    ///
    /// assert!(soft_dotted.contains('і'));  //U+0456 CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
    /// assert!(!soft_dotted.contains('ı'));  // U+0131 LATIN SMALL LETTER DOTLESS I
    /// ```

    pub fn load_soft_dotted();
}

make_set_property! {
    property: "Segment_Starter";
    marker: SegmentStarterProperty;
    keyed_data_marker: SegmentStarterV1Marker;
    func:
    /// Characters that are starters in terms of Unicode normalization and combining character
    /// sequences

    pub fn load_segment_starter();
}

make_set_property! {
    property: "Case_Sensitive";
    marker: CaseSensitiveProperty;
    keyed_data_marker: CaseSensitiveV1Marker;
    func:
    /// Characters that are either the source of a case mapping or in the target of a case
    /// mapping

    pub fn load_case_sensitive();
}

make_set_property! {
    property: "Sentence_Terminal";
    marker: SentenceTerminalProperty;
    keyed_data_marker: SentenceTerminalV1Marker;
    func:
    /// Punctuation characters that generally mark the end of sentences
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_sentence_terminal(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let sentence_terminal = data.as_borrowed();;
    ///
    /// assert!(sentence_terminal.contains('.'));
    /// assert!(sentence_terminal.contains('?'));
    /// assert!(sentence_terminal.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN
    /// assert!(!sentence_terminal.contains(','));
    /// assert!(!sentence_terminal.contains('¿'));  // U+00BF INVERTED QUESTION MARK
    /// ```

    pub fn load_sentence_terminal();
}

make_set_property! {
    property: "Terminal_Punctuation";
    marker: TerminalPunctuationProperty;
    keyed_data_marker: TerminalPunctuationV1Marker;
    func:
    /// Punctuation characters that generally mark the end of textual units
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_terminal_punctuation(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let terminal_punctuation = data.as_borrowed();;
    ///
    /// assert!(terminal_punctuation.contains('.'));
    /// assert!(terminal_punctuation.contains('?'));
    /// assert!(terminal_punctuation.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN
    /// assert!(terminal_punctuation.contains(','));
    /// assert!(!terminal_punctuation.contains('¿'));  // U+00BF INVERTED QUESTION MARK
    /// ```

    pub fn load_terminal_punctuation();
}

make_set_property! {
    property: "Unified_Ideograph";
    marker: UnifiedIdeographProperty;
    keyed_data_marker: UnifiedIdeographV1Marker;
    func:
    /// A property which specifies the exact set of Unified CJK Ideographs in the standard
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_unified_ideograph(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let unified_ideograph = data.as_borrowed();;
    ///
    /// assert!(unified_ideograph.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD
    /// assert!(unified_ideograph.contains('木'));  // U+6728 CJK UNIFIED IDEOGRAPH-6728
    /// assert!(!unified_ideograph.contains('𛅸'));  // U+1B178 NUSHU CHARACTER-1B178
    /// ```

    pub fn load_unified_ideograph();
}

make_set_property! {
    property: "Uppercase";
    marker: UppercaseProperty;
    keyed_data_marker: UppercaseV1Marker;
    func:
    /// Uppercase characters
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_uppercase(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let uppercase = data.as_borrowed();;
    ///
    /// assert!(uppercase.contains('U'));
    /// assert!(!uppercase.contains('u'));
    /// ```

    pub fn load_uppercase();
}

make_set_property! {
    property: "Variation_Selector";
    marker: VariationSelectorProperty;
    keyed_data_marker: VariationSelectorV1Marker;
    func:
    /// Characters that are Variation Selectors.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_variation_selector(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let variation_selector = data.as_borrowed();;
    ///
    /// assert!(variation_selector.contains32(0x180D));  // MONGOLIAN FREE VARIATION SELECTOR THREE
    /// assert!(!variation_selector.contains32(0x303E));  // IDEOGRAPHIC VARIATION INDICATOR
    /// assert!(variation_selector.contains32(0xFE0F));  // VARIATION SELECTOR-16
    /// assert!(!variation_selector.contains32(0xFE10));  // PRESENTATION FORM FOR VERTICAL COMMA
    /// assert!(variation_selector.contains32(0xE01EF));  // VARIATION SELECTOR-256
    /// ```

    pub fn load_variation_selector();
}

make_set_property! {
    property: "White_Space";
    marker: WhiteSpaceProperty;
    keyed_data_marker: WhiteSpaceV1Marker;
    func:
    /// Spaces, separator characters and other control characters which should be treated by
    /// programming languages as "white space" for the purpose of parsing elements
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_white_space(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let white_space = data.as_borrowed();;
    ///
    /// assert!(white_space.contains(' '));
    /// assert!(white_space.contains32(0x000A));  // NEW LINE
    /// assert!(white_space.contains32(0x00A0));  // NO-BREAK SPACE
    /// assert!(!white_space.contains32(0x200B));  // ZERO WIDTH SPACE
    /// ```

    pub fn load_white_space();
}

make_set_property! {
    property: "Xdigit";
    marker: XdigitProperty;
    keyed_data_marker: XdigitV1Marker;
    func:
    /// Hexadecimal digits
    /// This is defined for POSIX compatibility.

    pub fn load_xdigit();
}

make_set_property! {
    property: "XID_Continue";
    marker: XidContinueProperty;
    keyed_data_marker: XidContinueV1Marker;
    func:
    /// Characters that can come after the first character in an identifier.  See [`Unicode Standard Annex
    /// #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_xid_continue(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let xid_continue = data.as_borrowed();;
    ///
    /// assert!(xid_continue.contains('x'));
    /// assert!(xid_continue.contains('1'));
    /// assert!(xid_continue.contains('_'));
    /// assert!(xid_continue.contains('ߝ'));  // U+07DD NKO LETTER FA
    /// assert!(!xid_continue.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
    /// assert!(!xid_continue.contains32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
    /// ```

    pub fn load_xid_continue();
}

make_set_property! {
    property: "XID_Start";
    marker: XidStartProperty;
    keyed_data_marker: XidStartV1Marker;
    func:
    /// Characters that can begin an identifier. See [`Unicode
    /// Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
    /// details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_properties::sets;
    ///
    /// let data =
    ///     sets::load_xid_start(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let xid_start = data.as_borrowed();;
    ///
    /// assert!(xid_start.contains('x'));
    /// assert!(!xid_start.contains('1'));
    /// assert!(!xid_start.contains('_'));
    /// assert!(xid_start.contains('ߝ'));  // U+07DD NKO LETTER FA
    /// assert!(!xid_start.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
    /// assert!(!xid_start.contains32(0xFC5E));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
    /// ```

    pub fn load_xid_start();
}

//
// Enumerated property getter fns
//

/// Return a [`CodePointSetData`] for a value or a grouping of values of the General_Category property. See [`GeneralCategoryGroup`].
pub fn load_for_general_category_group(
    provider: &(impl DataProvider<GeneralCategoryV1Marker> + ?Sized),
    enum_val: GeneralCategoryGroup,
) -> Result<CodePointSetData, PropertiesError> {
    let gc_map_payload = maps::load_general_category(provider)?;
    let gc_map = gc_map_payload.as_borrowed();
    let matching_gc_ranges = gc_map
        .iter_ranges()
        .filter(|cpm_range| (1 << cpm_range.value as u32) & enum_val.0 != 0)
        .map(|cpm_range| cpm_range.range);
    let set = CodePointInversionList::from_iter(matching_gc_ranges);
    Ok(CodePointSetData::from_code_point_inversion_list(set))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_general_category() {
        use icu::properties::sets;
        use icu::properties::GeneralCategoryGroup;

        let digits_data = sets::load_for_general_category_group(
            &icu_testdata::unstable(),
            GeneralCategoryGroup::Number,
        )
        .expect("The data should be valid");
        let digits = digits_data.as_borrowed();

        assert!(digits.contains('5'));
        assert!(digits.contains('\u{0665}')); // U+0665 ARABIC-INDIC DIGIT FIVE
        assert!(digits.contains('\u{096b}')); // U+0969 DEVANAGARI DIGIT FIVE

        assert!(!digits.contains('A'));
    }

    #[test]
    fn test_script() {
        use icu::properties::maps;
        use icu::properties::Script;

        let data = maps::load_script(&icu_testdata::unstable()).expect("The data should be valid");
        let thai_data = data.as_borrowed().get_set_for_value(Script::Thai);
        let thai = thai_data.as_borrowed();

        assert!(thai.contains('\u{0e01}')); // U+0E01 THAI CHARACTER KO KAI
        assert!(thai.contains('\u{0e50}')); // U+0E50 THAI DIGIT ZERO

        assert!(!thai.contains('A'));
        assert!(!thai.contains('\u{0e3f}')); // U+0E50 THAI CURRENCY SYMBOL BAHT
    }

    #[test]
    fn test_gc_groupings() {
        use icu::properties::{maps, sets};
        use icu::properties::{GeneralCategory, GeneralCategoryGroup};
        use icu_collections::codepointinvlist::CodePointInversionListBuilder;

        let test_group = |category: GeneralCategoryGroup, subcategories: &[GeneralCategory]| {
            let category_set =
                sets::load_for_general_category_group(&icu_testdata::unstable(), category)
                    .expect("The data should be valid");
            let category_set = category_set
                .as_code_point_inversion_list()
                .expect("The data should be valid");

            let data = maps::load_general_category(&icu_testdata::unstable())
                .expect("The data should be valid");
            let gc = data.as_borrowed();

            let mut builder = CodePointInversionListBuilder::new();
            for subcategory in subcategories {
                let gc_set_data = &gc.get_set_for_value(*subcategory);
                let gc_set = gc_set_data.as_borrowed();
                for range in gc_set.iter_ranges() {
                    builder.add_range_u32(&range);
                }
            }
            let combined_set = builder.build();
            println!("{:?} {:?}", category, subcategories);
            assert_eq!(
                category_set.get_inversion_list_vec(),
                combined_set.get_inversion_list_vec()
            );
        };

        test_group(
            GeneralCategoryGroup::Letter,
            &[
                GeneralCategory::UppercaseLetter,
                GeneralCategory::LowercaseLetter,
                GeneralCategory::TitlecaseLetter,
                GeneralCategory::ModifierLetter,
                GeneralCategory::OtherLetter,
            ],
        );
        test_group(
            GeneralCategoryGroup::Other,
            &[
                GeneralCategory::Control,
                GeneralCategory::Format,
                GeneralCategory::Unassigned,
                GeneralCategory::PrivateUse,
                GeneralCategory::Surrogate,
            ],
        );
        test_group(
            GeneralCategoryGroup::Mark,
            &[
                GeneralCategory::SpacingMark,
                GeneralCategory::EnclosingMark,
                GeneralCategory::NonspacingMark,
            ],
        );
        test_group(
            GeneralCategoryGroup::Number,
            &[
                GeneralCategory::DecimalNumber,
                GeneralCategory::LetterNumber,
                GeneralCategory::OtherNumber,
            ],
        );
        test_group(
            GeneralCategoryGroup::Punctuation,
            &[
                GeneralCategory::ConnectorPunctuation,
                GeneralCategory::DashPunctuation,
                GeneralCategory::ClosePunctuation,
                GeneralCategory::FinalPunctuation,
                GeneralCategory::InitialPunctuation,
                GeneralCategory::OtherPunctuation,
                GeneralCategory::OpenPunctuation,
            ],
        );
        test_group(
            GeneralCategoryGroup::Symbol,
            &[
                GeneralCategory::CurrencySymbol,
                GeneralCategory::ModifierSymbol,
                GeneralCategory::MathSymbol,
                GeneralCategory::OtherSymbol,
            ],
        );
        test_group(
            GeneralCategoryGroup::Separator,
            &[
                GeneralCategory::LineSeparator,
                GeneralCategory::ParagraphSeparator,
                GeneralCategory::SpaceSeparator,
            ],
        );
    }

    #[test]
    fn test_gc_surrogate() {
        use icu::properties::maps;
        use icu::properties::GeneralCategory;

        let data = maps::load_general_category(&icu_testdata::unstable())
            .expect("The data should be valid");
        let gc = data.as_borrowed();
        let surrogates_data = gc.get_set_for_value(GeneralCategory::Surrogate);
        let surrogates = surrogates_data.as_borrowed();

        assert!(surrogates.contains32(0xd800));
        assert!(surrogates.contains32(0xd900));
        assert!(surrogates.contains32(0xdfff));

        assert!(!surrogates.contains('A'));
    }
}
