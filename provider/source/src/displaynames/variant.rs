// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::ModifiedSubtag;
use icu::experimental::displaynames::provider::*;
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
            payload: DataPayload::from_owned(VariantDisplayNames::from(data)),
        })
    }
}

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesVariantMediumV1,
    icu::locale::subtags::Variant,
    cldr_serde::displaynames::variant::Resource,
    "variants.json",
    variants,
    None::<&str>,
);

crate::displaynames::impl_displaynames_legacy_iter_v1!(VariantDisplayNamesV1, "variants.json");

impl From<&cldr_serde::displaynames::variant::Resource> for VariantDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::variant::Resource) -> Self {
        let mut names = BTreeMap::new();
        for (key, value) in other.main.value.localedisplaynames.variants.iter() {
            if key.menu_variant.is_some() {
                continue;
            }

            let variant = key.subtag;

            match key.alt_variant.as_deref() {
                None => {
                    names.insert(variant.to_tinystr(), value.as_str());
                }
                Some("secondary") => {
                    // TODO(#8012): Handle this with datagen alt flags.
                }
                Some(alt) => {
                    log::warn!("Unknown alt variant for variant: {}", alt);
                }
            }
        }
        Self {
            // Old CLDR versions may contain trivial entries, so filter
            names: names
                .into_iter()
                .filter(|&(k, v)| k != v)
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect(),
        }
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
    fn test_locale_names_variant_medium() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesVariantMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("posix").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Computer");
    }
}
