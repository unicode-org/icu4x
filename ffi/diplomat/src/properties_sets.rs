// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use core::str;
    use icu_properties::sets;

    use crate::errors::ffi::ICU4XError;
    use crate::properties_iter::ffi::CodePointRangeIterator;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    #[diplomat::rust_link(icu::properties, Mod)]
    #[diplomat::rust_link(icu::properties::sets::CodePointSetData, Struct)]
    #[diplomat::rust_link(icu::properties::sets::CodePointSetData::from_data, FnInStruct, hidden)]
    #[diplomat::rust_link(icu::properties::sets::CodePointSetDataBorrowed, Struct)]
    pub struct ICU4XCodePointSetData(pub sets::CodePointSetData);

    impl ICU4XCodePointSetData {
        /// Checks whether the code point is in the set.
        #[diplomat::rust_link(
            icu::properties::sets::CodePointSetDataBorrowed::contains,
            FnInStruct
        )]
        pub fn contains(&self, cp: char) -> bool {
            self.0.as_borrowed().contains(cp)
        }
        /// Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.
        #[diplomat::rust_link(
            icu::properties::sets::CodePointSetDataBorrowed::contains32,
            FnInStruct,
            hidden
        )]
        pub fn contains32(&self, cp: u32) -> bool {
            self.0.as_borrowed().contains32(cp)
        }

        /// Produces an iterator over ranges of code points contained in this set
        #[diplomat::rust_link(
            icu::properties::sets::CodePointSetDataBorrowed::iter_ranges,
            FnInStruct
        )]
        pub fn iter_ranges<'a>(&'a self) -> Box<CodePointRangeIterator<'a>> {
            Box::new(CodePointRangeIterator(Box::new(
                self.0.as_borrowed().iter_ranges(),
            )))
        }

        /// Produces an iterator over ranges of code points not contained in this set
        #[diplomat::rust_link(
            icu::properties::sets::CodePointSetDataBorrowed::iter_ranges_complemented,
            FnInStruct
        )]
        pub fn iter_ranges_complemented<'a>(&'a self) -> Box<CodePointRangeIterator<'a>> {
            Box::new(CodePointRangeIterator(Box::new(
                self.0.as_borrowed().iter_ranges_complemented(),
            )))
        }

        /// which is a mask with the same format as the `U_GC_XX_MASK` mask in ICU4C
        #[diplomat::rust_link(icu::properties::sets::for_general_category_group, Fn)]
        pub fn load_for_general_category_group(
            provider: &ICU4XDataProvider,
            group: u32,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::for_general_category_group [r => Ok(r)],
                sets::load_for_general_category_group,
                provider,
                group.into(),
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::ascii_hex_digit, Fn)]
        pub fn load_ascii_hex_digit(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::ascii_hex_digit [r => Ok(r.static_to_owned())],
                sets::load_ascii_hex_digit,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::alnum, Fn)]
        pub fn load_alnum(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::alnum [r => Ok(r.static_to_owned())],
                sets::load_alnum,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::alphabetic, Fn)]
        pub fn load_alphabetic(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::alphabetic [r => Ok(r.static_to_owned())],
                sets::load_alphabetic,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::bidi_control, Fn)]
        pub fn load_bidi_control(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::bidi_control [r => Ok(r.static_to_owned())],
                sets::load_bidi_control,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::bidi_mirrored, Fn)]
        pub fn load_bidi_mirrored(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::bidi_mirrored [r => Ok(r.static_to_owned())],
                sets::load_bidi_mirrored,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::blank, Fn)]
        pub fn load_blank(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::blank [r => Ok(r.static_to_owned())],
                sets::load_blank,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::cased, Fn)]
        pub fn load_cased(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::cased [r => Ok(r.static_to_owned())],
                sets::load_cased,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::case_ignorable, Fn)]
        pub fn load_case_ignorable(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::case_ignorable [r => Ok(r.static_to_owned())],
                sets::load_case_ignorable,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::full_composition_exclusion, Fn)]
        pub fn load_full_composition_exclusion(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::full_composition_exclusion [r => Ok(r.static_to_owned())],
                sets::load_full_composition_exclusion,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_casefolded, Fn)]
        pub fn load_changes_when_casefolded(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::changes_when_casefolded [r => Ok(r.static_to_owned())],
                sets::load_changes_when_casefolded,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_casemapped, Fn)]
        pub fn load_changes_when_casemapped(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::changes_when_casemapped [r => Ok(r.static_to_owned())],
                sets::load_changes_when_casemapped,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_nfkc_casefolded, Fn)]
        pub fn load_changes_when_nfkc_casefolded(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::changes_when_nfkc_casefolded [r => Ok(r.static_to_owned())],
                sets::load_changes_when_nfkc_casefolded,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_lowercased, Fn)]
        pub fn load_changes_when_lowercased(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::changes_when_lowercased [r => Ok(r.static_to_owned())],
                sets::load_changes_when_lowercased,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_titlecased, Fn)]
        pub fn load_changes_when_titlecased(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::changes_when_titlecased [r => Ok(r.static_to_owned())],
                sets::load_changes_when_titlecased,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_uppercased, Fn)]
        pub fn load_changes_when_uppercased(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::changes_when_uppercased [r => Ok(r.static_to_owned())],
                sets::load_changes_when_uppercased,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::dash, Fn)]
        pub fn load_dash(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::dash [r => Ok(r.static_to_owned())],
                sets::load_dash,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::deprecated, Fn)]
        pub fn load_deprecated(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::deprecated [r => Ok(r.static_to_owned())],
                sets::load_deprecated,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::default_ignorable_code_point, Fn)]
        pub fn load_default_ignorable_code_point(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::default_ignorable_code_point [r => Ok(r.static_to_owned())],
                sets::load_default_ignorable_code_point,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::diacritic, Fn)]
        pub fn load_diacritic(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::diacritic [r => Ok(r.static_to_owned())],
                sets::load_diacritic,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji_modifier_base, Fn)]
        pub fn load_emoji_modifier_base(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::emoji_modifier_base [r => Ok(r.static_to_owned())],
                sets::load_emoji_modifier_base,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji_component, Fn)]
        pub fn load_emoji_component(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::emoji_component [r => Ok(r.static_to_owned())],
                sets::load_emoji_component,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji_modifier, Fn)]
        pub fn load_emoji_modifier(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::emoji_modifier [r => Ok(r.static_to_owned())],
                sets::load_emoji_modifier,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji, Fn)]
        pub fn load_emoji(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::emoji [r => Ok(r.static_to_owned())],
                sets::load_emoji,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji_presentation, Fn)]
        pub fn load_emoji_presentation(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::emoji_presentation [r => Ok(r.static_to_owned())],
                sets::load_emoji_presentation,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::extender, Fn)]
        pub fn load_extender(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::extender [r => Ok(r.static_to_owned())],
                sets::load_extender,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::extended_pictographic, Fn)]
        pub fn load_extended_pictographic(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::extended_pictographic [r => Ok(r.static_to_owned())],
                sets::load_extended_pictographic,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::graph, Fn)]
        pub fn load_graph(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::graph [r => Ok(r.static_to_owned())],
                sets::load_graph,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::grapheme_base, Fn)]
        pub fn load_grapheme_base(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::grapheme_base [r => Ok(r.static_to_owned())],
                sets::load_grapheme_base,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::grapheme_extend, Fn)]
        pub fn load_grapheme_extend(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::grapheme_extend [r => Ok(r.static_to_owned())],
                sets::load_grapheme_extend,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::grapheme_link, Fn)]
        pub fn load_grapheme_link(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::grapheme_link [r => Ok(r.static_to_owned())],
                sets::load_grapheme_link,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::hex_digit, Fn)]
        pub fn load_hex_digit(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::hex_digit [r => Ok(r.static_to_owned())],
                sets::load_hex_digit,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::hyphen, Fn)]
        pub fn load_hyphen(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::hyphen [r => Ok(r.static_to_owned())],
                sets::load_hyphen,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::id_continue, Fn)]
        pub fn load_id_continue(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::id_continue [r => Ok(r.static_to_owned())],
                sets::load_id_continue,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::ideographic, Fn)]
        pub fn load_ideographic(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::ideographic [r => Ok(r.static_to_owned())],
                sets::load_ideographic,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::id_start, Fn)]
        pub fn load_id_start(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::id_start [r => Ok(r.static_to_owned())],
                sets::load_id_start,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::ids_binary_operator, Fn)]
        pub fn load_ids_binary_operator(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::ids_binary_operator [r => Ok(r.static_to_owned())],
                sets::load_ids_binary_operator,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::ids_trinary_operator, Fn)]
        pub fn load_ids_trinary_operator(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::ids_trinary_operator [r => Ok(r.static_to_owned())],
                sets::load_ids_trinary_operator,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::join_control, Fn)]
        pub fn load_join_control(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::join_control [r => Ok(r.static_to_owned())],
                sets::load_join_control,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::logical_order_exception, Fn)]
        pub fn load_logical_order_exception(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::logical_order_exception [r => Ok(r.static_to_owned())],
                sets::load_logical_order_exception,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::lowercase, Fn)]
        pub fn load_lowercase(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::lowercase [r => Ok(r.static_to_owned())],
                sets::load_lowercase,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::math, Fn)]
        pub fn load_math(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::math [r => Ok(r.static_to_owned())],
                sets::load_math,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::noncharacter_code_point, Fn)]
        pub fn load_noncharacter_code_point(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::noncharacter_code_point [r => Ok(r.static_to_owned())],
                sets::load_noncharacter_code_point,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::nfc_inert, Fn)]
        pub fn load_nfc_inert(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::nfc_inert [r => Ok(r.static_to_owned())],
                sets::load_nfc_inert,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::nfd_inert, Fn)]
        pub fn load_nfd_inert(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::nfd_inert [r => Ok(r.static_to_owned())],
                sets::load_nfd_inert,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::nfkc_inert, Fn)]
        pub fn load_nfkc_inert(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::nfkc_inert [r => Ok(r.static_to_owned())],
                sets::load_nfkc_inert,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::nfkd_inert, Fn)]
        pub fn load_nfkd_inert(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::nfkd_inert [r => Ok(r.static_to_owned())],
                sets::load_nfkd_inert,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::pattern_syntax, Fn)]
        pub fn load_pattern_syntax(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::pattern_syntax [r => Ok(r.static_to_owned())],
                sets::load_pattern_syntax,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::pattern_white_space, Fn)]
        pub fn load_pattern_white_space(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::pattern_white_space [r => Ok(r.static_to_owned())],
                sets::load_pattern_white_space,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::prepended_concatenation_mark, Fn)]
        pub fn load_prepended_concatenation_mark(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::prepended_concatenation_mark [r => Ok(r.static_to_owned())],
                sets::load_prepended_concatenation_mark,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::print, Fn)]
        pub fn load_print(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::print [r => Ok(r.static_to_owned())],
                sets::load_print,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::quotation_mark, Fn)]
        pub fn load_quotation_mark(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::quotation_mark [r => Ok(r.static_to_owned())],
                sets::load_quotation_mark,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::radical, Fn)]
        pub fn load_radical(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::radical [r => Ok(r.static_to_owned())],
                sets::load_radical,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::regional_indicator, Fn)]
        pub fn load_regional_indicator(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::regional_indicator [r => Ok(r.static_to_owned())],
                sets::load_regional_indicator,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::soft_dotted, Fn)]
        pub fn load_soft_dotted(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::soft_dotted [r => Ok(r.static_to_owned())],
                sets::load_soft_dotted,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::segment_starter, Fn)]
        pub fn load_segment_starter(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::segment_starter [r => Ok(r.static_to_owned())],
                sets::load_segment_starter,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::case_sensitive, Fn)]
        pub fn load_case_sensitive(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::case_sensitive [r => Ok(r.static_to_owned())],
                sets::load_case_sensitive,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::sentence_terminal, Fn)]
        pub fn load_sentence_terminal(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::sentence_terminal [r => Ok(r.static_to_owned())],
                sets::load_sentence_terminal,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::terminal_punctuation, Fn)]
        pub fn load_terminal_punctuation(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::terminal_punctuation [r => Ok(r.static_to_owned())],
                sets::load_terminal_punctuation,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::unified_ideograph, Fn)]
        pub fn load_unified_ideograph(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::unified_ideograph [r => Ok(r.static_to_owned())],
                sets::load_unified_ideograph,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::uppercase, Fn)]
        pub fn load_uppercase(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::uppercase [r => Ok(r.static_to_owned())],
                sets::load_uppercase,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::variation_selector, Fn)]
        pub fn load_variation_selector(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::variation_selector [r => Ok(r.static_to_owned())],
                sets::load_variation_selector,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::white_space, Fn)]
        pub fn load_white_space(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::white_space [r => Ok(r.static_to_owned())],
                sets::load_white_space,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::xdigit, Fn)]
        pub fn load_xdigit(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::xdigit [r => Ok(r.static_to_owned())],
                sets::load_xdigit,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::xid_continue, Fn)]
        pub fn load_xid_continue(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::xid_continue [r => Ok(r.static_to_owned())],
                sets::load_xid_continue,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::xid_start, Fn)]
        pub fn load_xid_start(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::xid_start [r => Ok(r.static_to_owned())],
                sets::load_xid_start,
                provider
            )?)))
        }

        /// Loads data for a property specified as a string as long as it is one of the
        /// [ECMA-262 binary properties][ecma] (not including Any, ASCII, and Assigned pseudoproperties).
        ///
        /// Returns `ICU4XError::PropertyUnexpectedPropertyNameError` in case the string does not
        /// match any property in the list
        ///
        /// [ecma]: https://tc39.es/ecma262/#table-binary-unicode-properties
        #[diplomat::rust_link(icu::properties::sets::for_ecma262, Fn)]
        #[diplomat::rust_link(icu::properties::sets::for_ecma262_with_any_provider, Fn, hidden)]
        #[diplomat::rust_link(icu::properties::sets::for_ecma262_with_buffer_provider, Fn, hidden)]
        pub fn load_for_ecma262(
            provider: &ICU4XDataProvider,
            property_name: &str,
        ) -> Result<Box<ICU4XCodePointSetData>, ICU4XError> {
            let name = property_name.as_bytes(); // #2520
            let name = if let Ok(s) = str::from_utf8(name) {
                s
            } else {
                return Err(ICU4XError::TinyStrNonAsciiError);
            };
            Ok(Box::new(ICU4XCodePointSetData(call_constructor_unstable!(
                sets::load_for_ecma262 [r => r.map(|r| r.static_to_owned())],
                sets::load_for_ecma262_unstable,
                provider,
                name
            )?)))
        }
    }
}
