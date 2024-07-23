// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::{DataError, LocaleParseError};
    use crate::locale_core::ffi::Locale;
    use crate::provider::ffi::DataProvider;

    use writeable::Writeable;

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::displaynames::LocaleDisplayNamesFormatter, Struct)]
    pub struct LocaleDisplayNamesFormatter(
        pub icu_experimental::displaynames::LocaleDisplayNamesFormatter,
    );

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::displaynames::RegionDisplayNames, Struct)]
    pub struct RegionDisplayNames(pub icu_experimental::displaynames::RegionDisplayNames);

    #[diplomat::rust_link(icu::displaynames::options::DisplayNamesOptions, Struct)]
    #[diplomat::attr(supports = non_exhaustive_structs, rename = "DisplayNamesOptions")]
    pub struct DisplayNamesOptionsV1 {
        /// The optional formatting style to use for display name.
        pub style: DisplayNamesStyle,
        /// The fallback return when the system does not have the
        /// requested display name, defaults to "code".
        pub fallback: DisplayNamesFallback,
        /// The language display kind, defaults to "dialect".
        pub language_display: LanguageDisplay,
    }

    #[diplomat::rust_link(icu::displaynames::options::Style, Enum)]
    pub enum DisplayNamesStyle {
        Auto,
        Narrow,
        Short,
        Long,
        Menu,
    }

    #[diplomat::rust_link(icu::displaynames::options::Fallback, Enum)]
    #[diplomat::enum_convert(icu_experimental::displaynames::Fallback, needs_wildcard)]
    pub enum DisplayNamesFallback {
        Code,
        None,
    }

    #[diplomat::rust_link(icu::displaynames::options::LanguageDisplay, Enum)]
    #[diplomat::enum_convert(icu_experimental::displaynames::LanguageDisplay, needs_wildcard)]
    pub enum LanguageDisplay {
        Dialect,
        Standard,
    }

    impl LocaleDisplayNamesFormatter {
        /// Creates a new `LocaleDisplayNamesFormatter` from locale data and an options bag.
        #[diplomat::rust_link(icu::displaynames::LocaleDisplayNamesFormatter::try_new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "create")]
        pub fn create_v1(
            provider: &DataProvider,
            locale: &Locale,
            options: DisplayNamesOptionsV1,
        ) -> Result<Box<LocaleDisplayNamesFormatter>, DataError> {
            let locale = locale.to_datalocale();
            let options = icu_experimental::displaynames::DisplayNamesOptions::from(options);

            Ok(Box::new(LocaleDisplayNamesFormatter(
                call_constructor!(
                    icu_experimental::displaynames::LocaleDisplayNamesFormatter::try_new,
                    icu_experimental::displaynames::LocaleDisplayNamesFormatter::try_new_with_any_provider,
                    icu_experimental::displaynames::LocaleDisplayNamesFormatter::try_new_with_buffer_provider,
                    provider,
                    &locale,
                    options,
                )?,
            )))
        }

        /// Returns the locale-specific display name of a locale.
        #[diplomat::rust_link(icu::displaynames::LocaleDisplayNamesFormatter::of, FnInStruct)]
        pub fn of(&self, locale: &Locale, write: &mut DiplomatWrite) {
            let _infallible = self.0.of(&locale.0).write_to(write);
        }
    }

    impl RegionDisplayNames {
        /// Creates a new `RegionDisplayNames` from locale data and an options bag.
        #[diplomat::rust_link(icu::displaynames::RegionDisplayNames::try_new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        pub fn create(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<RegionDisplayNames>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(RegionDisplayNames(call_constructor!(
                icu_experimental::displaynames::RegionDisplayNames::try_new,
                icu_experimental::displaynames::RegionDisplayNames::try_new_with_any_provider,
                icu_experimental::displaynames::RegionDisplayNames::try_new_with_buffer_provider,
                provider,
                &locale,
                Default::default()
            )?)))
        }

        /// Returns the locale specific display name of a region.
        /// Note that the function returns an empty string in case the display name for a given
        /// region code is not found.
        #[diplomat::rust_link(icu::displaynames::RegionDisplayNames::of, FnInStruct)]
        pub fn of(
            &self,
            region: &DiplomatStr,
            write: &mut DiplomatWrite,
        ) -> Result<(), LocaleParseError> {
            let _infallible = self
                .0
                .of(icu_locale_core::subtags::Region::try_from_utf8(region)?)
                .unwrap_or("")
                .write_to(write);
            Ok(())
        }
    }
}

impl From<ffi::DisplayNamesStyle> for Option<icu_experimental::displaynames::Style> {
    fn from(style: ffi::DisplayNamesStyle) -> Option<icu_experimental::displaynames::Style> {
        match style {
            ffi::DisplayNamesStyle::Auto => None,
            ffi::DisplayNamesStyle::Narrow => Some(icu_experimental::displaynames::Style::Narrow),
            ffi::DisplayNamesStyle::Short => Some(icu_experimental::displaynames::Style::Short),
            ffi::DisplayNamesStyle::Long => Some(icu_experimental::displaynames::Style::Long),
            ffi::DisplayNamesStyle::Menu => Some(icu_experimental::displaynames::Style::Menu),
        }
    }
}

impl From<ffi::DisplayNamesOptionsV1> for icu_experimental::displaynames::DisplayNamesOptions {
    fn from(
        other: ffi::DisplayNamesOptionsV1,
    ) -> icu_experimental::displaynames::DisplayNamesOptions {
        let mut options = icu_experimental::displaynames::DisplayNamesOptions::default();
        options.style = other.style.into();
        options.fallback = other.fallback.into();
        options.language_display = other.language_display.into();
        options
    }
}
