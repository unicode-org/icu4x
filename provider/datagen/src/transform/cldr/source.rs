// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::source::SerdeCache;
use icu_locid::LanguageIdentifier;
use icu_provider::DataError;
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
}

impl CldrCache {
    pub fn from_serde_cache(serde_cache: SerdeCache) -> Self {
        CldrCache {
            serde_cache,
            is_full: Default::default(),
        }
    }

    pub fn core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(&self, "cldr-core".to_owned())
    }

    pub fn numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(&self, "cldr-numbers".to_owned())
    }

    pub fn misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(&self, "cldr-misc".to_owned())
    }

    pub fn bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(&self, "cldr-bcp47/bcp47".to_string())
    }

    pub fn displaynames(&self) -> CldrDirLang<'_> {
        CldrDirLang(&self, "cldr-localenames".to_owned())
    }

    pub fn dates(&self, cal: &str) -> CldrDirLang<'_> {
        CldrDirLang(
            &self,
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
            .filter_map(|(locale, c)| levels.contains(c).then(|| locale))
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
        self.0
            .serde_cache
            .read_and_parse_json(&format!("{}-{dir_suffix}/main/{lang}/{file_name}", self.1))
    }

    pub fn list_langs(&self) -> Result<impl Iterator<Item = LanguageIdentifier>, DataError> {
        let dir_suffix = self.0.dir_suffix()?;
        Ok(self
            .0
            .serde_cache
            .list(&format!("{}-{dir_suffix}/main", self.1))?
            .map(|path| LanguageIdentifier::from_str(&path).unwrap()))
    }

    pub fn file_exists(
        &self,
        lang: &LanguageIdentifier,
        file_name: &str,
    ) -> Result<bool, DataError> {
        let dir_suffix = self.0.dir_suffix()?;
        self.0
            .serde_cache
            .file_exists(&format!("{}-{dir_suffix}/main/{lang}/{file_name}", self.1))
    }
}
