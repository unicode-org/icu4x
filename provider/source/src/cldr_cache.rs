// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(dead_code)] // features

use crate::CoverageLevel;
use crate::cldr_serde::eras::EraData;
use crate::datetime::DatagenCalendar;
use crate::source::{AbstractFs, SerdeCache};
use icu::locale::LanguageIdentifier;
use icu::locale::LocaleExpander;
use icu::locale::subtags::Language;
#[cfg(feature = "unstable")]
use icu::locale::subtags::Region;
use icu_provider::DataError;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::OnceLock;
use writeable::Writeable;

#[derive(Debug)]
pub(crate) struct CldrCache {
    pub(crate) serde_cache: SerdeCache,
    extended_locale_expander: OnceLock<Result<LocaleExpander, DataError>>,
    #[expect(clippy::type_complexity)]
    pub(crate) calendar_eras: OnceLock<
        Result<
            BTreeMap<DatagenCalendar, (Option<DatagenCalendar>, Vec<(usize, EraData)>)>,
            DataError,
        >,
    >,
    #[cfg(feature = "unstable")]
    // used by transforms/mod.rs
    pub(crate) transforms: OnceLock<
        Result<std::sync::Mutex<icu::experimental::transliterate::RuleCollection>, DataError>,
    >,
    pub(crate) tz_caches: crate::time_zones::Caches,
}

impl CldrCache {
    pub(crate) fn new(root: AbstractFs) -> Self {
        CldrCache {
            serde_cache: SerdeCache::new(root),
            extended_locale_expander: Default::default(),
            calendar_eras: Default::default(),
            #[cfg(feature = "unstable")]
            transforms: Default::default(),
            tz_caches: Default::default(),
        }
    }

    pub(crate) fn core(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-core")
    }

    pub(crate) fn numbers(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-numbers-full/main")
    }

    pub(crate) fn misc(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-misc-full/main")
    }

    pub(crate) fn bcp47(&self) -> CldrDirNoLang<'_> {
        CldrDirNoLang(self, "cldr-bcp47/bcp47")
    }

    pub(crate) fn personnames(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-person-names-full/main")
    }

    pub(crate) fn displaynames(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-localenames-full/main")
    }

    pub(crate) fn units(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-units-full/main")
    }

    pub(crate) fn segments(&self) -> CldrDirLang<'_> {
        CldrDirLang(self, "cldr-segments-full/segments")
    }

    pub(crate) fn dates(&self, cal: Option<DatagenCalendar>) -> CldrDirLang<'_> {
        CldrDirLang(
            self,
            match cal {
                Some(DatagenCalendar::Buddhist) => "cldr-cal-buddhist-full/main",
                Some(DatagenCalendar::Chinese) => "cldr-cal-chinese-full/main",
                Some(DatagenCalendar::Coptic) => "cldr-cal-coptic-full/main",
                Some(DatagenCalendar::Dangi) => "cldr-cal-dangi-full/main",
                Some(DatagenCalendar::Ethiopic) => "cldr-cal-ethiopic-full/main",
                Some(DatagenCalendar::Hebrew) => "cldr-cal-hebrew-full/main",
                Some(DatagenCalendar::Indian) => "cldr-cal-indian-full/main",
                Some(DatagenCalendar::Hijri) => "cldr-cal-islamic-full/main",
                Some(DatagenCalendar::Japanese) => "cldr-cal-japanese-full/main",
                Some(DatagenCalendar::Persian) => "cldr-cal-persian-full/main",
                Some(DatagenCalendar::Roc) => "cldr-cal-roc-full/main",
                Some(DatagenCalendar::Gregorian) | None => "cldr-dates-full/main",
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

    pub(crate) fn extended_locale_expander(&self) -> Result<&LocaleExpander, DataError> {
        use super::locale::likely_subtags::*;
        self.extended_locale_expander
            .get_or_init(|| {
                LocaleExpander::try_new_extended_unstable(
                    &LikelySubtagsResources::try_from_cldr_cache(self)?,
                )
                .map_err(|e| {
                    DataError::custom("creating LocaleExpander in CldrCache")
                        .with_display_context(&e)
                })
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
            LanguageIdentifier::from((locale.language, locale.script, locale.region));
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
        let mut langid = LanguageIdentifier::from((locale.language, locale.script, locale.region));
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
    ///  - "ar" -> "EG"
    ///  - "und" -> "001"
    #[cfg(feature = "unstable")]
    pub(crate) fn extract_or_infer_region(&self, locale: &DataLocale) -> Result<Region, DataError> {
        if let Some(region) = locale.region {
            return Ok(region);
        }

        let mut lang_id = LanguageIdentifier::from((locale.language, locale.script, locale.region));
        let _ = self.extended_locale_expander()?.maximize(&mut lang_id);
        Ok(lang_id
            .region
            .unwrap_or(icu::locale::subtags::region!("001")))
    }

    /// Computes the script-based locale group for a given locale.
    ///
    /// This finds the most likely language for the locale's script, then minimizes it
    /// (keeping the script if it's not the default for that language).
    ///
    /// Example:
    /// - "en-US" -> "en-Latn-US" -> "und-Latn" -> "en-Latn-US" -> "en"
    /// - "es-US" ->  "es-Latn-US" -> "und-Latn" -> "en-Latn-US" -> "en"
    /// - "fr-FR" -> "fr-Latn-FR" -> "und-Latn" -> "en-Latn-US" -> "en"
    /// - "ar-SA" -> "ar-Arab-SA" -> "und-Arab" -> "ar-Arab-EG" -> "ar"
    /// - "bm-Nkoo" -> "bm-Nkoo-ML" -> "und-Nkoo" -> "man-Nkoo-GN" -> "man-Nkoo"
    /// - "nqo" -> "nqo-Nkoo-GN" -> "und-Nkoo" -> "man-Nkoo-GN" -> "man-Nkoo"
    /// - "und-Latn" -> "en-Latn-US" -> "en"
    /// - "und-Arab" -> "ar-Arab-EG" -> "ar"
    /// - "und-US" -> "en-Latn-US" -> "en"
    /// - "und" -> "und"
    pub(crate) fn script_based_locale_group(
        &self,
        locale: &DataLocale,
    ) -> Result<DataLocale, DataError> {
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
        //    (e.g. "und-Nkoo" -> "man-Nkoo-GN")
        self.extended_locale_expander()?.maximize(&mut group);

        // 4. Minimizes the locale, keeping the script if it's not the default for the language
        //    (e.g. "en-Latn-US" -> "en")
        //    (e.g. "man-Nkoo-GN" -> "man-Nkoo")
        self.extended_locale_expander()?
            .minimize_favor_script(&mut group);
        Ok(group.into())
    }
}

pub(crate) struct CldrDirNoLang<'a>(&'a CldrCache, &'static str);

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

pub(crate) struct CldrDirLang<'a>(&'a CldrCache, &'static str);

impl<'a> CldrDirLang<'a> {
    pub(crate) fn read_and_parse<S>(
        &self,
        locale: &DataLocale,
        file_name: &str,
    ) -> Result<&'a S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        let path = format!("{}/{locale}/{file_name}", self.1);
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
        Ok(self
            .0
            .serde_cache
            .list(self.1)?
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
        let path = format!("{}/{lang}/{file_name}", self.1);
        if self.0.serde_cache.file_exists(&path)? {
            Ok(true)
        } else if let Some(new_locale) = self.0.add_script_extended(lang)? {
            self.file_exists(&new_locale, file_name)
        } else {
            Ok(false)
        }
    }
}

#[test]
fn test_script_based_locale_group() {
    use crate::SourceDataProvider;

    let provider = SourceDataProvider::new_testing();
    let cldr = provider.cldr().unwrap();

    // Test cases from the documentation
    // "en-US" -> "en"
    let en_us = DataLocale::from_str("en-US").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&en_us).unwrap().to_string(),
        "en"
    );

    // "es-US" -> "en" (Spanish uses Latin script, English is most common Latin-script language)
    let es_us = DataLocale::from_str("es-US").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&es_us).unwrap().to_string(),
        "en"
    );

    // "fr-FR" -> "en"
    let fr_fr = DataLocale::from_str("fr-FR").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&fr_fr).unwrap().to_string(),
        "en"
    );

    // "ar-SA" -> "ar" (Arabic uses Arabic script)
    let ar_sa = DataLocale::from_str("ar-SA").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&ar_sa).unwrap().to_string(),
        "ar"
    );

    // "nqo" -> "man-Nkoo" (N'Ko language uses N'Ko script, most likely language for N'Ko is Mandingo,
    // but N'Ko is not Mandingo's default script so it's kept)
    let nqo = DataLocale::from_str("nqo").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&nqo).unwrap().to_string(),
        "man-Nkoo"
    );

    // "bm-Nkoo" -> "man-Nkoo" (Bambara in N'Ko script -> Mandingo is most likely for N'Ko script,
    // but N'Ko is not Mandingo's default script so it's kept)
    let bm_nkoo = DataLocale::from_str("bm-Nkoo").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&bm_nkoo)
            .unwrap()
            .to_string(),
        "man-Nkoo"
    );

    // "man" -> "en" (Mandingo's default script is Latin, Latin's most likely language is English)
    let man = DataLocale::from_str("man").unwrap();
    assert_eq!(
        cldr.script_based_locale_group(&man).unwrap().to_string(),
        "en"
    );
}
