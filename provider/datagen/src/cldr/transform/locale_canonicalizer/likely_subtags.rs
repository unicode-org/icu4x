// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr::cldr_serde;
use crate::cldr::error::Error;
use crate::cldr::reader::open_reader;
use crate::cldr::CldrPaths;
use icu_locale_canonicalizer::provider::*;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;

use std::convert::TryFrom;
use std::path::PathBuf;

/// A data provider reading from CLDR JSON likely subtags rule files.
#[derive(Debug)]
pub struct LikelySubtagsProvider {
    path: PathBuf,
}

impl TryFrom<&dyn CldrPaths> for LikelySubtagsProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        Ok(Self {
            path: cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("likelySubtags.json"),
        })
    }
}

impl ResourceProvider<LikelySubtagsV1Marker> for LikelySubtagsProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<LikelySubtagsV1Marker>, DataError> {
        let langid = &req.options.langid;

        let data: cldr_serde::likely_subtags::Resource =
            serde_json::from_reader(open_reader(&self.path)?)
                .map_err(|e| Error::Json(e, Some(self.path.clone())))?;

        // We treat searching for und as a request for all data. Other requests
        // are not currently supported.
        if langid.is_none() {
            let metadata = DataResponseMetadata::default();
            // TODO(#1109): Set metadata.data_langid correctly.
            Ok(DataResponse {
                metadata,
                payload: Some(DataPayload::from_owned(LikelySubtagsV1::from(&data))),
            })
        } else {
            Err(DataErrorKind::ExtraneousResourceOptions.with_req(LikelySubtagsV1Marker::KEY, req))
        }
    }
}

icu_provider::impl_dyn_provider!(
    LikelySubtagsProvider,
    [LikelySubtagsV1Marker,],
    SERDE_SE,
    ITERABLE_SERDE_SE,
    DATA_CONVERTER
);

impl IterableResourceProvider<LikelySubtagsV1Marker> for LikelySubtagsProvider {
    fn supported_options(&self) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}

impl From<&cldr_serde::likely_subtags::Resource> for LikelySubtagsV1 {
    fn from(other: &cldr_serde::likely_subtags::Resource) -> Self {
        use icu_locid::LanguageIdentifier;

        let mut language_script = LiteMap::new();
        let mut language_region = LiteMap::new();
        let mut language = LiteMap::new();
        let mut script_region = LiteMap::new();
        let mut script = LiteMap::new();
        let mut region = LiteMap::new();
        let mut und = LanguageIdentifier::default();

        // Create a result LanguageIdentifier. We only need to store the delta
        // between the search LanguageIdentifier and the result LanguageIdentifier.
        let extract_result =
            |entry: &(LanguageIdentifier, LanguageIdentifier)| -> LanguageIdentifier {
                LanguageIdentifier {
                    language: if entry.0.language != entry.1.language {
                        entry.1.language
                    } else {
                        icu_locid::subtags::Language::UND
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
            if !entry.0.language.is_empty() {
                let lang = entry.0.language.into();
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
    use icu_locid::script;

    let cldr_paths = crate::cldr::cldr_paths::for_test();
    let provider = LikelySubtagsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();
    let result: DataPayload<LikelySubtagsV1Marker> = provider
        .load_resource(&Default::default())
        .unwrap()
        .take_payload()
        .unwrap();

    let entry = result.get().script.get(&(script!("Glag").into())).unwrap();
    assert_eq!(entry.language, "cu");
    assert_eq!(entry.region.unwrap(), "BG");
}
