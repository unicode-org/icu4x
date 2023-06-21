// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::internals::{CaseMapLocale, FoldOptions};
use crate::provider::data::MappingKind;
use crate::provider::CaseMappingV1Marker;
use crate::set::ClosureSet;
use alloc::string::String;
use icu_locid::Locale;
use icu_provider::prelude::*;
use writeable::Writeable;

/// A struct with the ability to convert characters and strings to uppercase or lowercase,
/// or fold them to a normalized form for case-insensitive comparison.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2535">#2535</a>
/// </div>
#[derive(Clone, Debug)]
pub struct CaseMapping {
    data: DataPayload<CaseMappingV1Marker>,
    locale: CaseMapLocale,
}

#[cfg(feature = "data")]
impl Default for CaseMapping {
    fn default() -> Self {
        Self::new()
    }
}

impl CaseMapping {
    /// A constructor which creates a [`CaseMapping`].
    ///
    /// ✨ **Enabled with the `"data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "data")]
    pub const fn new() -> Self {
        Self {
            data: DataPayload::from_static_ref(crate::provider::Baked::SINGLETON_PROPS_CASEMAP_V1),
            locale: CaseMapLocale::Root,
        }
    }

    /// A constructor which creates a [`CaseMapping`] for the given locale.
    ///
    /// ✨ **Enabled with the `"data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "data")]
    pub const fn new_with_locale(locale: &Locale) -> Self {
        let locale = CaseMapLocale::from_locale(locale);
        Self {
            data: DataPayload::from_static_ref(crate::provider::Baked::SINGLETON_PROPS_CASEMAP_V1),
            locale,
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(locale: skip, options: skip, error: DataError,
    #[cfg(skip)]
    functions: [
        new,
        try_new_with_any_provider,
        try_new_with_buffer_provider,
        try_new_unstable,
        Self,
    ]);

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<CaseMapping, DataError>
    where
        P: DataProvider<CaseMappingV1Marker> + ?Sized,
    {
        Self::try_new_with_locale_unstable(provider, &Locale::UND)
    }

    icu_provider::gen_any_buffer_data_constructors!(locale: skip, options: &Locale, error: DataError,
    #[cfg(skip)]
    functions: [
        new_with_locale,
        try_new_with_locale_with_any_provider,
        try_new_with_locale_with_buffer_provider,
        try_new_with_locale_unstable,
        Self,
    ]);

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_with_locale_unstable<P>(
        provider: &P,
        locale: &Locale,
    ) -> Result<CaseMapping, DataError>
    where
        P: DataProvider<CaseMappingV1Marker> + ?Sized,
    {
        let data = provider.load(Default::default())?.take_payload()?;
        let locale = CaseMapLocale::from_locale(locale);
        Ok(Self { data, locale })
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

    /// Returns the full lowercase mapping of the given string as a [`Writeable`].
    /// This function is context and locale sensitive.
    ///
    /// See [`Self::to_full_lowercase_string()`] for the equivalent convenience function that returns a String
    pub fn to_full_lowercase<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
        self.data
            .get()
            .full_helper_writeable(src, self.locale, MappingKind::Lower)
    }

    /// Returns the full uppercase mapping of the given string as a [`Writeable`].
    /// This function is context and locale sensitive.
    ///
    /// See [`Self::to_full_uppercase_string()`] for the equivalent convenience function that returns a String
    pub fn to_full_uppercase<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
        self.data
            .get()
            .full_helper_writeable(src, self.locale, MappingKind::Upper)
    }

    /// Case-folds the characters in the given string as a [`Writeable`].
    /// This function is locale-independent and context-insensitive.
    ///
    /// See [`Self::full_fold_string()`] for the equivalent convenience function that returns a String
    pub fn full_fold<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
        self.data
            .get()
            .full_helper_writeable(src, CaseMapLocale::Root, MappingKind::Fold)
    }

    /// Case-folds the characters in the given string as a [`Writeable`],
    /// using Turkic (T) mappings for dotted/dotless I.
    /// This function is locale-independent and context-insensitive.
    ///
    /// See [`Self::full_fold_turkic_string()`] for the equivalent convenience function that returns a String
    pub fn full_fold_turkic<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
        self.data
            .get()
            .full_helper_writeable(src, CaseMapLocale::Turkish, MappingKind::Fold)
    }

    /// Returns the full lowercase mapping of the given string as a String.
    /// This function is context and locale sensitive.
    ///
    /// See [`Self::to_full_lowercase()`] for the equivalent lower-level function that returns a [`Writeable`]
    pub fn to_full_lowercase_string(&self, src: &str) -> String {
        self.data
            .get()
            .full_helper_writeable(src, self.locale, MappingKind::Lower)
            .write_to_string()
            .into_owned()
    }

    /// Returns the full uppercase mapping of the given string as a String.
    /// This function is context and locale sensitive.
    ///
    /// See [`Self::to_full_uppercase()`] for the equivalent lower-level function that returns a [`Writeable`]
    pub fn to_full_uppercase_string(&self, src: &str) -> String {
        self.data
            .get()
            .full_helper_writeable(src, self.locale, MappingKind::Upper)
            .write_to_string()
            .into_owned()
    }

    /// Case-folds the characters in the given string as a String.
    /// This function is locale-independent and context-insensitive.
    ///
    /// See [`Self::full_fold()`] for the equivalent lower-level function that returns a [`Writeable`]
    pub fn full_fold_string(&self, src: &str) -> String {
        self.data
            .get()
            .full_helper_writeable(src, CaseMapLocale::Root, MappingKind::Fold)
            .write_to_string()
            .into_owned()
    }

    /// Case-folds the characters in the given string as a String,
    /// using Turkic (T) mappings for dotted/dotless I.
    /// This function is locale-independent and context-insensitive.
    ///
    /// See [`Self::full_fold_turkic()`] for the equivalent lower-level function that returns a [`Writeable`]
    pub fn full_fold_turkic_string(&self, src: &str) -> String {
        self.data
            .get()
            .full_helper_writeable(src, CaseMapLocale::Turkish, MappingKind::Fold)
            .write_to_string()
            .into_owned()
    }

    /// Adds all simple case mappings and the full case folding for `c` to `set`.
    /// Also adds special case closure mappings.
    ///
    /// In other words, this adds all strings/characters that this casemaps to, as
    /// well as all characters that may casemap to this one.
    ///
    /// The character itself is not added.
    ///
    /// For example, the mappings
    /// - for s include long s
    /// - for sharp s include ss
    /// - for k include the Kelvin sign
    pub fn add_case_closure<S: ClosureSet>(&self, c: char, set: &mut S) {
        self.data.get().add_case_closure(c, set);
    }

    /// Maps the string to single code points and adds the associated case closure
    /// mappings, if they exist.
    ///
    /// The string is mapped to code points if it is their full case folding string.
    /// In other words, this performs a reverse full case folding and then
    /// adds the case closure items of the resulting code points.
    /// If the string is found and its closure applied, then
    /// the string itself is added as well as part of its code points' closure.
    ///
    /// Returns true if the string was found
    pub fn add_string_case_closure<S: ClosureSet>(&self, s: &str, set: &mut S) -> bool {
        self.data.get().add_string_case_closure(s, set)
    }
}
