// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::{
        errors::ffi::ICU4XDataError,
        locale::ffi::ICU4XLocaleExpander,
        locale_core::ffi::ICU4XLocale,
        provider::{ffi::ICU4XDataProvider, ICU4XDataProviderInner},
    };

    #[diplomat::rust_link(icu::locale::Direction, Enum)]
    pub enum ICU4XLocaleDirection {
        LeftToRight,
        RightToLeft,
        Unknown,
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::locale::LocaleDirectionality, Struct)]
    pub struct ICU4XLocaleDirectionality(pub icu_locale::LocaleDirectionality);

    impl ICU4XLocaleDirectionality {
        /// Construct a new ICU4XLocaleDirectionality instance
        #[diplomat::rust_link(icu::locale::LocaleDirectionality::new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XLocaleDirectionality>, ICU4XDataError> {
            Ok(Box::new(ICU4XLocaleDirectionality(call_constructor!(
                icu_locale::LocaleDirectionality::new [r => Ok(r)],
                icu_locale::LocaleDirectionality::try_new_with_any_provider,
                icu_locale::LocaleDirectionality::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Construct a new ICU4XLocaleDirectionality instance with a custom expander
        #[diplomat::rust_link(icu::locale::LocaleDirectionality::new_with_expander, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "with_expander")]
        pub fn create_with_expander(
            provider: &ICU4XDataProvider,
            expander: &ICU4XLocaleExpander,
        ) -> Result<Box<ICU4XLocaleDirectionality>, ICU4XDataError> {
            #[allow(unused_imports)]
            use icu_provider::prelude::*;
            Ok(Box::new(ICU4XLocaleDirectionality(match &provider.0 {
                ICU4XDataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                    "This provider has been destroyed",
                ))?,
                ICU4XDataProviderInner::Empty => {
                    icu_locale::LocaleDirectionality::try_new_with_expander_unstable(
                        &icu_provider_adapters::empty::EmptyDataProvider::new(),
                        expander.0.clone(),
                    )?
                }
                #[cfg(feature = "buffer_provider")]
                ICU4XDataProviderInner::Buffer(buffer_provider) => {
                    icu_locale::LocaleDirectionality::try_new_with_expander_unstable(
                        &buffer_provider.as_deserializing(),
                        expander.0.clone(),
                    )?
                }
                #[cfg(feature = "compiled_data")]
                ICU4XDataProviderInner::Compiled => {
                    icu_locale::LocaleDirectionality::new_with_expander(expander.0.clone())
                }
            })))
        }

        #[diplomat::rust_link(icu::locale::LocaleDirectionality::get, FnInStruct)]
        #[diplomat::attr(supports = indexing, indexer)]
        pub fn get(&self, locale: &ICU4XLocale) -> ICU4XLocaleDirection {
            match self.0.get(&locale.0) {
                Some(icu_locale::Direction::LeftToRight) => ICU4XLocaleDirection::LeftToRight,
                Some(icu_locale::Direction::RightToLeft) => ICU4XLocaleDirection::RightToLeft,
                _ => ICU4XLocaleDirection::Unknown,
            }
        }

        #[diplomat::rust_link(icu::locale::LocaleDirectionality::is_left_to_right, FnInStruct)]
        pub fn is_left_to_right(&self, locale: &ICU4XLocale) -> bool {
            self.0.is_left_to_right(&locale.0)
        }

        #[diplomat::rust_link(icu::locale::LocaleDirectionality::is_right_to_left, FnInStruct)]
        pub fn is_right_to_left(&self, locale: &ICU4XLocale) -> bool {
            self.0.is_right_to_left(&locale.0)
        }
    }
}
