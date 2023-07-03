// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use core::convert::TryFrom;
use icu_displaynames::provider::*;
use icu_locid::{subtags::Variant, ParserError};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::str::FromStr;

impl DataProvider<VariantDisplayNamesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<VariantDisplayNamesV1Marker>, DataError> {
        self.check_req::<VariantDisplayNamesV1Marker>(req)?;
        let langid = req.locale.get_langid();

        let data: &cldr_serde::displaynames::variant::Resource = self
            .source
            .cldr()?
            .displaynames()
            .read_and_parse(&langid, "variants.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(
                VariantDisplayNamesV1::try_from(data).map_err(|e| {
                    DataError::custom("data for VariantDisplayNames").with_display_context(&e)
                })?,
            )),
        })
    }
}

impl IterableDataProvider<VariantDisplayNamesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self.filter_data_locales(
            self.source
                .cldr()?
                .displaynames()
                .list_langs()?
                .filter(|langid| {
                    // The directory might exist without variants.json
                    self.source
                        .cldr()
                        .unwrap()
                        .displaynames()
                        .file_exists(langid, "variants.json")
                        .unwrap_or_default()
                })
                .map(DataLocale::from)
                .collect(),
        ))
    }
}

/// Substring used to denote alternative display names data variants for a given variant. For example: "FONUPA-alt-secondary".
const ALT_SUBSTRING: &str = "-alt-";

impl TryFrom<&cldr_serde::displaynames::variant::Resource> for VariantDisplayNamesV1<'static> {
    type Error = ParserError;

    fn try_from(other: &cldr_serde::displaynames::variant::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        for lang_data_entry in other.main.0.iter() {
            for entry in lang_data_entry.1.localedisplaynames.variants.iter() {
                // TODO: Support alt variants for variant display names.
                if !entry.0.contains(ALT_SUBSTRING) {
                    names.insert(Variant::from_str(entry.0)?.into_tinystr(), entry.1.as_str());
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::{locale, subtags::variant};

    #[test]
    fn test_basic_variant_display_names() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<VariantDisplayNamesV1Marker> = provider
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
                .get(&variant!("POSIX").into_tinystr().to_unvalidated())
                .unwrap(),
            "Computer"
        );
    }
}
