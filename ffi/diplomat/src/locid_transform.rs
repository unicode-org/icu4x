// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_locid_transform::{LocaleCanonicalizer, LocaleExpander, TransformResult};

    use crate::{locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider};

    use crate::errors::ffi::ICU4XError;
    use diplomat_runtime::DiplomatResult;

    /// FFI version of `TransformResult`.
    #[diplomat::rust_link(icu::locale_canonicalizer::TransformResult, Enum)]
    pub enum ICU4XTransformResult {
        Modified,
        Unmodified,
    }

    // TODO(shadaj): replace with diplomat-ignored from impl
    fn canonicalization_result_to_ffi(result: TransformResult) -> ICU4XTransformResult {
        match result {
            TransformResult::Modified => ICU4XTransformResult::Modified,
            TransformResult::Unmodified => ICU4XTransformResult::Unmodified,
        }
    }

    /// A locale canonicalizer.
    #[diplomat::rust_link(icu::locale_canonicalizer::LocaleCanonicalizer, Struct)]
    #[diplomat::opaque]
    pub struct ICU4XLocaleCanonicalizer(LocaleCanonicalizer);

    impl ICU4XLocaleCanonicalizer {
        /// Create a new [`ICU4XLocaleCanonicalizer`].
        #[diplomat::rust_link(icu::locale_canonicalizer::LocaleCanonicalizer::new, FnInStruct)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XLocaleCanonicalizer>, ICU4XError> {
            LocaleCanonicalizer::try_new_unstable(&provider.0)
                .map(|lc| Box::new(ICU4XLocaleCanonicalizer(lc)))
                .map_err(Into::into)
                .into()
        }

        /// FFI version of `LocaleCanonicalizer::canonicalize()`.
        #[diplomat::rust_link(
            icu::locale_canonicalizer::LocaleCanonicalizer::canonicalize,
            FnInStruct
        )]
        pub fn canonicalize(&self, locale: &mut ICU4XLocale) -> ICU4XTransformResult {
            canonicalization_result_to_ffi(self.0.canonicalize(&mut locale.0))
        }
    }

    /// A locale expander.
    #[diplomat::rust_link(icu::locale_canonicalizer::LocaleExpander, Struct)]
    #[diplomat::opaque]
    pub struct ICU4XLocaleExpander(LocaleExpander);

    impl ICU4XLocaleExpander {
        /// Create a new [`ICU4XLocaleExpander`].
        #[diplomat::rust_link(icu::locale_canonicalizer::LocaleExpander::new, FnInStruct)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XLocaleExpander>, ICU4XError> {
            LocaleExpander::try_new_unstable(&provider.0)
                .map(|lc| Box::new(ICU4XLocaleExpander(lc)))
                .map_err(Into::into)
                .into()
        }

        /// FFI version of `LocaleExpander::maximize()`.
        #[diplomat::rust_link(icu::locale_canonicalizer::LocaleExpander::maximize, FnInStruct)]
        pub fn maximize(&self, locale: &mut ICU4XLocale) -> ICU4XTransformResult {
            canonicalization_result_to_ffi(self.0.maximize(&mut locale.0))
        }

        /// FFI version of `LocaleExpander::minimize()`.
        #[diplomat::rust_link(icu::locale_canonicalizer::LocaleExpander::minimize, FnInStruct)]
        pub fn minimize(&self, locale: &mut ICU4XLocale) -> ICU4XTransformResult {
            canonicalization_result_to_ffi(self.0.minimize(&mut locale.0))
        }
    }
}
