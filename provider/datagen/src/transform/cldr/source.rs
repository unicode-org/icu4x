// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)] // features

use icu_provider_adapters::fork::ForkByKeyProvider;
use icu_locid_transform::provider::LikelySubtagsForScriptRegionV1Marker;
use super::cldr_serde;
use super::locale_canonicalizer::likely_subtags::LikelySubtagsResources;
use crate::source::SerdeCache;
use icu_locid::LanguageIdentifier;
use icu_locid_transform::provider::LikelySubtagsForLanguageV1Marker;
use icu_locid_transform::LocaleExpander;
use icu_provider::DataError;
use icu_provider_adapters::any_payload::AnyPayloadProvider;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::RwLock;

/// A language's CLDR coverage level.
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Hash)]
#[non_exhaustive]
pub enum CoverageLevel {
    /// Locales listed as modern coverage targets by the CLDR subcomittee.
    ///
    /// This is the highest level of coverage.
    #[serde(rename = "modern")]
    Modern,
    /// Locales listed as moderate coverage targets by the CLDR subcomittee.
    ///
    /// This is a medium level of coverage.
    #[serde(rename = "moderate")]
    Moderate,
    /// Locales listed as basic coverage targets by the CLDR subcomittee.
    ///
    /// This is the lowest level of coverage.
    #[serde(rename = "basic")]
    Basic,
}

#[derive(Debug)]
pub(crate) struct CldrCache {
    serde_cache: SerdeCache,
    is_full: RwLock<Option<bool>>,
    locale_expander: LocaleExpander,
}

impl CldrCache {
    pub fn try_from_serde_cache(serde_cache: SerdeCache) -> Result<Self, DataError> {
        let likely_subtags: &cldr_serde::likely_subtags::Resource =
            serde_cache.read_and_parse_json("cldr-core/supplemental/likelySubtags.json")?;
        let coverage_levels: &cldr_serde::coverage_levels::Resource =
            serde_cache.read_and_parse_json("cldr-core/coverageLevels.json")?;
        let resources = LikelySubtagsResources::from_resources(likely_subtags, coverage_levels);
        let data = super::locale_canonicalizer::likely_subtags::transform(resources.get_common());
        let provider = ForkByKeyProvider::new(
            AnyPayloadProvider::from_owned::<LikelySubtagsForLanguageV1Marker>(data.clone().into()),
            AnyPayloadProvider::from_owned::<LikelySubtagsForScriptRegionV1Marker>(data.into()),
        );
        let locale_expander = LocaleExpander::try_new_with_any_provider(&provider).map_err(|e| {
            DataError::custom("creating LocaleExpander in CldrCache").with_display_context(&e)
        })?;
        Ok(CldrCache {
            serde_cache,
            is_full: Default::default(),
            locale_expander,
        })
    }

    pub fn core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-core".to_owned())
    }

    pub fn numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-numbers".to_owned())
    }

    pub fn misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-misc".to_owned())
    }

    pub fn bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-bcp47/bcp47".to_string())
    }

    pub fn displaynames(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-localenames".to_owned())
    }

    pub fn dates(&self, cal: &str) -> CldrDirLang<'_> {
        CldrDirLang(
            self,
            if cal == "gregorian" {
                "cldr-dates".to_owned()
            } else {
                format!("cldr-cal-{cal}")
            },
        )
    }

    pub fn locales(
        &self,
        levels: &[CoverageLevel],
    ) -> Result<Vec<icu_locid::LanguageIdentifier>, DataError> {
        Ok(self
            .serde_cache
            .read_and_parse_json::<crate::transform::cldr::cldr_serde::coverage_levels::Resource>(
                "cldr-core/coverageLevels.json",
            )?
            .coverage_levels
            .iter()
            .filter_map(|(locale, c)| levels.contains(c).then_some(locale))
            .cloned()
            .collect())
    }

    fn dir_suffix(&self) -> Result<&'static str, DataError> {
        let maybe_is_full = *self.is_full.read().expect("poison");
        let is_full = match maybe_is_full {
            Some(x) => x,
            None => {
                let is_full = self.serde_cache.list("cldr-misc-full")?.next().is_some();
                let _ = self.is_full.write().expect("poison").insert(is_full);
                is_full
            }
        };
        if is_full {
            Ok("full")
        } else {
            Ok("modern")
        }
    }

    /// CLDR sometimes stores regional variants with their script.
    /// Add in the likely subtags here to make that data reachable.
    fn add_script(&self, langid: &LanguageIdentifier) -> Option<LanguageIdentifier> {
        if langid.language.is_empty() || langid.script.is_some() || langid.region.is_none() {
            return None;
        }
        let mut langid = langid.clone();
        self.locale_expander.maximize(&mut langid);
        debug_assert!(langid.script.is_some());
        Some(langid)
    }

    /// ICU4X does not store regional variants with their script
    /// if the script is the default for the language.
    /// Perform that normalization mapping here.
    fn remove_script(&self, langid: &LanguageIdentifier) -> Option<LanguageIdentifier> {
        if langid.language.is_empty() || langid.script.is_none() || langid.region.is_none() {
            return None;
        }
        let region = langid.region;
        let mut langid = langid.clone();
        self.locale_expander.minimize(&mut langid);
        if langid.script.is_some() {
            // Wasn't able to minimize the script
            return None;
        }
        // Restore the region
        langid.region = region;
        Some(langid)
    }
}

pub(crate) struct CldrDirNoLang<'a>(&'a CldrCache, String);

impl<'a> CldrDirNoLang<'a> {
    pub fn read_and_parse<S>(&self, file_name: &str) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.0
            .serde_cache
            .read_and_parse_json(&format!("{}/{}", self.1, file_name))
    }
}

pub(crate) struct CldrDirLang<'a>(&'a CldrCache, String);

impl<'a> CldrDirLang<'a> {
    pub fn read_and_parse<S>(
        &self,
        lang: &LanguageIdentifier,
        file_name: &str,
    ) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        let dir_suffix = self.0.dir_suffix()?;
        let result = self
            .0
            .serde_cache
            .read_and_parse_json(&format!("{}-{dir_suffix}/main/{lang}/{file_name}", self.1));
        if result.is_err() {
            if let Some(new_langid) = self.0.add_script(lang) {
                return self.read_and_parse(&new_langid, file_name);
            }
        }
        result
    }

    pub fn list_langs(&self) -> Result<impl Iterator<Item = LanguageIdentifier> + '_, DataError> {
        let dir_suffix = self.0.dir_suffix()?;
        Ok(self
            .0
            .serde_cache
            .list(&format!("{}-{dir_suffix}/main", self.1))?
            .map(|path| {
                let langid = LanguageIdentifier::from_str(&path).unwrap();
                self.0.remove_script(&langid).unwrap_or(langid)
            }))
    }

    pub fn file_exists(
        &self,
        lang: &LanguageIdentifier,
        file_name: &str,
    ) -> Result<bool, DataError> {
        let dir_suffix = self.0.dir_suffix()?;
        let exists = self
            .0
            .serde_cache
            .file_exists(&format!("{}-{dir_suffix}/main/{lang}/{file_name}", self.1))?;
        if !exists {
            if let Some(new_langid) = self.0.add_script(lang) {
                return self.file_exists(&new_langid, file_name);
            }
        }
        Ok(exists)
    }
}
