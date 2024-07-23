// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::DataError;
    use crate::errors::ffi::Error;
    use crate::properties_iter::ffi::CodePointRangeIterator;
    use crate::provider::ffi::DataProvider;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    #[diplomat::rust_link(icu::properties, Mod)]
    #[diplomat::rust_link(icu::properties::sets::CodePointSetData, Struct)]
    #[diplomat::rust_link(icu::properties::sets::CodePointSetDataBorrowed, Struct)]
    pub struct CodePointSetData(pub icu_properties::sets::CodePointSetData);

    impl CodePointSetData {
        /// Checks whether the code point is in the set.
        #[diplomat::rust_link(
            icu::properties::sets::CodePointSetDataBorrowed::contains,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::properties::sets::CodePointSetDataBorrowed::contains32,
            FnInStruct,
            hidden
        )]
        pub fn contains(&self, cp: DiplomatChar) -> bool {
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
        #[diplomat::rust_link(icu::properties::sets::load_for_general_category_group, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "for_general_category_group")]
        pub fn load_for_general_category_group(
            provider: &DataProvider,
            group: u32,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::for_general_category_group [r => Ok(r)],
                icu_properties::sets::load_for_general_category_group,
                provider,
                group.into(),
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::ascii_hex_digit, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_ascii_hex_digit, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "ascii_hex_digit")]
        pub fn load_ascii_hex_digit(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::ascii_hex_digit [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_ascii_hex_digit,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::alnum, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_alnum, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "alnum")]
        pub fn load_alnum(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::alnum [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_alnum,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::alphabetic, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_alphabetic, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "alphabetic")]
        pub fn load_alphabetic(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::alphabetic [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_alphabetic,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::bidi_control, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_bidi_control, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "bidi_control")]
        pub fn load_bidi_control(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::bidi_control [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_bidi_control,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::bidi_mirrored, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_bidi_mirrored, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "bidi_mirrored")]
        pub fn load_bidi_mirrored(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::bidi_mirrored [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_bidi_mirrored,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::blank, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_blank, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "blank")]
        pub fn load_blank(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::blank [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_blank,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::cased, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_cased, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "cased")]
        pub fn load_cased(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::cased [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_cased,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::case_ignorable, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_case_ignorable, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "case_ignorable")]
        pub fn load_case_ignorable(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::case_ignorable [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_case_ignorable,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::full_composition_exclusion, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_full_composition_exclusion, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "full_composition_exclusion")]
        pub fn load_full_composition_exclusion(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::full_composition_exclusion [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_full_composition_exclusion,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_casefolded, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_changes_when_casefolded, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_casefolded")]
        pub fn load_changes_when_casefolded(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::changes_when_casefolded [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_changes_when_casefolded,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_casemapped, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_changes_when_casemapped, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_casemapped")]
        pub fn load_changes_when_casemapped(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::changes_when_casemapped [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_changes_when_casemapped,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_nfkc_casefolded, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_changes_when_nfkc_casefolded, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_nfkc_casefolded")]
        pub fn load_changes_when_nfkc_casefolded(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::changes_when_nfkc_casefolded [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_changes_when_nfkc_casefolded,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_lowercased, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_changes_when_lowercased, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_lowercased")]
        pub fn load_changes_when_lowercased(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::changes_when_lowercased [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_changes_when_lowercased,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_titlecased, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_changes_when_titlecased, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_titlecased")]
        pub fn load_changes_when_titlecased(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::changes_when_titlecased [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_changes_when_titlecased,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::changes_when_uppercased, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_changes_when_uppercased, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "changes_when_uppercased")]
        pub fn load_changes_when_uppercased(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::changes_when_uppercased [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_changes_when_uppercased,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::dash, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_dash, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "dash")]
        pub fn load_dash(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::dash [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_dash,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::deprecated, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_deprecated, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "deprecated")]
        pub fn load_deprecated(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::deprecated [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_deprecated,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::default_ignorable_code_point, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_default_ignorable_code_point, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "default_ignorable_code_point")]
        pub fn load_default_ignorable_code_point(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::default_ignorable_code_point [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_default_ignorable_code_point,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::diacritic, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_diacritic, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "diacritic")]
        pub fn load_diacritic(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::diacritic [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_diacritic,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji_modifier_base, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_emoji_modifier_base, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji_modifier_base")]
        pub fn load_emoji_modifier_base(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::emoji_modifier_base [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_emoji_modifier_base,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji_component, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_emoji_component, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji_component")]
        pub fn load_emoji_component(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::emoji_component [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_emoji_component,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji_modifier, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_emoji_modifier, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji_modifier")]
        pub fn load_emoji_modifier(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::emoji_modifier [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_emoji_modifier,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_emoji, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji")]
        pub fn load_emoji(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::emoji [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_emoji,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::emoji_presentation, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_emoji_presentation, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "emoji_presentation")]
        pub fn load_emoji_presentation(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::emoji_presentation [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_emoji_presentation,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::extender, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_extender, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "extender")]
        pub fn load_extender(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::extender [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_extender,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::extended_pictographic, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_extended_pictographic, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "extended_pictographic")]
        pub fn load_extended_pictographic(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::extended_pictographic [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_extended_pictographic,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::graph, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_graph, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "graph")]
        pub fn load_graph(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::graph [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_graph,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::grapheme_base, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_grapheme_base, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "grapheme_base")]
        pub fn load_grapheme_base(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::grapheme_base [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_grapheme_base,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::grapheme_extend, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_grapheme_extend, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "grapheme_extend")]
        pub fn load_grapheme_extend(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::grapheme_extend [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_grapheme_extend,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::grapheme_link, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_grapheme_link, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "grapheme_link")]
        pub fn load_grapheme_link(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::grapheme_link [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_grapheme_link,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::hex_digit, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_hex_digit, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "hex_digit")]
        pub fn load_hex_digit(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::hex_digit [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_hex_digit,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::hyphen, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_hyphen, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "hyphen")]
        pub fn load_hyphen(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::hyphen [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_hyphen,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::id_continue, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_id_continue, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "id_continue")]
        pub fn load_id_continue(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::id_continue [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_id_continue,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::ideographic, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_ideographic, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "ideographic")]
        pub fn load_ideographic(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::ideographic [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_ideographic,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::id_start, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_id_start, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "id_start")]
        pub fn load_id_start(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::id_start [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_id_start,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::ids_binary_operator, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_ids_binary_operator, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "ids_binary_operator")]
        pub fn load_ids_binary_operator(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::ids_binary_operator [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_ids_binary_operator,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::ids_trinary_operator, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_ids_trinary_operator, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "ids_trinary_operator")]
        pub fn load_ids_trinary_operator(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::ids_trinary_operator [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_ids_trinary_operator,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::join_control, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_join_control, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "join_control")]
        pub fn load_join_control(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::join_control [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_join_control,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::logical_order_exception, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_logical_order_exception, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "logical_order_exception")]
        pub fn load_logical_order_exception(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::logical_order_exception [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_logical_order_exception,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::lowercase, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_lowercase, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "lowercase")]
        pub fn load_lowercase(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::lowercase [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_lowercase,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::math, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_math, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "math")]
        pub fn load_math(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::math [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_math,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::noncharacter_code_point, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_noncharacter_code_point, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "noncharacter_code_point")]
        pub fn load_noncharacter_code_point(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::noncharacter_code_point [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_noncharacter_code_point,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::nfc_inert, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_nfc_inert, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "nfc_inert")]
        pub fn load_nfc_inert(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::nfc_inert [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_nfc_inert,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::nfd_inert, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_nfd_inert, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "nfd_inert")]
        pub fn load_nfd_inert(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::nfd_inert [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_nfd_inert,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::nfkc_inert, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_nfkc_inert, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "nfkc_inert")]
        pub fn load_nfkc_inert(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::nfkc_inert [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_nfkc_inert,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::nfkd_inert, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_nfkd_inert, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "nfkd_inert")]
        pub fn load_nfkd_inert(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::nfkd_inert [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_nfkd_inert,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::pattern_syntax, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_pattern_syntax, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "pattern_syntax")]
        pub fn load_pattern_syntax(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::pattern_syntax [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_pattern_syntax,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::pattern_white_space, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_pattern_white_space, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "pattern_white_space")]
        pub fn load_pattern_white_space(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::pattern_white_space [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_pattern_white_space,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::prepended_concatenation_mark, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_prepended_concatenation_mark, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "prepended_concatenation_mark")]
        pub fn load_prepended_concatenation_mark(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::prepended_concatenation_mark [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_prepended_concatenation_mark,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::print, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_print, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "print")]
        pub fn load_print(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::print [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_print,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::quotation_mark, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_quotation_mark, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "quotation_mark")]
        pub fn load_quotation_mark(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::quotation_mark [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_quotation_mark,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::radical, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_radical, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "radical")]
        pub fn load_radical(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::radical [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_radical,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::regional_indicator, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_regional_indicator, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "regional_indicator")]
        pub fn load_regional_indicator(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::regional_indicator [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_regional_indicator,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::soft_dotted, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_soft_dotted, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "soft_dotted")]
        pub fn load_soft_dotted(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::soft_dotted [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_soft_dotted,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::segment_starter, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_segment_starter, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "segment_starter")]
        pub fn load_segment_starter(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::segment_starter [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_segment_starter,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::case_sensitive, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_case_sensitive, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "case_sensitive")]
        pub fn load_case_sensitive(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::case_sensitive [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_case_sensitive,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::sentence_terminal, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_sentence_terminal, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "sentence_terminal")]
        pub fn load_sentence_terminal(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::sentence_terminal [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_sentence_terminal,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::terminal_punctuation, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_terminal_punctuation, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "terminal_punctuation")]
        pub fn load_terminal_punctuation(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::terminal_punctuation [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_terminal_punctuation,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::unified_ideograph, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_unified_ideograph, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "unified_ideograph")]
        pub fn load_unified_ideograph(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::unified_ideograph [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_unified_ideograph,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::uppercase, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_uppercase, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "uppercase")]
        pub fn load_uppercase(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::uppercase [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_uppercase,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::variation_selector, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_variation_selector, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "variation_selector")]
        pub fn load_variation_selector(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::variation_selector [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_variation_selector,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::white_space, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_white_space, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "white_space")]
        pub fn load_white_space(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::white_space [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_white_space,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::xdigit, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_xdigit, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "xdigit")]
        pub fn load_xdigit(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::xdigit [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_xdigit,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::xid_continue, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_xid_continue, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "xid_continue")]
        pub fn load_xid_continue(
            provider: &DataProvider,
        ) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::xid_continue [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_xid_continue,
                provider
            )?)))
        }

        #[diplomat::rust_link(icu::properties::sets::xid_start, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_xid_start, Fn, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "xid_start")]
        pub fn load_xid_start(provider: &DataProvider) -> Result<Box<CodePointSetData>, DataError> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::xid_start [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_xid_start,
                provider
            )?)))
        }

        /// Loads data for a property specified as a string as long as it is one of the
        /// [ECMA-262 binary properties][ecma] (not including Any, ASCII, and Assigned pseudoproperties).
        ///
        /// Returns `Error::PropertyUnexpectedPropertyNameError` in case the string does not
        /// match any property in the list
        ///
        /// [ecma]: https://tc39.es/ecma262/#table-binary-unicode-properties
        #[diplomat::rust_link(icu::properties::sets::for_ecma262, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_for_ecma262, Fn, hidden)]
        #[diplomat::rust_link(icu::properties::UnexpectedPropertyNameError, Struct, hidden)]
        #[diplomat::rust_link(icu::properties::UnexpectedPropertyNameOrDataError, Enum, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "for_ecma262")]
        pub fn load_for_ecma262(
            provider: &DataProvider,
            property_name: &str,
        ) -> Result<Box<CodePointSetData>, Error> {
            Ok(Box::new(CodePointSetData(call_constructor_unstable!(
                icu_properties::sets::load_for_ecma262 [r => r.map(|r| r.static_to_owned()).map_err(|icu_properties::UnexpectedPropertyNameError| icu_properties::UnexpectedPropertyNameOrDataError::UnexpectedPropertyName)],
                icu_properties::sets::load_for_ecma262_unstable,
                provider,
                property_name
            )?)))
        }
    }
}
