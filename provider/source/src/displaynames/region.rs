// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::WithAlt;
use crate::displaynames::{ALT_BIOT, ALT_CHAGOS, ALT_SHORT, ALT_VARIANT, extract_names};
use icu::experimental::displaynames::provider::*;
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
            payload: DataPayload::from_owned(RegionDisplayNames::from(data)),
        })
    }
}

crate::displaynames::impl_displaynames_legacy_iter_v1!(RegionDisplayNamesV1, "territories.json");

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesRegionMediumV1,
    icu::locale::subtags::Region,
    cldr_serde::displaynames::region::Resource,
    "territories.json",
    regions,
    None::<&str>,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesRegionShortV1,
    icu::locale::subtags::Region,
    cldr_serde::displaynames::region::Resource,
    "territories.json",
    regions,
    Some(ALT_SHORT),
);

impl From<&cldr_serde::displaynames::region::Resource> for RegionDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::region::Resource) -> Self {
        let extracted = extract_names(
            &other.main.value.localedisplaynames.regions,
            &[ALT_VARIANT, ALT_CHAGOS, ALT_BIOT],
            "region",
            |region, val| {
                let region_str = region.to_tinystr();
                if region_str.as_str() == val {
                    None
                } else {
                    Some(region_str)
                }
            },
        );

        let to_zero_map = |map: BTreeMap<tinystr::TinyAsciiStr<3>, &str>| {
            map.into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect()
        };

        Self {
            names: to_zero_map(extracted.names),
            short_names: to_zero_map(extracted.short_names),
        }
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
