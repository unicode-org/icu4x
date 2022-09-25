// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "experimental")]

use crate::transform::cldr::cldr_serde;
use icu_displaynames::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
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
            payload: Some(DataPayload::from_owned(TerritoryDisplayNamesV1::from(data))),
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

/// Suffix used to denote alternative region names data variants for a given territory.
const ALT_VARIANT_SUFFIX: &str = "-alt-variant";

impl From<&cldr_serde::displaynames::Resource> for TerritoryDisplayNamesV1<'static> {
    fn from(other: &cldr_serde::displaynames::Resource) -> Self {
        let mut names = ZeroMap::new();
        for lang_data_entry in other.main.0.iter() {
            for entry in lang_data_entry.1.localedisplaynames.territories.iter() {
                let region = entry.0;
                if !region.ends_with(ALT_VARIANT_SUFFIX) {
                    if let Ok(key) = <TinyAsciiStr<3>>::from_str(region) {
                        names.insert(&key, entry.1.as_ref());
                    }
                }
            }
        }
        Self { names }
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
