// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use core::convert::TryFrom;
use icu_displaynames::provider::*;
use icu_locid::subtags::Region;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::str::FromStr;

impl DataProvider<RegionDisplayNamesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<RegionDisplayNamesV1Marker>, DataError> {
        self.check_req::<RegionDisplayNamesV1Marker>(req)?;
        let langid = req.locale.get_langid();

        let data: &cldr_serde::displaynames::region::Resource = self
            .source
            .cldr()?
            .displaynames()
            .read_and_parse(&langid, "territories.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(
                RegionDisplayNamesV1::try_from(data).map_err(|e| {
                    DataError::custom("data for RegionDisplayNames").with_display_context(&e)
                })?,
            )),
        })
    }
}

impl IterableDataProvider<RegionDisplayNamesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self.filter_data_locales(
            self.source
                .cldr()?
                .displaynames()
                .list_langs()?
                .filter(|langid| {
                    // The directory might exist without territories.json
                    self.source
                        .cldr()
                        .unwrap()
                        .displaynames()
                        .file_exists(langid, "territories.json")
                        .unwrap_or_default()
                })
                .map(DataLocale::from)
                .collect(),
        ))
    }
}

/// Substring used to denote alternative region names data variants for a given region. For example: "BA-alt-short", "TL-alt-variant".
const ALT_SUBSTRING: &str = "-alt-";
/// Substring used to denote short region display names data variants for a given region. For example: "BA-alt-short".
const SHORT_SUBSTRING: &str = "-alt-short";

impl TryFrom<&cldr_serde::displaynames::region::Resource> for RegionDisplayNamesV1<'static> {
    type Error = icu_locid::ParserError;
    fn try_from(other: &cldr_serde::displaynames::region::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        let mut short_names = BTreeMap::new();
        for (_, lang_display_names) in other.main.0.iter() {
            for (region, value) in lang_display_names.localedisplaynames.regions.iter() {
                if let Some(region) = region.strip_suffix(SHORT_SUBSTRING) {
                    short_names.insert(Region::from_str(region)?.into_tinystr(), value.as_str());
                } else if !region.contains(ALT_SUBSTRING) {
                    names.insert(Region::from_str(region)?.into_tinystr(), value.as_str());
                }
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
    use icu_locid::{locale, subtags::region};

    #[test]
    fn test_basic() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<RegionDisplayNamesV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("en-001").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

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
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<RegionDisplayNamesV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("en-001").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            data.get()
                .short_names
                .get(&region!("BA").into_tinystr().to_unvalidated())
                .unwrap(),
            "Bosnia"
        );
    }
}
