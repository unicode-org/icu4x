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

use crate::provider::*;
use crate::runtime::UnicodeProperty;
#[cfg(doc)]
use crate::{
    props::{GeneralCategory, GeneralCategoryGroup, Script},
    script, CodePointMapData,
};
use core::ops::RangeInclusive;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_provider::prelude::*;

/// A wrapper around code point set data. It is returned by APIs that return Unicode
/// property data in a set-like form, ex: a set of code points sharing the same
/// value for a Unicode property. Access its data via the borrowed version,
/// [`CodePointSetDataBorrowed`].
#[derive(Debug)]
pub struct CodePointSetData {
    data: DataPayload<ErasedPropertyCodePointSetV1Marker>,
}

impl CodePointSetData {
    /// Creates a new [`CodePointSetData`] for a [`CodePointSetProperty`].
    ///
    /// See the documentation on [`CodePointSetProperty`] implementations for details.
    #[allow(clippy::new_ret_no_self)]
    #[cfg(feature = "compiled_data")]
    pub const fn new<P: CodePointSetProperty>() -> CodePointSetDataBorrowed<'static> {
        CodePointSetDataBorrowed { set: P::SINGLETON }
    }

    /// A version of [`Self::new()`] that uses custom data provided by a [`DataProvider`].
    ///
    /// Note that this will return an owned version of the data. Functionality is available on
    /// the borrowed version, accessible through [`CodePointSetData::as_borrowed`].
    pub fn try_new_unstable<P: CodePointSetProperty>(
        provider: &(impl DataProvider<P::DataMarker> + ?Sized),
    ) -> Result<CodePointSetData, DataError> {
        Ok(CodePointSetData::from_data(
            provider.load(Default::default())?.payload,
        ))
    }

    /// Returns a type capable of looking up values for a property specified as a string, as long as it is a
    /// [binary property listed in ECMA-262][ecma], using strict matching on the names in the spec.
    ///
    /// This handles every property required by ECMA-262 `/u` regular expressions, except for:
    ///
    /// - `Script` and `General_Category`: handle these directly using property values parsed via
    ///   [`GeneralCategory::get_name_to_enum_mapper()`] and [`Script::get_name_to_enum_mapper()`]
    ///    if necessary.
    /// - `Script_Extensions`: handle this directly using APIs from [`crate::script`], like [`script::load_script_with_extensions_unstable()`]
    /// - `General_Category` mask values: Handle this alongside `General_Category` using [`GeneralCategoryGroup`],
    ///    using property values parsed via [`GeneralCategoryGroup::get_name_to_enum_mapper()`] if necessary
    /// - `Assigned`, `All`, and `ASCII` pseudoproperties: Handle these using their equivalent sets:
    ///    - `Any` can be expressed as the range `[\u{0}-\u{10FFFF}]`
    ///    - `Assigned` can be expressed as the inverse of the set `gc=Cn` (i.e., `\P{gc=Cn}`).
    ///    - `ASCII` can be expressed as the range `[\u{0}-\u{7F}]`
    /// - `General_Category` property values can themselves be treated like properties using a shorthand in ECMA262,
    ///    simply create the corresponding `GeneralCategory` set.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    ///
    /// ```
    /// use icu::properties::sets;
    ///
    /// let emoji = sets::load_for_ecma262("Emoji").expect("loading data failed");
    ///
    /// assert!(emoji.contains('🔥')); // U+1F525 FIRE
    /// assert!(!emoji.contains('V'));
    /// ```
    ///
    /// [ecma]: https://tc39.es/ecma262/#table-binary-unicode-properties
    #[cfg(feature = "compiled_data")]
    pub fn new_for_ecma262(
        name: &str,
    ) -> Result<CodePointSetDataBorrowed<'static>, crate::UnexpectedPropertyNameError> {
        use crate::props::*;

        let prop = if let Some(prop) = UnicodeProperty::parse_ecma262_name(name) {
            prop
        } else {
            return Err(crate::UnexpectedPropertyNameError);
        };
        Ok(match prop {
            AsciiHexDigit::VALUE => Self::new::<AsciiHexDigit>(),
            Alphabetic::VALUE => Self::new::<Alphabetic>(),
            BidiControl::VALUE => Self::new::<BidiControl>(),
            BidiMirrored::VALUE => Self::new::<BidiMirrored>(),
            CaseIgnorable::VALUE => Self::new::<CaseIgnorable>(),
            Cased::VALUE => Self::new::<Cased>(),
            ChangesWhenCasefolded::VALUE => Self::new::<ChangesWhenCasefolded>(),
            ChangesWhenCasemapped::VALUE => Self::new::<ChangesWhenCasemapped>(),
            ChangesWhenLowercased::VALUE => Self::new::<ChangesWhenLowercased>(),
            ChangesWhenNfkcCasefolded::VALUE => Self::new::<ChangesWhenNfkcCasefolded>(),
            ChangesWhenTitlecased::VALUE => Self::new::<ChangesWhenTitlecased>(),
            ChangesWhenUppercased::VALUE => Self::new::<ChangesWhenUppercased>(),
            Dash::VALUE => Self::new::<Dash>(),
            DefaultIgnorableCodePoint::VALUE => Self::new::<DefaultIgnorableCodePoint>(),
            Deprecated::VALUE => Self::new::<Deprecated>(),
            Diacritic::VALUE => Self::new::<Diacritic>(),
            Emoji::VALUE => Self::new::<Emoji>(),
            EmojiComponent::VALUE => Self::new::<EmojiComponent>(),
            EmojiModifier::VALUE => Self::new::<EmojiModifier>(),
            EmojiModifierBase::VALUE => Self::new::<EmojiModifierBase>(),
            EmojiPresentation::VALUE => Self::new::<EmojiPresentation>(),
            ExtendedPictographic::VALUE => Self::new::<ExtendedPictographic>(),
            Extender::VALUE => Self::new::<Extender>(),
            GraphemeBase::VALUE => Self::new::<GraphemeBase>(),
            GraphemeExtend::VALUE => Self::new::<GraphemeExtend>(),
            HexDigit::VALUE => Self::new::<HexDigit>(),
            IdsBinaryOperator::VALUE => Self::new::<IdsBinaryOperator>(),
            IdsTrinaryOperator::VALUE => Self::new::<IdsTrinaryOperator>(),
            IdContinue::VALUE => Self::new::<IdContinue>(),
            IdStart::VALUE => Self::new::<IdStart>(),
            Ideographic::VALUE => Self::new::<Ideographic>(),
            JoinControl::VALUE => Self::new::<JoinControl>(),
            LogicalOrderException::VALUE => Self::new::<LogicalOrderException>(),
            Lowercase::VALUE => Self::new::<Lowercase>(),
            Math::VALUE => Self::new::<Math>(),
            NoncharacterCodePoint::VALUE => Self::new::<NoncharacterCodePoint>(),
            PatternSyntax::VALUE => Self::new::<PatternSyntax>(),
            PatternWhiteSpace::VALUE => Self::new::<PatternWhiteSpace>(),
            QuotationMark::VALUE => Self::new::<QuotationMark>(),
            Radical::VALUE => Self::new::<Radical>(),
            RegionalIndicator::VALUE => Self::new::<RegionalIndicator>(),
            SentenceTerminal::VALUE => Self::new::<SentenceTerminal>(),
            SoftDotted::VALUE => Self::new::<SoftDotted>(),
            TerminalPunctuation::VALUE => Self::new::<TerminalPunctuation>(),
            UnifiedIdeograph::VALUE => Self::new::<UnifiedIdeograph>(),
            Uppercase::VALUE => Self::new::<Uppercase>(),
            VariationSelector::VALUE => Self::new::<VariationSelector>(),
            WhiteSpace::VALUE => Self::new::<WhiteSpace>(),
            XidContinue::VALUE => Self::new::<XidContinue>(),
            XidStart::VALUE => Self::new::<XidStart>(),
            _ => return Err(crate::UnexpectedPropertyNameError),
        })
    }

    icu_provider::gen_any_buffer_data_constructors!(
        (name: &str) -> result: Result<CodePointSetData, crate::UnexpectedPropertyNameOrDataError>,
        functions: [
            new_for_ecma262: skip,
            new_for_ecma262_with_any_provider,
            new_for_ecma262_with_buffer_provider,
            try_new_for_ecma262_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_for_ecma262)]
    pub fn try_new_for_ecma262_unstable<P>(
        provider: &P,
        name: &str,
    ) -> Result<CodePointSetData, crate::UnexpectedPropertyNameOrDataError>
    where
        P: ?Sized
            + DataProvider<AsciiHexDigitV1Marker>
            + DataProvider<AlphabeticV1Marker>
            + DataProvider<BidiControlV1Marker>
            + DataProvider<BidiMirroredV1Marker>
            + DataProvider<CaseIgnorableV1Marker>
            + DataProvider<CasedV1Marker>
            + DataProvider<ChangesWhenCasefoldedV1Marker>
            + DataProvider<ChangesWhenCasemappedV1Marker>
            + DataProvider<ChangesWhenLowercasedV1Marker>
            + DataProvider<ChangesWhenNfkcCasefoldedV1Marker>
            + DataProvider<ChangesWhenTitlecasedV1Marker>
            + DataProvider<ChangesWhenUppercasedV1Marker>
            + DataProvider<DashV1Marker>
            + DataProvider<DefaultIgnorableCodePointV1Marker>
            + DataProvider<DeprecatedV1Marker>
            + DataProvider<DiacriticV1Marker>
            + DataProvider<EmojiV1Marker>
            + DataProvider<EmojiComponentV1Marker>
            + DataProvider<EmojiModifierV1Marker>
            + DataProvider<EmojiModifierBaseV1Marker>
            + DataProvider<EmojiPresentationV1Marker>
            + DataProvider<ExtendedPictographicV1Marker>
            + DataProvider<ExtenderV1Marker>
            + DataProvider<GraphemeBaseV1Marker>
            + DataProvider<GraphemeExtendV1Marker>
            + DataProvider<HexDigitV1Marker>
            + DataProvider<IdsBinaryOperatorV1Marker>
            + DataProvider<IdsTrinaryOperatorV1Marker>
            + DataProvider<IdContinueV1Marker>
            + DataProvider<IdStartV1Marker>
            + DataProvider<IdeographicV1Marker>
            + DataProvider<JoinControlV1Marker>
            + DataProvider<LogicalOrderExceptionV1Marker>
            + DataProvider<LowercaseV1Marker>
            + DataProvider<MathV1Marker>
            + DataProvider<NoncharacterCodePointV1Marker>
            + DataProvider<PatternSyntaxV1Marker>
            + DataProvider<PatternWhiteSpaceV1Marker>
            + DataProvider<QuotationMarkV1Marker>
            + DataProvider<RadicalV1Marker>
            + DataProvider<RegionalIndicatorV1Marker>
            + DataProvider<SentenceTerminalV1Marker>
            + DataProvider<SoftDottedV1Marker>
            + DataProvider<TerminalPunctuationV1Marker>
            + DataProvider<UnifiedIdeographV1Marker>
            + DataProvider<UppercaseV1Marker>
            + DataProvider<VariationSelectorV1Marker>
            + DataProvider<WhiteSpaceV1Marker>
            + DataProvider<XidContinueV1Marker>
            + DataProvider<XidStartV1Marker>,
    {
        use crate::props::*;

        let prop = if let Some(prop) = UnicodeProperty::parse_ecma262_name(name) {
            prop
        } else {
            return Err(crate::UnexpectedPropertyNameOrDataError::UnexpectedPropertyName);
        };
        Ok(match prop {
            AsciiHexDigit::VALUE => Self::try_new_unstable::<AsciiHexDigit>(provider),
            Alphabetic::VALUE => Self::try_new_unstable::<Alphabetic>(provider),
            BidiControl::VALUE => Self::try_new_unstable::<BidiControl>(provider),
            BidiMirrored::VALUE => Self::try_new_unstable::<BidiMirrored>(provider),
            CaseIgnorable::VALUE => Self::try_new_unstable::<CaseIgnorable>(provider),
            Cased::VALUE => Self::try_new_unstable::<Cased>(provider),
            ChangesWhenCasefolded::VALUE => {
                Self::try_new_unstable::<ChangesWhenCasefolded>(provider)
            }
            ChangesWhenCasemapped::VALUE => {
                Self::try_new_unstable::<ChangesWhenCasemapped>(provider)
            }
            ChangesWhenLowercased::VALUE => {
                Self::try_new_unstable::<ChangesWhenLowercased>(provider)
            }
            ChangesWhenNfkcCasefolded::VALUE => {
                Self::try_new_unstable::<ChangesWhenNfkcCasefolded>(provider)
            }
            ChangesWhenTitlecased::VALUE => {
                Self::try_new_unstable::<ChangesWhenTitlecased>(provider)
            }
            ChangesWhenUppercased::VALUE => {
                Self::try_new_unstable::<ChangesWhenUppercased>(provider)
            }
            Dash::VALUE => Self::try_new_unstable::<Dash>(provider),
            DefaultIgnorableCodePoint::VALUE => {
                Self::try_new_unstable::<DefaultIgnorableCodePoint>(provider)
            }
            Deprecated::VALUE => Self::try_new_unstable::<Deprecated>(provider),
            Diacritic::VALUE => Self::try_new_unstable::<Diacritic>(provider),
            Emoji::VALUE => Self::try_new_unstable::<Emoji>(provider),
            EmojiComponent::VALUE => Self::try_new_unstable::<EmojiComponent>(provider),
            EmojiModifier::VALUE => Self::try_new_unstable::<EmojiModifier>(provider),
            EmojiModifierBase::VALUE => Self::try_new_unstable::<EmojiModifierBase>(provider),
            EmojiPresentation::VALUE => Self::try_new_unstable::<EmojiPresentation>(provider),
            ExtendedPictographic::VALUE => Self::try_new_unstable::<ExtendedPictographic>(provider),
            Extender::VALUE => Self::try_new_unstable::<Extender>(provider),
            GraphemeBase::VALUE => Self::try_new_unstable::<GraphemeBase>(provider),
            GraphemeExtend::VALUE => Self::try_new_unstable::<GraphemeExtend>(provider),
            HexDigit::VALUE => Self::try_new_unstable::<HexDigit>(provider),
            IdsBinaryOperator::VALUE => Self::try_new_unstable::<IdsBinaryOperator>(provider),
            IdsTrinaryOperator::VALUE => Self::try_new_unstable::<IdsTrinaryOperator>(provider),
            IdContinue::VALUE => Self::try_new_unstable::<IdContinue>(provider),
            IdStart::VALUE => Self::try_new_unstable::<IdStart>(provider),
            Ideographic::VALUE => Self::try_new_unstable::<Ideographic>(provider),
            JoinControl::VALUE => Self::try_new_unstable::<JoinControl>(provider),
            LogicalOrderException::VALUE => {
                Self::try_new_unstable::<LogicalOrderException>(provider)
            }
            Lowercase::VALUE => Self::try_new_unstable::<Lowercase>(provider),
            Math::VALUE => Self::try_new_unstable::<Math>(provider),
            NoncharacterCodePoint::VALUE => {
                Self::try_new_unstable::<NoncharacterCodePoint>(provider)
            }
            PatternSyntax::VALUE => Self::try_new_unstable::<PatternSyntax>(provider),
            PatternWhiteSpace::VALUE => Self::try_new_unstable::<PatternWhiteSpace>(provider),
            QuotationMark::VALUE => Self::try_new_unstable::<QuotationMark>(provider),
            Radical::VALUE => Self::try_new_unstable::<Radical>(provider),
            RegionalIndicator::VALUE => Self::try_new_unstable::<RegionalIndicator>(provider),
            SentenceTerminal::VALUE => Self::try_new_unstable::<SentenceTerminal>(provider),
            SoftDotted::VALUE => Self::try_new_unstable::<SoftDotted>(provider),
            TerminalPunctuation::VALUE => Self::try_new_unstable::<TerminalPunctuation>(provider),
            UnifiedIdeograph::VALUE => Self::try_new_unstable::<UnifiedIdeograph>(provider),
            Uppercase::VALUE => Self::try_new_unstable::<Uppercase>(provider),
            VariationSelector::VALUE => Self::try_new_unstable::<VariationSelector>(provider),
            WhiteSpace::VALUE => Self::try_new_unstable::<WhiteSpace>(provider),
            XidContinue::VALUE => Self::try_new_unstable::<XidContinue>(provider),
            XidStart::VALUE => Self::try_new_unstable::<XidStart>(provider),
            _ => Err(DataError::custom("Unknown property")),
        }?)
    }

    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This owned version if returned by functions that use a runtime data provider.
    #[inline]
    pub fn as_borrowed(&self) -> CodePointSetDataBorrowed<'_> {
        CodePointSetDataBorrowed {
            set: self.data.get(),
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters like [`load_ascii_hex_digit()`] instead
    pub(crate) fn from_data<M>(data: DataPayload<M>) -> Self
    where
        M: DynamicDataMarker<DataStruct = PropertyCodePointSetV1<'static>>,
    {
        Self { data: data.cast() }
    }

    /// Construct a new owned [`CodePointInversionList`]
    pub fn from_code_point_inversion_list(set: CodePointInversionList<'static>) -> Self {
        let set = PropertyCodePointSetV1::from_code_point_inversion_list(set);
        CodePointSetData::from_data(
            DataPayload::<ErasedPropertyCodePointSetV1Marker>::from_owned(set),
        )
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
#[derive(Clone, Copy, Debug)]
pub struct CodePointSetDataBorrowed<'a> {
    set: &'a PropertyCodePointSetV1<'a>,
}

impl CodePointSetDataBorrowed<'static> {
    /// Cheaply converts a [`CodePointSetDataBorrowed<'static>`] into a [`CodePointSetData`].
    ///
    /// Note: Due to branching and indirection, using [`CodePointSetData`] might inhibit some
    /// compile-time optimizations that are possible with [`CodePointSetDataBorrowed`].
    pub const fn static_to_owned(self) -> CodePointSetData {
        CodePointSetData {
            data: DataPayload::from_static_ref(self.set),
        }
    }
}

impl<'a> CodePointSetDataBorrowed<'a> {
    /// Check if the set contains a character
    ///
    /// ```rust
    /// use icu::properties::sets;
    ///
    /// let alphabetic = sets::alphabetic();
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
    /// use icu::properties::sets;
    ///
    /// let alphabetic = sets::alphabetic();
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
    /// use icu::properties::sets;
    ///
    /// let alphabetic = sets::alphabetic();
    /// let mut ranges = alphabetic.iter_ranges();
    ///
    /// assert_eq!(Some(0x0041..=0x005A), ranges.next()); // 'A'..'Z'
    /// assert_eq!(Some(0x0061..=0x007A), ranges.next()); // 'a'..'z'
    /// ```
    #[inline]
    pub fn iter_ranges(self) -> impl Iterator<Item = RangeInclusive<u32>> + 'a {
        self.set.iter_ranges()
    }

    // Yields an [`Iterator`] returning the ranges of the code points that are
    /// *not* included in the [`CodePointSetData`]
    ///
    /// Ranges are returned as [`RangeInclusive`], which is inclusive of its
    /// `end` bound value. An end-inclusive behavior matches the ICU4C/J
    /// behavior of ranges, ex: `UnicodeSet::contains(UChar32 start, UChar32 end)`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::sets;
    ///
    /// let alphabetic = sets::alphabetic();
    /// let mut ranges = alphabetic.iter_ranges();
    ///
    /// assert_eq!(Some(0x0041..=0x005A), ranges.next()); // 'A'..'Z'
    /// assert_eq!(Some(0x0061..=0x007A), ranges.next()); // 'a'..'z'
    /// ```
    #[inline]
    pub fn iter_ranges_complemented(self) -> impl Iterator<Item = RangeInclusive<u32>> + 'a {
        self.set.iter_ranges_complemented()
    }
}

/// TODO
pub trait CodePointSetProperty: crate::private::Sealed {
    #[doc(hidden)]
    type DataMarker: DataMarker<DataStruct = PropertyCodePointSetV1<'static>>;
    #[doc(hidden)]
    #[cfg(feature = "compiled_data")]
    const SINGLETON: &'static PropertyCodePointSetV1<'static>;
    #[doc(hidden)]
    const VALUE: UnicodeProperty;
}
