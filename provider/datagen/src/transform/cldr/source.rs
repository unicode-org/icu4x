// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::source::SerdeCache;
use icu_locid::LanguageIdentifier;
use icu_provider::DataError;
use std::fmt::Debug;
use std::str::FromStr;

/// Specifies a variant of CLDR JSON
#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize)]
#[non_exhaustive]
pub enum LocaleSubset {
    /// Deprecated. Use `Modern` + `Moderate` + `Basic`
    #[deprecated]
    #[serde(skip)]
    Full,
    /// Includes locales listed as modern coverage targets by the CLDR subcomittee
    #[serde(rename = "modern")]
    Modern,
    /// Includes locales listed as moderate coverage targets by the CLDR subcomittee
    #[serde(rename = "moderate")]
    Moderate,
    /// Includes locales listed as basic coverage targets by the CLDR subcomittee
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
        CldrDirLang(&self.0, "cldr-numbers-full/main".to_owned())
    }

    pub fn misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(&self.0, "cldr-misc-full/main".to_owned())
    }

    pub fn bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(&self.0, "cldr-bcp47/bcp47".to_string())
    }

    pub fn displaynames(&self) -> CldrDirLang<'_> {
        CldrDirLang(&self.0, "cldr-localenames-full/main".to_owned())
    }

    pub fn dates(&self, cal: &str) -> CldrDirLang<'_> {
        CldrDirLang(
            &self.0,
            if cal == "gregorian" {
                "cldr-dates-full/main".to_owned()
            } else {
                format!("cldr-cal-{}-full/main", cal)
            },
        )
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
            .read_and_parse_json(&format!("{}/{}/{}", self.1, lang, file_name))
    }

    pub fn list_langs(&self) -> Result<impl Iterator<Item = LanguageIdentifier>, DataError> {
        Ok(self.0.list(&self.1)?.into_iter().map(|path| {
            LanguageIdentifier::from_str(&path.file_name().unwrap().to_string_lossy()).unwrap()
        }))
    }
}
