// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::internals::{CaseMapLocale, FoldOptions};
use crate::provider::data::MappingKind;
use crate::provider::CaseMapV1Marker;
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
pub struct CaseMapper {
    data: DataPayload<CaseMapV1Marker>,
}

#[cfg(feature = "compiled_data")]
impl Default for CaseMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl CaseMapper {
    /// A constructor which creates a [`CaseMapper`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    /// use icu_locid::langid;
    ///
    /// let cm = CaseMapper::new();
    ///
    /// assert_eq!(cm.uppercase_to_string("hello world", &langid!("und")), "HELLO WORLD");
    /// ```
    ///
    /// ✨ **Enabled with the `"compiled_data"` feature.**
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
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
    pub fn try_new_unstable<P>(provider: &P) -> Result<CaseMapper, DataError>
    where
        P: DataProvider<CaseMapV1Marker> + ?Sized,
    {
        let data = provider.load(Default::default())?.take_payload()?;
        Ok(Self { data })
    }

    /// Returns the full lowercase mapping of the given string as a [`Writeable`].
    /// This function is context and language sensitive. Callers should pass the text's language
    /// as a `LanguageIdentifier` (usually the `id` field of the `Locale`) if available, or
    /// `Default::default()` for the root locale.
    ///
    /// See [`Self::lowercase_to_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn lowercase<'a>(
        &'a self,
        src: &'a str,
        langid: &LanguageIdentifier,
    ) -> impl Writeable + 'a {
        self.data.get().full_helper_writeable::<false>(
            src,
            CaseMapLocale::from_langid(langid),
            MappingKind::Lower,
        )
    }

    /// Returns the full uppercase mapping of the given string as a [`Writeable`].
    /// This function is context and language sensitive. Callers should pass the text's language
    /// as a `LanguageIdentifier` (usually the `id` field of the `Locale`) if available, or
    /// `Default::default()` for the root locale.
    ///
    /// See [`Self::uppercase_to_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn uppercase<'a>(
        &'a self,
        src: &'a str,
        langid: &LanguageIdentifier,
    ) -> impl Writeable + 'a {
        self.data.get().full_helper_writeable::<false>(
            src,
            CaseMapLocale::from_langid(langid),
            MappingKind::Upper,
        )
    }

    /// Returns the full titlecase mapping of the given string as a [`Writeable`], treating
    /// the string as a single segment (and thus only titlecasing the beginning of it).
    ///
    /// This should typically be used as a lower-level helper to construct the titlecasing operation desired
    /// by the application, for example one can titlecase on a per-word basis by mixing this with
    /// a `WordSegmenter`.
    ///
    /// This function is context and language sensitive. Callers should pass the text's language
    /// as a `LanguageIdentifier` (usually the `id` field of the `Locale`) if available, or
    /// `Default::default()` for the root locale.
    ///
    /// See [`Self::titlecase_to_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn titlecase_segment<'a>(
        &'a self,
        src: &'a str,
        langid: &LanguageIdentifier,
    ) -> impl Writeable + 'a {
        self.data.get().full_helper_writeable::<true>(
            src,
            CaseMapLocale::from_langid(langid),
            MappingKind::Title,
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
            .full_helper_writeable::<false>(src, CaseMapLocale::Root, MappingKind::Fold)
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
        self.data.get().full_helper_writeable::<false>(
            src,
            CaseMapLocale::Turkish,
            MappingKind::Fold,
        )
    }

    /// Returns the full lowercase mapping of the given string as a String.
    ///
    /// This function is context and language sensitive. Callers should pass the text's language
    /// as a `LanguageIdentifier` (usually the `id` field of the `Locale`) if available, or
    /// `Default::default()` for the root locale.
    ///
    /// See [`Self::lowercase()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    /// use icu_locid::langid;
    ///
    /// let cm = CaseMapper::new();
    /// let root = langid!("und");
    ///
    /// assert_eq!(cm.lowercase_to_string("hEllO WorLd", &root), "hello world");
    /// assert_eq!(cm.lowercase_to_string("Γειά σου Κόσμε", &root), "γειά σου κόσμε");
    /// assert_eq!(cm.lowercase_to_string("नमस्ते दुनिया", &root), "नमस्ते दुनिया");
    /// assert_eq!(cm.lowercase_to_string("Привет мир", &root), "привет мир");
    ///
    /// // Some behavior is language-sensitive
    /// assert_eq!(cm.lowercase_to_string("CONSTANTINOPLE", &root), "constantinople");
    /// assert_eq!(cm.lowercase_to_string("CONSTANTINOPLE", &langid!("tr")), "constantınople");
    /// ```
    pub fn lowercase_to_string(&self, src: &str, langid: &LanguageIdentifier) -> String {
        self.data
            .get()
            .full_helper_writeable::<false>(
                src,
                CaseMapLocale::from_langid(langid),
                MappingKind::Lower,
            )
            .write_to_string()
            .into_owned()
    }

    /// Returns the full uppercase mapping of the given string as a String.
    ///
    /// This function is context and language sensitive. Callers should pass the text's language
    /// as a `LanguageIdentifier` (usually the `id` field of the `Locale`) if available, or
    /// `Default::default()` for the root locale.
    ///
    /// See [`Self::uppercase()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    /// use icu_locid::langid;
    ///
    /// let cm = CaseMapper::new();
    /// let root = langid!("und");
    ///
    /// assert_eq!(cm.uppercase_to_string("hEllO WorLd", &root), "HELLO WORLD");
    /// assert_eq!(cm.uppercase_to_string("Γειά σου Κόσμε", &root), "ΓΕΙΆ ΣΟΥ ΚΌΣΜΕ");
    /// assert_eq!(cm.uppercase_to_string("नमस्ते दुनिया", &root), "नमस्ते दुनिया");
    /// assert_eq!(cm.uppercase_to_string("Привет мир", &root), "ПРИВЕТ МИР");
    ///
    /// // Some behavior is language-sensitive
    /// assert_eq!(cm.uppercase_to_string("istanbul", &root), "ISTANBUL");
    /// assert_eq!(cm.uppercase_to_string("istanbul", &langid!("tr")), "İSTANBUL"); // Turkish dotted i
    ///
    /// assert_eq!(cm.uppercase_to_string("և Երևանի", &root), "ԵՒ ԵՐԵՒԱՆԻ");
    /// assert_eq!(cm.uppercase_to_string("և Երևանի", &langid!("hy")), "ԵՎ ԵՐԵՎԱՆԻ"); // Eastern Armenian ech-yiwn ligature
    /// ```
    pub fn uppercase_to_string(&self, src: &str, langid: &LanguageIdentifier) -> String {
        self.data
            .get()
            .full_helper_writeable::<false>(
                src,
                CaseMapLocale::from_langid(langid),
                MappingKind::Upper,
            )
            .write_to_string()
            .into_owned()
    }

    /// Returns the full titlecase mapping of the given string as a String, treating
    /// the string as a single segment (and thus only titlecasing the beginning of it).
    ///
    /// This should typically be used as a lower-level helper to construct the titlecasing operation desired
    /// by the application, for example one can titlecase on a per-word basis by mixing this with
    /// a `WordSegmenter`.
    ///
    /// This function is context and language sensitive. Callers should pass the text's language
    /// as a `LanguageIdentifier` (usually the `id` field of the `Locale`) if available, or
    /// `Default::default()` for the root locale.
    ///
    /// See [`Self::titlecase_segment()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    /// use icu_locid::langid;
    ///
    /// let cm = CaseMapper::new();
    /// let root = langid!("und");
    ///
    /// // note that the subsequent words are not titlecased, this function assumes
    /// // that the entire string is a single segment and only titlecases at the beginning.
    /// assert_eq!(cm.titlecase_segment_to_string("hEllO WorLd", &root), "Hello world");
    /// assert_eq!(cm.titlecase_segment_to_string("Γειά σου Κόσμε", &root), "Γειά σου κόσμε");
    /// assert_eq!(cm.titlecase_segment_to_string("नमस्ते दुनिया", &root), "नमस्ते दुनिया");
    /// assert_eq!(cm.titlecase_segment_to_string("Привет мир", &root), "Привет мир");
    ///
    /// // Some behavior is language-sensitive
    /// assert_eq!(cm.titlecase_segment_to_string("istanbul", &root), "Istanbul");
    /// assert_eq!(cm.titlecase_segment_to_string("istanbul", &langid!("tr")), "İstanbul"); // Turkish dotted i
    ///
    /// assert_eq!(cm.titlecase_segment_to_string("և Երևանի", &root), "Եւ երևանի");
    /// assert_eq!(cm.titlecase_segment_to_string("և Երևանի", &langid!("hy")), "Եվ երևանի"); // Eastern Armenian ech-yiwn ligature
    ///
    /// assert_eq!(cm.titlecase_segment_to_string("ijkdijk", &root), "Ijkdijk");
    /// assert_eq!(cm.titlecase_segment_to_string("ijkdijk", &langid!("nl")), "IJkdijk"); // Dutch IJ digraph
    /// ```
    pub fn titlecase_segment_to_string(&self, src: &str, langid: &LanguageIdentifier) -> String {
        self.data
            .get()
            .full_helper_writeable::<true>(
                src,
                CaseMapLocale::from_langid(langid),
                MappingKind::Title,
            )
            .write_to_string()
            .into_owned()
    }

    /// Case-folds the characters in the given string as a String.
    /// This function is locale-independent and context-insensitive.
    ///
    /// Can be used to test if two strings are case-insensitively equivalent.
    ///
    /// See [`Self::fold()`] for the equivalent lower-level function that returns a [`Writeable`]
    ///s s
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    ///
    /// let cm = CaseMapper::new();
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
            .full_helper_writeable::<false>(src, CaseMapLocale::Root, MappingKind::Fold)
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
    /// use icu_casemap::CaseMapper;
    ///
    /// let cm = CaseMapper::new();
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
            .full_helper_writeable::<false>(src, CaseMapLocale::Turkish, MappingKind::Fold)
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
    /// use icu_casemap::CaseMapper;
    /// use icu_collections::codepointinvlist::CodePointInversionListBuilder;
    ///
    /// let cm = CaseMapper::new();
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
    /// use icu_casemap::CaseMapper;
    /// use icu_collections::codepointinvlist::CodePointInversionListBuilder;
    ///
    /// let cm = CaseMapper::new();
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
    /// For full mappings, use [`CaseMapper::lowercase`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    ///
    /// let cm = CaseMapper::new();
    ///
    /// assert_eq!(cm.simple_lowercase('C'), 'c');
    /// assert_eq!(cm.simple_lowercase('c'), 'c');
    /// assert_eq!(cm.simple_lowercase('Ć'), 'ć');
    /// assert_eq!(cm.simple_lowercase('Γ'), 'γ');
    /// ```
    pub fn simple_lowercase(&self, c: char) -> char {
        self.data.get().simple_lower(c)
    }

    /// Returns the uppercase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    /// For full mappings, use [`CaseMapper::uppercase`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    ///
    /// let cm = CaseMapper::new();
    ///
    /// assert_eq!(cm.simple_uppercase('c'), 'C');
    /// assert_eq!(cm.simple_uppercase('C'), 'C');
    /// assert_eq!(cm.simple_uppercase('ć'), 'Ć');
    /// assert_eq!(cm.simple_uppercase('γ'), 'Γ');
    ///
    /// assert_eq!(cm.simple_uppercase('ǳ'), 'Ǳ');
    /// ```
    pub fn simple_uppercase(&self, c: char) -> char {
        self.data.get().simple_upper(c)
    }

    /// Returns the titlecase mapping of the given `char`.
    /// This function only implements simple and common mappings. Full mappings,
    /// which can map one `char` to a string, are not included.
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    ///
    /// let cm = CaseMapper::new();
    ///
    /// assert_eq!(cm.simple_titlecase('ǳ'), 'ǲ');
    ///
    /// assert_eq!(cm.simple_titlecase('c'), 'C');
    /// assert_eq!(cm.simple_titlecase('C'), 'C');
    /// assert_eq!(cm.simple_titlecase('ć'), 'Ć');
    /// assert_eq!(cm.simple_titlecase('γ'), 'Γ');
    /// ```
    pub fn simple_titlecase(&self, c: char) -> char {
        self.data.get().simple_title(c)
    }

    /// Returns the simple case folding of the given char.
    /// For full mappings, use [`CaseMapper::fold`].
    ///
    /// This function can be used to perform caseless matches on
    /// individual characters.
    /// > *Note:* With Unicode 15.0 data, there are three
    /// > pairs of characters for which equivalence under this
    /// > function is inconsistent with equivalence of the
    /// > one-character strings under [`CaseMapper::fold`].
    /// > This is resolved in Unicode 15.1 and later.
    ///
    /// For compatibility applications where simple case folding
    /// of strings is required, this function can be applied to
    /// each character of a string.  Note that the resulting
    /// equivalence relation is different from that obtained
    /// by [`CaseMapper::fold`]:
    /// The strings "Straße" and "STRASSE" are distinct
    /// under simple case folding, but are equivalent under
    /// default (full) case folding.
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    ///
    /// let cm = CaseMapper::new();
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
    /// [`CaseMapper::fold()`].
    ///
    /// You can use the case folding to perform Turkic caseless matches on characters
    /// provided they don't full-casefold to strings. To avoid that situation,
    /// convert to a string and use [`CaseMapper::fold_turkic`].
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu_casemap::CaseMapper;
    ///
    /// let cm = CaseMapper::new();
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
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use icu_locid::langid;

    #[test]
    /// Tests for SpecialCasing.txt. Some of the special cases are data-driven, some are code-driven
    fn test_special_cases() {
        let cm = CaseMapper::new();
        let root = langid!("und");

        // Ligatures

        // U+FB00 LATIN SMALL LIGATURE FF
        assert_eq!(cm.uppercase_to_string("ﬀ", &root), "FF");
        // U+FB05 LATIN SMALL LIGATURE LONG S T
        assert_eq!(cm.uppercase_to_string("ﬅ", &root), "ST");

        // No corresponding uppercased character

        // U+0149 LATIN SMALL LETTER N PRECEDED BY APOSTROPHE
        assert_eq!(cm.uppercase_to_string("ŉ", &root), "ʼN");

        // U+1F50 GREEK SMALL LETTER UPSILON WITH PSILI
        assert_eq!(cm.uppercase_to_string("ὐ", &root), "Υ̓");
        // U+1FF6 GREEK SMALL LETTER OMEGA WITH PERISPOMENI
        assert_eq!(cm.uppercase_to_string("ῶ", &root), "Ω͂");

        // YPOGEGRAMMENI / PROSGEGRAMMENI special cases

        // E.g. <alpha><iota_subscript><acute> is uppercased to <ALPHA><acute><IOTA>
        assert_eq!(
            cm.uppercase_to_string("α\u{0313}\u{0345}", &root),
            "Α\u{0313}Ι"
        );
        // but the YPOGEGRAMMENI should not titlecase
        assert_eq!(
            cm.titlecase_segment_to_string("α\u{0313}\u{0345}", &root),
            "Α\u{0313}\u{0345}"
        );

        // U+1F80 GREEK SMALL LETTER ALPHA WITH PSILI AND YPOGEGRAMMENI
        assert_eq!(cm.titlecase_segment_to_string("ᾀ", &root), "ᾈ");
        assert_eq!(cm.uppercase_to_string("ᾀ", &root), "ἈΙ");

        // U+1FFC GREEK CAPITAL LETTER OMEGA WITH PROSGEGRAMMENI
        assert_eq!(cm.lowercase_to_string("ῼ", &root), "ῳ");
        assert_eq!(cm.titlecase_segment_to_string("ῼ", &root), "ῼ");
        assert_eq!(cm.uppercase_to_string("ῼ", &root), "ΩΙ");

        // U+1F98 GREEK CAPITAL LETTER ETA WITH PSILI AND PROSGEGRAMMENI
        assert_eq!(cm.lowercase_to_string("ᾘ", &root), "ᾐ");
        assert_eq!(cm.titlecase_segment_to_string("ᾘ", &root), "ᾘ");
        assert_eq!(cm.uppercase_to_string("ᾘ", &root), "ἨΙ");

        // U+1FB2 GREEK SMALL LETTER ALPHA WITH VARIA AND YPOGEGRAMMENI
        assert_eq!(cm.lowercase_to_string("ᾲ", &root), "ᾲ");
        assert_eq!(cm.titlecase_segment_to_string("ᾲ", &root), "Ὰ\u{345}");
        assert_eq!(cm.uppercase_to_string("ᾲ", &root), "ᾺΙ");

        // Final sigma test
        // U+03A3 GREEK CAPITAL LETTER SIGMA in Final_Sigma context
        assert_eq!(cm.lowercase_to_string("ΙΙΙΣ", &root), "ιιις");

        // Turkish / Azeri
        let tr = langid!("tr");
        let az = langid!("az");
        // U+0130 LATIN CAPITAL LETTER I WITH DOT ABOVE
        assert_eq!(cm.lowercase_to_string("İ", &tr), "i");
        assert_eq!(cm.lowercase_to_string("İ", &az), "i");
        assert_eq!(cm.titlecase_segment_to_string("İ", &tr), "İ");
        assert_eq!(cm.titlecase_segment_to_string("İ", &az), "İ");
        assert_eq!(cm.uppercase_to_string("İ", &tr), "İ");
        assert_eq!(cm.uppercase_to_string("İ", &az), "İ");

        // U+0049 LATIN CAPITAL LETTER I and U+0307 COMBINING DOT ABOVE
        assert_eq!(cm.lowercase_to_string("I\u{0307}", &tr), "i");
        assert_eq!(cm.lowercase_to_string("I\u{0307}", &az), "i");
        assert_eq!(
            cm.titlecase_segment_to_string("I\u{0307}", &tr),
            "I\u{0307}"
        );
        assert_eq!(
            cm.titlecase_segment_to_string("I\u{0307}", &az),
            "I\u{0307}"
        );
        assert_eq!(cm.uppercase_to_string("I\u{0307}", &tr), "I\u{0307}");
        assert_eq!(cm.uppercase_to_string("I\u{0307}", &az), "I\u{0307}");

        // U+0049 LATIN CAPITAL LETTER I
        assert_eq!(cm.lowercase_to_string("I", &tr), "ı");
        assert_eq!(cm.lowercase_to_string("I", &az), "ı");
        assert_eq!(cm.titlecase_segment_to_string("I", &tr), "I");
        assert_eq!(cm.titlecase_segment_to_string("I", &az), "I");
        assert_eq!(cm.uppercase_to_string("I", &tr), "I");
        assert_eq!(cm.uppercase_to_string("I", &az), "I");

        // U+0069 LATIN SMALL LETTER I
        assert_eq!(cm.lowercase_to_string("i", &tr), "i");
        assert_eq!(cm.lowercase_to_string("i", &az), "i");
        assert_eq!(cm.titlecase_segment_to_string("i", &tr), "İ");
        assert_eq!(cm.titlecase_segment_to_string("i", &az), "İ");
        assert_eq!(cm.uppercase_to_string("i", &tr), "İ");
        assert_eq!(cm.uppercase_to_string("i", &az), "İ");
    }

    #[test]
    fn test_greek_upper() {
        let cm = CaseMapper::new();
        let modern_greek = &langid!("el");

        // https://unicode-org.atlassian.net/browse/ICU-5456
        assert_eq!(
            cm.uppercase_to_string("άδικος, κείμενο, ίριδα", modern_greek),
            "ΑΔΙΚΟΣ, ΚΕΙΜΕΝΟ, ΙΡΙΔΑ"
        );
        // https://bugzilla.mozilla.org/show_bug.cgi?id=307039
        // https://bug307039.bmoattachments.org/attachment.cgi?id=194893
        assert_eq!(cm.uppercase_to_string("Πατάτα", modern_greek), "ΠΑΤΑΤΑ");
        assert_eq!(
            cm.uppercase_to_string("Αέρας, Μυστήριο, Ωραίο", modern_greek),
            "ΑΕΡΑΣ, ΜΥΣΤΗΡΙΟ, ΩΡΑΙΟ"
        );
        assert_eq!(
            cm.uppercase_to_string("Μαΐου, Πόρος, Ρύθμιση", modern_greek),
            "ΜΑΪΟΥ, ΠΟΡΟΣ, ΡΥΘΜΙΣΗ"
        );
        assert_eq!(
            cm.uppercase_to_string("ΰ, Τηρώ, Μάιος", modern_greek),
            "Ϋ, ΤΗΡΩ, ΜΑΪΟΣ"
        );
        assert_eq!(cm.uppercase_to_string("άυλος", modern_greek), "ΑΫΛΟΣ");
        assert_eq!(cm.uppercase_to_string("ΑΫΛΟΣ", modern_greek), "ΑΫΛΟΣ");
        assert_eq!(
            cm.uppercase_to_string("Άκλιτα ρήματα ή άκλιτες μετοχές", modern_greek),
            "ΑΚΛΙΤΑ ΡΗΜΑΤΑ Ή ΑΚΛΙΤΕΣ ΜΕΤΟΧΕΣ"
        );
        // http://www.unicode.org/udhr/d/udhr_ell_monotonic.html
        assert_eq!(
            cm.uppercase_to_string("Επειδή η αναγνώριση της αξιοπρέπειας", modern_greek),
            "ΕΠΕΙΔΗ Η ΑΝΑΓΝΩΡΙΣΗ ΤΗΣ ΑΞΙΟΠΡΕΠΕΙΑΣ"
        );
        assert_eq!(
            cm.uppercase_to_string("νομικού ή διεθνούς", modern_greek),
            "ΝΟΜΙΚΟΥ Ή ΔΙΕΘΝΟΥΣ"
        );
        // http://unicode.org/udhr/d/udhr_ell_polytonic.html
        assert_eq!(
            cm.uppercase_to_string("Ἐπειδὴ ἡ ἀναγνώριση", modern_greek),
            "ΕΠΕΙΔΗ Η ΑΝΑΓΝΩΡΙΣΗ"
        );
        assert_eq!(
            cm.uppercase_to_string("νομικοῦ ἢ διεθνοῦς", modern_greek),
            "ΝΟΜΙΚΟΥ Ή ΔΙΕΘΝΟΥΣ"
        );
        // From Google bug report
        assert_eq!(
            cm.uppercase_to_string("Νέο, Δημιουργία", modern_greek),
            "ΝΕΟ, ΔΗΜΙΟΥΡΓΙΑ"
        );
        // http://crbug.com/234797
        assert_eq!(
            cm.uppercase_to_string("Ελάτε να φάτε τα καλύτερα παϊδάκια!", modern_greek),
            "ΕΛΑΤΕ ΝΑ ΦΑΤΕ ΤΑ ΚΑΛΥΤΕΡΑ ΠΑΪΔΑΚΙΑ!"
        );
        assert_eq!(
            cm.uppercase_to_string("Μαΐου, τρόλεϊ", modern_greek),
            "ΜΑΪΟΥ, ΤΡΟΛΕΪ"
        );
        assert_eq!(
            cm.uppercase_to_string("Το ένα ή το άλλο.", modern_greek),
            "ΤΟ ΕΝΑ Ή ΤΟ ΑΛΛΟ."
        );
        // http://multilingualtypesetting.co.uk/blog/greek-typesetting-tips/
        assert_eq!(cm.uppercase_to_string("ρωμέικα", modern_greek), "ΡΩΜΕΪΚΑ");
        assert_eq!(cm.uppercase_to_string("ή.", modern_greek), "Ή.");

        assert_eq!(
            cm.uppercase_to_string("ᾠδή, -ήν, -ῆς, -ῇ", modern_greek),
            "ΩΙΔΗ, -ΗΝ, -ΗΣ, -ΗΙ"
        );
        assert_eq!(
            cm.uppercase_to_string("ᾠδή, -ήν, -ῆς, -ῇ", modern_greek),
            "ΩΙΔΗ, -ΗΝ, -ΗΣ, -ΗΙ"
        );
        assert_eq!(cm.uppercase_to_string("ᾍδης", modern_greek), "ΑΙΔΗΣ");
    }
}
