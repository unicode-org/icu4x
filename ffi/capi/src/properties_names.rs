// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::DataError;
    use crate::provider::ffi::DataProvider;

    /// A type capable of looking up a property value from a string name.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::properties::names::PropertyValueNameToEnumMapper, Struct)]
    #[diplomat::rust_link(icu::properties::names::PropertyValueNameToEnumMapperBorrowed, Struct)]
    pub struct PropertyValueNameToEnumMapper(
        icu_properties::names::PropertyValueNameToEnumMapper<u16>,
    );

    impl PropertyValueNameToEnumMapper {
        /// Get the property value matching the given name, using strict matching
        ///
        /// Returns -1 if the name is unknown for this property
        #[diplomat::rust_link(
            icu::properties::names::PropertyValueNameToEnumMapperBorrowed::get_strict,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::properties::names::PropertyValueNameToEnumMapperBorrowed::get_strict_u16,
            FnInStruct,
            hidden
        )]
        pub fn get_strict(&self, name: &DiplomatStr) -> i16 {
            if let Ok(name) = core::str::from_utf8(name) {
                self.0.as_borrowed().get_strict(name)
            } else {
                None
            }
            .map(|u_16| u_16 as i16)
            .unwrap_or(-1)
        }

        /// Get the property value matching the given name, using loose matching
        ///
        /// Returns -1 if the name is unknown for this property
        #[diplomat::rust_link(
            icu::properties::names::PropertyValueNameToEnumMapperBorrowed::get_loose,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::properties::names::PropertyValueNameToEnumMapperBorrowed::get_loose_u16,
            FnInStruct,
            hidden
        )]
        pub fn get_loose(&self, name: &DiplomatStr) -> i16 {
            if let Ok(name) = core::str::from_utf8(name) {
                self.0.as_borrowed().get_loose(name)
            } else {
                None
            }
            .map(|u_16| u_16 as i16)
            .unwrap_or(-1)
        }

        #[diplomat::rust_link(icu::properties::GeneralCategory::name_to_enum_mapper, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::GeneralCategory::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "general_category")]
        pub fn load_general_category(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::GeneralCategory::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::GeneralCategory::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }

        #[diplomat::rust_link(icu::properties::HangulSyllableType::name_to_enum_mapper, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::HangulSyllableType::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "hangul_syllable_type")]
        pub fn load_hangul_syllable_type(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::HangulSyllableType::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::HangulSyllableType::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }

        #[diplomat::rust_link(icu::properties::EastAsianWidth::name_to_enum_mapper, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::EastAsianWidth::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "east_asian_width")]
        pub fn load_east_asian_width(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::EastAsianWidth::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::EastAsianWidth::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }

        #[diplomat::rust_link(icu::properties::BidiClass::name_to_enum_mapper, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::BidiClass::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "bidi_class")]
        pub fn load_bidi_class(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::BidiClass::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::BidiClass::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }

        #[diplomat::rust_link(
            icu::properties::IndicSyllabicCategory::name_to_enum_mapper,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::properties::IndicSyllabicCategory::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "indic_syllabic_category")]
        pub fn load_indic_syllabic_category(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::IndicSyllabicCategory::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::IndicSyllabicCategory::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }

        #[diplomat::rust_link(icu::properties::LineBreak::name_to_enum_mapper, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::LineBreak::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "line_break")]
        pub fn load_line_break(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::LineBreak::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::LineBreak::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }

        #[diplomat::rust_link(
            icu::properties::GraphemeClusterBreak::name_to_enum_mapper,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::properties::GraphemeClusterBreak::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "grapheme_cluster_break")]
        pub fn load_grapheme_cluster_break(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::GraphemeClusterBreak::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::GraphemeClusterBreak::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }

        #[diplomat::rust_link(icu::properties::WordBreak::name_to_enum_mapper, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::WordBreak::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "word_break")]
        pub fn load_word_break(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::WordBreak::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::WordBreak::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }

        #[diplomat::rust_link(icu::properties::SentenceBreak::name_to_enum_mapper, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::SentenceBreak::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "sentence_break")]
        pub fn load_sentence_break(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::SentenceBreak::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::SentenceBreak::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }

        #[diplomat::rust_link(icu::properties::Script::name_to_enum_mapper, FnInStruct)]
        #[diplomat::rust_link(icu::properties::Script::get_name_to_enum_mapper, FnInStruct, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "script")]
        pub fn load_script(
            provider: &DataProvider,
        ) -> Result<Box<PropertyValueNameToEnumMapper>, DataError> {
            Ok(Box::new(PropertyValueNameToEnumMapper(
                call_constructor_unstable!(
                    icu_properties::Script::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::Script::get_name_to_enum_mapper,
                    provider,
                )?
                .erase(),
            )))
        }
    }

    /// A type capable of looking up General Category mask values from a string name.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::properties::GeneralCategoryGroup::name_to_enum_mapper, FnInStruct)]
    #[diplomat::rust_link(
        icu::properties::GeneralCategoryGroup::get_name_to_enum_mapper,
        FnInStruct,
        hidden
    )]
    #[diplomat::rust_link(icu::properties::names::PropertyValueNameToEnumMapper, Struct)]
    pub struct GeneralCategoryNameToMaskMapper(
        icu_properties::names::PropertyValueNameToEnumMapper<icu_properties::GeneralCategoryGroup>,
    );

    impl GeneralCategoryNameToMaskMapper {
        /// Get the mask value matching the given name, using strict matching
        ///
        /// Returns 0 if the name is unknown for this property
        // #[diplomat::rust_link(icu::properties::maps::PropertyValueNameToEnumMapperBorrowed::get_strict, FnInStruct)]
        // #[diplomat::rust_link(icu::properties::maps::PropertyValueNameToEnumMapperBorrowed::get_strict_u16, FnInStruct, hidden)]
        pub fn get_strict(&self, name: &DiplomatStr) -> u32 {
            if let Ok(name) = core::str::from_utf8(name) {
                self.0.as_borrowed().get_strict(name)
            } else {
                None
            }
            .map(Into::into)
            .unwrap_or_default()
        }

        /// Get the mask value matching the given name, using loose matching
        ///
        /// Returns 0 if the name is unknown for this property
        // #[diplomat::rust_link(icu::properties::maps::PropertyValueNameToEnumMapperBorrowed::get_loose, FnInStruct)]
        // #[diplomat::rust_link(icu::properties::maps::PropertyValueNameToEnumMapperBorrowed::get_loose_u16, FnInStruct, hidden)]
        pub fn get_loose(&self, name: &DiplomatStr) -> u32 {
            if let Ok(name) = core::str::from_utf8(name) {
                self.0.as_borrowed().get_loose(name)
            } else {
                None
            }
            .map(Into::into)
            .unwrap_or_default()
        }

        #[diplomat::rust_link(
            icu::properties::GeneralCategoryGroup::name_to_enum_mapper,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu::properties::GeneralCategoryGroup::get_name_to_enum_mapper,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        pub fn load(
            provider: &DataProvider,
        ) -> Result<Box<GeneralCategoryNameToMaskMapper>, DataError> {
            Ok(Box::new(GeneralCategoryNameToMaskMapper(
                call_constructor_unstable!(
                    icu_properties::GeneralCategoryGroup::name_to_enum_mapper [r => Ok(r.static_to_owned())],
                    icu_properties::GeneralCategoryGroup::get_name_to_enum_mapper,
                    provider,
                )?,
            )))
        }
    }
}
