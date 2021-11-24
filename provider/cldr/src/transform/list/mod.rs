// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::error::Error;
use crate::reader::{get_langid_subdirectories, open_reader};
use crate::CldrPaths;
use icu_list::provider::*;
use icu_locid::LanguageIdentifier;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use litemap::LiteMap;
use std::convert::TryFrom;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 3] = [
    key::LIST_FORMAT_AND_V1,
    key::LIST_FORMAT_OR_V1,
    key::LIST_FORMAT_UNIT_V1,
];

/// A data provider reading from CLDR JSON list rule files.
#[derive(PartialEq, Debug)]
pub struct ListProvider {
    data: LiteMap<LanguageIdentifier, cldr_serde::list_patterns::LangListPatterns>,
}

impl TryFrom<&dyn CldrPaths> for ListProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = LiteMap::new();
        for dir in get_langid_subdirectories(&cldr_paths.cldr_misc()?.join("main"))? {
            let path = dir.join("listPatterns.json");
            let resource: cldr_serde::list_patterns::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.extend_from_litemap(resource.main.0);
        }
        Ok(Self { data })
    }
}

impl KeyedDataProvider for ListProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        key::LIST_FORMAT_AND_V1
            .match_key(*resc_key)
            .or_else(|_| key::LIST_FORMAT_OR_V1.match_key(*resc_key))
            .or_else(|_| key::LIST_FORMAT_UNIT_V1.match_key(*resc_key))
    }
}

impl DataProvider<ListFormatterPatternsV1Marker> for ListProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<ListFormatterPatternsV1Marker>, DataError> {
        Self::supports_key(&req.resource_path.key)?;
        let langid = req.try_langid()?;
        let data = match self.data.get(&langid) {
            Some(v) => &v.list_patterns,
            None => return Err(DataError::MissingResourceOptions(req.clone())),
        };

        let patterns = match req.resource_path.key {
            key::LIST_FORMAT_AND_V1 => parse_and_patterns(data),
            key::LIST_FORMAT_OR_V1 => parse_or_patterns(data),
            key::LIST_FORMAT_UNIT_V1 => parse_unit_patterns(data),
            _ => panic!("Cannot happen due to check in supports_key"),
        }
        .map_err(DataError::new_resc_error)?;

        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(patterns)),
        })
    }
}

icu_provider::impl_dyn_provider!(ListProvider, {
    _ => ListFormatterPatternsV1Marker,
}, SERDE_SE);

impl IterableDataProviderCore for ListProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = self
            .data
            .iter()
            // ur-IN has a buggy pattern ("{1}, {0}") which violates
            // our invariant that {0} is at index 0 (and rotates the output).
            // See https://github.com/unicode-org/icu4x/issues/1282
            .filter(|(l, _)| *l != &icu_locid_macros::langid!("ur-IN"))
            .map(|(l, _)| ResourceOptions {
                variant: None,
                langid: Some(l.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

fn parse_and_patterns<'a>(
    raw: &cldr_serde::list_patterns::ListPatterns,
) -> Result<ListFormatterPatternsV1<'a>, icu_list::error::Error> {
    Ok(ListFormatterPatternsV1::new(
        raw.standard.start.parse()?,
        raw.standard.middle.parse()?,
        raw.standard.end.parse()?,
        raw.standard.pair.parse()?,
        raw.standard_short.start.parse()?,
        raw.standard_short.middle.parse()?,
        raw.standard_short.end.parse()?,
        raw.standard_short.pair.parse()?,
        raw.standard_narrow.start.parse()?,
        raw.standard_narrow.middle.parse()?,
        raw.standard_narrow.end.parse()?,
        raw.standard_narrow.pair.parse()?,
    ))
}

fn parse_or_patterns<'a>(
    raw: &cldr_serde::list_patterns::ListPatterns,
) -> Result<ListFormatterPatternsV1<'a>, icu_list::error::Error> {
    Ok(ListFormatterPatternsV1::new(
        raw.or.start.parse()?,
        raw.or.middle.parse()?,
        raw.or.end.parse()?,
        raw.or.pair.parse()?,
        raw.or_short.start.parse()?,
        raw.or_short.middle.parse()?,
        raw.or_short.end.parse()?,
        raw.or_short.pair.parse()?,
        raw.or_narrow.start.parse()?,
        raw.or_narrow.middle.parse()?,
        raw.or_narrow.end.parse()?,
        raw.or_narrow.pair.parse()?,
    ))
}

fn parse_unit_patterns<'a>(
    raw: &cldr_serde::list_patterns::ListPatterns,
) -> Result<ListFormatterPatternsV1<'a>, icu_list::error::Error> {
    Ok(ListFormatterPatternsV1::new(
        raw.unit.start.parse()?,
        raw.unit.middle.parse()?,
        raw.unit.end.parse()?,
        raw.unit.pair.parse()?,
        raw.unit_short.start.parse()?,
        raw.unit_short.middle.parse()?,
        raw.unit_short.end.parse()?,
        raw.unit_short.pair.parse()?,
        raw.unit_narrow.start.parse()?,
        raw.unit_narrow.middle.parse()?,
        raw.unit_narrow.end.parse()?,
        raw.unit_narrow.pair.parse()?,
    ))
}

#[test]
fn test_basic() {
    use icu_list::options::Width;
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = ListProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let fr_and_list: DataPayload<ListFormatterPatternsV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::LIST_FORMAT_AND_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("fr")),
                },
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(
        fr_and_list.get().pair(Width::Wide),
        &"{0} et {1}".parse().unwrap()
    );

    let es_or_list: DataPayload<ListFormatterPatternsV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::LIST_FORMAT_OR_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("es")),
                },
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(
        es_or_list.get().middle(Width::Wide),
        &"{0}, {1}".parse().unwrap()
    );
}
