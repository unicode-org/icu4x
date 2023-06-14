// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "datagen")]
use crate::error::Error;
use crate::internals::{CaseMapLocale, FoldOptions};
#[cfg(feature = "datagen")]
use crate::provider::CaseMappingV1;
use crate::provider::CaseMappingV1Marker;
#[cfg(feature = "datagen")]
use icu_collections::codepointtrie::CodePointTrieHeader;
use icu_locid::Locale;
use icu_provider::prelude::*;

/// A struct with the ability to convert characters and strings to uppercase or lowercase,
/// or fold them to a normalized form for case-insensitive comparison.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2535">#2535</a>
/// </div>
#[derive(Clone)]
pub struct CaseMapping {
    data: DataPayload<CaseMappingV1Marker>,
    locale: CaseMapLocale,
}

impl CaseMapping {
    /// A constructor which takes a [`DataProvider`] and creates a [`CaseMapping`].
    ///
    /// TODO before stabilitzation: make this return a crate-scoped error.
    pub fn try_new<P>(provider: &P) -> Result<CaseMapping, DataError>
    where
        P: DataProvider<CaseMappingV1Marker> + ?Sized,
    {
        Self::try_new_with_locale(provider, &Locale::UND)
    }

    /// A constructor which takes a [`DataProvider`] and creates a [`CaseMapping`] for the given locale.
    ///
    /// TODO before stabilitzation: make this return a crate-scoped error.
    pub fn try_new_with_locale<P>(provider: &P, locale: &Locale) -> Result<CaseMapping, DataError>
    where
        P: DataProvider<CaseMappingV1Marker> + ?Sized,
    {
        let data = provider.load(Default::default())?.take_payload()?;
        debug_assert!(data.get().validate().is_ok());
        let locale = CaseMapLocale::from(locale);
        Ok(Self { data, locale })
    }

    /// Creates a new CaseMapping using data exported by the `icuexportdata` tool
    /// in ICU4C. Validates that the data is consistent.
    #[cfg(feature = "datagen")]
    pub fn try_from_icu(
        trie_header: CodePointTrieHeader,
        trie_index: &[u16],
        trie_data: &[u16],
        exceptions: &[u16],
        unfold: &[u16],
    ) -> Result<Self, Error> {
        let data =
            CaseMappingV1::try_from_icu(trie_header, trie_index, trie_data, exceptions, unfold)?;
        let locale = CaseMapLocale::Root;
        Ok(Self {
            data: DataPayload::from_owned(data),
            locale,
        })
    }

    /// Returns the lowercase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    /// For full mappings, use [`CaseMapping::to_full_lowercase`].
    pub fn to_lowercase(&self, c: char) -> char {
        self.data.get().simple_lower(c)
    }

    /// Returns the uppercase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    /// For full mappings, use [`CaseMapping::to_full_uppercase`].
    pub fn to_uppercase(&self, c: char) -> char {
        self.data.get().simple_upper(c)
    }

    /// Returns the titlecase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    pub fn to_titlecase(&self, c: char) -> char {
        self.data.get().simple_title(c)
    }

    /// Returns the simple case folding mapping of the given char.
    /// For full mappings, use [`CaseMapping::full_fold`].
    pub fn fold(&self, c: char) -> char {
        self.data.get().simple_fold(c, FoldOptions::default())
    }

    /// Returns the simple case folding mapping of the given char, using Turkic (T) mappings for
    /// dotted/dotless i. This function does not fold `i` and `I` to the same character. Instead,
    /// `I` will fold to `ı`, and `İ` will fold to `i`. Otherwise, this is the same as
    /// [`CaseMapping::fold()`].
    pub fn fold_turkic(&self, c: char) -> char {
        self.data
            .get()
            .simple_fold(c, FoldOptions::with_turkic_mappings())
    }

    /// Returns the full lowercase mapping of the given string.
    /// This function is context and locale sensitive.
    pub fn to_full_lowercase(&self, src: &str) -> String {
        self.data.get().full_lowercase(src, self.locale)
    }

    /// Returns the full uppercase mapping of the given string.
    /// This function is context and locale sensitive.
    pub fn to_full_uppercase(&self, src: &str) -> String {
        self.data.get().full_uppercase(src, self.locale)
    }

    /// Case-folds the characters in the given string.
    /// This function is locale-independent and context-insensitive.
    pub fn full_fold(&self, src: &str) -> String {
        self.data.get().full_folding(src, CaseMapLocale::Root)
    }

    /// Case-folds the characters in the given string, using Turkic (T) mappings for dotted/dotless I.
    /// This function is locale-independent and context-insensitive.
    pub fn full_fold_turkic(&self, src: &str) -> String {
        self.data.get().full_folding(src, CaseMapLocale::Turkish)
    }
}
