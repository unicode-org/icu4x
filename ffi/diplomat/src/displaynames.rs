// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    use icu_displaynames::{LanguageDisplayNames, RegionDisplayNames};
    // use icu_displaynames::options::{DisplayNamesOptions, Fallback, LanguageDisplay, Style};

    use crate::errors::ffi::ICU4XError;
    use crate::locale::ffi::ICU4XLocale;
    use crate::provider::ffi::ICU4XDataProvider;

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::displaynames::LanguageDisplayNames, Struct)]
    pub struct ICU4XLanguageDisplayNames(pub LanguageDisplayNames);

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::displaynames::RegionDisplayNames, Struct)]
    pub struct ICU4XRegionDisplayNames(pub RegionDisplayNames);

    // #[diplomat::rust_link(icu::displaynames::options::DisplayNamesOptions, Struct)]
    // pub struct ICU4XDisplayNamesOptions<'a> {
    //     /// The optional formatting style to use for display name.
    //     pub style: &'a ICU4XStyle,
    //     /// The fallback return when the system does not have the
    //     /// requested display name, defaults to "code".
    //     pub fallback: ICU4XFallback,
    //     /// The language display kind, defaults to "dialect".
    //     pub language_display: ICU4XLanguageDisplay,
    // }

    // /// FFI version of `Style`.
    // #[diplomat::rust_link(icu::displaynames::options::Style, Enum)]
    // #[diplomat::enum_convert(Style)]
    // pub enum ICU4XStyle {
    //     Narrow,
    //     Short,
    //     Long,
    //     Menu,
    // }

    // /// FFI version of `Fallback`.
    // #[diplomat::rust_link(icu::displaynames::options::Fallback, Enum)]
    // #[diplomat::enum_convert(Fallback)]
    // pub enum ICU4XFallback {
    //     Code,
    //     None,
    // }

    // /// FFI version of `LanguageDisplay`.
    // #[diplomat::rust_link(icu::displaynames::options::LanguageDisplay, Enum)]
    // #[diplomat::enum_convert(LanguageDisplay)]
    // pub enum ICU4XLanguageDisplay {
    //     Dialect,
    //     Standard,
    // }

    impl ICU4XLanguageDisplayNames {
        /// Creates a new `LanguageDisplayNames` from locale data and an options bag.
        #[diplomat::rust_link(icu::displaynames::LanguageDisplayNames::try_new_unstable, FnInStruct)]
        pub fn try_new_unstable(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
        ) -> Result<Box<ICU4XLanguageDisplayNames>, ICU4XError> {
            let locale = locale.to_datalocale();

            Ok(Box::new(ICU4XLanguageDisplayNames(LanguageDisplayNames::try_new_unstable(
                &provider.0,
                &locale,
                Default::default(),
            )?)))
        }

        // Returns the locale specific display name of a language for a given string.
        // #[diplomat::rust_link(icu::displaynames::LanguageDisplayNames::of, FnInStruct)]
        // pub fn of<'a>(&self, code: &str) -> Result<&'a str, ()> {
        //     self.0.of(code).map(|x| x.0).ok_or()
        // }
    }

    impl ICU4XRegionDisplayNames {
        /// Creates a new `RegionDisplayNames` from locale data and an options bag.
        #[diplomat::rust_link(
            icu::displaynames::RegionDisplayNames::try_new_unstable,
            FnInStruct
        )]
        pub fn try_new_unstable(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
        ) -> Result<Box<ICU4XRegionDisplayNames>, ICU4XError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ICU4XRegionDisplayNames(RegionDisplayNames::try_new_unstable(
                &provider.0,
                &locale,
                Default::default(),
            )?)))
        }

        // Returns the locale specific display name of a region for a given string.
        // #[diplomat::rust_link(icu::displaynames::RegionDisplayNames::of, FnInStruct)]
        // pub fn of<'a>(&self, code: &str) -> Result<&'a str, ()> {
        //     self.0.of(code).map(|x| x.0).ok_or()
        // }
    }
}