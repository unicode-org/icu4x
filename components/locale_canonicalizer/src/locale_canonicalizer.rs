// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::LocaleCanonicalizer;
use crate::LocaleCanonicalizerError;
use icu_locid::subtags;
use icu_locid::LanguageIdentifier;
use icu_locid::Locale;
use icu_provider::prelude::*;
use icu_provider::structs::likelysubtags::*;
use std::borrow::Cow;

impl LocaleCanonicalizer {
    /// A constructor which takes a DataProvider and creates a
    /// LocaleCanonicalizer.
    pub fn new(provider: &dyn DataProvider) -> Result<LocaleCanonicalizer, DataError> {
        let payload: Cow<LikelySubtagsV1> = provider
            .load(&DataRequest {
                data_key: key::LIKELY_SUBTAGS_V1,
                data_entry: DataEntry {
                    variant: None,
                    langid: "und".parse().unwrap(),
                },
            })?
            .take_payload()?;

        Ok(LocaleCanonicalizer {
            likely_subtags: payload.into_owned(),
        })
    }

    /// This returns a new Locale that is the result of running the
    /// 'Add Likely Subtags' algorithm from
    /// https://www.unicode.org/reports/tr35/#Likely_Subtags.
    pub fn maximize(&self, locale: &mut Locale) -> Result<bool, LocaleCanonicalizerError> {
        let mut key = LanguageIdentifier {
            language: locale.language,
            script: locale.script,
            region: locale.region,
            variants: subtags::Variants::default(),
        };

        let maybe_update_locale = |index: usize, locale: &mut Locale| -> bool {
            let entry = &self.likely_subtags.entries[index].1;
            let modified =
                locale.language.is_empty() || locale.script.is_none() || locale.region.is_none();
            if locale.language.is_empty() {
                locale.language = entry.language;
            }
            locale.script = locale.script.or(entry.script);
            locale.region = locale.region.or(entry.region);
            modified
        };

        // languages_scripts_regions
        if locale.script.is_some() && locale.region.is_some() {
            if let Ok(index) = self
                .likely_subtags
                .entries
                .binary_search_by_key(&&key, |(l, _)| l)
            {
                return Ok(maybe_update_locale(index, locale));
            }
        }

        // languages_regions
        if locale.region.is_some() {
            key.script = None;
            if let Ok(index) = self
                .likely_subtags
                .entries
                .binary_search_by_key(&&key, |(l, _)| l)
            {
                return Ok(maybe_update_locale(index, locale));
            }
        }

        // languages_scripts
        if locale.script.is_some() {
            key.script = locale.script;
            key.region = None;
            if let Ok(index) = self
                .likely_subtags
                .entries
                .binary_search_by_key(&&key, |(l, _)| l)
            {
                return Ok(maybe_update_locale(index, locale));
            }
        }

        // languages
        key.script = None;
        key.region = None;
        if let Ok(index) = self
            .likely_subtags
            .entries
            .binary_search_by_key(&&key, |(l, _)| l)
        {
            return Ok(maybe_update_locale(index, locale));
        }

        // und_scripts
        if locale.script.is_some() {
            key.language = subtags::Language::default();
            key.script = locale.script;
            key.region = None;
            if let Ok(index) = self
                .likely_subtags
                .entries
                .binary_search_by_key(&&key, |(l, _)| l)
            {
                return Ok(maybe_update_locale(index, locale));
            }
        }

        Err(LocaleCanonicalizerError::NotMatched)
    }

    /// This returns a new Locale that is the result of running the
    /// 'Remove Likely Subtags' algorithm from
    /// https://www.unicode.org/reports/tr35/#Likely_Subtags.
    pub fn minimize(&self, locale: &mut Locale) -> Result<bool, LocaleCanonicalizerError> {
        let mut max = locale.clone();
        let modified = self.maximize(&mut max)?;
        let mut trial = max.clone();
        trial.variants.clear();

        trial.script = None;
        trial.region = None;
        if self.maximize(&mut trial)? && trial == max {
            locale.script = None;
            locale.region = None;
            return Ok(true);
        }

        trial.script = None;
        trial.region = max.region;
        if self.maximize(&mut trial)? && trial == max {
            locale.script = None;
            locale.region = max.region;
            return Ok(true);
        }

        trial.script = max.script;
        trial.region = None;
        if self.maximize(&mut trial)? && trial == max {
            locale.script = max.script;
            locale.region = None;
            return Ok(true);
        }
        *locale = max;
        Ok(modified)
    }
}
