// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::error::Error;
use crate::reader::open_reader;
use crate::support::KeyedDataProvider;
use crate::CldrPaths;
use icu_locale_canonicalizer::provider::*;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;

use std::convert::TryFrom;
use tinystr::TinyStr4;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [key::LIKELY_SUBTAGS_V1];

/// A data provider reading from CLDR JSON likely subtags rule files.
#[derive(PartialEq, Debug)]
pub struct LikelySubtagsProvider {
    data: cldr_serde::likely_subtags::Resource,
}

impl TryFrom<&dyn CldrPaths> for LikelySubtagsProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let data: cldr_serde::likely_subtags::Resource = {
            let path = cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("likelySubtags.json");
            serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?
        };
        Ok(Self { data })
    }
}

impl KeyedDataProvider for LikelySubtagsProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        key::LIKELY_SUBTAGS_V1.match_key(*resc_key)
    }
}

impl DataProvider<LikelySubtagsV1Marker> for LikelySubtagsProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<LikelySubtagsV1Marker>, DataError> {
        LikelySubtagsProvider::supports_key(&req.resource_path.key)?;
        let langid = &req.resource_path.options.langid;

        // We treat searching for und as a request for all data. Other requests
        // are not currently supported.
        if langid.is_none() {
            let metadata = DataResponseMetadata::default();
            // TODO(#1109): Set metadata.data_langid correctly.
            Ok(DataResponse {
                metadata,
                payload: Some(DataPayload::from_owned(LikelySubtagsV1::from(&self.data))),
            })
        } else {
            Err(DataErrorKind::ExtraneousResourceOptions.with_req(req))
        }
    }
}

icu_provider::impl_dyn_provider!(LikelySubtagsProvider, {
    _ => LikelySubtagsV1Marker,
}, SERDE_SE);

impl IterableProvider for LikelySubtagsProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}

impl From<&cldr_serde::likely_subtags::Resource> for LikelySubtagsV1 {
    fn from(other: &cldr_serde::likely_subtags::Resource) -> Self {
        use icu_locid::LanguageIdentifier;

        let mut language_script: LiteMap<(TinyStr4, TinyStr4), LanguageIdentifier> = LiteMap::new();
        let mut language_region: LiteMap<(TinyStr4, TinyStr4), LanguageIdentifier> = LiteMap::new();
        let mut language: LiteMap<TinyStr4, LanguageIdentifier> = LiteMap::new();
        let mut script_region: LiteMap<(TinyStr4, TinyStr4), LanguageIdentifier> = LiteMap::new();
        let mut script: LiteMap<TinyStr4, LanguageIdentifier> = LiteMap::new();
        let mut region: LiteMap<TinyStr4, LanguageIdentifier> = LiteMap::new();
        let mut und = LanguageIdentifier::default();

        // Create a result LanguageIdentifier. We only need to store the delta
        // between the search LanguageIdentifier and the result LanguageIdentifier.
        let extract_result =
            |entry: &(LanguageIdentifier, LanguageIdentifier)| -> LanguageIdentifier {
                LanguageIdentifier {
                    language: if entry.0.language != entry.1.language {
                        entry.1.language
                    } else {
                        icu_locid::subtags::Language::und()
                    },
                    script: if entry.0.script != entry.1.script {
                        entry.1.script
                    } else {
                        None
                    },
                    region: if entry.0.region != entry.1.region {
                        entry.1.region
                    } else {
                        None
                    },
                    variants: icu_locid::subtags::Variants::default(),
                }
            };

        for entry in other.supplemental.likely_subtags.iter() {
            if let Some(lang) = entry.0.language.into() {
                if let Some(script) = entry.0.script {
                    language_script.insert((lang, script.into()), extract_result(entry));
                } else if let Some(region) = entry.0.region {
                    language_region.insert((lang, region.into()), extract_result(entry));
                } else {
                    language.insert(lang, extract_result(entry));
                }
            } else if let Some(scr) = entry.0.script {
                if let Some(reg) = entry.0.region {
                    script_region.insert((scr.into(), reg.into()), extract_result(entry));
                } else {
                    script.insert(scr.into(), extract_result(entry));
                }
            } else if let Some(reg) = entry.0.region {
                region.insert(reg.into(), extract_result(entry));
            } else {
                und = entry.1.clone();
            }
        }

        Self {
            language_script,
            language_region,
            language,
            script_region,
            script,
            region,
            und,
        }
    }
}

#[test]
fn test_basic() {
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = LikelySubtagsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();
    let result: DataPayload<LikelySubtagsV1Marker> = provider
        .load_payload(&DataRequest::from(key::LIKELY_SUBTAGS_V1))
        .unwrap()
        .take_payload()
        .unwrap();

    let langid = langid!("cu-Glag");
    let entry = result
        .get()
        .script
        .get(&(langid.script.unwrap().into()))
        .unwrap();
    assert_eq!(entry.language, "cu");
    assert_eq!(entry.region.unwrap(), "BG");
}
