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

impl DataProvider<RegionDisplayNamesV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<RegionDisplayNamesV1Marker>, DataError> {
        self.check_req::<RegionDisplayNamesV1Marker>(req)?;

        let data: &cldr_serde::displaynames::region::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "territories.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(RegionDisplayNamesV1::try_from(data).map_err(
                |e| DataError::custom("data for RegionDisplayNames").with_display_context(&e),
            )?),
        })
    }
}

impl IterableDataProviderCached<RegionDisplayNamesV1Marker> for SourceDataProvider {
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

/// Substring used to denote alternative region names data variants for a given region. For example: "BA-alt-short", "TL-alt-variant".
const ALT_SUBSTRING: &str = "-alt-";
/// Substring used to denote short region display names data variants for a given region. For example: "BA-alt-short".
const SHORT_SUBSTRING: &str = "-alt-short";

impl TryFrom<&cldr_serde::displaynames::region::Resource> for RegionDisplayNamesV1<'static> {
    type Error = icu::locale::ParseError;
    fn try_from(other: &cldr_serde::displaynames::region::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        for (region, value) in other.main.value.localedisplaynames.regions.iter() {
            if let Some(region) = region.strip_suffix(SHORT_SUBSTRING) {
                short_names.insert(Region::try_from_str(region)?.into_tinystr(), value.as_str());
            } else if !region.contains(ALT_SUBSTRING) {
                names.insert(Region::try_from_str(region)?.into_tinystr(), value.as_str());
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

        let data: DataPayload<RegionDisplayNamesV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&region!("AE").into_tinystr().to_unvalidated())
                .unwrap(),
            "United Arab Emirates"
        );
    }

    #[test]
    fn test_basic_short_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<RegionDisplayNamesV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .short_names
                .get(&region!("BA").into_tinystr().to_unvalidated())
                .unwrap(),
            "Bosnia"
        );
    }
}
