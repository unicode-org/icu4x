// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::errors::ffi::ICU4XError;
    use crate::locale_core::ffi::ICU4XLocale;
    use crate::provider::ffi::ICU4XDataProvider;

    #[diplomat::rust_link(icu::locale::TransformResult, Enum)]
    #[diplomat::enum_convert(icu_locale::TransformResult)]
    pub enum ICU4XTransformResult {
        Modified,
        Unmodified,
    }

    /// A locale canonicalizer.
    #[diplomat::rust_link(icu::locale::LocaleCanonicalizer, Struct)]
    #[diplomat::opaque]
    pub struct ICU4XLocaleCanonicalizer(icu_locale::LocaleCanonicalizer);

    impl ICU4XLocaleCanonicalizer {
        /// Create a new [`ICU4XLocaleCanonicalizer`].
        #[diplomat::rust_link(icu::locale::LocaleCanonicalizer::new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XLocaleCanonicalizer>, ICU4XError> {
            Ok(Box::new(ICU4XLocaleCanonicalizer(call_constructor!(
                icu_locale::LocaleCanonicalizer::new [r => Ok(r)],
                icu_locale::LocaleCanonicalizer::try_new_with_any_provider,
                icu_locale::LocaleCanonicalizer::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Create a new [`ICU4XLocaleCanonicalizer`] with extended data.
        #[diplomat::rust_link(icu::locale::LocaleCanonicalizer::new_with_expander, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "extended")]
        pub fn create_extended(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XLocaleCanonicalizer>, ICU4XError> {
            let expander = call_constructor!(
                icu_locale::LocaleExpander::new_extended [r => Ok(r)],
                icu_locale::LocaleExpander::try_new_with_any_provider,
                icu_locale::LocaleExpander::try_new_with_buffer_provider,
                provider,
            )?;
            Ok(Box::new(ICU4XLocaleCanonicalizer(call_constructor!(
                icu_locale::LocaleCanonicalizer::new_with_expander [r => Ok(r)],
                icu_locale::LocaleCanonicalizer::try_new_with_expander_with_any_provider,
                icu_locale::LocaleCanonicalizer::try_new_with_expander_with_buffer_provider,
                provider,
                expander
            )?)))
        }

        #[diplomat::rust_link(icu::locale::LocaleCanonicalizer::canonicalize, FnInStruct)]
        pub fn canonicalize(&self, locale: &mut ICU4XLocale) -> ICU4XTransformResult {
            self.0.canonicalize(&mut locale.0).into()
        }
    }

    /// A locale expander.
    #[diplomat::rust_link(icu::locale::LocaleExpander, Struct)]
    #[diplomat::opaque]
    pub struct ICU4XLocaleExpander(pub icu_locale::LocaleExpander);

    impl ICU4XLocaleExpander {
        /// Create a new [`ICU4XLocaleExpander`].
        #[diplomat::rust_link(icu::locale::LocaleExpander::new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XLocaleExpander>, ICU4XError> {
            Ok(Box::new(ICU4XLocaleExpander(call_constructor!(
                icu_locale::LocaleExpander::new [r => Ok(r)],
                icu_locale::LocaleExpander::try_new_with_any_provider,
                icu_locale::LocaleExpander::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Create a new [`ICU4XLocaleExpander`] with extended data.
        #[diplomat::rust_link(icu::locale::LocaleExpander::new_extended, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "extended")]
        pub fn create_extended(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XLocaleExpander>, ICU4XError> {
            Ok(Box::new(ICU4XLocaleExpander(call_constructor!(
                icu_locale::LocaleExpander::new_extended [r => Ok(r)],
                icu_locale::LocaleExpander::try_new_with_any_provider,
                icu_locale::LocaleExpander::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        #[diplomat::rust_link(icu::locale::LocaleExpander::maximize, FnInStruct)]
        pub fn maximize(&self, locale: &mut ICU4XLocale) -> ICU4XTransformResult {
            self.0.maximize(&mut locale.0).into()
        }

        #[diplomat::rust_link(icu::locale::LocaleExpander::minimize, FnInStruct)]
        pub fn minimize(&self, locale: &mut ICU4XLocale) -> ICU4XTransformResult {
            self.0.minimize(&mut locale.0).into()
        }

        #[diplomat::rust_link(icu::locale::LocaleExpander::minimize_favor_script, FnInStruct)]
        pub fn minimize_favor_script(&self, locale: &mut ICU4XLocale) -> ICU4XTransformResult {
            self.0.minimize_favor_script(&mut locale.0).into()
        }
    }
}
