// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use core::convert::TryFrom;
use icu::experimental::displaynames::provider::*;
use icu::locale::subtags::Region;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use zerovec::VarZeroCow;

impl DataProvider<RegionDisplayNamesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<RegionDisplayNamesV1>, DataError> {
        self.check_req::<RegionDisplayNamesV1>(req)?;

        let data: &cldr_serde::displaynames::region::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "territories.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(RegionDisplayNames::try_from(data).map_err(|e| {
                DataError::custom("data for RegionDisplayNames").with_display_context(&e)
            })?),
        })
    }
}

impl IterableDataProviderCached<RegionDisplayNamesV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .displaynames()
            .list_locales()?
            .filter(|locale| {
                // The directory might exist without territories.json
                self.cldr()
                    .unwrap()
                    .displaynames()
                    .file_exists(locale, "territories.json")
                    .unwrap_or_default()
            })
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

impl DataProvider<LocaleNamesRegionLongV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LocaleNamesRegionLongV1>, DataError> {
        self.check_req::<LocaleNamesRegionLongV1>(req)?;

        let data: &cldr_serde::displaynames::region::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "territories.json")?;

        let name = data
            .main
            .value
            .localedisplaynames
            .regions
            .get(req.id.marker_attributes.as_str())
            .ok_or_else(|| {
                DataError::custom("data for RegionDisplayNames")
                    .with_req(LocaleNamesRegionLongV1::INFO, req)
            })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(SingleDisplayName {
                name: VarZeroCow::from_encodeable(&name),
            }),
        })
    }
}

impl IterableDataProviderCached<LocaleNamesRegionLongV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut result = HashSet::new();
        let displaynames = self.cldr()?.displaynames();
        for locale in displaynames.list_locales()?.filter(|locale| {
            // The directory might exist without territories.json
            self.cldr()
                .unwrap()
                .displaynames()
                .file_exists(locale, "territories.json")
                .unwrap_or_default()
        }) {
            let data: &cldr_serde::displaynames::region::Resource =
                displaynames.read_and_parse(&locale, "territories.json")?;
            for region_str in data.main.value.localedisplaynames.regions.keys() {
                if region_str.contains("-alt-") {
                    continue;
                }
                let data_identifier = DataIdentifierCow::from_owned(
                    DataMarkerAttributes::try_from_string(region_str.clone()).map_err(|_| {
                        DataError::custom("Failed to parse region as attribute")
                            .with_debug_context(&region_str)
                    })?,
                    locale,
                );
                result.insert(data_identifier);
            }
        }
        Ok(result)
    }
}

/// Substring used to denote alternative region names data variants for a given region. For example: "BA-alt-short", "TL-alt-variant".
const ALT_SUBSTRING: &str = "-alt-";
/// Substring used to denote short region display names data variants for a given region. For example: "BA-alt-short".
const SHORT_SUBSTRING: &str = "-alt-short";

impl TryFrom<&cldr_serde::displaynames::region::Resource> for RegionDisplayNames<'static> {
    type Error = icu::locale::ParseError;
    fn try_from(other: &cldr_serde::displaynames::region::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        for (region, value) in other.main.value.localedisplaynames.regions.iter() {
            if let Some(region) = region.strip_suffix(SHORT_SUBSTRING) {
                short_names.insert(Region::try_from_str(region)?.to_tinystr(), value.as_str());
            } else if !region.contains(ALT_SUBSTRING) {
                names.insert(Region::try_from_str(region)?.to_tinystr(), value.as_str());
            }
        }
        Ok(Self {
            // Old CLDR versions may contain trivial entries, so filter
            names: names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
            short_names: short_names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu::locale::{langid, subtags::region};

    #[test]
    fn test_basic() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<RegionDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&region!("AE").to_tinystr().to_unvalidated())
                .unwrap(),
            "United Arab Emirates"
        );
    }

    #[test]
    fn test_basic_short_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<RegionDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .short_names
                .get(&region!("BA").to_tinystr().to_unvalidated())
                .unwrap(),
            "Bosnia"
        );
    }
}
