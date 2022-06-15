// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{HashMap, HashSet};

use crate::transform::cldr::cldr_serde;
use crate::SourceData;
use icu_locale_canonicalizer::provider::*;
use icu_locid::{language, region, script, subtags::Script, LanguageIdentifier};
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::provider::*;
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

        // First collect the l2s and l2r maps
        for (minimized, maximized) in source_data
            .likely_subtags_data
            .supplemental
            .likely_subtags
            .iter()
            // Skip "und" for vertical fallback
            .filter(|(lid, _)| !lid.language.is_empty())
            // Find language-only entries
            .filter(|(lid, _)| **lid == LanguageIdentifier::from(lid.language))
        {
            let language = minimized.language;
            let script = maximized.script.expect("maximized");
            let region = maximized.region.expect("maximized");
            if script != DEFAULT_SCRIPT {
                l2s.insert(&language.into(), &script);
            }
            if region != DEFAULT_REGION {
                l2r.insert(&language.into(), &region);
            }
        }

        // Now populate the other maps
        for (minimized, maximized) in source_data
            .likely_subtags_data
            .supplemental
            .likely_subtags
            .iter()
            // Skip "und" for vertical fallback
            .filter(|(lid, _)| !lid.language.is_empty())
            // Find non-language-only entries
            .filter(|(lid, _)| **lid != LanguageIdentifier::from(lid.language))
        {
            let language = maximized.language;
            let script = maximized.script.expect("maximized");
            let region = maximized.region.expect("maximized");
            if minimized.script.is_some() {
                assert!(minimized.region.is_none(), "{:?}", minimized);
                let region_for_lang = l2r.get_copied(&language.into()).unwrap_or(DEFAULT_REGION);
                if region != region_for_lang {
                    ls2r.insert(&language.into(), &script.into(), &region);
                }
                continue;
            }
            if minimized.region.is_some() {
                let script_for_lang = l2s.get_copied(&language.into()).unwrap_or(DEFAULT_SCRIPT);
                if script != script_for_lang {
                    lr2s.insert(&language.into(), &region.into(), &script);
                }
                continue;
            }
            unreachable!();
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

    assert_eq!(
        data.get().l2s.get_copied(&language!("zh").into()),
        Some(script!("Hans"))
    );
    assert_eq!(
        data.get()
            .lr2s
            .get_copied(&language!("zh").into(), &region!("TW").into()),
        Ok(script!("Hant"))
    );
    assert_eq!(
        data.get().l2r.get_copied(&language!("zh").into()),
        Some(region!("CN"))
    );
    assert_eq!(
        data.get()
            .ls2r
            .get_copied(&language!("zh").into(), &script!("Hant").into()),
        Ok(region!("TW"))
    );
}
