// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};

    use crate::{locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider};

    /// FFI version of `CanonicalizationResult`.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/enum.CanonicalizationResult.html) for more details.
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
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html) for more details.
    #[diplomat::opaque]
    pub struct ICU4XLocaleCanonicalizer(LocaleCanonicalizer<'static, 'static>);

    impl ICU4XLocaleCanonicalizer {
        /// Create a new [`ICU4XLocaleCanonicalizer`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.new) for more details.
        fn create(provider: &ICU4XDataProvider) -> Option<Box<ICU4XLocaleCanonicalizer>> {
            let provider = provider.0.as_ref();
            LocaleCanonicalizer::new(provider)
                .ok()
                .map(|lc| Box::new(ICU4XLocaleCanonicalizer(lc)))
        }

        /// FFI version of `LocaleCanonicalizer::canonicalize()`.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.canonicalize) for more details.
        fn canonicalize(&self, locale: &mut ICU4XLocale) -> ICU4XCanonicalizationResult {
            canonicalization_result_to_ffi(self.0.canonicalize(&mut locale.0))
        }

        /// FFI version of `LocaleCanonicalizer::maximize()`.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.maximize) for more details.
        fn maximize(&self, locale: &mut ICU4XLocale) -> ICU4XCanonicalizationResult {
            canonicalization_result_to_ffi(self.0.maximize(&mut locale.0))
        }

        /// FFI version of `LocaleCanonicalizer::minimize()`.
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.minimize) for more details.
        fn minimize(&self, locale: &mut ICU4XLocale) -> ICU4XCanonicalizationResult {
            canonicalization_result_to_ffi(self.0.minimize(&mut locale.0))
        }
    }
}
