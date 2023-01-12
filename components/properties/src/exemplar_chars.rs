// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module provides APIs for getting exemplar characters for a locale.
//!
//! Exemplars are characters used by a language, separated into different sets.
//! The sets are: main, auxiliary, punctuation, numbers, and index.
//!
//! The sets define, according to typical usage in the language,
//! which characters occur in which contexts with which frequency.
//! For more information, see the documentation in the
//! [Exemplars section in Unicode Technical Standard #35](https://unicode.org/reports/tr35/tr35-general.html#Exemplars)
//! of the LDML specification.
//!
//! # Examples
//!
//! ```
//! use icu::locid::locale;
//! use icu::properties::exemplar_chars;
//!
//! let locale = locale!("en-001").into();
//! let data =
//!     exemplar_chars::load_exemplars_main(&icu_testdata::unstable(), &locale)
//!         .expect("The data should be valid");
//! let exemplars_main = data.as_borrowed();
//!
//! assert!(exemplars_main.contains_char('a'));
//! assert!(exemplars_main.contains_char('z'));
//! assert!(exemplars_main.contains("a"));
//! assert!(!exemplars_main.contains("ä"));
//! assert!(!exemplars_main.contains("ng"));
//! ```
//!
//! Note that some combinations of locales and set types of exemplar characters, source data
//! may be missing. In such cases, the empty set will be returned.
//!
//! ```
//! use icu::locid::locale;
//! use icu::properties::exemplar_chars;
//!
//! let locale = locale!("und").into();
//! let data =
//!     exemplar_chars::load_exemplars_main(&icu_testdata::unstable(), &locale)
//!         .expect("The data should be valid");
//! let exemplars_main_und = data.as_borrowed();
//!
//! assert!(!exemplars_main_und.contains_char('a'));
//! assert!(!exemplars_main_und.contains_char('z'));
//! assert!(!exemplars_main_und.contains("a"));
//! assert!(!exemplars_main_und.contains("ä"));
//! assert!(!exemplars_main_und.contains("ng"));
//!
//! let cpilsl = data.as_code_point_inversion_list_string_list();
//! println!("underlying data = {:?}", cpilsl);
//! let num_chars = cpilsl.expect("The data should be valid").size();
//! assert_eq!(0, num_chars);
//! ```

use crate::provider::*;
use crate::sets::UnicodeSetData;
use crate::PropertiesError;
use icu_provider::prelude::*;

macro_rules! make_exemplar_chars_unicode_set_property {
    (
        // currently unused
        marker: $marker_name:ident;
        keyed_data_marker: $keyed_data_marker:ty;
        func:
        $(#[$attr:meta])*
        $vis:vis fn $funcname:ident();
    ) => {
        $(#[$attr])*
        $vis fn $funcname(
            provider: &(impl DataProvider<$keyed_data_marker> + ?Sized),
            locale: &DataLocale,
        ) -> Result<UnicodeSetData, PropertiesError> {
            Ok(provider.load(
                DataRequest {
                    locale,
                    metadata: Default::default(),
                })
                .and_then(DataResponse::take_payload)
                .map(UnicodeSetData::from_data)?
            )
        }
    }
}

make_exemplar_chars_unicode_set_property!(
    marker: ExemplarCharactersMain;
    keyed_data_marker: ExemplarCharactersMainV1Marker;
    func:
    /// Get the "main" set of exemplar characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::properties::exemplar_chars;
    ///
    /// let locale = locale!("en-001").into();
    /// let data =
    ///     exemplar_chars::load_exemplars_main(&icu_testdata::unstable(), &locale)
    ///         .expect("The data should be valid");
    /// let exemplars_main = data.as_borrowed();
    ///
    /// assert!(exemplars_main.contains_char('a'));
    /// assert!(exemplars_main.contains_char('z'));
    /// assert!(exemplars_main.contains("a"));
    /// assert!(!exemplars_main.contains("ä"));
    /// assert!(!exemplars_main.contains("ng"));
    /// assert!(!exemplars_main.contains("A"));
    /// ```

    pub fn load_exemplars_main();
);

make_exemplar_chars_unicode_set_property!(
    marker: ExemplarCharactersAuxiliary;
    keyed_data_marker: ExemplarCharactersAuxiliaryV1Marker;
    func:
    /// Get the "auxiliary" set of exemplar characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::properties::exemplar_chars;
    ///
    /// let locale = locale!("en-001").into();
    /// let data =
    ///     exemplar_chars::load_exemplars_auxiliary(&icu_testdata::unstable(), &locale)
    ///         .expect("The data should be valid");
    /// let exemplars_auxiliary = data.as_borrowed();
    ///
    /// assert!(!exemplars_auxiliary.contains_char('a'));
    /// assert!(!exemplars_auxiliary.contains_char('z'));
    /// assert!(!exemplars_auxiliary.contains("a"));
    /// assert!(exemplars_auxiliary.contains("ä"));
    /// assert!(!exemplars_auxiliary.contains("ng"));
    /// assert!(!exemplars_auxiliary.contains("A"));
    /// ```

    pub fn load_exemplars_auxiliary();
);

make_exemplar_chars_unicode_set_property!(
    marker: ExemplarCharactersPunctuation;
    keyed_data_marker: ExemplarCharactersPunctuationV1Marker;
    func:
    /// Get the "punctuation" set of exemplar characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::properties::exemplar_chars;
    ///
    /// let locale = locale!("en-001").into();
    /// let data =
    ///     exemplar_chars::load_exemplars_punctuation(&icu_testdata::unstable(), &locale)
    ///         .expect("The data should be valid");
    /// let exemplars_punctuation = data.as_borrowed();
    ///
    /// assert!(!exemplars_punctuation.contains_char('0'));
    /// assert!(!exemplars_punctuation.contains_char('9'));
    /// assert!(!exemplars_punctuation.contains_char('%'));
    /// assert!(exemplars_punctuation.contains_char(','));
    /// assert!(exemplars_punctuation.contains_char('.'));
    /// assert!(exemplars_punctuation.contains_char('!'));
    /// assert!(exemplars_punctuation.contains_char('?'));
    /// ```

    pub fn load_exemplars_punctuation();
);

make_exemplar_chars_unicode_set_property!(
    marker: ExemplarCharactersNumbers;
    keyed_data_marker: ExemplarCharactersNumbersV1Marker;
    func:
    /// Get the "numbers" set of exemplar characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::properties::exemplar_chars;
    ///
    /// let locale = locale!("en-001").into();
    /// let data =
    ///     exemplar_chars::load_exemplars_numbers(&icu_testdata::unstable(), &locale)
    ///         .expect("The data should be valid");
    /// let exemplars_numbers = data.as_borrowed();
    ///
    /// assert!(exemplars_numbers.contains_char('0'));
    /// assert!(exemplars_numbers.contains_char('9'));
    /// assert!(exemplars_numbers.contains_char('%'));
    /// assert!(exemplars_numbers.contains_char(','));
    /// assert!(exemplars_numbers.contains_char('.'));
    /// assert!(!exemplars_numbers.contains_char('!'));
    /// assert!(!exemplars_numbers.contains_char('?'));
    /// ```

    pub fn load_exemplars_numbers();
);

make_exemplar_chars_unicode_set_property!(
    marker: ExemplarCharactersIndex;
    keyed_data_marker: ExemplarCharactersIndexV1Marker;
    func:
    /// Get the "index" set of exemplar characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::properties::exemplar_chars;
    ///
    /// let locale = locale!("en-001").into();
    /// let data =
    ///     exemplar_chars::load_exemplars_index(&icu_testdata::unstable(), &locale)
    ///         .expect("The data should be valid");
    /// let exemplars_index = data.as_borrowed();
    ///
    /// assert!(!exemplars_index.contains_char('a'));
    /// assert!(!exemplars_index.contains_char('z'));
    /// assert!(!exemplars_index.contains("a"));
    /// assert!(!exemplars_index.contains("ä"));
    /// assert!(!exemplars_index.contains("ng"));
    /// assert!(exemplars_index.contains("A"));
    /// ```

    pub fn load_exemplars_index();
);
