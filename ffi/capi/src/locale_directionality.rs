// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    #[cfg(feature = "buffer_provider")]
    use crate::errors::ffi::DataError;
    #[cfg(feature = "buffer_provider")]
    use crate::provider::{ffi::DataProvider, DataProviderInner};

    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::locale::ffi::LocaleExpander;
    use crate::locale_core::ffi::Locale;

    #[diplomat::rust_link(icu::locale::Direction, Enum)]
    pub enum LocaleDirection {
        LeftToRight,
        RightToLeft,
        Unknown,
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::locale::LocaleDirectionality, Struct)]
    pub struct LocaleDirectionality(pub icu_locale::LocaleDirectionality);

    impl LocaleDirectionality {
        /// Construct a new LocaleDirectionality instance using compiled data.
        #[diplomat::rust_link(icu::locale::LocaleDirectionality::new, FnInStruct)]
        #[diplomat::attr(supports = constructors, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Box<LocaleDirectionality> {
            Box::new(LocaleDirectionality(icu_locale::LocaleDirectionality::new()))
        }

        /// Construct a new LocaleDirectionality instance using a particular data source.
        #[diplomat::rust_link(icu::locale::LocaleDirectionality::new, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<LocaleDirectionality>, DataError> {
            Ok(Box::new(LocaleDirectionality(call_constructor!(
                icu_locale::LocaleDirectionality::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Construct a new LocaleDirectionality instance with a custom expander and compiled data.
        #[diplomat::rust_link(icu::locale::LocaleDirectionality::new_with_expander, FnInStruct)]
        #[diplomat::attr(supports = named_constructors, named_constructor = "with_expander")]
        #[cfg(feature = "compiled_data")]
        pub fn create_with_expander(expander: &LocaleExpander) -> Box<LocaleDirectionality> {
            Box::new(LocaleDirectionality(
                icu_locale::LocaleDirectionality::new_with_expander(expander.0.clone()),
            ))
        }

        /// Construct a new LocaleDirectionality instance with a custom expander and a particular data source.
        #[diplomat::rust_link(icu::locale::LocaleDirectionality::new_with_expander, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "with_expander_and_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_expander_and_provider(
            provider: &DataProvider,
            expander: &LocaleExpander,
        ) -> Result<Box<LocaleDirectionality>, DataError> {
            #[allow(unused_imports)]
            use icu_provider::prelude::*;
            Ok(Box::new(LocaleDirectionality(match &provider.0 {
                DataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                    "This provider has been destroyed",
                ))?,

                DataProviderInner::Buffer(buffer_provider) => {
                    icu_locale::LocaleDirectionality::try_new_with_expander_unstable(
                        &buffer_provider.as_deserializing(),
                        expander.0.clone(),
                    )?
                }
            })))
        }

        #[diplomat::rust_link(icu::locale::LocaleDirectionality::get, FnInStruct)]
        #[diplomat::attr(auto, indexer)]
        pub fn get(&self, locale: &Locale) -> LocaleDirection {
            match self.0.get(&locale.0.id) {
                Some(icu_locale::Direction::LeftToRight) => LocaleDirection::LeftToRight,
                Some(icu_locale::Direction::RightToLeft) => LocaleDirection::RightToLeft,
                _ => LocaleDirection::Unknown,
            }
        }

        #[diplomat::rust_link(icu::locale::LocaleDirectionality::is_left_to_right, FnInStruct)]
        pub fn is_left_to_right(&self, locale: &Locale) -> bool {
            self.0.is_left_to_right(&locale.0.id)
        }

        #[diplomat::rust_link(icu::locale::LocaleDirectionality::is_right_to_left, FnInStruct)]
        pub fn is_right_to_left(&self, locale: &Locale) -> bool {
            self.0.is_right_to_left(&locale.0.id)
        }
    }
}
