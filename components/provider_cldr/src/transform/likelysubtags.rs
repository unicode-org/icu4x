// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::error::Error;
use crate::reader::open_reader;
use crate::CldrPaths;
use icu_locale_canonicalizer::provider::*;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::marker::PhantomData;

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

impl<'d> TryFrom<&'d str> for LikelySubtagsProvider<'d> {
    type Error = serde_json::error::Error;
    /// Attempt to parse a JSON string.
    fn try_from(s: &'d str) -> Result<Self, Self::Error> {
        let data: cldr_json::Resource = serde_json::from_str(s)?;
        Ok(Self {
            data,
            _phantom: PhantomData,
        })
    }
}

impl<'d> KeyedDataProvider for LikelySubtagsProvider<'d> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::LikelySubtags {
            return Err((&resc_key.category).into());
        }
        if resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'d> DataProvider<'d, LikelySubtagsV1> for LikelySubtagsProvider<'d> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, LikelySubtagsV1>, DataError> {
        LikelySubtagsProvider::supports_key(&req.resource_path.key)?;
        let langid = &req.resource_path.options.langid;

        // We treat searching for und as a request for all data. Other requests
        // are not currently supported.
        if langid.is_none() {
            Ok(DataResponse {
                metadata: DataResponseMetadata {
                    data_langid: langid.clone(),
                },
                payload: Some(Cow::Owned(LikelySubtagsV1::from(&self.data))),
            })
        } else {
            Err(DataError::UnavailableResourceOptions(req.clone()))
        }
    }
}

icu_provider::impl_erased!(LikelySubtagsProvider<'d>, 'd);

impl<'d> IterableDataProvider<'d> for LikelySubtagsProvider<'d> {
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
        use icu_locid::subtags::{Language, Region, Script};
        use icu_locid::LanguageIdentifier;

        let mut language_script_region: Vec<((Language, Script, Region), LikelySubtagsResultV1)> =
            Vec::new();
        let mut language_script: Vec<((Language, Script), LikelySubtagsResultV1)> = Vec::new();
        let mut language_region: Vec<((Language, Region), LikelySubtagsResultV1)> = Vec::new();
        let mut language: Vec<(Language, LikelySubtagsResultV1)> = Vec::new();

        // Create a result struct. We only need to store the delta between the
        // search LanguageIdentifier and the result LanguageIdentifier.
        let extract_result =
            |entry: &(LanguageIdentifier, LanguageIdentifier)| -> LikelySubtagsResultV1 {
                LikelySubtagsResultV1 {
                    language: if entry.0.language != entry.1.language {
                        Some(entry.1.language)
                    } else {
                        None
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
                }
            };

        for entry in other.supplemental.likely_subtags.iter() {
            if entry.0.script.is_some() && entry.0.region.is_some() {
                language_script_region.push((
                    (
                        entry.0.language,
                        entry.0.script.unwrap(),
                        entry.0.region.unwrap(),
                    ),
                    extract_result(entry),
                ));
            } else if entry.0.script.is_some() {
                language_script.push((
                    (entry.0.language, entry.0.script.unwrap()),
                    extract_result(entry),
                ));
            } else if entry.0.region.is_some() {
                language_region.push((
                    (entry.0.language, entry.0.region.unwrap()),
                    extract_result(entry),
                ));
            } else {
                language.push((entry.0.language, extract_result(entry)));
            }
        }

        // We sort here because the ordering from sorting by LanguageIdentifier
        // is not necessarily the order in the underlying CLDR data.
        language_script_region.sort_unstable_by_key(|k| k.0);
        language_script.sort_unstable_by_key(|k| k.0);
        language_region.sort_unstable_by_key(|k| k.0);
        language.sort_unstable_by_key(|k| k.0);
        Self {
            language_script_region,
            language_script,
            language_region,
            language,
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
    use std::borrow::Cow;

    let json_str = std::fs::read_to_string("tests/testdata/likelySubtags.json").unwrap();
    let provider = LikelySubtagsProvider::try_from(json_str.as_str()).unwrap();
    let result: Cow<LikelySubtagsV1> = provider
        .load_payload(&DataRequest::from(key::LIKELY_SUBTAGS_V1))
        .unwrap()
        .take_payload()
        .unwrap();

    let langid = langid!("cu-Glag");
    let entry = result
        .language_script
        .binary_search_by_key(&(langid.language, langid.script.unwrap()), |(l, _)| *l)
        .unwrap();
    assert!(result.language_script[entry].1.language.is_none());
    assert!(result.language_script[entry].1.script.is_none());
    assert_eq!(result.language_script[entry].1.region.unwrap(), "BG");
}
