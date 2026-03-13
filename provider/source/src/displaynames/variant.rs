// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::displaynames::{ALT_SHORT_SUBSTRING, ALT_SUBSTRING};
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

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesVariantLongV1,
    cldr_serde::displaynames::variant::Resource,
    "variants.json",
    variants,
    None::<&str>,
    "VariantDisplayNames"
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesVariantShortV1,
    cldr_serde::displaynames::variant::Resource,
    "variants.json",
    variants,
    Some(ALT_SHORT_SUBSTRING),
    "VariantDisplayNames"
);

crate::displaynames::impl_displaynames_main_iter_v1!(VariantDisplayNamesV1, "variants.json");

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
