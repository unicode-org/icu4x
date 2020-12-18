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
    pub fn maximize(&self, locale: &Locale) -> Result<Locale, LocaleCanonicalizerError> {
        let mut key = LanguageIdentifier {
            language: locale.language,
            script: locale.script,
            region: locale.region,
            variants: subtags::Variants::default(),
        };

        let to_result = |index: usize| -> Locale {
            let entry = &self.likely_subtags.entries[index].1;
            let language = if locale.language.is_empty() {
                entry.language
            } else {
                locale.language
            };
            Locale {
                language,
                script: locale.script.or(entry.script),
                region: locale.region.or(entry.region),
                variants: locale.variants.clone(),
                extensions: locale.extensions.clone(),
            }
        };

        // languages_scripts_regions
        if locale.script.is_some() && locale.region.is_some() {
            if let Ok(index) = self
                .likely_subtags
                .entries
                .binary_search_by_key(&&key, |(l, _)| l)
            {
                return Ok(to_result(index));
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
                return Ok(to_result(index));
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
                return Ok(to_result(index));
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
            return Ok(to_result(index));
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
                return Ok(to_result(index));
            }
        }

        Err(LocaleCanonicalizerError::NotMatched)
    }

    /// This returns a new Locale that is the result of running the
    /// 'Remove Likely Subtags' algorithm from
    /// https://www.unicode.org/reports/tr35/#Likely_Subtags.
    pub fn minimize(&self, locale: &Locale) -> Result<Locale, LocaleCanonicalizerError> {
        let max = self.maximize(locale)?;
        let mut trial = max.clone();
        trial.variants.clear();

        trial.script = None;
        trial.region = None;
        if self.maximize(&trial)? == max {
            trial.variants = max.variants;
            return Ok(trial);
        }

        trial.script = None;
        trial.region = max.region;
        if self.maximize(&trial)? == max {
            trial.variants = max.variants;
            return Ok(trial);
        }

        trial.script = max.script;
        trial.region = None;
        if self.maximize(&trial)? == max {
            trial.variants = max.variants;
            return Ok(trial);
        }

        Ok(max)
    }
}
