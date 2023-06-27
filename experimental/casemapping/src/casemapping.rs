// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::internals::{CaseMapLocale, FoldOptions};
use crate::provider::data::MappingKind;
use crate::provider::CaseMappingV1Marker;
use crate::set::ClosureSet;
use alloc::string::String;
use icu_locid::LanguageIdentifier;
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
    /// use icu_locid::langid;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.to_uppercase_string("hello world", &langid!("und")), "HELLO WORLD");
    /// ```
    ///
    /// ✨ **Enabled with the `"data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "data")]
    pub const fn new() -> Self {
        Self {
            data: DataPayload::from_static_ref(crate::provider::Baked::SINGLETON_PROPS_CASEMAP_V1),
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
        let data = provider.load(Default::default())?.take_payload()?;
        Ok(Self { data })
    }

    /// Returns the full lowercase mapping of the given string as a [`Writeable`].
    /// This function is context and language sensitive. One should pass in the `LanguageIdentifier`
    /// (or the `id` field of the `Locale`) for the current text if it is available. If not, use
    /// the root locale via `langid!("und")` or `Default::default()`.
    ///
    /// See [`Self::to_lowercase_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn to_lowercase<'a>(
        &'a self,
        src: &'a str,
        langid: &LanguageIdentifier,
    ) -> impl Writeable + 'a {
        self.data.get().full_helper_writeable(
            src,
            CaseMapLocale::from_langid(langid),
            MappingKind::Lower,
        )
    }

    /// Returns the full uppercase mapping of the given string as a [`Writeable`].
    /// This function is context and language sensitive. One should pass in the `LanguageIdentifier`
    /// (or the `id` field of the `Locale`) for the current text if it is available. If not, use
    /// the root locale via `langid!("und")` or `Default::default()`.
    ///
    /// See [`Self::to_uppercase_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn to_uppercase<'a>(
        &'a self,
        src: &'a str,
        langid: &LanguageIdentifier,
    ) -> impl Writeable + 'a {
        self.data.get().full_helper_writeable(
            src,
            CaseMapLocale::from_langid(langid),
            MappingKind::Upper,
        )
    }

    /// Case-folds the characters in the given string as a [`Writeable`].
    /// This function is locale-independent and context-insensitive.
    ///
    /// Can be used to test if two strings are case-insensitively equivalent.
    ///
    /// See [`Self::fold_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn fold<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
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
    /// See [`Self::fold_turkic_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn fold_turkic<'a>(&'a self, src: &'a str) -> impl Writeable + 'a {
        self.data
            .get()
            .full_helper_writeable(src, CaseMapLocale::Turkish, MappingKind::Fold)
    }

    /// Returns the full lowercase mapping of the given string as a String.
    /// This function is context and language sensitive. One should pass in the `LanguageIdentifier`
    /// (or the `id` field of the `Locale`) for the current text if it is available. If not, use
    /// the root locale via `langid!("und")` or `Default::default()`.
    ///
    /// See [`Self::to_lowercase()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    /// use icu_locid::langid;
    ///
    /// let cm = CaseMapping::new();
    /// let root = langid!("und");
    ///
    /// assert_eq!(cm.to_lowercase_string("hEllO WorLd", &root), "hello world");
    /// assert_eq!(cm.to_lowercase_string("Γειά σου Κόσμε", &root), "γειά σου κόσμε");
    /// assert_eq!(cm.to_lowercase_string("नमस्ते दुनिया", &root), "नमस्ते दुनिया");
    /// assert_eq!(cm.to_lowercase_string("Привет мир", &root), "привет мир");
    ///
    /// // Some behavior is language-sensitive
    /// assert_eq!(cm.to_lowercase_string("CONSTANTINOPLE", &root), "constantinople");
    /// assert_eq!(cm.to_lowercase_string("CONSTANTINOPLE", &langid!("tr")), "constantınople");
    /// ```
    pub fn to_lowercase_string(&self, src: &str, langid: &LanguageIdentifier) -> String {
        self.data
            .get()
            .full_helper_writeable(src, CaseMapLocale::from_langid(langid), MappingKind::Lower)
            .write_to_string()
            .into_owned()
    }

    /// Returns the full uppercase mapping of the given string as a String.
    /// This function is context and language sensitive. One should pass in the `LanguageIdentifier`
    /// (or the `id` field of the `Locale`) for the current text if it is available. If not, use
    /// the root locale via `langid!("und")` or `Default::default()`.
    ///
    /// See [`Self::to_uppercase()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    /// use icu_locid::langid;
    ///
    /// let cm = CaseMapping::new();
    /// let root = langid!("und");
    ///
    /// assert_eq!(cm.to_uppercase_string("hEllO WorLd", &root), "HELLO WORLD");
    /// assert_eq!(cm.to_uppercase_string("Γειά σου Κόσμε", &root), "ΓΕΙΆ ΣΟΥ ΚΌΣΜΕ");
    /// assert_eq!(cm.to_uppercase_string("नमस्ते दुनिया", &root), "नमस्ते दुनिया");
    /// assert_eq!(cm.to_uppercase_string("Привет мир", &root), "ПРИВЕТ МИР");
    ///
    /// // Some behavior is language-sensitive
    /// assert_eq!(cm.to_uppercase_string("istanbul", &root), "ISTANBUL");
    /// assert_eq!(cm.to_uppercase_string("istanbul", &langid!("tr")), "İSTANBUL"); // Turkish dotted i
    ///
    /// assert_eq!(cm.to_uppercase_string("և Երևանի", &root), "ԵՒ ԵՐԵՒԱՆԻ");
    /// assert_eq!(cm.to_uppercase_string("և Երևանի", &langid!("hy")), "ԵՎ ԵՐԵՎԱՆԻ"); // Eastern Armenian ech-yiwn ligature
    /// ```
    pub fn to_uppercase_string(&self, src: &str, langid: &LanguageIdentifier) -> String {
        self.data
            .get()
            .full_helper_writeable(src, CaseMapLocale::from_langid(langid), MappingKind::Upper)
            .write_to_string()
            .into_owned()
    }

    /// Case-folds the characters in the given string as a String.
    /// This function is locale-independent and context-insensitive.
    ///
    /// Can be used to test if two strings are case-insensitively equivalent.
    ///
    /// See [`Self::fold()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// // Check if two strings are equivalent case insensitively
    /// assert_eq!(cm.fold_string("hEllO WorLd"), cm.fold_string("HELLO worlD"));
    ///
    /// assert_eq!(cm.fold_string("hEllO WorLd"), "hello world");
    /// assert_eq!(cm.fold_string("Γειά σου Κόσμε"), "γειά σου κόσμε");
    /// assert_eq!(cm.fold_string("नमस्ते दुनिया"), "नमस्ते दुनिया");
    /// assert_eq!(cm.fold_string("Привет мир"), "привет мир");
    /// ```
    pub fn fold_string(&self, src: &str) -> String {
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
    /// See [`Self::fold_turkic()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// // Check if two strings are equivalent case insensitively
    /// assert_eq!(cm.fold_turkic_string("İstanbul"), cm.fold_turkic_string("iSTANBUL"));
    ///
    /// assert_eq!(cm.fold_turkic_string("İstanbul not Constantinople"), "istanbul not constantinople");
    /// assert_eq!(cm.fold_turkic_string("Istanbul not Constantınople"), "ıstanbul not constantınople");
    ///
    /// assert_eq!(cm.fold_turkic_string("hEllO WorLd"), "hello world");
    /// assert_eq!(cm.fold_turkic_string("Γειά σου Κόσμε"), "γειά σου κόσμε");
    /// assert_eq!(cm.fold_turkic_string("नमस्ते दुनिया"), "नमस्ते दुनिया");
    /// assert_eq!(cm.fold_turkic_string("Привет мир"), "привет мир");
    /// ```
    pub fn fold_turkic_string(&self, src: &str) -> String {
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

    /// Returns the lowercase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    /// For full mappings, use [`CaseMapping::to_lowercase`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.to_simple_lowercase('C'), 'c');
    /// assert_eq!(cm.to_simple_lowercase('c'), 'c');
    /// assert_eq!(cm.to_simple_lowercase('Ć'), 'ć');
    /// assert_eq!(cm.to_simple_lowercase('Γ'), 'γ');
    /// ```
    pub fn to_simple_lowercase(&self, c: char) -> char {
        self.data.get().simple_lower(c)
    }

    /// Returns the uppercase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    /// For full mappings, use [`CaseMapping::to_uppercase`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.to_simple_uppercase('c'), 'C');
    /// assert_eq!(cm.to_simple_uppercase('C'), 'C');
    /// assert_eq!(cm.to_simple_uppercase('ć'), 'Ć');
    /// assert_eq!(cm.to_simple_uppercase('γ'), 'Γ');
    ///
    /// assert_eq!(cm.to_simple_uppercase('ǳ'), 'Ǳ');
    /// ```
    pub fn to_simple_uppercase(&self, c: char) -> char {
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
    /// assert_eq!(cm.to_simple_titlecase('ǳ'), 'ǲ');
    ///
    /// assert_eq!(cm.to_simple_titlecase('c'), 'C');
    /// assert_eq!(cm.to_simple_titlecase('C'), 'C');
    /// assert_eq!(cm.to_simple_titlecase('ć'), 'Ć');
    /// assert_eq!(cm.to_simple_titlecase('γ'), 'Γ');
    /// ```
    pub fn to_simple_titlecase(&self, c: char) -> char {
        self.data.get().simple_title(c)
    }

    /// Returns the simple case folding of the given char.
    /// For full mappings, use [`CaseMapping::fold`].
    ///
    /// This function can be used to perform caseless matches on
    /// individual characters.
    /// > *Note:* With Unicode 15.0 data, there are three
    /// > pairs of characters for which equivalence under this
    /// > function is inconsistent with equivalence of the
    /// > one-character strings under [`CaseMapping::fold`].
    /// > This is resolved in Unicode 15.1 and later.
    ///
    /// For compatibility applications where simple case folding
    /// of strings is required, this function can be applied to
    /// each character of a string.  Note that the resulting
    /// equivalence relation is different from that obtained
    /// by [`CaseMapping::fold`]:
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
    /// assert_eq!(cm.simple_fold('σ'), cm.simple_fold('ς'));
    /// assert_eq!(cm.simple_fold('Σ'), cm.simple_fold('ς'));
    ///
    /// assert_eq!(cm.simple_fold('c'), 'c');
    /// assert_eq!(cm.simple_fold('Ć'), 'ć');
    /// assert_eq!(cm.simple_fold('Γ'), 'γ');
    /// assert_eq!(cm.simple_fold('ς'), 'σ');
    ///
    /// assert_eq!(cm.simple_fold('ß'), 'ß');
    /// assert_eq!(cm.simple_fold('I'), 'i');
    /// assert_eq!(cm.simple_fold('İ'), 'İ');
    /// assert_eq!(cm.simple_fold('ı'), 'ı');
    /// ```
    pub fn simple_fold(&self, c: char) -> char {
        self.data.get().simple_fold(c, FoldOptions::default())
    }

    /// Returns the simple case folding of the given char, using Turkic (T) mappings for
    /// dotted/dotless i. This function does not fold `i` and `I` to the same character. Instead,
    /// `I` will fold to `ı`, and `İ` will fold to `i`. Otherwise, this is the same as
    /// [`CaseMapping::fold()`].
    ///
    /// You can use the case folding to perform Turkic caseless matches on characters
    /// provided they don't full-casefold to strings. To avoid that situation,
    /// convert to a string and use [`CaseMapping::fold_turkic`].
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemapping::CaseMapping;
    ///
    /// let cm = CaseMapping::new();
    ///
    /// assert_eq!(cm.simple_fold_turkic('I'), 'ı');
    /// assert_eq!(cm.simple_fold_turkic('İ'), 'i');
    /// ```
    pub fn simple_fold_turkic(&self, c: char) -> char {
        self.data
            .get()
            .simple_fold(c, FoldOptions::with_turkic_mappings())
    }
}

#[cfg(test)]
#[cfg(feature = "data")]
mod tests {
    use super::*;
    use icu_locid::langid;

    impl CaseMapping {
        /// Only for testing titlecase special-cases, does NOT
        /// segment input string
        fn to_titlecase_string_test(&self, src: &str, langid: &LanguageIdentifier) -> String {
            self.data
                .get()
                .full_helper_writeable(src, CaseMapLocale::from_langid(langid), MappingKind::Title)
                .write_to_string()
                .into_owned()
        }
    }

    #[test]
    /// Tests for SpecialCasing.txt. Some of the special cases are data-driven, some are code-driven
    fn test_special_cases() {
        let cm = CaseMapping::new();
        let root = langid!("und");

        // Ligatures

        // U+FB00 LATIN SMALL LIGATURE FF
        assert_eq!(cm.to_uppercase_string("ﬀ", &root), "FF");
        // U+FB05 LATIN SMALL LIGATURE LONG S T
        assert_eq!(cm.to_uppercase_string("ﬅ", &root), "ST");

        // No corresponding uppercased character

        // U+0149 LATIN SMALL LETTER N PRECEDED BY APOSTROPHE
        assert_eq!(cm.to_uppercase_string("ŉ", &root), "ʼN");

        // U+1F50 GREEK SMALL LETTER UPSILON WITH PSILI
        assert_eq!(cm.to_uppercase_string("ὐ", &root), "Υ̓");
        // U+1FF6 GREEK SMALL LETTER OMEGA WITH PERISPOMENI
        assert_eq!(cm.to_uppercase_string("ῶ", &root), "Ω͂");

        // YPOGEGRAMMENI / PROSGEGRAMMENI special cases

        // E.g. <alpha><iota_subscript><acute> is uppercased to <ALPHA><acute><IOTA>
        assert_eq!(
            cm.to_uppercase_string("α\u{0313}\u{0345}", &root),
            "Α\u{0313}Ι"
        );

        // U+1F80 GREEK SMALL LETTER ALPHA WITH PSILI AND YPOGEGRAMMENI
        assert_eq!(cm.to_titlecase_string_test("ᾀ", &root), "ᾈ");
        assert_eq!(cm.to_uppercase_string("ᾀ", &root), "ἈΙ");

        // U+1FFC GREEK CAPITAL LETTER OMEGA WITH PROSGEGRAMMENI
        assert_eq!(cm.to_lowercase_string("ῼ", &root), "ῳ");
        assert_eq!(cm.to_titlecase_string_test("ῼ", &root), "ῼ");
        assert_eq!(cm.to_uppercase_string("ῼ", &root), "ΩΙ");

        // U+1F98 GREEK CAPITAL LETTER ETA WITH PSILI AND PROSGEGRAMMENI
        assert_eq!(cm.to_lowercase_string("ᾘ", &root), "ᾐ");
        assert_eq!(cm.to_titlecase_string_test("ᾘ", &root), "ᾘ");
        assert_eq!(cm.to_uppercase_string("ᾘ", &root), "ἨΙ");

        // U+1FB2 GREEK SMALL LETTER ALPHA WITH VARIA AND YPOGEGRAMMENI
        assert_eq!(cm.to_lowercase_string("ᾲ", &root), "ᾲ");
        assert_eq!(cm.to_titlecase_string_test("ᾲ", &root), "Ὰ\u{345}");
        assert_eq!(cm.to_uppercase_string("ᾲ", &root), "ᾺΙ");

        // Final sigma test
        // U+03A3 GREEK CAPITAL LETTER SIGMA in Final_Sigma context
        assert_eq!(cm.to_lowercase_string("ΙΙΙΣ", &root), "ιιις");

        // Turkish / Azeri
        let tr = langid!("tr");
        let az = langid!("az");
        // U+0130 LATIN CAPITAL LETTER I WITH DOT ABOVE
        assert_eq!(cm.to_lowercase_string("İ", &tr), "i");
        assert_eq!(cm.to_lowercase_string("İ", &az), "i");
        assert_eq!(cm.to_titlecase_string_test("İ", &tr), "İ");
        assert_eq!(cm.to_titlecase_string_test("İ", &az), "İ");
        assert_eq!(cm.to_uppercase_string("İ", &tr), "İ");
        assert_eq!(cm.to_uppercase_string("İ", &az), "İ");

        // U+0049 LATIN CAPITAL LETTER I and U+0307 COMBINING DOT ABOVE
        assert_eq!(cm.to_lowercase_string("I\u{0307}", &tr), "i");
        assert_eq!(cm.to_lowercase_string("I\u{0307}", &az), "i");
        assert_eq!(cm.to_titlecase_string_test("I\u{0307}", &tr), "I\u{0307}");
        assert_eq!(cm.to_titlecase_string_test("I\u{0307}", &az), "I\u{0307}");
        assert_eq!(cm.to_uppercase_string("I\u{0307}", &tr), "I\u{0307}");
        assert_eq!(cm.to_uppercase_string("I\u{0307}", &az), "I\u{0307}");

        // U+0049 LATIN CAPITAL LETTER I
        assert_eq!(cm.to_lowercase_string("I", &tr), "ı");
        assert_eq!(cm.to_lowercase_string("I", &az), "ı");
        assert_eq!(cm.to_titlecase_string_test("I", &tr), "I");
        assert_eq!(cm.to_titlecase_string_test("I", &az), "I");
        assert_eq!(cm.to_uppercase_string("I", &tr), "I");
        assert_eq!(cm.to_uppercase_string("I", &az), "I");

        // U+0069 LATIN SMALL LETTER I
        assert_eq!(cm.to_lowercase_string("i", &tr), "i");
        assert_eq!(cm.to_lowercase_string("i", &az), "i");
        assert_eq!(cm.to_titlecase_string_test("i", &tr), "İ");
        assert_eq!(cm.to_titlecase_string_test("i", &az), "İ");
        assert_eq!(cm.to_uppercase_string("i", &tr), "İ");
        assert_eq!(cm.to_uppercase_string("i", &az), "İ");
    }
}
