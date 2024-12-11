// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_properties::props::GeneralCategory;
    use icu_properties::props::{
        Alnum, Alphabetic, AsciiHexDigit, BidiControl, BidiMirrored, Blank, CaseIgnorable,
        CaseSensitive, Cased, ChangesWhenCasefolded, ChangesWhenCasemapped, ChangesWhenLowercased,
        ChangesWhenNfkcCasefolded, ChangesWhenTitlecased, ChangesWhenUppercased, Dash,
        DefaultIgnorableCodePoint, Deprecated, Diacritic, Emoji, EmojiComponent, EmojiModifier,
        EmojiModifierBase, EmojiPresentation, ExtendedPictographic, Extender,
        FullCompositionExclusion, Graph, GraphemeBase, GraphemeExtend, GraphemeLink, HexDigit,
        Hyphen, IdContinue, IdStart, Ideographic, IdsBinaryOperator, IdsTrinaryOperator,
        JoinControl, LogicalOrderException, Lowercase, Math, NfcInert, NfdInert, NfkcInert,
        NfkdInert, NoncharacterCodePoint, PatternSyntax, PatternWhiteSpace,
        PrependedConcatenationMark, Print, QuotationMark, Radical, RegionalIndicator,
        SegmentStarter, SentenceTerminal, SoftDotted, TerminalPunctuation, UnifiedIdeograph,
        Uppercase, VariationSelector, WhiteSpace, Xdigit, XidContinue, XidStart,
    };

    use crate::errors::ffi::DataError;
    use crate::properties_iter::ffi::CodePointRangeIterator;
    use crate::provider::ffi::DataProvider;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    #[diplomat::rust_link(icu::properties, Mod)]
    #[diplomat::rust_link(icu::properties::CodePointSetData, Struct)]
    #[diplomat::rust_link(icu::properties::CodePointSetData::new, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::properties::CodePointSetDataBorrowed, Struct)]
    pub struct CodePointSetData(pub icu_properties::CodePointSetData);

    impl CodePointSetData {
        /// Checks whether the code point is in the set.
        #[diplomat::rust_link(icu::properties::CodePointSetDataBorrowed::contains, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::CodePointSetDataBorrowed::contains32,
            FnInStruct,
            hidden
        )]
        pub fn contains(&self, cp: DiplomatChar) -> bool {
            self.0.as_borrowed().contains32(cp)
        }

        /// Produces an iterator over ranges of code points contained in this set
        #[diplomat::rust_link(icu::properties::CodePointSetDataBorrowed::iter_ranges, FnInStruct)]
        pub fn iter_ranges<'a>(&'a self) -> Box<CodePointRangeIterator<'a>> {
            Box::new(CodePointRangeIterator(Box::new(
                self.0.as_borrowed().iter_ranges(),
            )))
        }

        /// Produces an iterator over ranges of code points not contained in this set
        #[diplomat::rust_link(
            icu::properties::CodePointSetDataBorrowed::iter_ranges_complemented,
            FnInStruct
        )]
        pub fn iter_ranges_complemented<'a>(&'a self) -> Box<CodePointRangeIterator<'a>> {
            Box::new(CodePointRangeIterator(Box::new(
                self.0.as_borrowed().iter_ranges_complemented(),
            )))
        }

        /// Produces a set for obtaining General Category Group values
        /// which is a mask with the same format as the `U_GC_XX_MASK` mask in ICU4C, using compiled data.
        #[diplomat::rust_link(icu::properties::props::GeneralCategoryGroup, Struct)]
        #[diplomat::rust_link(
            icu::properties::CodePointMapDataBorrowed::get_set_for_value_group,
            FnInStruct
        )]
        #[diplomat::attr(auto, named_constructor = "general_category_group")]
        #[cfg(feature = "compiled_data")]
        pub fn create_general_category_group(group: u32) -> Box<CodePointSetData> {
            let data = icu_properties::CodePointMapData::<GeneralCategory>::new().static_to_owned();

            Box::new(CodePointSetData(
                data.as_borrowed().get_set_for_value_group(group.into()),
            ))
        }

        /// Produces a set for obtaining General Category Group values
        /// which is a mask with the same format as the `U_GC_XX_MASK` mask in ICU4C, using a provided data source.
        #[diplomat::rust_link(icu::properties::props::GeneralCategoryGroup, Struct)]
        #[diplomat::rust_link(
            icu::properties::CodePointMapDataBorrowed::get_set_for_value_group,
            FnInStruct
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "general_category_group_with_provider")]
        pub fn create_general_category_group_with_provider(
            provider: &DataProvider,
            group: u32,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(
                call_constructor_unstable!(
                    icu_properties::CodePointMapData::<GeneralCategory>::new [r => Ok(r.static_to_owned())],
                    icu_properties::CodePointMapData::<GeneralCategory>::try_new_unstable,
                    provider,
                )?
                .as_borrowed()
                .get_set_for_value_group(group.into()),
            )))
        }

        #[diplomat::rust_link(icu::properties::props::AsciiHexDigit, Struct)]
        #[diplomat::attr(auto, named_constructor = "ascii_hex_digit")]
        #[cfg(feature = "compiled_data")]
        pub fn create_ascii_hex_digit() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<AsciiHexDigit>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::AsciiHexDigit, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "ascii_hex_digit_with_provider")]
        pub fn create_ascii_hex_digit_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<AsciiHexDigit> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<AsciiHexDigit>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Alnum, Struct)]
        #[diplomat::attr(auto, named_constructor = "alnum")]
        #[cfg(feature = "compiled_data")]
        pub fn create_alnum() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Alnum>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Alnum, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "alnum_with_provider")]
        pub fn create_alnum_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Alnum> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Alnum>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Alphabetic, Struct)]
        #[diplomat::attr(auto, named_constructor = "alphabetic")]
        #[cfg(feature = "compiled_data")]
        pub fn create_alphabetic() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Alphabetic>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Alphabetic, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "alphabetic_with_provider")]
        pub fn create_alphabetic_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Alphabetic> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Alphabetic>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::BidiControl, Struct)]
        #[diplomat::attr(auto, named_constructor = "bidi_control")]
        #[cfg(feature = "compiled_data")]
        pub fn create_bidi_control() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<BidiControl>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::BidiControl, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "bidi_control_with_provider")]
        pub fn create_bidi_control_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<BidiControl> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<BidiControl>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::BidiMirrored, Struct)]
        #[diplomat::attr(auto, named_constructor = "bidi_mirrored")]
        #[cfg(feature = "compiled_data")]
        pub fn create_bidi_mirrored() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<BidiMirrored>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::BidiMirrored, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "bidi_mirrored_with_provider")]
        pub fn create_bidi_mirrored_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<BidiMirrored> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<BidiMirrored>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Blank, Struct)]
        #[diplomat::attr(auto, named_constructor = "blank")]
        #[cfg(feature = "compiled_data")]
        pub fn create_blank() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Blank>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Blank, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "blank_with_provider")]
        pub fn create_blank_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Blank> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Blank>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Cased, Struct)]
        #[diplomat::attr(auto, named_constructor = "cased")]
        #[cfg(feature = "compiled_data")]
        pub fn create_cased() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Cased>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Cased, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "cased_with_provider")]
        pub fn create_cased_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Cased> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Cased>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::CaseIgnorable, Struct)]
        #[diplomat::attr(auto, named_constructor = "case_ignorable")]
        #[cfg(feature = "compiled_data")]
        pub fn create_case_ignorable() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<CaseIgnorable>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::CaseIgnorable, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "case_ignorable_with_provider")]
        pub fn create_case_ignorable_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<CaseIgnorable> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<CaseIgnorable>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::FullCompositionExclusion, Struct)]
        #[diplomat::attr(auto, named_constructor = "full_composition_exclusion")]
        #[cfg(feature = "compiled_data")]
        pub fn create_full_composition_exclusion() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<FullCompositionExclusion>()
                    .static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::FullCompositionExclusion, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "full_composition_exclusion_with_provider")]
        pub fn create_full_composition_exclusion_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<FullCompositionExclusion> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<FullCompositionExclusion>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenCasefolded, Struct)]
        #[diplomat::attr(auto, named_constructor = "changes_when_casefolded")]
        #[cfg(feature = "compiled_data")]
        pub fn create_changes_when_casefolded() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<ChangesWhenCasefolded>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenCasefolded, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_casefolded_with_provider")]
        pub fn create_changes_when_casefolded_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<ChangesWhenCasefolded> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<ChangesWhenCasefolded>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenCasemapped, Struct)]
        #[diplomat::attr(auto, named_constructor = "changes_when_casemapped")]
        #[cfg(feature = "compiled_data")]
        pub fn create_changes_when_casemapped() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<ChangesWhenCasemapped>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenCasemapped, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_casemapped_with_provider")]
        pub fn create_changes_when_casemapped_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<ChangesWhenCasemapped> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<ChangesWhenCasemapped>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenNfkcCasefolded, Struct)]
        #[diplomat::attr(auto, named_constructor = "changes_when_nfkc_casefolded")]
        #[cfg(feature = "compiled_data")]
        pub fn create_changes_when_nfkc_casefolded() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<ChangesWhenNfkcCasefolded>()
                    .static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenNfkcCasefolded, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_nfkc_casefolded_with_provider")]
        pub fn create_changes_when_nfkc_casefolded_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<ChangesWhenNfkcCasefolded> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<ChangesWhenNfkcCasefolded>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenLowercased, Struct)]
        #[diplomat::attr(auto, named_constructor = "changes_when_lowercased")]
        #[cfg(feature = "compiled_data")]
        pub fn create_changes_when_lowercased() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<ChangesWhenLowercased>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenLowercased, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_lowercased_with_provider")]
        pub fn create_changes_when_lowercased_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<ChangesWhenLowercased> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<ChangesWhenLowercased>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenTitlecased, Struct)]
        #[diplomat::attr(auto, named_constructor = "changes_when_titlecased")]
        #[cfg(feature = "compiled_data")]
        pub fn create_changes_when_titlecased() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<ChangesWhenTitlecased>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenTitlecased, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_titlecased_with_provider")]
        pub fn create_changes_when_titlecased_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<ChangesWhenTitlecased> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<ChangesWhenTitlecased>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenUppercased, Struct)]
        #[diplomat::attr(auto, named_constructor = "changes_when_uppercased")]
        #[cfg(feature = "compiled_data")]
        pub fn create_changes_when_uppercased() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<ChangesWhenUppercased>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::ChangesWhenUppercased, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_uppercased_with_provider")]
        pub fn create_changes_when_uppercased_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<ChangesWhenUppercased> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<ChangesWhenUppercased>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Dash, Struct)]
        #[diplomat::attr(auto, named_constructor = "dash")]
        #[cfg(feature = "compiled_data")]
        pub fn create_dash() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Dash>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Dash, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "dash_with_provider")]
        pub fn create_dash_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Dash> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Dash>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Deprecated, Struct)]
        #[diplomat::attr(auto, named_constructor = "deprecated")]
        #[cfg(feature = "compiled_data")]
        pub fn create_deprecated() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Deprecated>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Deprecated, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "deprecated_with_provider")]
        pub fn create_deprecated_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Deprecated> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Deprecated>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::DefaultIgnorableCodePoint, Struct)]
        #[diplomat::attr(auto, named_constructor = "default_ignorable_code_point")]
        #[cfg(feature = "compiled_data")]
        pub fn create_default_ignorable_code_point() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<DefaultIgnorableCodePoint>()
                    .static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::DefaultIgnorableCodePoint, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "default_ignorable_code_point_with_provider")]
        pub fn create_default_ignorable_code_point_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<DefaultIgnorableCodePoint> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<DefaultIgnorableCodePoint>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Diacritic, Struct)]
        #[diplomat::attr(auto, named_constructor = "diacritic")]
        #[cfg(feature = "compiled_data")]
        pub fn create_diacritic() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Diacritic>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Diacritic, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "diacritic_with_provider")]
        pub fn create_diacritic_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Diacritic> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Diacritic>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::EmojiModifierBase, Struct)]
        #[diplomat::attr(auto, named_constructor = "emoji_modifier_base")]
        #[cfg(feature = "compiled_data")]
        pub fn create_emoji_modifier_base() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<EmojiModifierBase>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::EmojiModifierBase, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji_modifier_base_with_provider")]
        pub fn create_emoji_modifier_base_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<EmojiModifierBase> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<EmojiModifierBase>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::EmojiComponent, Struct)]
        #[diplomat::attr(auto, named_constructor = "emoji_component")]
        #[cfg(feature = "compiled_data")]
        pub fn create_emoji_component() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<EmojiComponent>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::EmojiComponent, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji_component_with_provider")]
        pub fn create_emoji_component_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<EmojiComponent> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<EmojiComponent>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::EmojiModifier, Struct)]
        #[diplomat::attr(auto, named_constructor = "emoji_modifier")]
        #[cfg(feature = "compiled_data")]
        pub fn create_emoji_modifier() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<EmojiModifier>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::EmojiModifier, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji_modifier_with_provider")]
        pub fn create_emoji_modifier_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<EmojiModifier> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<EmojiModifier>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Emoji, Struct)]
        #[diplomat::attr(auto, named_constructor = "emoji")]
        #[cfg(feature = "compiled_data")]
        pub fn create_emoji() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Emoji>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Emoji, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji_with_provider")]
        pub fn create_emoji_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Emoji> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Emoji>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::EmojiPresentation, Struct)]
        #[diplomat::attr(auto, named_constructor = "emoji_presentation")]
        #[cfg(feature = "compiled_data")]
        pub fn create_emoji_presentation() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<EmojiPresentation>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::EmojiPresentation, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji_presentation_with_provider")]
        pub fn create_emoji_presentation_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<EmojiPresentation> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<EmojiPresentation>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Extender, Struct)]
        #[diplomat::attr(auto, named_constructor = "extender")]
        #[cfg(feature = "compiled_data")]
        pub fn create_extender() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Extender>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Extender, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "extender_with_provider")]
        pub fn create_extender_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Extender> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Extender>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::ExtendedPictographic, Struct)]
        #[diplomat::attr(auto, named_constructor = "extended_pictographic")]
        #[cfg(feature = "compiled_data")]
        pub fn create_extended_pictographic() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<ExtendedPictographic>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::ExtendedPictographic, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "extended_pictographic_with_provider")]
        pub fn create_extended_pictographic_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<ExtendedPictographic> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<ExtendedPictographic>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Graph, Struct)]
        #[diplomat::attr(auto, named_constructor = "graph")]
        #[cfg(feature = "compiled_data")]
        pub fn create_graph() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Graph>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Graph, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "graph_with_provider")]
        pub fn create_graph_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Graph> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Graph>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::GraphemeBase, Struct)]
        #[diplomat::attr(auto, named_constructor = "grapheme_base")]
        #[cfg(feature = "compiled_data")]
        pub fn create_grapheme_base() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<GraphemeBase>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::GraphemeBase, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "grapheme_base_with_provider")]
        pub fn create_grapheme_base_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<GraphemeBase> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<GraphemeBase>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::GraphemeExtend, Struct)]
        #[diplomat::attr(auto, named_constructor = "grapheme_extend")]
        #[cfg(feature = "compiled_data")]
        pub fn create_grapheme_extend() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<GraphemeExtend>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::GraphemeExtend, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "grapheme_extend_with_provider")]
        pub fn create_grapheme_extend_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<GraphemeExtend> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<GraphemeExtend>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::GraphemeLink, Struct)]
        #[diplomat::attr(auto, named_constructor = "grapheme_link")]
        #[cfg(feature = "compiled_data")]
        pub fn create_grapheme_link() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<GraphemeLink>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::GraphemeLink, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "grapheme_link_with_provider")]
        pub fn create_grapheme_link_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<GraphemeLink> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<GraphemeLink>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::HexDigit, Struct)]
        #[diplomat::attr(auto, named_constructor = "hex_digit")]
        #[cfg(feature = "compiled_data")]
        pub fn create_hex_digit() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<HexDigit>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::HexDigit, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "hex_digit_with_provider")]
        pub fn create_hex_digit_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<HexDigit> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<HexDigit>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Hyphen, Struct)]
        #[diplomat::attr(auto, named_constructor = "hyphen")]
        #[cfg(feature = "compiled_data")]
        pub fn create_hyphen() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Hyphen>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Hyphen, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "hyphen_with_provider")]
        pub fn create_hyphen_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Hyphen> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Hyphen>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::IdContinue, Struct)]
        #[diplomat::attr(auto, named_constructor = "id_continue")]
        #[cfg(feature = "compiled_data")]
        pub fn create_id_continue() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<IdContinue>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::IdContinue, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "id_continue_with_provider")]
        pub fn create_id_continue_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<IdContinue> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<IdContinue>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Ideographic, Struct)]
        #[diplomat::attr(auto, named_constructor = "ideographic")]
        #[cfg(feature = "compiled_data")]
        pub fn create_ideographic() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Ideographic>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Ideographic, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "ideographic_with_provider")]
        pub fn create_ideographic_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Ideographic> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Ideographic>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::IdStart, Struct)]
        #[diplomat::attr(auto, named_constructor = "id_start")]
        #[cfg(feature = "compiled_data")]
        pub fn create_id_start() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<IdStart>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::IdStart, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "id_start_with_provider")]
        pub fn create_id_start_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<IdStart> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<IdStart>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::IdsBinaryOperator, Struct)]
        #[diplomat::attr(auto, named_constructor = "ids_binary_operator")]
        #[cfg(feature = "compiled_data")]
        pub fn create_ids_binary_operator() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<IdsBinaryOperator>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::IdsBinaryOperator, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "ids_binary_operator_with_provider")]
        pub fn create_ids_binary_operator_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<IdsBinaryOperator> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<IdsBinaryOperator>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::IdsTrinaryOperator, Struct)]
        #[diplomat::attr(auto, named_constructor = "ids_trinary_operator")]
        #[cfg(feature = "compiled_data")]
        pub fn create_ids_trinary_operator() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<IdsTrinaryOperator>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::IdsTrinaryOperator, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "ids_trinary_operator_with_provider")]
        pub fn create_ids_trinary_operator_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<IdsTrinaryOperator> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<IdsTrinaryOperator>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::JoinControl, Struct)]
        #[diplomat::attr(auto, named_constructor = "join_control")]
        #[cfg(feature = "compiled_data")]
        pub fn create_join_control() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<JoinControl>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::JoinControl, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "join_control_with_provider")]
        pub fn create_join_control_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<JoinControl> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<JoinControl>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::LogicalOrderException, Struct)]
        #[diplomat::attr(auto, named_constructor = "logical_order_exception")]
        #[cfg(feature = "compiled_data")]
        pub fn create_logical_order_exception() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<LogicalOrderException>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::LogicalOrderException, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "logical_order_exception_with_provider")]
        pub fn create_logical_order_exception_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<LogicalOrderException> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<LogicalOrderException>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Lowercase, Struct)]
        #[diplomat::attr(auto, named_constructor = "lowercase")]
        #[cfg(feature = "compiled_data")]
        pub fn create_lowercase() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Lowercase>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Lowercase, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "lowercase_with_provider")]
        pub fn create_lowercase_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Lowercase> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Lowercase>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Math, Struct)]
        #[diplomat::attr(auto, named_constructor = "math")]
        #[cfg(feature = "compiled_data")]
        pub fn create_math() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Math>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Math, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "math_with_provider")]
        pub fn create_math_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Math> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Math>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::NoncharacterCodePoint, Struct)]
        #[diplomat::attr(auto, named_constructor = "noncharacter_code_point")]
        #[cfg(feature = "compiled_data")]
        pub fn create_noncharacter_code_point() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<NoncharacterCodePoint>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::NoncharacterCodePoint, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "noncharacter_code_point_with_provider")]
        pub fn create_noncharacter_code_point_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<NoncharacterCodePoint> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<NoncharacterCodePoint>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::NfcInert, Struct)]
        #[diplomat::attr(auto, named_constructor = "nfc_inert")]
        #[cfg(feature = "compiled_data")]
        pub fn create_nfc_inert() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<NfcInert>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::NfcInert, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "nfc_inert_with_provider")]
        pub fn create_nfc_inert_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<NfcInert> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<NfcInert>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::NfdInert, Struct)]
        #[diplomat::attr(auto, named_constructor = "nfd_inert")]
        #[cfg(feature = "compiled_data")]
        pub fn create_nfd_inert() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<NfdInert>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::NfdInert, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "nfd_inert_with_provider")]
        pub fn create_nfd_inert_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<NfdInert> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<NfdInert>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::NfkcInert, Struct)]
        #[diplomat::attr(auto, named_constructor = "nfkc_inert")]
        #[cfg(feature = "compiled_data")]
        pub fn create_nfkc_inert() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<NfkcInert>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::NfkcInert, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "nfkc_inert_with_provider")]
        pub fn create_nfkc_inert_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<NfkcInert> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<NfkcInert>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::NfkdInert, Struct)]
        #[diplomat::attr(auto, named_constructor = "nfkd_inert")]
        #[cfg(feature = "compiled_data")]
        pub fn create_nfkd_inert() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<NfkdInert>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::NfkdInert, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "nfkd_inert_with_provider")]
        pub fn create_nfkd_inert_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<NfkdInert> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<NfkdInert>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::PatternSyntax, Struct)]
        #[diplomat::attr(auto, named_constructor = "pattern_syntax")]
        #[cfg(feature = "compiled_data")]
        pub fn create_pattern_syntax() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<PatternSyntax>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::PatternSyntax, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "pattern_syntax_with_provider")]
        pub fn create_pattern_syntax_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<PatternSyntax> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<PatternSyntax>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::PatternWhiteSpace, Struct)]
        #[diplomat::attr(auto, named_constructor = "pattern_white_space")]
        #[cfg(feature = "compiled_data")]
        pub fn create_pattern_white_space() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<PatternWhiteSpace>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::PatternWhiteSpace, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "pattern_white_space_with_provider")]
        pub fn create_pattern_white_space_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<PatternWhiteSpace> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<PatternWhiteSpace>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::PrependedConcatenationMark, Struct)]
        #[diplomat::attr(auto, named_constructor = "prepended_concatenation_mark")]
        #[cfg(feature = "compiled_data")]
        pub fn create_prepended_concatenation_mark() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<PrependedConcatenationMark>()
                    .static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::PrependedConcatenationMark, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "prepended_concatenation_mark_with_provider")]
        pub fn create_prepended_concatenation_mark_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<PrependedConcatenationMark> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<PrependedConcatenationMark>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Print, Struct)]
        #[diplomat::attr(auto, named_constructor = "print")]
        #[cfg(feature = "compiled_data")]
        pub fn create_print() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Print>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Print, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "print_with_provider")]
        pub fn create_print_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Print> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Print>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::QuotationMark, Struct)]
        #[diplomat::attr(auto, named_constructor = "quotation_mark")]
        #[cfg(feature = "compiled_data")]
        pub fn create_quotation_mark() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<QuotationMark>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::QuotationMark, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "quotation_mark_with_provider")]
        pub fn create_quotation_mark_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<QuotationMark> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<QuotationMark>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Radical, Struct)]
        #[diplomat::attr(auto, named_constructor = "radical")]
        #[cfg(feature = "compiled_data")]
        pub fn create_radical() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Radical>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Radical, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "radical_with_provider")]
        pub fn create_radical_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Radical> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Radical>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::RegionalIndicator, Struct)]
        #[diplomat::attr(auto, named_constructor = "regional_indicator")]
        #[cfg(feature = "compiled_data")]
        pub fn create_regional_indicator() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<RegionalIndicator>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::RegionalIndicator, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "regional_indicator_with_provider")]
        pub fn create_regional_indicator_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<RegionalIndicator> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<RegionalIndicator>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::SoftDotted, Struct)]
        #[diplomat::attr(auto, named_constructor = "soft_dotted")]
        #[cfg(feature = "compiled_data")]
        pub fn create_soft_dotted() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<SoftDotted>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::SoftDotted, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "soft_dotted_with_provider")]
        pub fn create_soft_dotted_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<SoftDotted> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<SoftDotted>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::SegmentStarter, Struct)]
        #[diplomat::attr(auto, named_constructor = "segment_starter")]
        #[cfg(feature = "compiled_data")]
        pub fn create_segment_starter() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<SegmentStarter>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::SegmentStarter, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "segment_starter_with_provider")]
        pub fn create_segment_starter_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<SegmentStarter> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<SegmentStarter>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::CaseSensitive, Struct)]
        #[diplomat::attr(auto, named_constructor = "case_sensitive")]
        #[cfg(feature = "compiled_data")]
        pub fn create_case_sensitive() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<CaseSensitive>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::CaseSensitive, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "case_sensitive_with_provider")]
        pub fn create_case_sensitive_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<CaseSensitive> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<CaseSensitive>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::SentenceTerminal, Struct)]
        #[diplomat::attr(auto, named_constructor = "sentence_terminal")]
        #[cfg(feature = "compiled_data")]
        pub fn create_sentence_terminal() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<SentenceTerminal>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::SentenceTerminal, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "sentence_terminal_with_provider")]
        pub fn create_sentence_terminal_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<SentenceTerminal> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<SentenceTerminal>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::TerminalPunctuation, Struct)]
        #[diplomat::attr(auto, named_constructor = "terminal_punctuation")]
        #[cfg(feature = "compiled_data")]
        pub fn create_terminal_punctuation() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<TerminalPunctuation>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::TerminalPunctuation, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "terminal_punctuation_with_provider")]
        pub fn create_terminal_punctuation_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<TerminalPunctuation> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<TerminalPunctuation>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::UnifiedIdeograph, Struct)]
        #[diplomat::attr(auto, named_constructor = "unified_ideograph")]
        #[cfg(feature = "compiled_data")]
        pub fn create_unified_ideograph() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<UnifiedIdeograph>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::UnifiedIdeograph, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "unified_ideograph_with_provider")]
        pub fn create_unified_ideograph_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<UnifiedIdeograph> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<UnifiedIdeograph>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Uppercase, Struct)]
        #[diplomat::attr(auto, named_constructor = "uppercase")]
        #[cfg(feature = "compiled_data")]
        pub fn create_uppercase() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Uppercase>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Uppercase, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "uppercase_with_provider")]
        pub fn create_uppercase_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Uppercase> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Uppercase>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::VariationSelector, Struct)]
        #[diplomat::attr(auto, named_constructor = "variation_selector")]
        #[cfg(feature = "compiled_data")]
        pub fn create_variation_selector() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<VariationSelector>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::VariationSelector, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "variation_selector_with_provider")]
        pub fn create_variation_selector_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<VariationSelector> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<VariationSelector>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::WhiteSpace, Struct)]
        #[diplomat::attr(auto, named_constructor = "white_space")]
        #[cfg(feature = "compiled_data")]
        pub fn create_white_space() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<WhiteSpace>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::WhiteSpace, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "white_space_with_provider")]
        pub fn create_white_space_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<WhiteSpace> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<WhiteSpace>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::Xdigit, Struct)]
        #[diplomat::attr(auto, named_constructor = "xdigit")]
        #[cfg(feature = "compiled_data")]
        pub fn create_xdigit() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<Xdigit>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::Xdigit, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "xdigit_with_provider")]
        pub fn create_xdigit_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<Xdigit> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<Xdigit>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::XidContinue, Struct)]
        #[diplomat::attr(auto, named_constructor = "xid_continue")]
        #[cfg(feature = "compiled_data")]
        pub fn create_xid_continue() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<XidContinue>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::XidContinue, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "xid_continue_with_provider")]
        pub fn create_xid_continue_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<XidContinue> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<XidContinue>,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::props::XidStart, Struct)]
        #[diplomat::attr(auto, named_constructor = "xid_start")]
        #[cfg(feature = "compiled_data")]
        pub fn create_xid_start() -> Box<CodePointSetData> {
            Box::new(CodePointSetData(
                icu_properties::CodePointSetData::new::<XidStart>().static_to_owned(),
            ))
        }

        #[diplomat::rust_link(icu::properties::props::XidStart, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "xid_start_with_provider")]
        pub fn create_xid_start_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new::<XidStart> [r => Ok(r.static_to_owned())],
                icu_properties::CodePointSetData::try_new_unstable::<XidStart>,
                provider
            )?)))
        }

        /// [ecma]: https://tc39.es/ecma262/#table-binary-unicode-properties
        #[diplomat::rust_link(icu::properties::CodePointSetData::new_for_ecma262, FnInStruct)]
        #[diplomat::attr(auto, named_constructor = "for_ecma262")]
        #[cfg(feature = "compiled_data")]
        pub fn create_for_ecma262(
            property_name: &DiplomatStr,
        ) -> Result<Box<CodePointSetData>, DataError> {
            let data = icu_properties::CodePointSetData::new_for_ecma262(property_name)
                .ok_or(DataError::Custom)?
                .static_to_owned();
            Ok(Box::new(CodePointSetData(data)))
        }

        /// [ecma]: https://tc39.es/ecma262/#table-binary-unicode-properties
        #[diplomat::rust_link(icu::properties::CodePointSetData::new_for_ecma262, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "for_ecma262_with_provider")]
        pub fn create_for_ecma262_with_provider(
            provider: &DataProvider,
            property_name: &DiplomatStr,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::CodePointSetData::new_for_ecma262 [r => r.map(|d| Ok(d.static_to_owned()))],
                icu_properties::CodePointSetData::try_new_for_ecma262_unstable,
                provider,
                property_name
            ).ok_or(DataError::Custom)??)))
        }
    }
}
