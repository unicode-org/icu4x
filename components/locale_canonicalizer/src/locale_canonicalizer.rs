// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::provider::*;
use icu_locid::LanguageIdentifier;
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
    pub fn maximize<T: AsMut<LanguageIdentifier>>(&self, mut langid: T) -> CanonicalizationResult {
        let update_langid = |entry: &LanguageIdentifier,
                             langid: &mut LanguageIdentifier|
         -> CanonicalizationResult {
            if langid.language.is_empty() {
                langid.language = entry.language;
            }
            langid.script = langid.script.or(entry.script);
            langid.region = langid.region.or(entry.region);
            CanonicalizationResult::Modified
        };

        let langid = langid.as_mut();

        if !langid.language.is_empty() && langid.script.is_some() && langid.region.is_some() {
            return CanonicalizationResult::Unmodified;
        }

        if !langid.language.is_empty() {
            if langid.script.is_some() {
                let key = (&langid.language, &langid.script);
                if let Ok(index) = self
                    .likely_subtags
                    .language_script
                    .binary_search_by_key(&key, |(l, _)| (&l.language, &l.script))
                {
                    let entry = &self.likely_subtags.language_script[index].1;
                    return update_langid(entry, langid);
                }
            }

            if langid.region.is_some() {
                let key = (&langid.language, &langid.region);
                if let Ok(index) = self
                    .likely_subtags
                    .language_region
                    .binary_search_by_key(&key, |(l, _)| (&l.language, &l.region))
                {
                    let entry = &self.likely_subtags.language_region[index].1;
                    return update_langid(entry, langid);
                }
            }

            let key = &langid.language;
            if let Ok(index) = self
                .likely_subtags
                .language
                .binary_search_by_key(key, |(l, _)| l.language)
            {
                let entry = &self.likely_subtags.language[index].1;
                return update_langid(entry, langid);
            }
        } else if langid.script.is_some() {
            if langid.region.is_some() {
                let key = (&langid.script, &langid.region);
                if let Ok(index) = self
                    .likely_subtags
                    .script_region
                    .binary_search_by_key(&key, |(l, _)| (&l.script, &l.region))
                {
                    let entry = &self.likely_subtags.script_region[index].1;
                    return update_langid(entry, langid);
                }
            }

            let key = &langid.script;
            if let Ok(index) = self
                .likely_subtags
                .script
                .binary_search_by_key(&key, |(l, _)| &l.script)
            {
                let entry = &self.likely_subtags.script[index].1;
                return update_langid(entry, langid);
            }
        } else if langid.region.is_some() {
            let key = &langid.region;
            if let Ok(index) = self
                .likely_subtags
                .region
                .binary_search_by_key(&key, |(l, _)| &l.region)
            {
                let entry = &self.likely_subtags.region[index].1;
                return update_langid(entry, langid);
            }
        } else {
            return update_langid(&self.likely_subtags.und, langid);
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
    pub fn minimize<T: AsMut<LanguageIdentifier>>(&self, mut langid: T) -> CanonicalizationResult {
        let langid = langid.as_mut();

        let mut max = langid.clone();
        self.maximize(&mut max);
        max.variants.clear();
        let mut trial = max.clone();

        trial.script = None;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if langid.script.is_some() || langid.script.is_some() {
                langid.script = None;
                langid.region = None;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        trial.script = None;
        trial.region = max.region;
        self.maximize(&mut trial);
        if trial == max {
            if langid.script.is_some() || langid.region != max.region {
                langid.script = None;
                langid.region = max.region;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        trial.script = max.script;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if langid.script != max.script || langid.region.is_some() {
                langid.script = max.script;
                langid.region = None;
                return CanonicalizationResult::Modified;
            } else {
                return CanonicalizationResult::Unmodified;
            }
        }

        if langid.script != max.script || langid.region != max.region {
            langid.script = max.script;
            langid.region = max.region;
            CanonicalizationResult::Modified
        } else {
            CanonicalizationResult::Unmodified
        }
    }
}
