// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_displaynames::{LanguageDisplayNames, RegionDisplayNames};
    use icu_displaynames::options::DisplayNamesOptions;
    use diplomat_runtime::DiplomatWriteable;
    use crate::errors::ffi::ICU4XError;
    use crate::locale::ffi::ICU4XLocale;
    use crate::provider::ffi::ICU4XDataProvider;
    use writeable::Writeable;

    //  FFI version of `LanguageDisplayNames`.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::displaynames::LanguageDisplayNames, Struct)]
    pub struct ICU4XLanguageDisplayNames(pub LanguageDisplayNames);

    //  FFI version of `RegionDisplayNames`.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::displaynames::RegionDisplayNames, Struct)]
    pub struct ICU4XRegionDisplayNames(pub RegionDisplayNames);

    //  FFI version of `DisplayNamesOptions`.
    #[diplomat::rust_link(icu::displaynames::options::DisplayNamesOptions, Struct)]
    pub struct ICU4XDisplayNamesOptions {
        /// The optional formatting style to use for display name.
        pub style: ICU4XStyle,
        /// The fallback return when the system does not have the
        /// requested display name, defaults to "code".
        pub fallback: ICU4XFallback,
        /// The language display kind, defaults to "dialect".
        pub language_display: ICU4XLanguageDisplay,
    }

    // FFI version of `Style`.
    #[diplomat::rust_link(icu::displaynames::options::Style, Enum)]
    pub enum ICU4XStyle {
        Auto,
        Narrow,
        Short,
        Long,
        Menu,
    }

    // FFI version of `Fallback`.
    #[diplomat::rust_link(icu::displaynames::options::Fallback, Enum)]
    pub enum ICU4XFallback {
        Code,
        None,
    }

    // FFI version of `LanguageDisplay`.
    #[diplomat::rust_link(icu::displaynames::options::LanguageDisplay, Enum)]
    pub enum ICU4XLanguageDisplay {
        Dialect,
        Standard,
    }

    impl ICU4XLanguageDisplayNames {
        /// Creates a new `LanguageDisplayNames` from locale data and an options bag.
        #[diplomat::rust_link(icu::displaynames::LanguageDisplayNames::try_new_unstable, FnInStruct)]
        pub fn try_new_unstable(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            options: ICU4XDisplayNamesOptions,
        ) -> Result<Box<ICU4XLanguageDisplayNames>, ICU4XError> {
            let locale = locale.to_datalocale();
            let options = DisplayNamesOptions::from(options);

            Ok(Box::new(ICU4XLanguageDisplayNames(LanguageDisplayNames::try_new_unstable(
                &provider.0,
                &locale,
                options,
            )?)))
        }

        // Returns the locale specific display name of a language for a given string.
        // #[diplomat::rust_link(icu::displaynames::LanguageDisplayNames::of, FnInStruct)]
        pub fn of(&self, code: &str, write: &mut DiplomatWriteable) -> Result<(), ()> {
            self.0.of(code).unwrap_or("").write_to(write);
            Ok(())
        }
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
        pub fn of(&self, code: &str, write: &mut DiplomatWriteable) -> Result<(), ()> {
            self.0.of(code).unwrap_or("").write_to(write);
            Ok(())
        }
    }
}

use icu_displaynames::{DisplayNamesOptions, Style, Fallback, LanguageDisplay};

impl From<ffi::ICU4XStyle> for Option<Style> {
    fn from(style: ffi::ICU4XStyle) -> Option<Style> {
        match style {
            ffi::ICU4XStyle::Auto => None,
            ffi::ICU4XStyle::Narrow => Some(Style::Narrow),
            ffi::ICU4XStyle::Short => Some(Style::Short),
            ffi::ICU4XStyle::Long => Some(Style::Long),
            ffi::ICU4XStyle::Menu => Some(Style::Menu),
        }
    }
}

impl From<ffi::ICU4XFallback> for Fallback {
    fn from(fallback: ffi::ICU4XFallback) -> Fallback {
        match fallback {
            ffi::ICU4XFallback::Code => Fallback::Code,
            ffi::ICU4XFallback::None => Fallback::None,
        }
    }
}

impl From<ffi::ICU4XLanguageDisplay> for LanguageDisplay {
    fn from(language_display: ffi::ICU4XLanguageDisplay) -> LanguageDisplay {
        match language_display {
            ffi::ICU4XLanguageDisplay::Dialect => LanguageDisplay::Dialect,
            ffi::ICU4XLanguageDisplay::Standard => LanguageDisplay::Standard,
        }
    }
}

impl From<ffi::ICU4XDisplayNamesOptions> for DisplayNamesOptions {
    fn from(other: ffi::ICU4XDisplayNamesOptions) -> DisplayNamesOptions {
        let mut options = DisplayNamesOptions::default();
        options.style = other.style.into();
        options.fallback = other.fallback.into();
        options.language_display = other.language_display.into();
        options
    }
}
