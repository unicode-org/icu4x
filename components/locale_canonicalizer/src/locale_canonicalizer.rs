// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::provider::*;
use icu_locid::{LanguageIdentifier, Locale};
use icu_provider::prelude::*;
use std::borrow::Cow;

/// CanonicalizationResult is used to track the result of a canonicalization
/// operation that potentially modifies its argument in place.
#[derive(Debug, PartialEq)]
pub enum CanonicalizationResult {
    Modified,
    Unmodified,
}

pub struct LocaleCanonicalizer<'a> {
    likely_subtags: Cow<'a, LikelySubtagsV1>,
}

impl LocaleCanonicalizer<'_> {
    /// A constructor which takes a DataProvider and creates a
    /// LocaleCanonicalizer.
    pub fn new<'d>(
        provider: &(impl DataProvider<'d, LikelySubtagsV1> + ?Sized),
    ) -> Result<LocaleCanonicalizer<'d>, DataError> {
        let payload: Cow<LikelySubtagsV1> = provider
            .load_payload(&DataRequest::from(key::LIKELY_SUBTAGS_V1))?
            .take_payload()?;

        Ok(LocaleCanonicalizer {
            likely_subtags: payload,
        })
    }

    /// The maximize method potentially updates a passed in locale in place
    /// depending up the results of running the 'Add Likely Subtags' algorithm
    /// from https://www.unicode.org/reports/tr35/#Likely_Subtags.
    ///
    /// If the result of running the algorithm would result in a new locale, the
    /// locale argument is updated in place to match the result, and the method
    /// returns `CanonicalizationResult::Modified`. Otherwise, the method
    /// returns `CanonicalizationResult::Unmodified` and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "provider_serde")] {
    /// use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    /// use icu_locid::Locale;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let lc = LocaleCanonicalizer::new(&provider).unwrap();
    ///
    /// let mut locale : Locale = "en-US".parse().unwrap();
    /// assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Modified);
    /// assert_eq!(locale.to_string(), "en-Latn-US");
    ///
    /// let mut locale : Locale = "en-Latn-DE".parse().unwrap();
    /// assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Unmodified);
    /// assert_eq!(locale.to_string(), "en-Latn-DE");
    /// # } // feature = "provider_serde"
    /// ```
    pub fn maximize(&self, locale: &mut Locale) -> CanonicalizationResult {
        let maybe_update_locale = |entry: &LanguageIdentifier,
                                   locale: &mut Locale|
         -> CanonicalizationResult {
            if locale.language.is_empty() || locale.script.is_none() || locale.region.is_none() {
                if locale.language.is_empty() {
                    locale.language = entry.language;
                }
                locale.script = locale.script.or(entry.script);
                locale.region = locale.region.or(entry.region);
                CanonicalizationResult::Modified
            } else {
                CanonicalizationResult::Unmodified
            }
        };

        // languages_scripts_regions
        if locale.script.is_some() && locale.region.is_some() {
            let key = (&locale.language, &locale.script, &locale.region);
            if let Ok(index) = self
                .likely_subtags
                .language_script_region
                .binary_search_by_key(&key, |(l, _)| (&l.language, &l.script, &l.region))
            {
                let entry = &self.likely_subtags.language_script_region[index].1;
                return maybe_update_locale(entry, locale);
            }
        }

        // languages_scripts
        if locale.script.is_some() {
            let key = (&locale.language, &locale.script);
            if let Ok(index) = self
                .likely_subtags
                .language_script
                .binary_search_by_key(&key, |(l, _)| (&l.language, &l.script))
            {
                let entry = &self.likely_subtags.language_script[index].1;
                return maybe_update_locale(entry, locale);
            }
        }

        // languages_regions
        if locale.region.is_some() {
            let key = (&locale.language, &locale.region);
            if let Ok(index) = self
                .likely_subtags
                .language_region
                .binary_search_by_key(&key, |(l, _)| (&l.language, &l.region))
            {
                let entry = &self.likely_subtags.language_region[index].1;
                return maybe_update_locale(entry, locale);
            }
        }

        // languages
        let key = &locale.language;
        if let Ok(index) = self
            .likely_subtags
            .language
            .binary_search_by_key(key, |(l, _)| l.language)
        {
            let entry = &self.likely_subtags.language[index].1;
            return maybe_update_locale(entry, locale);
        }

        // und_scripts
        if locale.script.is_some() {
            let key = &locale.script;
            if let Ok(index) = self
                .likely_subtags
                .language_script
                .binary_search_by_key(&key, |(l, _)| &l.script)
            {
                let entry = &self.likely_subtags.language_script[index].1;
                return maybe_update_locale(entry, locale);
            }
        }
        CanonicalizationResult::Unmodified
    }

    /// This returns a new Locale that is the result of running the
    /// 'Remove Likely Subtags' algorithm from
    /// https://www.unicode.org/reports/tr35/#Likely_Subtags.
    ///
    /// If the result of running the algorithm would result in a new locale, the
    /// locale argument is updated in place to match the result, and the method
    /// returns `CanonicalizationResult::Modified`. Otherwise, the method
    /// returns `CanonicalizationResult::Unmodified` and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "provider_serde")] {
    /// use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    /// use icu_locid::Locale;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let lc = LocaleCanonicalizer::new(&provider).unwrap();
    ///
    /// let mut locale : Locale = "en-Latn-US".parse().unwrap();
    /// assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Modified);
    /// assert_eq!(locale.to_string(), "en");
    ///
    /// let mut locale : Locale = "en".parse().unwrap();
    /// assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Unmodified);
    /// assert_eq!(locale.to_string(), "en");
    /// # } // feature = "provider_serde"
    /// ```
    pub fn minimize(&self, locale: &mut Locale) -> CanonicalizationResult {
        let mut max = locale.clone();
        self.maximize(&mut max);
        max.variants.clear();
        let mut trial = max.clone();

        trial.script = None;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if locale.script.is_some() || locale.script.is_some() {
                locale.script = None;
                locale.region = None;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        trial.script = None;
        trial.region = max.region;
        self.maximize(&mut trial);
        if trial == max {
            if locale.script.is_some() || locale.region != max.region {
                locale.script = None;
                locale.region = max.region;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        trial.script = max.script;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if locale.script != max.script || locale.region.is_some() {
                locale.script = max.script;
                locale.region = None;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        if locale.script != max.script || locale.region != max.region {
            locale.script = max.script;
            locale.region = max.region;
            CanonicalizationResult::Modified
        } else {
            CanonicalizationResult::Unmodified
        }
    }
}
