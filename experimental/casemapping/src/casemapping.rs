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
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.to_full_uppercase_string("hello world"), "HELLO WORLD");
    /// ```
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
    /// This does not perform any complex fallback: the locale used must have an exact match
    /// of a language with special casemapping for this to have any effect. Currently, this list
    /// comprises of Turkish, Lithuanian, Greek, Dutch, and Armenian. All other languages
    /// will have the behavior of the root locale. Regions and scripts are ignored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    /// use icu_locid::locale;
    ///
    /// let cm = CaseMapping::new_with_locale(&locale!("tr"));
    ///
    /// assert_eq!(cm.to_full_uppercase_string("istanbul"), "İSTANBUL");
    /// ```
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.to_lowercase('C'), 'c');
    /// assert_eq!(cm.to_lowercase('c'), 'c');
    /// assert_eq!(cm.to_lowercase('Ć'), 'ć');
    /// assert_eq!(cm.to_lowercase('Γ'), 'γ');
    /// ```
    pub fn to_lowercase(&self, c: char) -> char {
        self.data.get().simple_lower(c)
    }

    /// Returns the uppercase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    /// For full mappings, use [`CaseMapping::to_full_uppercase`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.to_uppercase('c'), 'C');
    /// assert_eq!(cm.to_uppercase('C'), 'C');
    /// assert_eq!(cm.to_uppercase('ć'), 'Ć');
    /// assert_eq!(cm.to_uppercase('γ'), 'Γ');
    ///
    /// assert_eq!(cm.to_uppercase('ǳ'), 'Ǳ');
    /// ```
    pub fn to_uppercase(&self, c: char) -> char {
        self.data.get().simple_upper(c)
    }

    /// Returns the titlecase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.to_titlecase('ǳ'), 'ǲ');
    ///
    /// assert_eq!(cm.to_titlecase('c'), 'C');
    /// assert_eq!(cm.to_titlecase('C'), 'C');
    /// assert_eq!(cm.to_titlecase('ć'), 'Ć');
    /// assert_eq!(cm.to_titlecase('γ'), 'Γ');
    /// ```
    pub fn to_titlecase(&self, c: char) -> char {
        self.data.get().simple_title(c)
    }

    /// Returns the simple case folding of the given char.
    /// For full mappings, use [`CaseMapping::full_fold`].
    ///
    /// This function can be used to perform caseless matches on
    /// individual characters.
    /// > *Note:* With Unicode 15.0 data, there are three
    /// > pairs of characters for which equivalence under this
    /// > function is inconsistent with equivalence of the
    /// > one-character strings under [`CaseMapping::full_fold`].
    /// > This is resolved in Unicode 15.1 and later.
    ///
    /// For compatibility applications where simple case folding
    /// of strings is required, this function can be applied to
    /// each character of a string.  Note that the resulting
    /// equivalence relation is different from that obtained
    /// by [`CaseMapping::full_fold`]:
    /// The strings "Straße" and "STRASSE" are distinct
    /// under simple case folding, but are equivalent under
    /// default (full) case folding.
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// // perform case insensitive checks
    /// assert_eq!(cm.fold('σ'), cm.fold('ς'));
    /// assert_eq!(cm.fold('Σ'), cm.fold('ς'));
    ///
    /// assert_eq!(cm.fold('c'), 'c');
    /// assert_eq!(cm.fold('Ć'), 'ć');
    /// assert_eq!(cm.fold('Γ'), 'γ');
    /// assert_eq!(cm.fold('ς'), 'σ');
    ///
    /// assert_eq!(cm.fold('ß'), 'ß');
    /// assert_eq!(cm.fold('I'), 'i');
    /// assert_eq!(cm.fold('İ'), 'İ');
    /// assert_eq!(cm.fold('ı'), 'ı');
    /// ```
    pub fn fold(&self, c: char) -> char {
        self.data.get().simple_fold(c, FoldOptions::default())
    }

    /// Returns the simple case folding of the given char, using Turkic (T) mappings for
    /// dotted/dotless i. This function does not fold `i` and `I` to the same character. Instead,
    /// `I` will fold to `ı`, and `İ` will fold to `i`. Otherwise, this is the same as
    /// [`CaseMapping::fold()`].
    ///
    /// You can use the case folding to perform Turkic caseless matches on characters
    /// provided they don't full-casefold to strings. To avoid that situation,
    /// convert to a string and use [`CaseMapping::full_fold_turkic`].
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.fold_turkic('I'), 'ı');
    /// assert_eq!(cm.fold_turkic('İ'), 'i');
    /// ```
    pub fn fold_turkic(&self, c: char) -> char {
        self.data
            .get()
            .simple_fold(c, FoldOptions::with_turkic_mappings())
    }

    /// Returns the full lowercase mapping of the given string as a [`Writeable`].
    /// This function is context and locale sensitive.
    ///
    /// See [`Self::to_full_lowercase_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn to_full_lowercase<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
        self.data
            .get()
            .full_helper_writeable(src, self.locale, MappingKind::Lower)
    }

    /// Returns the full uppercase mapping of the given string as a [`Writeable`].
    /// This function is context and locale sensitive.
    ///
    /// See [`Self::to_full_uppercase_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn to_full_uppercase<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
        self.data
            .get()
            .full_helper_writeable(src, self.locale, MappingKind::Upper)
    }

    /// Case-folds the characters in the given string as a [`Writeable`].
    /// This function is locale-independent and context-insensitive.
    ///
    /// Can be used to test if two strings are case-insensitively equivalent.
    ///
    /// See [`Self::full_fold_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn full_fold<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
        self.data
            .get()
            .full_helper_writeable(src, CaseMapLocale::Root, MappingKind::Fold)
    }

    /// Case-folds the characters in the given string as a [`Writeable`],
    /// using Turkic (T) mappings for dotted/dotless I.
    /// This function is locale-independent and context-insensitive.
    ///
    /// Can be used to test if two strings are case-insensitively equivalent.
    ///
    /// See [`Self::full_fold_turkic_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn full_fold_turkic<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
        self.data
            .get()
            .full_helper_writeable(src, CaseMapLocale::Turkish, MappingKind::Fold)
    }

    /// Returns the full lowercase mapping of the given string as a String.
    /// This function is context and locale sensitive.
    ///
    /// See [`Self::to_full_lowercase()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.to_full_lowercase_string("hEllO WorLd"), "hello world");
    /// assert_eq!(cm.to_full_lowercase_string("Γειά σου Κόσμε"), "γειά σου κόσμε");
    /// assert_eq!(cm.to_full_lowercase_string("नमस्ते दुनिया"), "नमस्ते दुनिया");
    /// assert_eq!(cm.to_full_lowercase_string("Привет мир"), "привет мир");
    /// ```
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.to_full_uppercase_string("hEllO WorLd"), "HELLO WORLD");
    /// assert_eq!(cm.to_full_uppercase_string("Γειά σου Κόσμε"), "ΓΕΙΆ ΣΟΥ ΚΌΣΜΕ");
    /// assert_eq!(cm.to_full_uppercase_string("नमस्ते दुनिया"), "नमस्ते दुनिया");
    /// assert_eq!(cm.to_full_uppercase_string("Привет мир"), "ПРИВЕТ МИР");
    /// ```
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
    /// Can be used to test if two strings are case-insensitively equivalent.
    ///
    /// See [`Self::full_fold()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// // Check if two strings are equivalent case insensitively
    /// assert_eq!(cm.full_fold_string("hEllO WorLd"), cm.full_fold_string("HELLO worlD"));
    ///
    /// assert_eq!(cm.full_fold_string("hEllO WorLd"), "hello world");
    /// assert_eq!(cm.full_fold_string("Γειά σου Κόσμε"), "γειά σου κόσμε");
    /// assert_eq!(cm.full_fold_string("नमस्ते दुनिया"), "नमस्ते दुनिया");
    /// assert_eq!(cm.full_fold_string("Привет мир"), "привет мир");
    /// ```
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
    /// Can be used to test if two strings are case-insensitively equivalent.
    ///
    /// See [`Self::full_fold_turkic()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// // Check if two strings are equivalent case insensitively
    /// assert_eq!(cm.full_fold_turkic_string("İstanbul"), cm.full_fold_turkic_string("iSTANBUL"));
    ///
    /// assert_eq!(cm.full_fold_turkic_string("İstanbul not Constantinople"), "istanbul not constantinople");
    /// assert_eq!(cm.full_fold_turkic_string("Istanbul not Constantınople"), "ıstanbul not constantınople");
    ///
    /// assert_eq!(cm.full_fold_turkic_string("hEllO WorLd"), "hello world");
    /// assert_eq!(cm.full_fold_turkic_string("Γειά σου Κόσμε"), "γειά σου κόσμε");
    /// assert_eq!(cm.full_fold_turkic_string("नमस्ते दुनिया"), "नमस्ते दुनिया");
    /// assert_eq!(cm.full_fold_turkic_string("Привет мир"), "привет мир");
    /// ```
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    /// use icu_collections::codepointinvlist::CodePointInversionListBuilder;
    ///
    /// let cm = CaseMapping::new();
    /// let mut builder = CodePointInversionListBuilder::new();
    /// cm.add_case_closure('s', &mut builder);
    ///
    /// let set = builder.build();
    ///
    /// assert!(set.contains('S'));
    /// assert!(set.contains('ſ'));
    /// assert!(!set.contains('s')); // does not contain itself
    /// ```
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    /// use icu_collections::codepointinvlist::CodePointInversionListBuilder;
    ///
    /// let cm = CaseMapping::new();
    /// let mut builder = CodePointInversionListBuilder::new();
    /// let found = cm.add_string_case_closure("ffi", &mut builder);
    /// assert!(found);
    /// let set = builder.build();
    ///
    /// assert!(set.contains('ﬃ'));
    ///
    /// let mut builder = CodePointInversionListBuilder::new();
    /// let found = cm.add_string_case_closure("ss", &mut builder);
    /// assert!(found);
    /// let set = builder.build();
    ///
    /// assert!(set.contains('ß'));
    /// assert!(set.contains('ẞ'));
    /// ```
    pub fn add_string_case_closure<S: ClosureSet>(&self, s: &str, set: &mut S) -> bool {
        self.data.get().add_string_case_closure(s, set)
    }
}
