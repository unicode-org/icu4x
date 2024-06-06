// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Titlecasing-specific try_new_with_mapper_unstable
use crate::provider::CaseMapV1Marker;
use crate::CaseMapper;
use alloc::string::String;
use icu_locale_core::LanguageIdentifier;
use icu_properties::maps::CodePointMapData;
use icu_properties::provider::GeneralCategoryV1Marker;
use icu_properties::{GeneralCategory, GeneralCategoryGroup};
use icu_provider::prelude::*;
use writeable::Writeable;

/// How to handle the rest of the string once the beginning of the
/// string has been titlecased.
///
/// # Examples
///
/// ```rust
/// use icu::casemap::titlecase::{TitlecaseOptions, TrailingCase};
/// use icu::casemap::TitlecaseMapper;
/// use icu::locale::langid;
///
/// let cm = TitlecaseMapper::new();
/// let root = langid!("und");
///
/// let default_options = Default::default();
/// let mut preserve_case: TitlecaseOptions = Default::default();
/// preserve_case.trailing_case = TrailingCase::Unchanged;
///
/// // Exhibits trailing case when set:
/// assert_eq!(
///     cm.titlecase_segment_to_string("spOngeBoB", &root, default_options),
///     "Spongebob"
/// );
/// assert_eq!(
///     cm.titlecase_segment_to_string("spOngeBoB", &root, preserve_case),
///     "SpOngeBoB"
/// );
/// ```
#[non_exhaustive]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub enum TrailingCase {
    /// Preserve the casing of the rest of the string ("spoNgEBoB" -> "SpoNgEBoB")
    Unchanged,
    /// Lowercase the rest of the string ("spoNgEBoB" -> "Spongebob")
    #[default]
    Lower,
}

/// Where to start casing the string.
///
/// [`TitlecaseMapper`] by default performs "leading adjustment", where it searches for the first "relevant" character
/// in the string before initializing the actual titlecasing. For example, it will skip punctuation at the beginning
/// of a string, allowing for strings like `'twas` or `«hello»` to be appropriately titlecased.
///
/// Opinions on exactly what is a "relevant" character may differ. In "adjust to cased" mode the first cased character is considered "relevant",
/// whereas in the "auto" mode, it is the first character that is a letter, number, symbol, or private use character. This means
/// that the strings `49ers` and `«丰(abc)»` will titlecase in "adjust to cased" mode to `49Ers` and `«丰(Abc)»`, whereas in the "auto" mode they stay unchanged.
/// This difference largely matters for things that mix numbers and letters, or mix writing systems, within a single segment.
///
/// # Examples
///
/// ```rust
/// use icu::casemap::titlecase::{LeadingAdjustment, TitlecaseOptions};
/// use icu::casemap::TitlecaseMapper;
/// use icu::locale::langid;
///
/// let cm = TitlecaseMapper::new();
/// let root = langid!("und");
///
/// let default_options = Default::default(); // head adjustment set to Auto
/// let mut no_adjust: TitlecaseOptions = Default::default();
/// let mut adjust_to_cased: TitlecaseOptions = Default::default();
/// no_adjust.leading_adjustment = LeadingAdjustment::None;
/// adjust_to_cased.leading_adjustment = LeadingAdjustment::ToCased;
///
/// // Exhibits leading adjustment when set:
/// assert_eq!(
///     cm.titlecase_segment_to_string("«hello»", &root, default_options),
///     "«Hello»"
/// );
/// assert_eq!(
///     cm.titlecase_segment_to_string("«hello»", &root, adjust_to_cased),
///     "«Hello»"
/// );
/// assert_eq!(
///     cm.titlecase_segment_to_string("«hello»", &root, no_adjust),
///     "«hello»"
/// );
///
/// // Only changed in adjust-to-cased mode:
/// assert_eq!(
///     cm.titlecase_segment_to_string("丰(abc)", &root, default_options),
///     "丰(abc)"
/// );
/// assert_eq!(
///     cm.titlecase_segment_to_string("丰(abc)", &root, adjust_to_cased),
///     "丰(Abc)"
/// );
/// assert_eq!(
///     cm.titlecase_segment_to_string("丰(abc)", &root, no_adjust),
///     "丰(abc)"
/// );
///
/// // Only changed in adjust-to-cased mode:
/// assert_eq!(
///     cm.titlecase_segment_to_string("49ers", &root, default_options),
///     "49ers"
/// );
/// assert_eq!(
///     cm.titlecase_segment_to_string("49ers", &root, adjust_to_cased),
///     "49Ers"
/// );
/// assert_eq!(
///     cm.titlecase_segment_to_string("49ers", &root, no_adjust),
///     "49ers"
/// );
/// ```
#[non_exhaustive]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub enum LeadingAdjustment {
    /// Start titlecasing immediately, even if the character is not one that is relevant for casing
    /// ("'twixt" -> "'twixt", "twixt" -> "Twixt")
    None,
    /// Adjust the string to the first relevant character before beginning to apply casing
    /// ("'twixt" -> "'Twixt"). "Relevant" character is picked by best available algorithm,
    /// by default will adjust to first letter, number, symbol, or private use character,
    /// but if no data is available (e.g. this API is being called via [`CaseMapper::titlecase_segment_with_only_case_data()`]),
    /// then may be equivalent to "adjust to cased".
    ///
    /// This is the default
    #[default]
    Auto,
    /// Adjust the string to the first cased character before beginning to apply casing
    /// ("'twixt" -> "'Twixt")
    ToCased,
}

/// Various options for controlling titlecasing
///
/// See docs of [`TitlecaseMapper`] for examples.
#[non_exhaustive]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct TitlecaseOptions {
    /// How to handle the rest of the string once the head of the
    /// string has been titlecased
    pub trailing_case: TrailingCase,
    /// Whether to start casing at the beginning of the string or at the first
    /// relevant character.
    pub leading_adjustment: LeadingAdjustment,
}

/// A wrapper around [`CaseMapper`] that can compute titlecasing stuff, and is able to load additional data
/// to support the non-legacy "head adjustment" behavior.
///
///
/// By default, [`Self::titlecase_segment()`] and [`Self::titlecase_segment_to_string()`] perform "leading adjustment",
/// where they wait till the first relevant character to begin titlecasing. For example, in the string `'twixt`, the apostrophe
/// is ignored because the word starts at the first "t", which will get titlecased (producing `'Twixt`). Other punctuation will
/// also be ignored, like in the string `«hello»`, which will get titlecased to `«Hello»`.
///
/// This is a separate type from [`CaseMapper`] because it loads the additional data
/// required by [`LeadingAdjustment::Auto`] to perform the best possible leading adjustment.
///
/// If you are planning on only using [`LeadingAdjustment::None`] or [`LeadingAdjustment::ToCased`], consider using [`CaseMapper`] directly; this
/// type will have no additional behavior.
///
/// # Examples
///
/// Basic casemapping behavior:
///
/// ```rust
/// use icu::casemap::TitlecaseMapper;
/// use icu::locale::langid;
///
/// let cm = TitlecaseMapper::new();
/// let root = langid!("und");
///
/// let default_options = Default::default();
///
/// // note that the subsequent words are not titlecased, this function assumes
/// // that the entire string is a single segment and only titlecases at the beginning.
/// assert_eq!(cm.titlecase_segment_to_string("hEllO WorLd", &root, default_options), "Hello world");
/// assert_eq!(cm.titlecase_segment_to_string("Γειά σου Κόσμε", &root, default_options), "Γειά σου κόσμε");
/// assert_eq!(cm.titlecase_segment_to_string("नमस्ते दुनिया", &root, default_options), "नमस्ते दुनिया");
/// assert_eq!(cm.titlecase_segment_to_string("Привет мир", &root, default_options), "Привет мир");
///
/// // Some behavior is language-sensitive
/// assert_eq!(cm.titlecase_segment_to_string("istanbul", &root, default_options), "Istanbul");
/// assert_eq!(cm.titlecase_segment_to_string("istanbul", &langid!("tr"), default_options), "İstanbul"); // Turkish dotted i
///
/// assert_eq!(cm.titlecase_segment_to_string("և Երևանի", &root, default_options), "Եւ երևանի");
/// assert_eq!(cm.titlecase_segment_to_string("և Երևանի", &langid!("hy"), default_options), "Եվ երևանի"); // Eastern Armenian ech-yiwn ligature
///
/// assert_eq!(cm.titlecase_segment_to_string("ijkdijk", &root, default_options), "Ijkdijk");
/// assert_eq!(cm.titlecase_segment_to_string("ijkdijk", &langid!("nl"), default_options), "IJkdijk"); // Dutch IJ digraph
/// ```
#[derive(Clone, Debug)]
pub struct TitlecaseMapper<CM> {
    cm: CM,
    gc: CodePointMapData<GeneralCategory>,
}

#[cfg(feature = "compiled_data")]
impl Default for TitlecaseMapper<CaseMapper> {
    fn default() -> Self {
        Self::new()
    }
}

impl TitlecaseMapper<CaseMapper> {
    /// A constructor which creates a [`TitlecaseMapper`] using compiled data
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new() -> Self {
        Self {
            cm: CaseMapper::new(),
            gc: icu_properties::maps::general_category().static_to_owned(),
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
    pub fn try_new_unstable<P>(provider: &P) -> Result<Self, DataError>
    where
        P: DataProvider<CaseMapV1Marker> + DataProvider<GeneralCategoryV1Marker> + ?Sized,
    {
        let cm = CaseMapper::try_new_unstable(provider)?;
        let gc = icu_properties::maps::load_general_category(provider)?;
        Ok(Self { cm, gc })
    }
}

// We use Borrow, not AsRef, since we want the blanket impl on T
impl<CM: AsRef<CaseMapper>> TitlecaseMapper<CM> {
    icu_provider::gen_any_buffer_data_constructors!(locale: skip, casemapper: CM, error: DataError,
    #[cfg(skip)]
    functions: [
        new_with_mapper,
        try_new_with_mapper_with_any_provider,
        try_new_with_mapper_with_buffer_provider,
        try_new_with_mapper_unstable,
        Self,
    ]);

    /// A constructor which creates a [`TitlecaseMapper`] from an existing [`CaseMapper`]
    /// (either owned or as a reference) and compiled data
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new_with_mapper(casemapper: CM) -> Self {
        Self {
            cm: casemapper,
            gc: icu_properties::maps::general_category().static_to_owned(),
        }
    }

    /// Construct this object to wrap an existing CaseMapper (or a reference to one), loading additional data as needed.
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_with_mapper)]
    pub fn try_new_with_mapper_unstable<P>(provider: &P, casemapper: CM) -> Result<Self, DataError>
    where
        P: DataProvider<CaseMapV1Marker> + DataProvider<GeneralCategoryV1Marker> + ?Sized,
    {
        let gc = icu_properties::maps::load_general_category(provider)?;
        Ok(Self { cm: casemapper, gc })
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
    /// See [`Self::titlecase_segment_to_string()`] for the equivalent convenience function that returns a String,
    /// as well as for an example.
    pub fn titlecase_segment<'a>(
        &'a self,
        src: &'a str,
        langid: &LanguageIdentifier,
        options: TitlecaseOptions,
    ) -> impl Writeable + 'a {
        if options.leading_adjustment == LeadingAdjustment::Auto {
            // letter, number, symbol, or private use code point
            const HEAD_GROUPS: GeneralCategoryGroup = GeneralCategoryGroup::Letter
                .union(GeneralCategoryGroup::Number)
                .union(GeneralCategoryGroup::Symbol)
                .union(GeneralCategoryGroup::PrivateUse);
            self.cm
                .as_ref()
                .titlecase_segment_with_adjustment(src, langid, options, |_data, ch| {
                    HEAD_GROUPS.contains(self.gc.as_borrowed().get(ch))
                })
        } else {
            self.cm
                .as_ref()
                .titlecase_segment_with_adjustment(src, langid, options, |data, ch| {
                    data.is_cased(ch)
                })
        }
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
    /// # Examples
    ///
    /// ```rust
    /// use icu::casemap::TitlecaseMapper;
    /// use icu::locale::langid;
    ///
    /// let cm = TitlecaseMapper::new();
    /// let root = langid!("und");
    ///
    /// let default_options = Default::default();
    ///
    /// // note that the subsequent words are not titlecased, this function assumes
    /// // that the entire string is a single segment and only titlecases at the beginning.
    /// assert_eq!(cm.titlecase_segment_to_string("hEllO WorLd", &root, default_options), "Hello world");
    /// assert_eq!(cm.titlecase_segment_to_string("Γειά σου Κόσμε", &root, default_options), "Γειά σου κόσμε");
    /// assert_eq!(cm.titlecase_segment_to_string("नमस्ते दुनिया", &root, default_options), "नमस्ते दुनिया");
    /// assert_eq!(cm.titlecase_segment_to_string("Привет мир", &root, default_options), "Привет мир");
    ///
    /// // Some behavior is language-sensitive
    /// assert_eq!(cm.titlecase_segment_to_string("istanbul", &root, default_options), "Istanbul");
    /// assert_eq!(cm.titlecase_segment_to_string("istanbul", &langid!("tr"), default_options), "İstanbul"); // Turkish dotted i
    ///
    /// assert_eq!(cm.titlecase_segment_to_string("և Երևանի", &root, default_options), "Եւ երևանի");
    /// assert_eq!(cm.titlecase_segment_to_string("և Երևանի", &langid!("hy"), default_options), "Եվ երևանի"); // Eastern Armenian ech-yiwn ligature
    ///
    /// assert_eq!(cm.titlecase_segment_to_string("ijkdijk", &root, default_options), "Ijkdijk");
    /// assert_eq!(cm.titlecase_segment_to_string("ijkdijk", &langid!("nl"), default_options), "IJkdijk"); // Dutch IJ digraph
    /// ```
    ///
    /// Leading adjustment behaviors:
    ///
    /// ```rust
    /// use icu::casemap::titlecase::{LeadingAdjustment, TitlecaseOptions};
    /// use icu::casemap::TitlecaseMapper;
    /// use icu::locale::langid;
    ///
    /// let cm = TitlecaseMapper::new();
    /// let root = langid!("und");
    ///
    /// let default_options = Default::default();
    /// let mut no_adjust: TitlecaseOptions = Default::default();
    /// no_adjust.leading_adjustment = LeadingAdjustment::None;
    ///
    /// // Exhibits leading adjustment when set:
    /// assert_eq!(
    ///     cm.titlecase_segment_to_string("«hello»", &root, default_options),
    ///     "«Hello»"
    /// );
    /// assert_eq!(
    ///     cm.titlecase_segment_to_string("«hello»", &root, no_adjust),
    ///     "«hello»"
    /// );
    ///
    /// assert_eq!(
    ///     cm.titlecase_segment_to_string("'Twas", &root, default_options),
    ///     "'Twas"
    /// );
    /// assert_eq!(
    ///     cm.titlecase_segment_to_string("'Twas", &root, no_adjust),
    ///     "'twas"
    /// );
    ///
    /// assert_eq!(
    ///     cm.titlecase_segment_to_string("", &root, default_options),
    ///     ""
    /// );
    /// assert_eq!(cm.titlecase_segment_to_string("", &root, no_adjust), "");
    /// ```
    ///
    /// Tail casing behaviors:
    ///
    /// ```rust
    /// use icu::casemap::titlecase::{TitlecaseOptions, TrailingCase};
    /// use icu::casemap::TitlecaseMapper;
    /// use icu::locale::langid;
    ///
    /// let cm = TitlecaseMapper::new();
    /// let root = langid!("und");
    ///
    /// let default_options = Default::default();
    /// let mut preserve_case: TitlecaseOptions = Default::default();
    /// preserve_case.trailing_case = TrailingCase::Unchanged;
    ///
    /// // Exhibits trailing case when set:
    /// assert_eq!(
    ///     cm.titlecase_segment_to_string("spOngeBoB", &root, default_options),
    ///     "Spongebob"
    /// );
    /// assert_eq!(
    ///     cm.titlecase_segment_to_string("spOngeBoB", &root, preserve_case),
    ///     "SpOngeBoB"
    /// );
    /// ```
    pub fn titlecase_segment_to_string(
        &self,
        src: &str,
        langid: &LanguageIdentifier,
        options: TitlecaseOptions,
    ) -> String {
        self.titlecase_segment(src, langid, options)
            .write_to_string()
            .into_owned()
    }
}
