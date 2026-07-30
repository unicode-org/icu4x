// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::CoverageLevel;
use crate::SourceDataProvider;
use crate::cldr_serde;
use icu::locale::LanguageIdentifier;
use icu::locale::provider::*;
use icu::locale::subtags::{Language, Region, Script, region, script};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};

impl DataProvider<LocaleLikelySubtagsExtendedV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LocaleLikelySubtagsExtendedV1>, DataError> {
        self.check_req::<LocaleLikelySubtagsExtendedV1>(req)?;
        LikelySubtagsResources::try_from_cldr_cache(self.cldr()?)?.load(req)
    }
}

impl crate::IterableDataProviderCached<LocaleLikelySubtagsExtendedV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<LocaleLikelySubtagsLanguageV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LocaleLikelySubtagsLanguageV1>, DataError> {
        self.check_req::<LocaleLikelySubtagsLanguageV1>(req)?;
        LikelySubtagsResources::try_from_cldr_cache(self.cldr()?)?.load(req)
    }
}

impl crate::IterableDataProviderCached<LocaleLikelySubtagsLanguageV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<LocaleLikelySubtagsScriptRegionV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LocaleLikelySubtagsScriptRegionV1>, DataError> {
        self.check_req::<LocaleLikelySubtagsScriptRegionV1>(req)?;
        LikelySubtagsResources::try_from_cldr_cache(self.cldr()?)?.load(req)
    }
}

impl crate::IterableDataProviderCached<LocaleLikelySubtagsScriptRegionV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl LikelySubtagsResources {
    // We need to store some und-S-R/und-S/und-R -> LSR expansion in `LikelySubtagsForLanguage::und`. For backward
    // compatibility, this is und-Latn-US.
    const UND_SR: (Script, Region) = (script!("Latn"), region!("US"));

    pub(crate) fn try_from_cldr_cache(
        cache: &super::super::CldrCache,
    ) -> Result<LikelySubtagsResources, DataError> {
        let likely_subtags: &cldr_serde::likely_subtags::Resource = cache
            .core()
            .read_and_parse("supplemental/likelySubtags.json")?;
        let core_languages = cache
            .locales([
                CoverageLevel::Basic,
                CoverageLevel::Moderate,
                CoverageLevel::Modern,
            ])?
            .into_iter()
            .map(|l| l.language)
            .filter(|l| !l.is_unknown())
            .collect();
        Ok(transform(
            likely_subtags.supplemental.likely_subtags.iter(),
            core_languages,
        ))
    }
}

pub(crate) struct LikelySubtagsResources {
    language_script: BTreeMap<(Language, Script), Region>,
    language_region: BTreeMap<(Language, Region), Script>,
    language: BTreeMap<Language, (Script, Region)>,
    script_region: BTreeMap<(Script, Region), Language>,
    script: BTreeMap<Script, (Language, Region)>,
    region: BTreeMap<Region, (Language, Script)>,
    und_l: Language,
    core_languages: HashSet<Language>,
}

impl DataProvider<LocaleLikelySubtagsLanguageV1> for LikelySubtagsResources {
    fn load(
        &self,
        _req: DataRequest,
    ) -> Result<DataResponse<LocaleLikelySubtagsLanguageV1>, DataError> {
        let langs = LikelySubtagsForLanguage {
            language_script: self
                .language_script
                .iter()
                .filter(|&(&(l, _), _)| self.core_languages.contains(&l))
                .map(|((k1, k2), v)| {
                    (
                        (
                            k1.to_tinystr().to_unvalidated(),
                            k2.to_tinystr().to_unvalidated(),
                        ),
                        v,
                    )
                })
                .collect(),
            language_region: self
                .language_region
                .iter()
                .filter(|&(&(l, _), _)| self.core_languages.contains(&l))
                .map(|((k1, k2), v)| {
                    (
                        (
                            k1.to_tinystr().to_unvalidated(),
                            k2.to_tinystr().to_unvalidated(),
                        ),
                        v,
                    )
                })
                .collect(),
            language: self
                .language
                .iter()
                .filter(|&(&l, _)| self.core_languages.contains(&l))
                .map(|(k, v)| (k.to_tinystr().to_unvalidated(), v))
                .collect(),
            und: (self.und_l, Self::UND_SR.0, Self::UND_SR.1),
        };

        Ok(DataResponse {
            payload: DataPayload::from_owned(langs),
            metadata: Default::default(),
        })
    }
}

impl DataProvider<LocaleLikelySubtagsScriptRegionV1> for LikelySubtagsResources {
    fn load(
        &self,
        _req: DataRequest,
    ) -> Result<DataResponse<LocaleLikelySubtagsScriptRegionV1>, DataError> {
        let script_region = LikelySubtagsForScriptRegion {
            script_region: self
                .script_region
                .iter()
                .filter(|&(_, &l)| self.core_languages.contains(&l))
                .map(|((k1, k2), v)| {
                    (
                        (
                            k1.to_tinystr().to_unvalidated(),
                            k2.to_tinystr().to_unvalidated(),
                        ),
                        v,
                    )
                })
                .collect(),
            script: self
                .script
                .iter()
                .filter(|&(_, &(l, _))| self.core_languages.contains(&l))
                .map(|(k, v)| (k.to_tinystr().to_unvalidated(), v))
                .collect(),
            region: self
                .region
                .iter()
                .filter(|&(_, &(l, _))| self.core_languages.contains(&l))
                .map(|(k, v)| (k.to_tinystr().to_unvalidated(), v))
                .collect(),
        };

        Ok(DataResponse {
            payload: DataPayload::from_owned(script_region),
            metadata: Default::default(),
        })
    }
}

impl DataProvider<LocaleLikelySubtagsExtendedV1> for LikelySubtagsResources {
    fn load(
        &self,
        _req: DataRequest,
    ) -> Result<DataResponse<LocaleLikelySubtagsExtendedV1>, DataError> {
        let extended = LikelySubtagsExtended {
            language_script: self
                .language_script
                .iter()
                .filter(|&(&(l, _), _)| !self.core_languages.contains(&l))
                .map(|((k1, k2), v)| {
                    (
                        (
                            k1.to_tinystr().to_unvalidated(),
                            k2.to_tinystr().to_unvalidated(),
                        ),
                        v,
                    )
                })
                .collect(),
            language_region: self
                .language_region
                .iter()
                .filter(|&(&(l, _), _)| !self.core_languages.contains(&l))
                .map(|((k1, k2), v)| {
                    (
                        (
                            k1.to_tinystr().to_unvalidated(),
                            k2.to_tinystr().to_unvalidated(),
                        ),
                        v,
                    )
                })
                .collect(),
            language: self
                .language
                .iter()
                .filter(|&(&l, _)| !self.core_languages.contains(&l))
                .map(|(k, v)| (k.to_tinystr().to_unvalidated(), v))
                .collect(),
            script_region: self
                .script_region
                .iter()
                .filter(|&(_, &l)| !self.core_languages.contains(&l))
                .map(|((k1, k2), v)| {
                    (
                        (
                            k1.to_tinystr().to_unvalidated(),
                            k2.to_tinystr().to_unvalidated(),
                        ),
                        v,
                    )
                })
                .collect(),
            script: self
                .script
                .iter()
                .filter(|&(_, &(l, _))| !self.core_languages.contains(&l))
                .map(|(k, v)| (k.to_tinystr().to_unvalidated(), v))
                .collect(),
            region: self
                .region
                .iter()
                .filter(|&(_, &(l, _))| !self.core_languages.contains(&l))
                .map(|(k, v)| (k.to_tinystr().to_unvalidated(), v))
                .collect(),
        };

        Ok(DataResponse {
            payload: DataPayload::from_owned(extended),
            metadata: Default::default(),
        })
    }
}

pub(crate) fn transform<'x>(
    it: impl Iterator<Item = (&'x LanguageIdentifier, &'x LanguageIdentifier)> + 'x,
    core_languages: HashSet<Language>,
) -> LikelySubtagsResources {
    let mut language_script = BTreeMap::new();
    let mut language_region = BTreeMap::new();
    let mut language = BTreeMap::new();
    let mut script_region = BTreeMap::new();
    let mut script = BTreeMap::new();
    let mut region = BTreeMap::new();

    for entry in it {
        // Computes the delta of the entry and assigns to the pattern.
        // Errors if the delta is not assignable to the pattern.
        macro_rules! with_diff {
            ($pat:pat => $stmt:expr ) => {
                if let $pat = (
                    if entry.0.language != entry.1.language {
                        entry.1.language
                    } else {
                        Language::UNKNOWN
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
                    log::error!(
                        "The expansion {:?} -> {:?} can not be stored in the pattern {}, skipping",
                        entry.0,
                        entry.1,
                        stringify!($pat)
                    );
                    continue;
                }
            };
        }

        if !entry.0.language.is_unknown() {
            let l = entry.0.language;
            if let Some(s) = entry.0.script {
                with_diff!((Language::UNKNOWN, None, Some(r)) => language_script.insert((l, s), r));
            } else if let Some(r) = entry.0.region {
                with_diff!((Language::UNKNOWN, Some(s), None) => language_region.insert((l, r), s));
            } else {
                with_diff!((Language::UNKNOWN, Some(s), Some(r)) => language.insert(l, (s, r)));
            }
        } else if let Some(s) = entry.0.script {
            if let Some(r) = entry.0.region {
                with_diff!((l, None, None) => script_region.insert((s, r), l));
            } else {
                with_diff!((l, None, Some(r)) => script.insert(s, (l, r)));
            }
        } else if let Some(r) = entry.0.region {
            with_diff!((l, Some(s), None) => region.insert(r, (l, s)));
        } else {
            // Treat the und->LSR expansion as und-S-R->LSR, und-S->LSR, and und-R->LSR. CLDR 50+
            // will contain these expansions (https://unicode-org.atlassian.net/browse/CLDR-14524).
            with_diff!((l, Some(s), Some(r)) => {
                script_region.insert((s, r), l);
                script.insert(s, (l, r));
                region.insert(r, (l, s));

            });
        }
    }

    let und_l = script_region
        .remove(&LikelySubtagsResources::UND_SR)
        .unwrap();
    let und_s = LikelySubtagsResources::UND_SR.0;
    let und_r = LikelySubtagsResources::UND_SR.1;

    let ls = region.remove(&und_r);
    if ls != Some((und_l, und_s)) {
        log::warn!(
            "cannot store in und: und-{und_s}-{und_r} -> {und_l}-{und_s}-{und_r}, but {}",
            if let Some((l, s)) = ls {
                format!("und-{und_r} -> {l}-{s}-{und_r}")
            } else {
                format!("no mapping for und-{und_r}")
            }
        );
    }

    let lr = script.remove(&und_s);
    if lr != Some((und_l, und_r)) {
        log::warn!(
            "cannot store in und: und-{und_s}-{und_r} -> {und_l}-{und_s}-{und_r}, but {}",
            if let Some((l, r)) = lr {
                format!("und-{und_s} -> {l}-{und_s}-{r}")
            } else {
                format!("no mapping for und-{und_s}")
            }
        );
    }

    LikelySubtagsResources {
        language_script,
        language_region,
        language,
        script_region,
        script,
        region,
        und_l,
        core_languages,
    }
}

#[test]
fn test_basic() {
    use icu::locale::subtags::{language, region, script};

    let provider = SourceDataProvider::new_testing();
    let result_common_sr: DataResponse<LocaleLikelySubtagsScriptRegionV1> =
        provider.load(Default::default()).unwrap();
    let result_extended: DataResponse<LocaleLikelySubtagsExtendedV1> =
        provider.load(Default::default()).unwrap();

    let entry = result_common_sr
        .payload
        .get()
        .script
        .get_copied(&script!("Hant").to_tinystr().to_unvalidated())
        .unwrap();
    assert_eq!(entry.0, language!("zh"));
    assert_eq!(entry.1, region!("TW"));

    let entry = result_extended
        .payload
        .get()
        .script
        .get_copied(&script!("Glag").to_tinystr().to_unvalidated())
        .unwrap();
    assert_eq!(entry.0, language!("cu"));
    assert_eq!(entry.1, region!("BG"));
}

#[test]
fn test_exhaustive() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();
    let expander = icu::locale::LocaleExpander::try_new_extended_unstable(&provider).unwrap();

    for (source, mut expected) in provider
        .cldr()
        .unwrap()
        .core()
        .read_and_parse::<cldr_serde::likely_subtags::Resource>("supplemental/likelySubtags.json")
        .unwrap()
        .supplemental
        .likely_subtags
        .clone()
        .into_iter()
        .chain([
            (langid!("tlh"), langid!("tlh")),
            (langid!("tlh-Arab"), langid!("tlh-Arab")),
            (langid!("tlh-Latn"), langid!("tlh-Latn")),
            (langid!("tlh-US"), langid!("tlh-US")),
            (langid!("tlh-SA"), langid!("tlh-SA")),
        ])
    {
        if source.is_unknown() {
            expected = LanguageIdentifier::UNKNOWN;
        }

        let mut actual = source.clone();
        let r = expander.maximize(&mut actual);
        assert_eq!(
            r,
            if source != expected {
                icu::locale::TransformResult::Modified
            } else {
                icu::locale::TransformResult::Unmodified
            }
        );
        assert_eq!(actual, expected);

        assert_eq!(expander.get_likely_script(&source), expected.script);
    }
}
