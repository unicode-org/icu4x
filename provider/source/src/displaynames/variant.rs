// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::coverage_experimental::CoverageLevelForXPath;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::{Alt, WithAlt};
use crate::displaynames::extract_names_for_zeromap_struct;
use icu::experimental::displaynames::provider::*;
use icu::locale::provider::names::*;
use icu::locale::subtags::Variant;
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
    LocaleNamesVariantMediumHeavyV1,
    Variant,
    cldr_serde::displaynames::variant::Resource,
    "variants.json",
    variants,
    None,
    variant,
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);

crate::displaynames::impl_displaynames_legacy_iter_v1!(VariantDisplayNamesV1, "variants.json");

// TODO: Support alt variants for variant display names.
impl From<&cldr_serde::displaynames::variant::Resource> for VariantDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::variant::Resource) -> Self {
        let extracted = extract_names_for_zeromap_struct(
            &other.main.value.localedisplaynames.variants,
            // TODO(#8012): Handle preference-specific alt variants, perhaps with datagen alt flags.
            &[Alt::Secondary],
            "variant",
            |variant| Some(variant.to_tinystr()),
        );

        let to_zero_map = |map: BTreeMap<tinystr::TinyAsciiStr<8>, &str>| {
            map.into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect()
        };

        Self {
            names: to_zero_map(extracted.names),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::displaynames::coverage_experimental::CheckAltCoverage;
    use icu::locale::{data_locale, subtags::variant};

    #[test]
    fn test_basic_variant_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<VariantDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en-001")),
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
    fn test_locale_names_variant_medium_heavy() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesVariantMediumHeavyV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("posix").unwrap(),
                    &data_locale!("en-001"),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Computer");
    }

    /// The cartesian product of Variant x (Short | Medium) x (Minimal | Core | Extended)
    /// contains some data markers that are uninhabited. This test ensures that every variant display name
    /// key and coverage tier combination in CLDR is covered by an existing marker, so if future CLDR releases
    /// add data for uninhabited markers, we learn about it and can take action.
    #[test]
    fn test_empty_coverage_tiers_assert_no_data() {
        let provider = SourceDataProvider::new_testing();
        let cldr = provider.cldr().unwrap();

        crate::displaynames::coverage_experimental::for_each_cldr_key_and_tier(
            cldr,
            "variants.json",
            // TODO(#8012): Handle preference-specific alt variants, perhaps with datagen alt flags.
            &[Alt::Secondary],
            |l| &l.variant,
            |res: &cldr_serde::displaynames::variant::Resource| {
                &res.main.value.localedisplaynames.variants
            },
            |locale, key, tier| {
                if LocaleNamesVariantMediumHeavyV1::contains_key(key, tier) {
                    return;
                }

                panic!(
                    "Found unexpected alt, menu, and tier combination for variant: {key:?} in locale: {locale:?} and tier: {tier:?}"
                );
            },
        );
    }
}
