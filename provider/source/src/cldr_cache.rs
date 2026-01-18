// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)] // features

use crate::cldr_serde::eras::EraData;
use crate::datetime::DatagenCalendar;
use crate::source::SerdeCache;
use crate::CoverageLevel;
use icu::locale::provider::{
    LocaleLikelySubtagsExtendedV1, LocaleLikelySubtagsLanguageV1, LocaleLikelySubtagsScriptRegionV1,
};
#[cfg(feature = "experimental")]
use icu::locale::subtags::Region;
#[cfg(feature = "experimental")]
use icu::locale::LanguageIdentifier;
use icu::locale::LocaleExpander;
use icu_provider::prelude::*;
use icu_provider::DataError;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::OnceLock;
use writeable::Writeable;

#[derive(Debug)]
pub(crate) struct CldrCache {
    pub(crate) serde_cache: SerdeCache,
    dir_suffix: OnceLock<Result<&'static str, DataError>>,
    extended_locale_expander: OnceLock<Result<LocaleExpander, DataError>>,
    #[expect(clippy::type_complexity)]
    pub(crate) calendar_eras: OnceLock<
        Result<
            BTreeMap<DatagenCalendar, (Option<DatagenCalendar>, Vec<(usize, EraData)>)>,
            DataError,
        >,
    >,
    #[cfg(feature = "experimental")]
    // used by transforms/mod.rs
    pub(crate) transforms: OnceLock<
        Result<std::sync::Mutex<icu::experimental::transliterate::RuleCollection>, DataError>,
    >,
    pub(crate) tz_caches: crate::time_zones::Caches,
}

impl CldrCache {
    pub(crate) fn from_serde_cache(serde_cache: SerdeCache) -> Self {
        CldrCache {
            serde_cache,
            dir_suffix: Default::default(),
            extended_locale_expander: Default::default(),
            calendar_eras: Default::default(),
            #[cfg(feature = "experimental")]
            transforms: Default::default(),
            tz_caches: Default::default(),
        }
    }

    pub(crate) fn core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-core".to_owned())
    }

    pub(crate) fn numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-numbers".to_owned())
    }

    pub(crate) fn misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-misc".to_owned())
    }

    pub(crate) fn bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-bcp47/bcp47".to_string())
    }

    pub(crate) fn personnames(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-person-names".to_owned())
    }

    pub(crate) fn displaynames(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-localenames".to_owned())
    }

    pub(crate) fn units(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-units".to_owned())
    }

    pub(crate) fn dates(&self, cal: &str) -> CldrDirLang<'_> {
        CldrDirLang(
            self,
            if cal == "gregorian" || cal == "generic" {
                "cldr-dates".to_owned()
            } else {
                format!("cldr-cal-{cal}")
            },
        )
    }

    pub(crate) fn locales(
        &self,
        levels: impl IntoIterator<Item = CoverageLevel>,
    ) -> Result<Vec<DataLocale>, DataError> {
        let levels = levels.into_iter().collect::<HashSet<_>>();
        let mut locales: Vec<DataLocale> = self
            .serde_cache
            .read_and_parse_json::<crate::cldr_serde::coverage_levels::Resource>(
                "cldr-core/coverageLevels.json",
            )?
            .coverage_levels
            .iter()
            .filter_map(|(locale, c)| levels.contains(c).then_some(locale))
            .cloned()
            .map(Into::into)
            // `und` needs to be part of every set
            .chain([Default::default()])
            .collect();
        locales.sort_by(|a, b| {
            let b = b.write_to_string();
            a.strict_cmp(b.as_bytes())
        });
        Ok(locales)
    }

    pub(crate) fn dir_suffix(&self) -> Result<&'static str, DataError> {
        *self.dir_suffix.get_or_init(|| {
            if self.serde_cache.list("cldr-misc-full")?.next().is_some() {
                Ok("full")
            } else {
                Ok("modern")
            }
        })
    }

    pub(crate) fn extended_locale_expander(&self) -> Result<&LocaleExpander, DataError> {
        use super::locale::likely_subtags::*;
        self.extended_locale_expander
            .get_or_init(|| {
                use icu_provider::prelude::*;
                struct Provider {
                    common: TransformResult,
                    extended: TransformResult,
                }
                impl DataProvider<LocaleLikelySubtagsLanguageV1> for Provider {
                    fn load(
                        &self,
                        _req: DataRequest,
                    ) -> Result<DataResponse<LocaleLikelySubtagsLanguageV1>, DataError>
                    {
                        Ok(DataResponse {
                            payload: DataPayload::from_owned(self.common.as_langs()),
                            metadata: Default::default(),
                        })
                    }
                }
                impl DataProvider<LocaleLikelySubtagsScriptRegionV1> for Provider {
                    fn load(
                        &self,
                        _req: DataRequest,
                    ) -> Result<DataResponse<LocaleLikelySubtagsScriptRegionV1>, DataError>
                    {
                        Ok(DataResponse {
                            payload: DataPayload::from_owned(self.common.as_script_region()),
                            metadata: Default::default(),
                        })
                    }
                }
                impl DataProvider<LocaleLikelySubtagsExtendedV1> for Provider {
                    fn load(
                        &self,
                        _req: DataRequest,
                    ) -> Result<DataResponse<LocaleLikelySubtagsExtendedV1>, DataError>
                    {
                        Ok(DataResponse {
                            payload: DataPayload::from_owned(self.extended.as_extended()),
                            metadata: Default::default(),
                        })
                    }
                }
                let common =
                    transform(LikelySubtagsResources::try_from_cldr_cache(self)?.get_common());
                let extended =
                    transform(LikelySubtagsResources::try_from_cldr_cache(self)?.get_extended());

                LocaleExpander::try_new_extended_unstable(&Provider { common, extended }).map_err(
                    |e| {
                        DataError::custom("creating LocaleExpander in CldrCache")
                            .with_display_context(&e)
                    },
                )
            })
            .as_ref()
            .map_err(|&e| e)
    }

    /// CLDR sometimes stores locales with default scripts.
    /// Add in the likely script here to make that data reachable.
    fn add_script_extended(&self, locale: &DataLocale) -> Result<Option<DataLocale>, DataError> {
        if locale.language.is_unknown() || locale.script.is_some() {
            return Ok(None);
        }
        let mut new_langid =
            icu::locale::LanguageIdentifier::from((locale.language, locale.script, locale.region));
        self.extended_locale_expander()?.maximize(&mut new_langid);
        debug_assert!(
            new_langid.script.is_some(),
            "Script not found for: {new_langid:?}"
        );
        if locale.region.is_none() {
            new_langid.region = None;
        }
        Ok(Some(new_langid.into()))
    }

    /// ICU4X does not store locales with their script
    /// if the script is the default for the language.
    /// Perform that normalization mapping here.
    fn remove_script_extended(&self, locale: &DataLocale) -> Result<Option<DataLocale>, DataError> {
        if locale.language.is_unknown() || locale.script.is_none() {
            return Ok(None);
        }
        let mut langid =
            icu::locale::LanguageIdentifier::from((locale.language, locale.script, locale.region));
        self.extended_locale_expander()?.minimize(&mut langid);
        if langid.script.is_some() || (locale.region.is_none() && langid.region.is_some()) {
            // Wasn't able to minimize the script, or had to add a region
            return Ok(None);
        }
        // Restore the region
        langid.region = locale.region;
        Ok(Some(langid.into()))
    }

    /// Extracts the region from a [`DataLocale`].
    ///
    /// If the locale already has a region, it is returned.  
    /// Otherwise, the likely region is inferred from the language.
    ///
    /// # Example
    ///  - "en-US" -> "US"
    ///  - "en" -> "US"
    #[cfg(feature = "experimental")]
    pub(crate) fn extract_or_infer_region(&self, locale: &DataLocale) -> Result<Region, DataError> {
        if let Some(region) = locale.region {
            return Ok(region);
        }

        let mut lang_id = LanguageIdentifier::from((locale.language, locale.script, locale.region));
        let _ = self.extended_locale_expander()?.maximize(&mut lang_id);
        Ok(lang_id.region.unwrap())
    }

    /// Computes the likely script-based locale group for a given locale.
    ///
    /// Example:
    /// - "en-US" -> "en-Latn-US" -> "und-Latn" -> "en-Latn-US" -> "en"
    /// - "es-US" ->  "es-Latn-US" -> "und-Latn" -> "en-Latn-US" -> "en"
    /// - "fr-FR" -> "fr-Latn-FR" -> "und-Latn" -> "en-Latn-US" -> "en"
    /// - "ar-SA" -> "ar-Arab-SA" -> "und-Arab" -> "ar-Arab-EG" -> "ar"
    pub(crate) fn script_locale_group(&self, locale: &DataLocale) -> Result<DataLocale, DataError> {
        use icu::locale::subtags::Language;
        use icu::locale::LanguageIdentifier;

        if locale.to_string() == "nqo" {
            return DataLocale::try_from_str("nqo")
                .map_err(|e| DataError::custom("invalid locale: nqo").with_display_context(&e));
        }

        if locale.to_string() == "bm-Nkoo" {
            return DataLocale::try_from_str("bm").map_err(|e| {
                DataError::custom("invalid locale: bm-Nkoo").with_display_context(&e)
            });
        }

        let mut group = LanguageIdentifier::from((locale.language, locale.script, locale.region));

        // 1. Maximizes the input locale to get full language/script/region
        //    (e.g. "es-US" -> "es-Latn-US")
        self.extended_locale_expander()?.maximize(&mut group);

        // 2. Strips language and region, keeping only script
        //    (e.g. "es-Latn-US" -> "und-Latn")
        group.language = Language::UNKNOWN;
        group.region = Default::default();

        // 3. Maximizes again to find the most likely language for that script
        //    (e.g. "und-Latn" -> "en-Latn-US")
        self.extended_locale_expander()?.maximize(&mut group);

        // 4. Minimizes to keep just the language
        //    (e.g. "en-Latn-US" -> "en")
        self.extended_locale_expander()?
            .minimize_favor_script(&mut group);
        Ok(group.into())
    }
}

pub(crate) struct CldrDirNoLang<'a>(&'a CldrCache, String);

impl<'a> CldrDirNoLang<'a> {
    pub(crate) fn read_and_parse<S>(&self, file_name: &str) -> Result<&'a S, DataError>
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
    pub(crate) fn read_and_parse<S>(
        &self,
        locale: &DataLocale,
        file_name: &str,
    ) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        let dir_suffix = self.0.dir_suffix()?;
        let path = format!("{}-{dir_suffix}/main/{locale}/{file_name}", self.1);
        if self.0.serde_cache.file_exists(&path)? {
            self.0.serde_cache.read_and_parse_json(&path)
        } else if let Some(new_locale) = self.0.add_script_extended(locale)? {
            self.read_and_parse(&new_locale, file_name)
        } else {
            Err(DataErrorKind::Io(std::io::ErrorKind::NotFound)
                .into_error()
                .with_display_context(&path))
        }
    }

    pub(crate) fn list_locales(&self) -> Result<impl Iterator<Item = DataLocale> + '_, DataError> {
        let dir_suffix = self.0.dir_suffix()?;
        let path = format!("{}-{dir_suffix}/main", self.1);
        Ok(self
            .0
            .serde_cache
            .list(&path)?
            .map(|path| -> Result<DataLocale, DataError> {
                let locale = DataLocale::from_str(&path).unwrap();
                Ok(self.0.remove_script_extended(&locale)?.unwrap_or(locale))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter())
    }

    pub(crate) fn file_exists(
        &self,
        lang: &DataLocale,
        file_name: &str,
    ) -> Result<bool, DataError> {
        let dir_suffix = self.0.dir_suffix()?;
        let path = format!("{}-{dir_suffix}/main/{lang}/{file_name}", self.1);
        if self.0.serde_cache.file_exists(&path)? {
            Ok(true)
        } else if let Some(new_locale) = self.0.add_script_extended(lang)? {
            self.file_exists(&new_locale, file_name)
        } else {
            Ok(false)
        }
    }
}
