// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};

    use crate::{locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider};

    /// FFI version of `CanonicalizationResult`.
    #[diplomat::rust_link(icu::locale_canonicalizer::CanonicalizationResult, Enum)]
    pub enum ICU4XCanonicalizationResult {
        Modified,
        Unmodified,
    }

    // TODO(shadaj): replace with diplomat-ignored from impl
    fn canonicalization_result_to_ffi(
        result: CanonicalizationResult,
    ) -> ICU4XCanonicalizationResult {
        match result {
            CanonicalizationResult::Modified => ICU4XCanonicalizationResult::Modified,
            CanonicalizationResult::Unmodified => ICU4XCanonicalizationResult::Unmodified,
        }
    }

    /// A locale canonicalizer.
    #[diplomat::rust_link(icu::locale_canonicalizer::LocaleCanonicalizer, Struct)]
    #[diplomat::opaque]
    pub struct ICU4XLocaleCanonicalizer(LocaleCanonicalizer);

    impl ICU4XLocaleCanonicalizer {
        /// Create a new [`ICU4XLocaleCanonicalizer`].
        #[diplomat::rust_link(icu::locale_canonicalizer::LocaleCanonicalizer::new, FnInStruct)]
        pub fn create(provider: &ICU4XDataProvider) -> Option<Box<ICU4XLocaleCanonicalizer>> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            LocaleCanonicalizer::new(&provider)
                .ok()
                .map(|lc| Box::new(ICU4XLocaleCanonicalizer(lc)))
        }

        /// FFI version of `LocaleCanonicalizer::canonicalize()`.
        #[diplomat::rust_link(
            icu::locale_canonicalizer::LocaleCanonicalizer::canonicalize,
            FnInStruct
        )]
        pub fn canonicalize(&self, locale: &mut ICU4XLocale) -> ICU4XCanonicalizationResult {
            canonicalization_result_to_ffi(self.0.canonicalize(&mut locale.0))
        }

        /// FFI version of `LocaleCanonicalizer::maximize()`.
        #[diplomat::rust_link(icu::locale_canonicalizer::LocaleCanonicalizer::maximize, FnInStruct)]
        pub fn maximize(&self, locale: &mut ICU4XLocale) -> ICU4XCanonicalizationResult {
            canonicalization_result_to_ffi(self.0.maximize(&mut locale.0))
        }

        /// FFI version of `LocaleCanonicalizer::minimize()`.
        #[diplomat::rust_link(icu::locale_canonicalizer::LocaleCanonicalizer::minimize, FnInStruct)]
        pub fn minimize(&self, locale: &mut ICU4XLocale) -> ICU4XCanonicalizationResult {
            canonicalization_result_to_ffi(self.0.minimize(&mut locale.0))
        }
    }
}
