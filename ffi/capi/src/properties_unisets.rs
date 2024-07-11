// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "ICU4X{0}"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::DataError;
    use crate::locale_core::ffi::Locale;
    use crate::provider::ffi::DataProvider;

    #[diplomat::opaque]
    /// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    #[diplomat::rust_link(icu::properties, Mod)]
    #[diplomat::rust_link(icu::properties::sets::UnicodeSetData, Struct)]
    #[diplomat::rust_link(icu::properties::sets::UnicodeSetDataBorrowed, Struct)]
    pub struct UnicodeSetData(pub icu_properties::sets::UnicodeSetData);

    impl UnicodeSetData {
        /// Checks whether the string is in the set.
        #[diplomat::rust_link(icu::properties::sets::UnicodeSetDataBorrowed::contains, FnInStruct)]
        pub fn contains(&self, s: &DiplomatStr) -> bool {
            let Ok(s) = core::str::from_utf8(s) else {
                return false;
            };
            self.0.as_borrowed().contains(s)
        }
        /// Checks whether the code point is in the set.
        #[diplomat::rust_link(
            icu::properties::sets::UnicodeSetDataBorrowed::contains_char,
            FnInStruct
        )]
        pub fn contains_char(&self, cp: DiplomatChar) -> bool {
            self.0.as_borrowed().contains32(cp)
        }
        /// Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.
        #[diplomat::rust_link(
            icu::properties::sets::UnicodeSetDataBorrowed::contains32,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(dart, disable)]
        pub fn contains32(&self, cp: u32) -> bool {
            self.contains_char(cp)
        }

        #[diplomat::rust_link(icu::properties::sets::basic_emoji, Fn)]
        #[diplomat::rust_link(icu::properties::sets::load_basic_emoji, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "basic_emoji")]
        pub fn load_basic_emoji(provider: &DataProvider) -> Result<Box<UnicodeSetData>, DataError> {
            Ok(Box::new(UnicodeSetData(call_constructor_unstable!(
                icu_properties::sets::basic_emoji [r => Ok(r.static_to_owned())],
                icu_properties::sets::load_basic_emoji,
                provider,
            )?)))
        }

        #[diplomat::rust_link(icu::properties::exemplar_chars::exemplars_main, Fn)]
        #[diplomat::rust_link(icu::properties::exemplar_chars::load_exemplars_main, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "exemplars_main")]
        pub fn load_exemplars_main(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<UnicodeSetData>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(UnicodeSetData(call_constructor_unstable!(
                icu_properties::exemplar_chars::exemplars_main,
                icu_properties::exemplar_chars::load_exemplars_main,
                provider,
                &locale
            )?)))
        }

        #[diplomat::rust_link(icu::properties::exemplar_chars::exemplars_auxiliary, Fn)]
        #[diplomat::rust_link(
            icu::properties::exemplar_chars::load_exemplars_auxiliary,
            Fn,
            hidden
        )]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "exemplars_auxiliary")]
        pub fn load_exemplars_auxiliary(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<UnicodeSetData>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(UnicodeSetData(call_constructor_unstable!(
                icu_properties::exemplar_chars::exemplars_auxiliary,
                icu_properties::exemplar_chars::load_exemplars_auxiliary,
                provider,
                &locale
            )?)))
        }

        #[diplomat::rust_link(icu::properties::exemplar_chars::exemplars_punctuation, Fn)]
        #[diplomat::rust_link(
            icu::properties::exemplar_chars::load_exemplars_punctuation,
            Fn,
            hidden
        )]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "exemplars_punctuation")]
        pub fn load_exemplars_punctuation(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<UnicodeSetData>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(UnicodeSetData(call_constructor_unstable!(
                icu_properties::exemplar_chars::exemplars_punctuation,
                icu_properties::exemplar_chars::load_exemplars_punctuation,
                provider,
                &locale
            )?)))
        }

        #[diplomat::rust_link(icu::properties::exemplar_chars::exemplars_numbers, Fn)]
        #[diplomat::rust_link(icu::properties::exemplar_chars::load_exemplars_numbers, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "exemplars_numbers")]
        pub fn load_exemplars_numbers(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<UnicodeSetData>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(UnicodeSetData(call_constructor_unstable!(
                icu_properties::exemplar_chars::exemplars_numbers,
                icu_properties::exemplar_chars::load_exemplars_numbers,
                provider,
                &locale
            )?)))
        }

        #[diplomat::rust_link(icu::properties::exemplar_chars::exemplars_index, Fn)]
        #[diplomat::rust_link(icu::properties::exemplar_chars::load_exemplars_index, Fn, hidden)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "exemplars_index")]
        pub fn load_exemplars_index(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<UnicodeSetData>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(UnicodeSetData(call_constructor_unstable!(
                icu_properties::exemplar_chars::exemplars_index,
                icu_properties::exemplar_chars::load_exemplars_index,
                provider,
                &locale
            )?)))
        }
    }
}
