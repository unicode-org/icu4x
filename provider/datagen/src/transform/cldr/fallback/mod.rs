// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use crate::SourceData;
use icu_locale_canonicalizer::provider::*;
use icu_locid::{language, subtags, LanguageIdentifier};
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::*;
use tinystr::TinyAsciiStr;
use zerovec::{ZeroMap, ZeroSlice};

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
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(LocaleFallbackRulesV1::from(FallbackSourceData {
                likely_subtags_data,
                parents_data
            }))),
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
        todo!()
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
}
