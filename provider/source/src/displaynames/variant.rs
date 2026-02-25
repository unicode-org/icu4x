// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use core::convert::TryFrom;
use icu::experimental::displaynames::provider::*;
use icu::locale::{subtags::Variant, ParseError};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use zerovec::VarZeroCow;

impl DataProvider<VariantDisplayNamesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<VariantDisplayNamesV1>, DataError> {
        self.check_req::<VariantDisplayNamesV1>(req)?;

        let data: &cldr_serde::displaynames::variant::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "variants.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(VariantDisplayNames::try_from(data).map_err(|e| {
                DataError::custom("data for VariantDisplayNames").with_display_context(&e)
            })?),
        })
    }
}

impl DataProvider<LocaleNamesVariantLongV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LocaleNamesVariantLongV1>, DataError> {
        self.check_req::<LocaleNamesVariantLongV1>(req)?;

        let data: &cldr_serde::displaynames::variant::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "variants.json")?;

        let name = data
            .main
            .value
            .localedisplaynames
            .variants
            .get(req.id.marker_attributes.as_str())
            .ok_or_else(|| {
                DataError::custom("data for VariantDisplayNames")
                    .with_req(LocaleNamesVariantLongV1::INFO, req)
            })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(VarZeroCow::from_encodeable(name)),
        })
    }
}

impl IterableDataProviderCached<LocaleNamesVariantLongV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut result = HashSet::new();
        let displaynames = self.cldr()?.displaynames();
        for locale in displaynames.list_locales()?.filter(|locale| {
            // The directory might exist without variants.json
            self.cldr()
                .unwrap()
                .displaynames()
                .file_exists(locale, "variants.json")
                .unwrap_or_default()
        }) {
            let data: &cldr_serde::displaynames::variant::Resource =
                displaynames.read_and_parse(&locale, "variants.json")?;
            for variant_str in data.main.value.localedisplaynames.variants.keys() {
                if variant_str.contains("-alt-") {
                    continue;
                }
                let data_identifier = DataIdentifierCow::from_owned(
                    DataMarkerAttributes::try_from_string(variant_str.clone()).map_err(|_| {
                        DataError::custom("Failed to parse variant as attribute")
                            .with_debug_context(&variant_str)
                    })?,
                    locale,
                );
                result.insert(data_identifier);
            }
        }
        Ok(result)
    }
}

impl IterableDataProviderCached<VariantDisplayNamesV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .displaynames()
            .list_locales()?
            .filter(|locale| {
                // The directory might exist without variants.json
                self.cldr()
                    .unwrap()
                    .displaynames()
                    .file_exists(locale, "variants.json")
                    .unwrap_or_default()
            })
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

/// Substring used to denote alternative display names data variants for a given variant. For example: "FONUPA-alt-secondary".
const ALT_SUBSTRING: &str = "-alt-";

impl TryFrom<&cldr_serde::displaynames::variant::Resource> for VariantDisplayNames<'static> {
    type Error = ParseError;

    fn try_from(other: &cldr_serde::displaynames::variant::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        for entry in other.main.value.localedisplaynames.variants.iter() {
            // TODO: Support alt variants for variant display names.
            if !entry.0.contains(ALT_SUBSTRING) {
                names.insert(
                    Variant::try_from_str(entry.0)?.to_tinystr(),
                    entry.1.as_str(),
                );
            }
        }
        Ok(Self {
            // Old CLDR versions may contain trivial entries, so filter
            names: names
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
    use icu::locale::{langid, subtags::variant};

    #[test]
    fn test_basic_variant_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<VariantDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&variant!("POSIX").to_tinystr().to_unvalidated())
                .unwrap(),
            "Computer"
        );
    }

    #[test]
    fn test_locale_names_variant_long() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesVariantLongV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("POSIX").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Computer");
    }
}
