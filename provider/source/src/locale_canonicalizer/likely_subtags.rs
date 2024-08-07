// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::CoverageLevel;
use crate::SourceDataProvider;
use icu::locale::provider::*;
use icu::locale::subtags::{Language, Region, Script};
use icu::locale::LanguageIdentifier;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use tinystr::TinyAsciiStr;

impl DataProvider<LikelySubtagsExtendedV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LikelySubtagsExtendedV1Marker>, DataError> {
        self.check_req::<LikelySubtagsExtendedV1Marker>(req)?;
        let resources = LikelySubtagsResources::try_from_cldr_cache(self.cldr()?)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(transform(resources.get_extended()).as_extended()),
        })
    }
}

impl crate::IterableDataProviderCached<LikelySubtagsExtendedV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<LikelySubtagsForLanguageV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LikelySubtagsForLanguageV1Marker>, DataError> {
        self.check_req::<LikelySubtagsForLanguageV1Marker>(req)?;
        let resources = LikelySubtagsResources::try_from_cldr_cache(self.cldr()?)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(transform(resources.get_common()).as_langs()),
        })
    }
}

impl crate::IterableDataProviderCached<LikelySubtagsForLanguageV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<LikelySubtagsForScriptRegionV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LikelySubtagsForScriptRegionV1Marker>, DataError> {
        self.check_req::<LikelySubtagsForScriptRegionV1Marker>(req)?;
        let resources = LikelySubtagsResources::try_from_cldr_cache(self.cldr()?)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(transform(resources.get_common()).as_script_region()),
        })
    }
}

impl crate::IterableDataProviderCached<LikelySubtagsForScriptRegionV1Marker>
    for SourceDataProvider
{
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

pub(crate) struct LikelySubtagsResources<'a> {
    likely_subtags: &'a cldr_serde::likely_subtags::Resource,
    basic_plus_languages: HashSet<Language>,
}

impl<'a> LikelySubtagsResources<'a> {
    pub(crate) fn try_from_cldr_cache(
        cache: &'a super::super::CldrCache,
    ) -> Result<LikelySubtagsResources, DataError> {
        let likely_subtags: &cldr_serde::likely_subtags::Resource = cache
            .core()
            .read_and_parse("supplemental/likelySubtags.json")?;
        let coverage_levels: &cldr_serde::coverage_levels::Resource =
            cache.core().read_and_parse("coverageLevels.json")?;
        let basic_plus_languages = Self::get_basic_plus_languages(coverage_levels);
        Ok(Self {
            likely_subtags,
            basic_plus_languages,
        })
    }

    fn get_basic_plus_languages(
        coverage_levels: &cldr_serde::coverage_levels::Resource,
    ) -> HashSet<Language> {
        #[allow(clippy::unnecessary_filter_map)] // better for future refactoring
        coverage_levels
            .coverage_levels
            .iter()
            .filter_map(|(langid, level)| {
                match level {
                    // NOTE: If more coverage levels get added, ones below Basic should be filtered here
                    CoverageLevel::Basic | CoverageLevel::Moderate | CoverageLevel::Modern => {
                        Some(langid.language)
                    }
                }
            })
            .collect()
    }

    fn common_predicate(&self, min_max: &(&LanguageIdentifier, &LanguageIdentifier)) -> bool {
        let (minimized, maximized) = min_max;
        self.basic_plus_languages.contains(&maximized.language)
            || **minimized == LanguageIdentifier::UND
    }

    pub(crate) fn get_common(
        &self,
    ) -> impl Iterator<Item = (&LanguageIdentifier, &LanguageIdentifier)> + '_ {
        self.likely_subtags
            .supplemental
            .likely_subtags
            .iter()
            .filter(|min_max| self.common_predicate(min_max))
    }

    pub(crate) fn get_extended(
        &self,
    ) -> impl Iterator<Item = (&LanguageIdentifier, &LanguageIdentifier)> + '_ {
        self.likely_subtags
            .supplemental
            .likely_subtags
            .iter()
            .filter(|min_max| !self.common_predicate(min_max))
    }
}

#[derive(Default)]
pub(crate) struct TransformResult {
    language_script: BTreeMap<(TinyAsciiStr<3>, TinyAsciiStr<4>), Region>,
    language_region: BTreeMap<(TinyAsciiStr<3>, TinyAsciiStr<3>), Script>,
    language: BTreeMap<TinyAsciiStr<3>, (Script, Region)>,
    script_region: BTreeMap<(TinyAsciiStr<4>, TinyAsciiStr<3>), Language>,
    script: BTreeMap<TinyAsciiStr<4>, (Language, Region)>,
    region: BTreeMap<TinyAsciiStr<3>, (Language, Script)>,
    und: Option<(Language, Script, Region)>,
}

impl TransformResult {
    pub(crate) fn as_langs(&self) -> LikelySubtagsForLanguageV1<'static> {
        LikelySubtagsForLanguageV1 {
            language_script: self
                .language_script
                .iter()
                .map(|((k1, k2), v)| ((k1.to_unvalidated(), k2.to_unvalidated()), v))
                .collect(),
            language_region: self
                .language_region
                .iter()
                .map(|((k1, k2), v)| ((k1.to_unvalidated(), k2.to_unvalidated()), v))
                .collect(),
            language: self
                .language
                .iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            und: self.und.unwrap_or((
                icu::locale::subtags::language!("und"),
                icu::locale::subtags::script!("Zzzz"),
                icu::locale::subtags::region!("ZZ"),
            )),
        }
    }

    pub(crate) fn as_script_region(&self) -> LikelySubtagsForScriptRegionV1<'static> {
        LikelySubtagsForScriptRegionV1 {
            script_region: self
                .script_region
                .iter()
                .map(|((k1, k2), v)| ((k1.to_unvalidated(), k2.to_unvalidated()), v))
                .collect(),
            script: self
                .script
                .iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            region: self
                .region
                .iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
        }
    }

    pub(crate) fn as_extended(&self) -> LikelySubtagsExtendedV1<'static> {
        LikelySubtagsExtendedV1 {
            language_script: self
                .language_script
                .iter()
                .map(|((k1, k2), v)| ((k1.to_unvalidated(), k2.to_unvalidated()), v))
                .collect(),
            language_region: self
                .language_region
                .iter()
                .map(|((k1, k2), v)| ((k1.to_unvalidated(), k2.to_unvalidated()), v))
                .collect(),
            language: self
                .language
                .iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            script_region: self
                .script_region
                .iter()
                .map(|((k1, k2), v)| ((k1.to_unvalidated(), k2.to_unvalidated()), v))
                .collect(),
            script: self
                .script
                .iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            region: self
                .region
                .iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
        }
    }
}

pub(crate) fn transform<'x>(
    it: impl Iterator<Item = (&'x LanguageIdentifier, &'x LanguageIdentifier)> + 'x,
) -> TransformResult {
    let mut language_script = BTreeMap::new();
    let mut language_region = BTreeMap::new();
    let mut language = BTreeMap::new();
    let mut script_region = BTreeMap::new();
    let mut script = BTreeMap::new();
    let mut region = BTreeMap::new();
    let mut und = None;

    for entry in it {
        // Computes the delta of the entry and assigns to the pattern.
        // Errors if the delta is not assignable to the pattern.
        macro_rules! with_diff {
            ($pat:pat => $stmt:expr ) => {
                if let $pat = (
                    if entry.0.language != entry.1.language {
                        entry.1.language
                    } else {
                        Language::UND
                    },
                    if entry.0.script != entry.1.script {
                        entry.1.script
                    } else {
                        None
                    },
                    if entry.0.region != entry.1.region {
                        entry.1.region
                    } else {
                        None
                    },
                ) {
                    $stmt;
                } else {
                    panic!(
                        "The expansion {:?} -> {:?} can not be stored in the pattern {}",
                        entry.0,
                        entry.1,
                        stringify!($pat)
                    );
                }
            };
        }

        if !entry.0.language.is_empty() {
            let lang = entry.0.language;
            if let Some(script) = entry.0.script {
                with_diff!((Language::UND, None, Some(region)) => language_script.insert((lang.into_tinystr(), script.into_tinystr()), region));
            } else if let Some(region) = entry.0.region {
                with_diff!((Language::UND, Some(script), None) => language_region.insert((lang.into_tinystr(), region.into_tinystr()), script));
            } else {
                with_diff!((Language::UND, Some(script), Some(region)) => language.insert(lang.into_tinystr(), (script, region)));
            }
        } else if let Some(scr) = entry.0.script {
            if let Some(region) = entry.0.region {
                with_diff!((language, None, None) => script_region.insert((scr.into_tinystr(), region.into_tinystr()), language));
            } else {
                with_diff!((language, None, Some(region)) => script.insert(scr.into_tinystr(), (language, region)));
            }
        } else if let Some(reg) = entry.0.region {
            // Some of the target regions here are not equal to the source, such as und-002 -> en-Latn-NG.
            // However in the `maximize` method we do not replace tags, so we don't need to store the region.
            with_diff!((language, Some(script), _) => region.insert(reg.into_tinystr(), (language, script)));
        } else {
            und = Some((
                entry.1.language,
                entry.1.script.expect("targets are complete language codes"),
                entry.1.region.expect("targets are complete language codes"),
            ));
        }
    }

    TransformResult {
        language_script,
        language_region,
        language,
        script_region,
        script,
        region,
        und,
    }
}

#[test]
fn test_basic() {
    use icu::locale::subtags::{language, region, script};

    let provider = SourceDataProvider::new_testing();
    let result_common_sr: DataResponse<LikelySubtagsForScriptRegionV1Marker> =
        provider.load(Default::default()).unwrap();
    let result_extended: DataResponse<LikelySubtagsExtendedV1Marker> =
        provider.load(Default::default()).unwrap();

    let entry = result_common_sr
        .payload
        .get()
        .script
        .get_copied(&script!("Hant").into_tinystr().to_unvalidated())
        .unwrap();
    assert_eq!(entry.0, language!("zh"));
    assert_eq!(entry.1, region!("TW"));

    let entry = result_extended
        .payload
        .get()
        .script
        .get_copied(&script!("Glag").into_tinystr().to_unvalidated())
        .unwrap();
    assert_eq!(entry.0, language!("cu"));
    assert_eq!(entry.1, region!("BG"));
}
