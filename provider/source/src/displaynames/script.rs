// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_cache::CoverageLevelForXPath;
use crate::cldr_serde;
use crate::cldr_serde::displaynames::{Alt, WithAlt};
use crate::displaynames::extract_names_for_zeromap_struct;
use icu::experimental::displaynames::provider::*;
use icu::locale::subtags::Script;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};
use zerovec::VarZeroCow;

impl DataProvider<ScriptDisplayNamesV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<ScriptDisplayNamesV1>, DataError> {
        self.check_req::<ScriptDisplayNamesV1>(req)?;

        let data: &cldr_serde::displaynames::script::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "scripts.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(ScriptDisplayNames::from(data)),
        })
    }
}

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesScriptMinimalMediumV1,
    Script,
    cldr_serde::displaynames::script::Resource,
    "scripts.json",
    scripts,
    None,
    CoverageLevelForXPath::Basic | CoverageLevelForXPath::Core,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesScriptCoreMediumV1,
    Script,
    cldr_serde::displaynames::script::Resource,
    "scripts.json",
    scripts,
    None,
    CoverageLevelForXPath::Moderate,
);
crate::displaynames::impl_displaynames_v1!(
    LocaleNamesScriptExtendedMediumV1,
    Script,
    cldr_serde::displaynames::script::Resource,
    "scripts.json",
    scripts,
    None,
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);

crate::displaynames::impl_displaynames_v1!(
    LocaleNamesScriptExtendedShortV1,
    Script,
    cldr_serde::displaynames::script::Resource,
    "scripts.json",
    scripts,
    Some(Alt::Short),
    CoverageLevelForXPath::Modern | CoverageLevelForXPath::Comprehensive,
);

crate::displaynames::impl_displaynames_legacy_iter_v1!(ScriptDisplayNamesV1, "scripts.json");

impl From<&cldr_serde::displaynames::script::Resource> for ScriptDisplayNames<'static> {
    fn from(other: &cldr_serde::displaynames::script::Resource) -> Self {
        let extracted = extract_names_for_zeromap_struct(
            &other.main.value.localedisplaynames.scripts,
            &[Alt::Variant, Alt::Secondary, Alt::StandAlone],
            "script",
            |script| Some(script.to_tinystr()),
        );

        let to_zero_map = |map: BTreeMap<tinystr::TinyAsciiStr<4>, &str>| {
            map.into_iter()
                .map(|(k, v)| (k.to_unvalidated(), v))
                .collect()
        };

        Self {
            names: to_zero_map(extracted.names),
            short_names: to_zero_map(extracted.short_names),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::displaynames::CheckAltCoverage;

    use super::*;
    use icu::locale::{langid, subtags::script};

    #[test]
    fn test_basic_script_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<ScriptDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .names
                .get(&script!("Cans").to_tinystr().to_unvalidated())
                .unwrap(),
            "Unified Canadian Aboriginal Syllabics"
        );
    }

    #[test]
    fn test_basic_script_short_display_names() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<ScriptDisplayNamesV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .short_names
                .get(&script!("Cans").to_tinystr().to_unvalidated())
                .unwrap(),
            "UCAS"
        );
    }

    #[test]
    fn test_locale_names_script_medium() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesScriptCoreMediumV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("Arab").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "Arabic");
    }

    #[test]
    fn test_locale_names_script_short() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesScriptExtendedShortV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str("Cans").unwrap(),
                    &langid!("en-001").into(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(&**data.get(), "UCAS");
    }

    /// The cartesian product of Script x (Short | Medium) x (Minimal | Core | Extended)
    /// contains some data markers that are uninhabited. This test ensures that every script display name
    /// key and coverage tier combination in CLDR is covered by an existing marker, so if future CLDR releases
    /// add data for uninhabited markers, we learn about it and can take action.
    #[test]
    fn test_empty_coverage_tiers_assert_no_data() {
        let provider = SourceDataProvider::new_testing();
        let cldr = provider.cldr().unwrap();

        crate::displaynames::for_each_cldr_key_and_tier(
            cldr,
            "scripts.json",
            "scripts",
            |res: &cldr_serde::displaynames::script::Resource| {
                &res.main.value.localedisplaynames.scripts
            },
            |locale, key, tier| {
                if LocaleNamesScriptMinimalMediumV1::contains_key(key, tier)
                    || LocaleNamesScriptCoreMediumV1::contains_key(key, tier)
                    || LocaleNamesScriptExtendedMediumV1::contains_key(key, tier)
                    || LocaleNamesScriptExtendedShortV1::contains_key(key, tier)
                {
                    return;
                }

                panic!(
                    "Found unexpected alt, menu, and tier combination for script: {key:?} in locale: {locale:?} and tier: {tier:?}"
                );
            },
        );
    }
}
