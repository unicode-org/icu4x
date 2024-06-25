// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::cldr_serde::units::data::Patterns;
use crate::cldr_serde::{self};
use crate::DatagenProvider;

use icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1Marker;

use icu::locale::LanguageIdentifier;
use icu_experimental::dimension::provider::units_essentials::UnitsEssentialsV1;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;
use zerovec::ZeroMap;

impl DataProvider<UnitsEssentialsV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsEssentialsV1Marker>, DataError> {
        self.check_req::<UnitsEssentialsV1Marker>(req)?;

        // Get langid and the unit.
        let langid = req.locale.get_langid();
        let length = match req.marker_attributes.parse::<String>() {
            Ok(aux_keys) => aux_keys,
            Err(_) => return Err(DataError::custom("Failed to get aux keys")),
        };

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource =
            self.cldr()?.units().read_and_parse(&langid, "units.json")?;
        let units_format_data = &units_format_data.main.value.units;
        let length_data = match length.as_str() {
            "long" => &units_format_data.long,
            "short" => &units_format_data.short,
            "narrow" => &units_format_data.narrow,
            _ => return Err(DataError::custom("Failed to get length data")),
        };

        let result = UnitsEssentialsV1 {
            per: "per".into(),
            times: "times".into(),
            prefixes: ZeroMap::new(),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<UnitsEssentialsV1Marker> for DatagenProvider {
    fn iter_requests_cached(
        &self,
    ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        let units = self.cldr()?.units();
        let langids = units.list_langs()?;
        let data_locales: HashSet<(DataLocale, DataMarkerAttributes)> = langids
            .into_iter()
            .flat_map(|langid| {
                ["long", "short", "narrow"].iter().map(move |&length| {
                    let data_locale = DataLocale::from(langid.clone());
                    let attribute: DataMarkerAttributes = length.parse().map_err(|_| {
                        DataError::custom("Failed to parse the attribute")
                            .with_debug_context(length)
                    })?;
                    Ok::<(DataLocale, DataMarkerAttributes), DataError>((data_locale, attribute))
                })
            })
            .collect::<Result<HashSet<_>, _>>()?;
        Ok(data_locales)
    }
}

// #[test]
// fn test_basic() {
//     use icu::locale::langid;
//     use icu_provider::prelude::*;

//     let provider = DatagenProvider::new_testing();

//     let us_locale: DataPayload<UnitsEssentialsV1Marker> = provider
//         .load(DataRequest {
//             locale: &langid!("en").into(),
//             marker_attributes: &"meter".parse().unwrap(),
//             ..Default::default()
//         })
//         .unwrap()
//         .payload;

//     let units_us = us_locale.get().to_owned();
//     let long = units_us.long.get(&Count::One).unwrap();
//     assert_eq!(long, "{0} meter");
//     let short = units_us.short.get(&Count::One).unwrap();
//     assert_eq!(short, "{0} m");
//     let narrow = units_us.narrow.get(&Count::One).unwrap();
//     assert_eq!(narrow, "{0}m");

//     let ar_eg_locale: DataPayload<UnitsEssentialsV1Marker> = provider
//         .load(DataRequest {
//             locale: &langid!("ar-EG").into(),
//             marker_attributes: &"meter".parse().unwrap(),
//             ..Default::default()
//         })
//         .unwrap()
//         .payload;

//     let ar_eg_units = ar_eg_locale.get().to_owned();
//     let long = ar_eg_units.long.get(&Count::One).unwrap();
//     assert_eq!(long, "متر");
//     let short = ar_eg_units.short.get(&Count::One).unwrap();
//     assert_eq!(short, "متر");
//     let narrow = ar_eg_units.narrow.get(&Count::One).unwrap();
//     assert_eq!(narrow, "{0} م");

//     let fr_locale: DataPayload<UnitsEssentialsV1Marker> = provider
//         .load(DataRequest {
//             locale: &langid!("fr").into(),
//             marker_attributes: &"meter".parse().unwrap(),
//             ..Default::default()
//         })
//         .unwrap()
//         .payload;

//     let fr_units = fr_locale.get().to_owned();
//     let long = fr_units.long.get(&Count::One).unwrap();
//     assert_eq!(long, "{0} mètre");
//     let short = fr_units.short.get(&Count::One).unwrap();
//     assert_eq!(short, "{0} m");
//     let narrow = fr_units.narrow.get(&Count::One).unwrap();
//     assert_eq!(narrow, "{0}m");
// }
