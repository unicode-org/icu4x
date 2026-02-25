// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::displaynames::{ALT_SHORT_SUBSTRING, ALT_SUBSTRING};
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

crate::displaynames::impl_displaynames_main_iter_v1!(RegionDisplayNamesV1, "territories.json");

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesRegionLongV1,
    cldr_serde::displaynames::region::Resource,
    "territories.json",
    regions,
    None::<&str>,
    "RegionDisplayNames"
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesRegionShortV1,
    cldr_serde::displaynames::region::Resource,
    "territories.json",
    regions,
    Some(ALT_SHORT_SUBSTRING),
    "RegionDisplayNames"
);

impl TryFrom<&cldr_serde::displaynames::region::Resource> for RegionDisplayNames<'static> {
    type Error = icu::locale::ParseError;
    fn try_from(other: &cldr_serde::displaynames::region::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        for (region, value) in other.main.value.localedisplaynames.regions.iter() {
            if let Some(region) = region.strip_suffix(ALT_SHORT_SUBSTRING) {
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

    #[test]
    fn test_locale_names_region_short() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesRegionShortV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("BA").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Bosnia");
    }
}
