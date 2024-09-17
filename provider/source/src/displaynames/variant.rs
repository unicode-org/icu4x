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

impl DataProvider<VariantDisplayNamesV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<VariantDisplayNamesV1Marker>, DataError> {
        self.check_req::<VariantDisplayNamesV1Marker>(req)?;

        let data: &cldr_serde::displaynames::variant::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "variants.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(VariantDisplayNamesV1::try_from(data).map_err(
                |e| DataError::custom("data for VariantDisplayNames").with_display_context(&e),
            )?),
        })
    }
}

impl IterableDataProviderCached<VariantDisplayNamesV1Marker> for SourceDataProvider {
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

impl TryFrom<&cldr_serde::displaynames::variant::Resource> for VariantDisplayNamesV1<'static> {
    type Error = ParseError;

    fn try_from(other: &cldr_serde::displaynames::variant::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        for entry in other.main.value.localedisplaynames.variants.iter() {
            // TODO: Support alt variants for variant display names.
            if !entry.0.contains(ALT_SUBSTRING) {
                names.insert(
                    Variant::try_from_str(entry.0)?.into_tinystr(),
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

        let data: DataPayload<VariantDisplayNamesV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&variant!("POSIX").into_tinystr().to_unvalidated())
                .unwrap(),
            "Computer"
        );
    }
}
