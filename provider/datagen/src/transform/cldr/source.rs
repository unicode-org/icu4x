// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::source::SerdeCache;
use icu_locid::LanguageIdentifier;
use icu_provider::DataError;
use std::fmt::Debug;
use std::str::FromStr;

/// Specifies a variant of CLDR JSON
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocaleSubset {
    /// Includes all data
    Full,
    /// Includes locales listed as modern coverage targets by the CLDR subcomittee
    Modern,
}

impl std::fmt::Display for LocaleSubset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Full => "full",
                Self::Modern => "modern",
            }
        )
    }
}

#[derive(Debug)]
pub(crate) struct CldrCache {
    locale_subset: LocaleSubset,
    cache: SerdeCache,
}

impl CldrCache {
    pub fn new<P: AsRef<std::path::Path>>(
        root: P,
        locale_subset: LocaleSubset,
    ) -> Result<Self, DataError> {
        Ok(Self {
            locale_subset,
            cache: SerdeCache::new(root)?,
        })
    }

    pub fn core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(&self.cache, "cldr-core".to_string())
    }

    pub fn numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(
            &self.cache,
            format!("cldr-numbers-{}/main", self.locale_subset),
        )
    }

    pub fn misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(
            &self.cache,
            format!("cldr-misc-{}/main", self.locale_subset),
        )
    }

    pub fn bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(&self.cache, "cldr-bcp47/bcp47".to_string())
    }

    #[cfg(feature = "experimental")]
    pub fn displaynames(&self) -> CldrDirLang<'_> {
        CldrDirLang(
            &self.cache,
            format!("cldr-localenames-{}/main", self.locale_subset),
        )
    }

    pub fn dates(&self, cal: &str) -> CldrDirLang<'_> {
        CldrDirLang(
            &self.cache,
            if cal == "gregorian" {
                format!("cldr-dates-{}/main", self.locale_subset)
            } else {
                format!("cldr-cal-{}-{}/main", cal, self.locale_subset)
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
