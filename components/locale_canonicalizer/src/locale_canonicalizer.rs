// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::error::LocaleCanonicalizerError;
use icu_locid::LanguageIdentifier;
use icu_locid::Locale;
use icu_provider::prelude::*;
use icu_provider::structs::likelysubtags::*;
use std::borrow::Cow;

pub struct LocaleCanonicalizer<'a> {
    provider: &'a dyn DataProvider<'a>,
    likely_subtags: Option<LikelySubtagsV1>,
}

impl LocaleCanonicalizer<'_> {
    fn ensure_likely_subtags(&mut self) -> Result<&LikelySubtagsV1, DataError> {
        if self.likely_subtags.is_none() {
            let payload: Cow<LikelySubtagsV1> = self
                .provider
                .load(&DataRequest {
                    data_key: key::LIKELY_SUBTAGS_V1,
                    data_entry: DataEntry {
                        variant: None,
                        langid: "und".parse().unwrap(),
                    },
                })?
                .take_payload()?;
            let mut likely_subtags = payload.into_owned();
            likely_subtags.entries.sort();
            self.likely_subtags = Some(likely_subtags);
        }
        Ok(&self.likely_subtags.as_ref().unwrap())
    }

    /// A constructor which takes a DataProvider and creates a
    /// LocaleCanonicalizer.
    pub fn new<'a>(provider: &'a dyn DataProvider<'a>) -> LocaleCanonicalizer<'a> {
        LocaleCanonicalizer {
            provider,
            likely_subtags: None,
        }
    }

    /// This returns a new Locale that is the result of running the
    /// 'Add Likely Subtags' algorithm from
    /// https://www.unicode.org/reports/tr35/#Likely_Subtags.
    pub fn maximize(&mut self, locale: &Locale) -> Result<Locale, LocaleCanonicalizerError> {
        let likely_subtags = self.ensure_likely_subtags()?;

        let mut key = LanguageIdentifier {
            language: locale.language,
            script: locale.script,
            region: locale.region,
            variants: icu_locid::subtags::Variants::from_vec_unchecked(Vec::new()),
        };

        let to_result = |index: usize| -> Locale {
            let entry = &likely_subtags.entries[index].1;
            let language = if locale.language == "und" {
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
            if let Ok(index) = likely_subtags
                .entries
                .binary_search_by_key(&&key, |(l, _)| l)
            {
                return Ok(to_result(index));
            }
        }

        // languages_regions
        if locale.region.is_some() {
            key.script = None;
            if let Ok(index) = likely_subtags
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
            if let Ok(index) = likely_subtags
                .entries
                .binary_search_by_key(&&key, |(l, _)| l)
            {
                return Ok(to_result(index));
            }
        }

        // languages
        key.script = None;
        key.region = None;
        if let Ok(index) = likely_subtags
            .entries
            .binary_search_by_key(&&key, |(l, _)| l)
        {
            return Ok(to_result(index));
        }

        // und_scripts
        if locale.script.is_some() {
            key.language = "und".parse().unwrap();
            key.script = locale.script;
            key.region = None;
            if let Ok(index) = likely_subtags
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
    pub fn minimize(&mut self, locale: &Locale) -> Result<Locale, LocaleCanonicalizerError> {
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
