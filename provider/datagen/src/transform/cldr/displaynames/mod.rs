// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "experimental")]

use crate::transform::cldr::cldr_serde;
use core::convert::TryFrom;
use icu_displaynames::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use tinystr::TinyStrError;
use zerovec::ZeroMap;

impl DataProvider<TerritoryDisplayNamesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TerritoryDisplayNamesV1Marker>, DataError> {
        let langid = req.locale.get_langid();

        let data: &cldr_serde::displaynames::Resource = self
            .source
            .cldr()?
            .displaynames()
            .read_and_parse(&langid, "territories.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(
                TerritoryDisplayNamesV1::try_from(data).map_err(|e| {
                    DataError::custom("data for TerritoryDisplayNames").with_display_context(&e)
                })?,
            )),
        })
    }
}

impl IterableDataProvider<TerritoryDisplayNamesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .displaynames()
            .list_langs()?
            .map(DataLocale::from)
            .collect())
    }
}

/// Substring used to denote alternative region names data variants for a given territory. For example: "BA-alt-short", "TL-alt-variant".
const ALT_SUBSTRING: &str = "-alt-";

impl TryFrom<&cldr_serde::displaynames::Resource> for TerritoryDisplayNamesV1<'static> {
    type Error = TinyStrError;
    fn try_from(other: &cldr_serde::displaynames::Resource) -> Result<Self, Self::Error> {
        let mut names = ZeroMap::new();
        for lang_data_entry in other.main.0.iter() {
            for entry in lang_data_entry.1.localedisplaynames.territories.iter() {
                let region = entry.0;
                if !region.contains(ALT_SUBSTRING) {
                    match <TinyAsciiStr<3>>::from_str(region) {
                        Ok(key) => {
                            names.insert(&key, entry.1.as_ref());
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
            }
        }
        Ok(Self { names })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::locale;
    use tinystr::tinystr;

    #[test]
    fn test_basic() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<TerritoryDisplayNamesV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("en-001").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        assert_eq!(
            data.get().names.get(&tinystr!(3, "AE")).unwrap(),
            "United Arab Emirates"
        );
    }
}
