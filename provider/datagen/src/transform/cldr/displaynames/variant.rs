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
        let langid = req.locale.get_langid();

        let data: &cldr_serde::variant_displaynames::Resource = self
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
        Ok(self.source.options.locales.filter_by_langid_equality(
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

impl TryFrom<&cldr_serde::variant_displaynames::Resource> for VariantDisplayNamesV1<'static> {
    type Error = ParserError;

    fn try_from(other: &cldr_serde::variant_displaynames::Resource) -> Result<Self, Self::Error> {
        let mut names = BTreeMap::new();
        for lang_data_entry in other.main.0.iter() {
            for entry in lang_data_entry.1.localedisplaynames.variants.iter() {
                names.insert(Variant::from_str(entry.0)?.into_tinystr(), entry.1.as_str());
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
    use icu_locid::{locale, subtags_variant as variant};

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
