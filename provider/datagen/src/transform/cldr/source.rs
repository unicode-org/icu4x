// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)] // features

use crate::provider::source::SerdeCache;
use crate::provider::CoverageLevel;
use icu_calendar::provider::EraStartDate;
use icu_locale::provider::{
    LikelySubtagsExtendedV1Marker, LikelySubtagsForLanguageV1Marker,
    LikelySubtagsForScriptRegionV1Marker,
};
use icu_locale::LocaleExpander;
use icu_locale_core::LanguageIdentifier;
use icu_provider::prelude::*;
use icu_provider::DataError;
use icu_provider_adapters::any_payload::AnyPayloadProvider;
use icu_provider_adapters::fork::ForkByMarkerProvider;
use icu_provider_adapters::make_forking_provider;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug)]
pub(in crate::provider) struct CldrCache {
    pub(in crate::provider) serde_cache: SerdeCache,
    dir_suffix: OnceLock<Result<&'static str, DataError>>,
    extended_locale_expander: OnceLock<Result<LocaleExpander, DataError>>,
    modern_japanese_eras: OnceLock<Result<BTreeSet<String>, DataError>>,
    #[cfg(feature = "experimental_components")]
    // used by transforms/mod.rs
    pub(in crate::provider) transforms: OnceLock<
        Result<std::sync::Mutex<icu_experimental::transliterate::RuleCollection>, DataError>,
    >,
}

impl CldrCache {
    pub(in crate::provider) fn from_serde_cache(serde_cache: SerdeCache) -> Self {
        CldrCache {
            serde_cache,
            dir_suffix: Default::default(),
            extended_locale_expander: Default::default(),
            modern_japanese_eras: Default::default(),
            #[cfg(feature = "experimental_components")]
            transforms: Default::default(),
        }
    }

    pub(in crate::provider) fn core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-core".to_owned())
    }

    pub(in crate::provider) fn numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-numbers".to_owned())
    }

    pub(in crate::provider) fn misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-misc".to_owned())
    }

    pub(in crate::provider) fn bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-bcp47/bcp47".to_string())
    }

    pub(in crate::provider) fn personnames(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-person-names".to_owned())
    }

    pub(in crate::provider) fn displaynames(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-localenames".to_owned())
    }

    pub(in crate::provider) fn dates(&self, cal: &str) -> CldrDirLang<'_> {
        CldrDirLang(
            self,
            if cal == "gregorian" || cal == "generic" {
                "cldr-dates".to_owned()
            } else {
                format!("cldr-cal-{cal}")
            },
        )
    }

    pub(in crate::provider) fn locales(
        &self,
        levels: impl IntoIterator<Item = CoverageLevel>,
    ) -> Result<Vec<icu_locale_core::LanguageIdentifier>, DataError> {
        let levels = levels.into_iter().collect::<HashSet<_>>();
        Ok(self
            .serde_cache
            .read_and_parse_json::<crate::provider::transform::cldr::cldr_serde::coverage_levels::Resource>(
                "cldr-core/coverageLevels.json",
            )?
            .coverage_levels
            .iter()
            .filter_map(|(locale, c)| levels.contains(c).then_some(locale))
            .cloned()
            // `und` needs to be part of every set
            .chain([Default::default()])
            .collect())
    }

    pub(in crate::provider) fn dir_suffix(&self) -> Result<&'static str, DataError> {
        *self.dir_suffix.get_or_init(|| {
            if self.serde_cache.list("cldr-misc-full")?.next().is_some() {
                Ok("full")
            } else {
                Ok("modern")
            }
        })
    }

    fn extended_locale_expander(&self) -> Result<&LocaleExpander, DataError> {
        use super::locale_canonicalizer::likely_subtags::*;
        self.extended_locale_expander
            .get_or_init(|| {
                let common_data =
                    transform(LikelySubtagsResources::try_from_cldr_cache(self)?.get_common());
                let extended_data =
                    transform(LikelySubtagsResources::try_from_cldr_cache(self)?.get_extended());
                let provider = make_forking_provider!(
                    ForkByMarkerProvider::new,
                    [
                        AnyPayloadProvider::from_owned::<LikelySubtagsForLanguageV1Marker>(
                            common_data.clone().into(),
                        ),
                        AnyPayloadProvider::from_owned::<LikelySubtagsForScriptRegionV1Marker>(
                            common_data.into(),
                        ),
                        AnyPayloadProvider::from_owned::<LikelySubtagsExtendedV1Marker>(
                            extended_data.into()
                        ),
                    ]
                );
                LocaleExpander::try_new_extended_unstable(&provider.as_downcasting()).map_err(|e| {
                    DataError::custom("creating LocaleExpander in CldrCache")
                        .with_display_context(&e)
                })
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// Get the list of eras in the japanese calendar considered "modern" (post-Meiji, inclusive)
    ///
    /// These will be in CLDR era index form; these are usually numbers
    pub(in crate::provider) fn modern_japanese_eras(&self) -> Result<&BTreeSet<String>, DataError> {
        self.modern_japanese_eras
            .get_or_init(|| {
                let era_dates: &super::cldr_serde::japanese::Resource = self
                    .core()
                    .read_and_parse("supplemental/calendarData.json")?;
                let mut set = BTreeSet::<String>::new();
                for (era_index, date) in era_dates.supplemental.calendar_data.japanese.eras.iter() {
                    let start_date =
                        EraStartDate::from_str(if let Some(start_date) = date.start.as_ref() {
                            start_date
                        } else {
                            continue;
                        })
                        .map_err(|_| {
                            DataError::custom(
                                "calendarData.json contains unparseable data for a japanese era",
                            )
                            .with_display_context(&format!("era index {}", era_index))
                        })?;

                    if start_date.year >= 1868 {
                        set.insert(era_index.into());
                    }
                }
                Ok(set)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// CLDR sometimes stores locales with default scripts.
    /// Add in the likely script here to make that data reachable.
    fn add_script_extended(
        &self,
        langid: &LanguageIdentifier,
    ) -> Result<Option<LanguageIdentifier>, DataError> {
        if langid.language.is_empty() || langid.script.is_some() {
            return Ok(None);
        }
        let mut new_langid = langid.clone();
        self.extended_locale_expander()?.maximize(&mut new_langid);
        debug_assert!(
            new_langid.script.is_some(),
            "Script not found for: {new_langid:?}"
        );
        if langid.region.is_none() {
            new_langid.region = None;
        }
        Ok(Some(new_langid))
    }

    /// ICU4X does not store locales with their script
    /// if the script is the default for the language.
    /// Perform that normalization mapping here.
    fn remove_script_extended(
        &self,
        langid: &LanguageIdentifier,
    ) -> Result<Option<LanguageIdentifier>, DataError> {
        if langid.language.is_empty() || langid.script.is_none() {
            return Ok(None);
        }
        let region = langid.region;
        let mut langid = langid.clone();
        self.extended_locale_expander()?.minimize(&mut langid);
        if langid.script.is_some() || (region.is_none() && langid.region.is_some()) {
            // Wasn't able to minimize the script, or had to add a region
            return Ok(None);
        }
        // Restore the region
        langid.region = region;
        Ok(Some(langid))
    }
}

pub(in crate::provider) struct CldrDirNoLang<'a>(&'a CldrCache, String);

impl<'a> CldrDirNoLang<'a> {
    pub(in crate::provider) fn read_and_parse<S>(&self, file_name: &str) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.0
            .serde_cache
            .read_and_parse_json(&format!("{}/{}", self.1, file_name))
    }
}

pub(in crate::provider) struct CldrDirLang<'a>(&'a CldrCache, String);

impl<'a> CldrDirLang<'a> {
    pub(in crate::provider) fn read_and_parse<S>(
        &self,
        lang: &LanguageIdentifier,
        file_name: &str,
    ) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        let dir_suffix = self.0.dir_suffix()?;
        let path = format!("{}-{dir_suffix}/main/{lang}/{file_name}", self.1);
        if self.0.serde_cache.file_exists(&path)? {
            self.0.serde_cache.read_and_parse_json(&path)
        } else if let Some(new_langid) = self.0.add_script_extended(lang)? {
            self.read_and_parse(&new_langid, file_name)
        } else {
            Err(DataErrorKind::Io(std::io::ErrorKind::NotFound)
                .into_error()
                .with_display_context(&path))
        }
    }

    pub(in crate::provider) fn list_langs(
        &self,
    ) -> Result<impl Iterator<Item = LanguageIdentifier> + '_, DataError> {
        let dir_suffix = self.0.dir_suffix()?;
        let path = format!("{}-{dir_suffix}/main", self.1);
        Ok(self
            .0
            .serde_cache
            .list(&path)?
            .map(|path| -> Result<LanguageIdentifier, DataError> {
                let langid = LanguageIdentifier::from_str(&path).unwrap();
                Ok(self.0.remove_script_extended(&langid)?.unwrap_or(langid))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter())
    }

    pub(in crate::provider) fn file_exists(
        &self,
        lang: &LanguageIdentifier,
        file_name: &str,
    ) -> Result<bool, DataError> {
        let dir_suffix = self.0.dir_suffix()?;
        let path = format!("{}-{dir_suffix}/main/{lang}/{file_name}", self.1);
        if self.0.serde_cache.file_exists(&path)? {
            Ok(true)
        } else if let Some(new_langid) = self.0.add_script_extended(lang)? {
            self.file_exists(&new_langid, file_name)
        } else {
            Ok(false)
        }
    }
}
