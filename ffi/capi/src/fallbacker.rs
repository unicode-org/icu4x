// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::LocaleParseError;
    use crate::{errors::ffi::DataError, locale_core::ffi::Locale, provider::ffi::DataProvider};

    /// An object that runs the ICU4X locale fallback algorithm.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::locale::fallback::LocaleFallbacker, Struct)]
    #[diplomat::rust_link(icu::locale::fallback::LocaleFallbackerBorrowed, Struct, hidden)]
    pub struct LocaleFallbacker(pub icu_locale::LocaleFallbacker);

    /// Priority mode for the ICU4X fallback algorithm.
    #[diplomat::enum_convert(icu_locale::fallback::LocaleFallbackPriority, needs_wildcard)]
    #[diplomat::rust_link(icu::locale::fallback::LocaleFallbackPriority, Enum)]
    #[diplomat::rust_link(
        icu::locale::fallback::LocaleFallbackPriority::const_default,
        FnInEnum,
        hidden
    )]
    pub enum LocaleFallbackPriority {
        Language = 0,
        Region = 1,
        Collation = 2,
    }

    /// What additional data is required to load when performing fallback.
    #[diplomat::rust_link(icu::locale::fallback::LocaleFallbackSupplement, Enum)]
    #[diplomat::rust_link(
        icu::locale::fallback::LocaleFallbackSupplement::const_default,
        FnInEnum,
        hidden
    )]
    pub enum LocaleFallbackSupplement {
        None = 0,
        Collation = 1,
    }

    /// Collection of configurations for the ICU4X fallback algorithm.
    #[diplomat::rust_link(icu::locale::fallback::LocaleFallbackConfig, Struct)]
    #[diplomat::rust_link(
        icu::locale::fallback::LocaleFallbackConfig::const_default,
        FnInStruct,
        hidden
    )]
    pub struct LocaleFallbackConfig<'a> {
        /// Choice of priority mode.
        pub priority: LocaleFallbackPriority,
        /// An empty string is considered `None`.
        pub extension_key: &'a DiplomatStr,
        /// Fallback supplement data key to customize fallback rules.
        pub fallback_supplement: LocaleFallbackSupplement,
    }

    /// An object that runs the ICU4X locale fallback algorithm with specific configurations.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::locale::fallback::LocaleFallbacker, Struct)]
    #[diplomat::rust_link(icu::locale::fallback::LocaleFallbackerWithConfig, Struct)]
    pub struct LocaleFallbackerWithConfig<'a>(
        pub icu_locale::fallback::LocaleFallbackerWithConfig<'a>,
    );

    /// An iterator over the locale under fallback.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::locale::fallback::LocaleFallbackIterator, Struct)]
    pub struct LocaleFallbackIterator<'a>(pub icu_locale::fallback::LocaleFallbackIterator<'a, 'a>);

    impl LocaleFallbacker {
        /// Creates a new `LocaleFallbacker` from a data provider.
        #[diplomat::rust_link(icu::locale::fallback::LocaleFallbacker::new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(provider: &DataProvider) -> Result<Box<LocaleFallbacker>, DataError> {
            Ok(Box::new(LocaleFallbacker(call_constructor!(
                icu_locale::LocaleFallbacker::new [r => Ok(r.static_to_owned())],
                icu_locale::LocaleFallbacker::try_new_with_any_provider,
                icu_locale::LocaleFallbacker::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Creates a new `LocaleFallbacker` without data for limited functionality.
        #[diplomat::rust_link(
            icu::locale::fallback::LocaleFallbacker::new_without_data,
            FnInStruct
        )]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "without_data")]
        pub fn create_without_data() -> Box<LocaleFallbacker> {
            Box::new(LocaleFallbacker(
                icu_locale::LocaleFallbacker::new_without_data(),
            ))
        }

        /// Associates this `LocaleFallbacker` with configuration options.
        #[diplomat::rust_link(icu::locale::fallback::LocaleFallbacker::for_config, FnInStruct)]
        #[diplomat::rust_link(
            icu::locale::fallback::LocaleFallbackerBorrowed::for_config,
            FnInStruct,
            hidden
        )]
        pub fn for_config<'a, 'temp>(
            &'a self,
            config: LocaleFallbackConfig<'temp>,
        ) -> Result<Box<LocaleFallbackerWithConfig<'a>>, LocaleParseError> {
            Ok(Box::new(LocaleFallbackerWithConfig(self.0.for_config(
                icu_locale::fallback::LocaleFallbackConfig::try_from(config)?,
            ))))
        }
    }

    impl<'a> LocaleFallbackerWithConfig<'a> {
        /// Creates an iterator from a locale with each step of fallback.
        #[diplomat::rust_link(icu::locale::fallback::LocaleFallbacker::fallback_for, FnInStruct)]
        #[diplomat::rust_link(
            icu::locale::fallback::LocaleFallbackerBorrowed::fallback_for,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(
            icu::locale::fallback::LocaleFallbackerWithConfig::fallback_for,
            FnInStruct,
            hidden
        )]
        pub fn fallback_for_locale<'b: 'a, 'temp>(
            &'b self,
            locale: &'temp Locale,
        ) -> Box<LocaleFallbackIterator<'a>> {
            Box::new(LocaleFallbackIterator(
                self.0.fallback_for((&locale.0).into()),
            ))
        }
    }

    impl<'a> LocaleFallbackIterator<'a> {
        #[diplomat::attr(supports = iterators, iterator)]
        #[diplomat::rust_link(
            icu::locale::fallback::LocaleFallbackIterator::get,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(
            icu::locale::fallback::LocaleFallbackIterator::step,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(
            icu::locale::fallback::LocaleFallbackIterator::take,
            FnInStruct,
            hidden
        )]
        pub fn next(&mut self) -> Option<Box<Locale>> {
            let current = self.0.get();
            if current.is_und() {
                None
            } else {
                let current = current.clone().into_locale();
                self.0.step();
                Some(Box::new(Locale(current)))
            }
        }
    }
}

impl TryFrom<ffi::LocaleFallbackConfig<'_>> for icu_locale::fallback::LocaleFallbackConfig {
    type Error = crate::errors::ffi::LocaleParseError;
    fn try_from(other: ffi::LocaleFallbackConfig) -> Result<Self, Self::Error> {
        let mut result = Self::default();
        result.priority = other.priority.into();
        result.extension_key = match other.extension_key {
            b"" => None,
            s => Some(icu_locale_core::extensions::unicode::Key::try_from_utf8(s)?),
        };
        result.fallback_supplement = match other.fallback_supplement {
            ffi::LocaleFallbackSupplement::None => None,
            ffi::LocaleFallbackSupplement::Collation => {
                Some(icu_locale::fallback::LocaleFallbackSupplement::Collation)
            }
        };
        Ok(result)
    }
}
