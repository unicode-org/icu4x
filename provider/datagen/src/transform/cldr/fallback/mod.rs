// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{HashMap, HashSet};

use crate::transform::cldr::cldr_serde;
use crate::SourceData;
use icu_locale_canonicalizer::provider::*;
use icu_locid::{language, subtags::Script, LanguageIdentifier};
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::*;
use tinystr::TinyAsciiStr;
use writeable::Writeable;
use zerovec::{maps::ZeroMap2d, ZeroMap, ZeroSlice};

use super::LikelySubtagsProvider;

/// A data provider reading from CLDR JSON likely subtags rule files.
#[derive(Debug)]
pub struct FallbackRulesProvider {
    source: SourceData,
    likely_subtags_provider: LikelySubtagsProvider,
}

impl From<&SourceData> for FallbackRulesProvider {
    fn from(source: &SourceData) -> Self {
        FallbackRulesProvider {
            source: source.clone(),
            likely_subtags_provider: source.into(),
        }
    }
}

struct FallbackSourceData<'a> {
    likely_subtags_data: &'a cldr_serde::likely_subtags::Resource,
    parents_data: &'a cldr_serde::parent_locales::Resource,
}

impl ResourceProvider<LocaleFallbackRulesV1Marker> for FallbackRulesProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<LocaleFallbackRulesV1Marker>, DataError> {
        // We treat searching for `und` as a request for all data. Other requests
        // are not currently supported.
        if !req.options.is_empty() {
            return Err(DataErrorKind::ExtraneousResourceOptions.into_error());
        }

        let likely_subtags_data: &cldr_serde::likely_subtags::Resource = self
            .source
            .get_cldr_paths()?
            .cldr_core()
            .read_and_parse("supplemental/likelySubtags.json")?;

        let parents_data: &cldr_serde::parent_locales::Resource = self
            .source
            .get_cldr_paths()?
            .cldr_core()
            .read_and_parse("supplemental/parentLocales.json")?;

        let metadata = DataResponseMetadata::default();
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(LocaleFallbackRulesV1::from(
                FallbackSourceData {
                    likely_subtags_data,
                    parents_data,
                },
            ))),
        })
    }
}

icu_provider::make_exportable_provider!(FallbackRulesProvider, [LocaleFallbackRulesV1Marker,]);

impl IterableResourceProvider<LocaleFallbackRulesV1Marker> for FallbackRulesProvider {
    fn supported_options(&self) -> Result<Vec<ResourceOptions>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl From<FallbackSourceData<'_>> for LocaleFallbackRulesV1<'static> {
    fn from(source_data: FallbackSourceData<'_>) -> Self {
        let mut l2s = ZeroMap::new();
        let mut lr2s = ZeroMap2d::new();
        let mut l2r = ZeroMap::new();
        let mut ls2r = ZeroMap2d::new();
        let mut parents = ZeroMap::new();

        // To find single-script languages, collect all language-script pairs
        let mut l2s_set = HashMap::new();
        for maximized in source_data
            .likely_subtags_data
            .supplemental
            .likely_subtags
            .iter_values()
        {
            let set: &mut HashSet<Script> = l2s_set.entry(maximized.language).or_default();
            set.insert(maximized.script.expect("maximized"));
        }

        // Now populate the maps
        for (minimized, maximized) in source_data
            .likely_subtags_data
            .supplemental
            .likely_subtags
            .iter()
        {
            let language = minimized.language;
            if language.is_empty() {
                // We never fill in a missing language in vertical fallback
                continue;
            }
            if let Some(script) = minimized.script {
                assert!(minimized.region.is_none());
                let region = maximized.region.expect("maximized");
                ls2r.insert(&language.into(), &script.into(), &region);
                continue;
            }
            if let Some(region) = minimized.region {
                // Skip if it is a single-script language
                if l2s_set.get(&language).unwrap().len() == 1 {
                    continue;
                }
                let script = maximized.script.expect("maximized");
                lr2s.insert(&language.into(), &region.into(), &script);
                continue;
            }
            let script = maximized.script.expect("maximized");
            let region = maximized.region.expect("maximized");
            l2r.insert(&language.into(), &region);
            // Skip if it is a single-script language
            if l2s_set.get(&language).unwrap().len() == 1 {
                continue;
            }
            l2s.insert(&language.into(), &script);
        }

        for (source, target) in source_data
            .parents_data
            .supplemental
            .parent_locales
            .parent_locale
            .iter()
        {
            assert!(!source.language.is_empty());
            if source.script.is_some() && source.region.is_none() {
                // We always fall back from language-script to und
                continue;
            }
            parents.insert(source.write_to_string().as_bytes(), &target.into());
        }

        LocaleFallbackRulesV1 {
            l2s,
            lr2s,
            l2r,
            ls2r,
            parents,
        }
    }
}

#[test]
fn test_basic() {
    use tinystr::tinystr;

    let provider = FallbackRulesProvider::from(&SourceData::for_test());
    let data: DataPayload<LocaleFallbackRulesV1Marker> = provider
        .load_resource(&DataRequest::default())
        .unwrap()
        .take_payload()
        .unwrap();

    println!("{:?}", data);
}
