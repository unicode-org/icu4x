// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::source::SerdeCache;
use icu_locid::LanguageIdentifier;
use icu_provider::DataError;
use std::fmt::Debug;
use std::str::FromStr;

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
pub(crate) struct CldrCache(pub SerdeCache);

impl CldrCache {
    pub fn core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(&self.0, "cldr-core".to_owned())
    }

    pub fn numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(&self.0, "cldr-numbers".to_owned())
    }

    pub fn misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(&self.0, "cldr-misc".to_owned())
    }

    pub fn bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(&self.0, "cldr-bcp47/bcp47".to_string())
    }

    pub fn displaynames(&self) -> CldrDirLang<'_> {
        CldrDirLang(&self.0, "cldr-localenames".to_owned())
    }

    pub fn dates(&self, cal: &str) -> CldrDirLang<'_> {
        CldrDirLang(
            &self.0,
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
            .0
            .read_and_parse_json::<crate::transform::cldr::cldr_serde::coverage_levels::Resource>(
                "cldr-core/coverageLevels.json",
            )?
            .coverage_levels
            .iter()
            .filter_map(|(locale, c)| levels.contains(c).then(|| locale))
            .cloned()
            .collect())
    }
}

pub(crate) struct CldrDirNoLang<'a>(&'a SerdeCache, String);

impl<'a> CldrDirNoLang<'a> {
    pub fn read_and_parse<S>(&self, file_name: &str) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.0
            .read_and_parse_json(&format!("{}/{}", self.1, file_name))
    }
}

pub(crate) struct CldrDirLang<'a>(&'a SerdeCache, String);

impl<'a> CldrDirLang<'a> {
    pub fn read_and_parse<S>(
        &self,
        lang: &LanguageIdentifier,
        file_name: &str,
    ) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.0
            .read_and_parse_json(&format!("{}/{}/{}", self.dir()?, lang, file_name))
    }

    pub fn list_langs(&self) -> Result<impl Iterator<Item = LanguageIdentifier>, DataError> {
        Ok(self
            .0
            .list(&self.dir()?)?
            .map(|path| LanguageIdentifier::from_str(&path).unwrap()))
    }

    fn dir(&self) -> Result<String, DataError> {
        let mut dir = self
            .0
            .list("")?
            .find(|dir| dir.starts_with(self.1.as_str()))
            .unwrap_or_else(|| format!("{}-full", self.1));
        dir.push_str("/main");
        Ok(dir)
    }

    pub fn file_exists(
        &self,
        lang: &LanguageIdentifier,
        file_name: &str,
    ) -> Result<bool, DataError> {
        self.0
            .file_exists(&format!("{}/{lang}/{file_name}", self.dir()?))
    }
}
