// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module provides APIs for getting exemplar characters for a locale.
//!
//! Exemplars are characters used by a language, separated into different sets.
//! The sets are: main, auxiliary, punctuation, numbers, and index.
//!
//! The sets define, according to typical usage in the langauge,
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
    /// ```

    pub fn load_exemplars_main();
);
