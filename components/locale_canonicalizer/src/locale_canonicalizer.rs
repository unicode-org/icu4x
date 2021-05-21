// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use icu_locid::LanguageIdentifier;
use icu_provider::prelude::*;

/// Used to track the result of a canonicalization operation that potentially modifies its argument in place.
#[derive(Debug, PartialEq)]
pub enum CanonicalizationResult {
    Modified,
    Unmodified,
}

pub struct LocaleCanonicalizer<'a> {
    likely_subtags: DataPayload<'a, LikelySubtagsV1>,
}

#[inline]
fn update_langid(
    entry: &LanguageIdentifier,
    langid: &mut LanguageIdentifier,
) -> CanonicalizationResult {
    if langid.language.is_empty() {
        langid.language = entry.language;
    }
    langid.script = langid.script.or(entry.script);
    langid.region = langid.region.or(entry.region);
    CanonicalizationResult::Modified
}

macro_rules! maximize_locale {
    ( $langid:ident, $table:expr, $key:expr ) => {{
        if let Ok(index) = $table.binary_search_by_key(&&$key, |(i1, _)| i1) {
            let entry = &$table[index].1;
            return update_langid(entry, $langid);
        }
    }};
    ( $langid:ident, $table:expr, $key1:expr, $key2:expr ) => {{
        if let Ok(index) = $table.binary_search_by_key(&(&$key1, &$key2), |(i1, i2, _)| (i1, i2)) {
            let entry = &$table[index].2;
            return update_langid(entry, $langid);
        }
    }};
}

impl LocaleCanonicalizer<'_> {
    /// A constructor which takes a [`DataProvider`] and creates a [`LocaleCanonicalizer`].
    pub fn new<'d>(
        provider: &(impl DataProvider<'d, LikelySubtagsV1> + ?Sized),
    ) -> Result<LocaleCanonicalizer<'d>, DataError> {
        let payload: DataPayload<LikelySubtagsV1> = provider
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
    /// returns [`CanonicalizationResult::Modified`]. Otherwise, the method
    /// returns [`CanonicalizationResult::Unmodified`] and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    /// use icu_locid::Locale;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let lc = LocaleCanonicalizer::new(&provider)
    ///     .expect("create failed");
    ///
    /// let mut locale : Locale = "zh-CN".parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Modified);
    /// assert_eq!(locale.to_string(), "zh-Hans-CN");
    ///
    /// let mut locale : Locale = "zh-Hant-TW".parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.maximize(&mut locale), CanonicalizationResult::Unmodified);
    /// assert_eq!(locale.to_string(), "zh-Hant-TW");
    /// ```
    pub fn maximize<T: AsMut<LanguageIdentifier>>(&self, mut langid: T) -> CanonicalizationResult {
        let langid = langid.as_mut();

        if !langid.language.is_empty() && langid.script.is_some() && langid.region.is_some() {
            return CanonicalizationResult::Unmodified;
        }

        if let Some(language) = langid.language.into() {
            if let Some(region) = langid.region {
                maximize_locale!(
                    langid,
                    self.likely_subtags.language_region,
                    language,
                    region.into()
                );
            }
            if let Some(script) = langid.script {
                maximize_locale!(
                    langid,
                    self.likely_subtags.language_script,
                    language,
                    script.into()
                );
            }
            maximize_locale!(langid, self.likely_subtags.language, language);
        } else if let Some(script) = langid.script {
            if let Some(region) = langid.region {
                maximize_locale!(
                    langid,
                    self.likely_subtags.script_region,
                    script.into(),
                    region.into()
                );
            }
            maximize_locale!(langid, self.likely_subtags.script, script.into());
        } else if let Some(region) = langid.region {
            maximize_locale!(langid, self.likely_subtags.region, region.into());
        }
        update_langid(&self.likely_subtags.und, langid)
    }

    /// This returns a new Locale that is the result of running the
    /// 'Remove Likely Subtags' algorithm from
    /// https://www.unicode.org/reports/tr35/#Likely_Subtags.
    ///
    /// If the result of running the algorithm would result in a new locale, the
    /// locale argument is updated in place to match the result, and the method
    /// returns [`CanonicalizationResult::Modified`]. Otherwise, the method
    /// returns [`CanonicalizationResult::Unmodified`] and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_canonicalizer::{CanonicalizationResult, LocaleCanonicalizer};
    /// use icu_locid::Locale;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let lc = LocaleCanonicalizer::new(&provider)
    ///     .expect("creation failed");
    ///
    /// let mut locale : Locale = "zh-Hans-CN".parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Modified);
    /// assert_eq!(locale.to_string(), "zh");
    ///
    /// let mut locale : Locale = "zh".parse()
    ///     .expect("parse failed");
    /// assert_eq!(lc.minimize(&mut locale), CanonicalizationResult::Unmodified);
    /// assert_eq!(locale.to_string(), "zh");
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
