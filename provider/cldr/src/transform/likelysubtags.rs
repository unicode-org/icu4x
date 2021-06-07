// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::reader::open_reader;
use crate::CldrPaths;
use icu_locale_canonicalizer::provider::*;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;

use std::convert::TryFrom;
use std::marker::PhantomData;
use tinystr::TinyStr4;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [key::LIKELY_SUBTAGS_V1];

/// A data provider reading from CLDR JSON likely subtags rule files.
#[derive(PartialEq, Debug)]
pub struct LikelySubtagsProvider<'d> {
    data: cldr_json::Resource,
    _phantom: PhantomData<&'d ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for LikelySubtagsProvider<'_> {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let data: cldr_json::Resource = {
            let path = cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("likelySubtags.json");
            serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?
        };
        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> KeyedDataProvider for LikelySubtagsProvider<'d> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::LikelySubtags || resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'d, 's> DataProvider<'d, 's, LikelySubtagsV1Marker> for LikelySubtagsProvider<'d> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, 's, LikelySubtagsV1Marker>, DataError> {
        LikelySubtagsProvider::supports_key(&req.resource_path.key)?;
        let langid = &req.resource_path.options.langid;

        // We treat searching for und as a request for all data. Other requests
        // are not currently supported.
        if langid.is_none() {
            Ok(DataResponse {
                metadata: DataResponseMetadata {
                    data_langid: langid.clone(),
                },
                payload: Some(DataPayload::from_owned(LikelySubtagsV1::from(&self.data))),
            })
        } else {
            Err(DataError::UnavailableResourceOptions(req.clone()))
        }
    }
}

icu_provider::impl_dyn_provider!(LikelySubtagsProvider<'d>, {
    _ => LikelySubtagsV1Marker,
}, SERDE_SE, 'd, 's);

impl<'d> IterableDataProviderCore for LikelySubtagsProvider<'d> {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}

impl From<&cldr_json::Resource> for LikelySubtagsV1 {
    fn from(other: &cldr_json::Resource) -> Self {
        use icu_locid::LanguageIdentifier;

        let mut language_script: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)> = Vec::new();
        let mut language_region: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)> = Vec::new();
        let mut language: Vec<(TinyStr4, LanguageIdentifier)> = Vec::new();
        let mut script_region: Vec<(TinyStr4, TinyStr4, LanguageIdentifier)> = Vec::new();
        let mut script: Vec<(TinyStr4, LanguageIdentifier)> = Vec::new();
        let mut region: Vec<(TinyStr4, LanguageIdentifier)> = Vec::new();
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
                    language_script.push((lang, script.into(), extract_result(entry)));
                } else if let Some(region) = entry.0.region {
                    language_region.push((lang, region.into(), extract_result(entry)));
                } else {
                    language.push((lang, extract_result(entry)));
                }
            } else if let Some(scr) = entry.0.script {
                if let Some(reg) = entry.0.region {
                    script_region.push((scr.into(), reg.into(), extract_result(entry)));
                } else {
                    script.push((scr.into(), extract_result(entry)));
                }
            } else if let Some(reg) = entry.0.region {
                region.push((reg.into(), extract_result(entry)));
            } else {
                und = entry.1.clone();
            }
        }

        // We sort here to ensure that they are sorted properly by the subtags
        // we will use to search the data. This is not necessary the order in
        // the underlying CLDR data.
        language_script.sort_unstable_by_key(|k| (k.0, k.1));
        language_region.sort_unstable_by_key(|k| (k.0, k.1));
        language.sort_unstable_by_key(|k| k.0);
        script_region.sort_unstable_by_key(|k| (k.0, k.1));
        script.sort_unstable_by_key(|k| k.0);
        region.sort_unstable_by_key(|k| k.0);
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

/// Serde structs for the CLDR JSON likely subtags file.
pub(self) mod cldr_json {
    use icu_locid::LanguageIdentifier;
    use serde::Deserialize;

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Supplemental {
        #[serde(with = "tuple_vec_map", rename = "likelySubtags")]
        pub likely_subtags: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub supplemental: Supplemental,
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
        .binary_search_by_key(&(langid.script.map(|s| s.into())), |(script, _)| {
            Some(*script)
        })
        .unwrap();
    assert_eq!(result.get().script[entry].1.language, "cu");
    assert_eq!(result.get().script[entry].1.region.unwrap(), "BG");
}
