// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use icu_properties::sets;

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    #[diplomat::rust_link(icu::properties, Mod)]
    #[diplomat::rust_link(icu::properties::sets::CodePointSetData, Struct)]
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
            icu::properties::sets::CodePointSetDataBorrowed::contains_u32,
            FnInStruct,
            hidden
        )]
        pub fn contains_u32(&self, cp: u32) -> bool {
            self.0.as_borrowed().contains_u32(cp)
        }
        /// which is a mask with the same format as the `U_GC_XX_MASK` mask in ICU4C
        #[diplomat::rust_link(icu::properties::sets::load_for_general_category_group, Fn)]
        pub fn load_for_general_category_group(
            provider: &ICU4XDataProvider,
            group: u32,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_for_general_category_group(&provider, group.into())
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_ascii_hex_digit, Fn)]
        pub fn load_ascii_hex_digit(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_ascii_hex_digit(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_alnum, Fn)]
        pub fn load_alnum(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_alnum(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_alphabetic, Fn)]
        pub fn load_alphabetic(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_alphabetic(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_bidi_control, Fn)]
        pub fn load_bidi_control(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_bidi_control(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_bidi_mirrored, Fn)]
        pub fn load_bidi_mirrored(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_bidi_mirrored(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_blank, Fn)]
        pub fn load_blank(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_blank(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_cased, Fn)]
        pub fn load_cased(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_cased(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_case_ignorable, Fn)]
        pub fn load_case_ignorable(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_case_ignorable(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_full_composition_exclusion, Fn)]
        pub fn load_full_composition_exclusion(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_full_composition_exclusion(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_changes_when_casefolded, Fn)]
        pub fn load_changes_when_casefolded(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_changes_when_casefolded(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_changes_when_casemapped, Fn)]
        pub fn load_changes_when_casemapped(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_changes_when_casemapped(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_changes_when_nfkc_casefolded, Fn)]
        pub fn load_changes_when_nfkc_casefolded(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_changes_when_nfkc_casefolded(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_changes_when_lowercased, Fn)]
        pub fn load_changes_when_lowercased(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_changes_when_lowercased(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_changes_when_titlecased, Fn)]
        pub fn load_changes_when_titlecased(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_changes_when_titlecased(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_changes_when_uppercased, Fn)]
        pub fn load_changes_when_uppercased(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_changes_when_uppercased(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_dash, Fn)]
        pub fn load_dash(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_dash(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_deprecated, Fn)]
        pub fn load_deprecated(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_deprecated(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_default_ignorable_code_point, Fn)]
        pub fn load_default_ignorable_code_point(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_default_ignorable_code_point(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_diacritic, Fn)]
        pub fn load_diacritic(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_diacritic(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_emoji_modifier_base, Fn)]
        pub fn load_emoji_modifier_base(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_emoji_modifier_base(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_emoji_component, Fn)]
        pub fn load_emoji_component(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_emoji_component(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_emoji_modifier, Fn)]
        pub fn load_emoji_modifier(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_emoji_modifier(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_emoji, Fn)]
        pub fn load_emoji(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_emoji(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_emoji_presentation, Fn)]
        pub fn load_emoji_presentation(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_emoji_presentation(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_extender, Fn)]
        pub fn load_extender(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_extender(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_extended_pictographic, Fn)]
        pub fn load_extended_pictographic(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_extended_pictographic(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_graph, Fn)]
        pub fn load_graph(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_graph(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_grapheme_base, Fn)]
        pub fn load_grapheme_base(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_grapheme_base(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_grapheme_extend, Fn)]
        pub fn load_grapheme_extend(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_grapheme_extend(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_grapheme_link, Fn)]
        pub fn load_grapheme_link(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_grapheme_link(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_hex_digit, Fn)]
        pub fn load_hex_digit(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_hex_digit(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_hyphen, Fn)]
        pub fn load_hyphen(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_hyphen(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_id_continue, Fn)]
        pub fn load_id_continue(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_id_continue(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_ideographic, Fn)]
        pub fn load_ideographic(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_ideographic(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_id_start, Fn)]
        pub fn load_id_start(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_id_start(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_ids_binary_operator, Fn)]
        pub fn load_ids_binary_operator(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_ids_binary_operator(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_ids_trinary_operator, Fn)]
        pub fn load_ids_trinary_operator(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_ids_trinary_operator(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_join_control, Fn)]
        pub fn load_join_control(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_join_control(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_logical_order_exception, Fn)]
        pub fn load_logical_order_exception(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_logical_order_exception(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_lowercase, Fn)]
        pub fn load_lowercase(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_lowercase(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_math, Fn)]
        pub fn load_math(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_math(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_noncharacter_code_point, Fn)]
        pub fn load_noncharacter_code_point(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_noncharacter_code_point(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_nfc_inert, Fn)]
        pub fn load_nfc_inert(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_nfc_inert(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_nfd_inert, Fn)]
        pub fn load_nfd_inert(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_nfd_inert(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_nfkc_inert, Fn)]
        pub fn load_nfkc_inert(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_nfkc_inert(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_nfkd_inert, Fn)]
        pub fn load_nfkd_inert(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_nfkd_inert(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_pattern_syntax, Fn)]
        pub fn load_pattern_syntax(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_pattern_syntax(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_pattern_white_space, Fn)]
        pub fn load_pattern_white_space(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_pattern_white_space(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_prepended_concatenation_mark, Fn)]
        pub fn load_prepended_concatenation_mark(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_prepended_concatenation_mark(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_print, Fn)]
        pub fn load_print(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_print(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_quotation_mark, Fn)]
        pub fn load_quotation_mark(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_quotation_mark(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_radical, Fn)]
        pub fn load_radical(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_radical(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_regional_indicator, Fn)]
        pub fn load_regional_indicator(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_regional_indicator(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_soft_dotted, Fn)]
        pub fn load_soft_dotted(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_soft_dotted(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_segment_starter, Fn)]
        pub fn load_segment_starter(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_segment_starter(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_case_sensitive, Fn)]
        pub fn load_case_sensitive(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_case_sensitive(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_sentence_terminal, Fn)]
        pub fn load_sentence_terminal(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_sentence_terminal(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_terminal_punctuation, Fn)]
        pub fn load_terminal_punctuation(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_terminal_punctuation(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_unified_ideograph, Fn)]
        pub fn load_unified_ideograph(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_unified_ideograph(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_uppercase, Fn)]
        pub fn load_uppercase(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_uppercase(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_variation_selector, Fn)]
        pub fn load_variation_selector(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_variation_selector(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_white_space, Fn)]
        pub fn load_white_space(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_white_space(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_xdigit, Fn)]
        pub fn load_xdigit(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_xdigit(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_xid_continue, Fn)]
        pub fn load_xid_continue(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_xid_continue(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::properties::sets::load_xid_start, Fn)]
        pub fn load_xid_start(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XCodePointSetData>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            sets::load_xid_start(&provider)
                .map(|data| Box::new(ICU4XCodePointSetData(data)))
                .map_err(Into::into)
                .into()
        }
    }
}
