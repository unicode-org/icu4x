// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_locid_transform::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use tinystr::TinyAsciiStr;

impl DataProvider<LikelySubtagsV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LikelySubtagsV1Marker>, DataError> {
        // We treat searching for und as a request for all data. Other requests
        // are not currently supported.
        if !req.locale.is_empty() {
            return Err(DataErrorKind::ExtraneousLocale.into_error());
        }

        let data: &cldr_serde::likely_subtags::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("supplemental/likelySubtags.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(LikelySubtagsV1::from(data))),
        })
    }
}

impl IterableDataProvider<LikelySubtagsV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl From<&cldr_serde::likely_subtags::Resource> for LikelySubtagsV1<'static> {
    fn from(other: &cldr_serde::likely_subtags::Resource) -> Self {
        use icu_locid::subtags::Language;

        let mut language_script = BTreeMap::new();
        let mut language_region = BTreeMap::new();
        let mut language = BTreeMap::<TinyAsciiStr<3>, _>::new();
        let mut script_region = BTreeMap::new();
        let mut script = BTreeMap::<TinyAsciiStr<4>, _>::new();
        let mut region = BTreeMap::<TinyAsciiStr<3>, _>::new();
        let mut und = None;

        for entry in other.supplemental.likely_subtags.iter() {
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
                    with_diff!((Language::UND, None, Some(region)) => language_script.insert((lang.into(), script.into()), region));
                } else if let Some(region) = entry.0.region {
                    with_diff!((Language::UND, Some(script), None) => language_region.insert((lang.into(), region.into()), script));
                } else {
                    with_diff!((Language::UND, Some(script), Some(region)) => language.insert(lang.into(), (script, region)));
                }
            } else if let Some(scr) = entry.0.script {
                if let Some(region) = entry.0.region {
                    with_diff!((language, None, None) => script_region.insert((scr.into(), region.into()), language));
                } else {
                    with_diff!((language, None, Some(region)) => script.insert(scr.into(), (language, region)));
                }
            } else if let Some(reg) = entry.0.region {
                // Some of the target regions here are not equal to the source, such as und-002 -> en-Latn-NG.
                // However in the `maximize` method we do not replace tags, so we don't need to store the region.
                with_diff!((language, Some(script), _) => region.insert(reg.into(), (language, script)));
            } else {
                und = Some((
                    entry.1.language,
                    entry.1.script.expect("targets are complete language codes"),
                    entry.1.region.expect("targets are complete language codes"),
                ));
            }
        }

        Self {
            language_script: language_script.into_iter().collect(),
            language_region: language_region.into_iter().collect(),
            language: language.into_iter().collect(),
            script_region: script_region.into_iter().collect(),
            script: script.into_iter().collect(),
            region: region.into_iter().collect(),
            und: und.expect("'und' has a mapping"),
        }
    }
}

#[test]
fn test_basic() {
    use icu_locid::{
        subtags_language as language, subtags_region as region, subtags_script as script,
    };

    let provider = crate::DatagenProvider::for_test();
    let result: DataPayload<LikelySubtagsV1Marker> = provider
        .load(Default::default())
        .unwrap()
        .take_payload()
        .unwrap();

    let entry = result.get().script.get(&script!("Glag").into()).unwrap();
    assert_eq!(entry.0, language!("cu"));
    assert_eq!(entry.1, region!("BG"));
}
