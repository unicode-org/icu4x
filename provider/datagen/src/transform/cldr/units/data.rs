// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, HashSet};

use crate::provider::transform::cldr::cldr_serde::units::data::Patterns;
use crate::provider::transform::cldr::cldr_serde::{self};
use crate::provider::DatagenProvider;

use icu_experimental::dimension::provider::units::{
    Count, UnitsDisplayNameV1, UnitsDisplayNameV1Marker,
};

use icu_locale::LanguageIdentifier;
use icu_provider::DataKeyAttributes;
use icu_provider::{
    datagen::IterableDataProvider, DataError, DataLocale, DataPayload, DataProvider, DataRequest,
    DataResponse,
};
use zerovec::ZeroMap;

impl DataProvider<UnitsDisplayNameV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsDisplayNameV1Marker>, DataError> {
        self.check_req::<UnitsDisplayNameV1Marker>(req)?;

        // Get langid and the unit.
        let langid = req.locale.get_langid();
        let unit = match req.key_attributes.parse::<String>() {
            Ok(aux_keys) => aux_keys,
            Err(_) => return Err(DataError::custom("Failed to get aux keys")),
        };

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource =
            self.cldr()?.units().read_and_parse(&langid, "units.json")?;
        let units_format_data = &units_format_data.main.value.units;

        fn add_unit_to_map_with_name(
            map: &mut BTreeMap<Count, String>,
            count: Count,
            unit: Option<&str>,
        ) {
            if let Some(unit) = unit {
                map.insert(count, unit.to_string());
            }
        }

        fn populate_unit_map(
            unit_length_map: &BTreeMap<String, Patterns>,
            unit: &str,
            map: &mut BTreeMap<Count, String>,
        ) -> Result<(), DataError> {
            // TODO(younies): this should be coming from the aux key or from the main key.
            let legth_key = "length-".to_string() + unit;
            let duration_key = "duration-".to_string() + unit;

            let unit_length_map = match (
                unit_length_map.get(&legth_key),
                unit_length_map.get(&duration_key),
            ) {
                (Some(length), None) => length,
                (None, Some(length)) => length,
                _ => return Ok(()),
            };

            add_unit_to_map_with_name(map, Count::One, unit_length_map.one.as_deref());
            add_unit_to_map_with_name(map, Count::Two, unit_length_map.two.as_deref());
            add_unit_to_map_with_name(map, Count::Few, unit_length_map.few.as_deref());
            add_unit_to_map_with_name(map, Count::Many, unit_length_map.many.as_deref());
            add_unit_to_map_with_name(map, Count::Other, unit_length_map.other.as_deref());

            Ok(())
        }

        let mut long = BTreeMap::new();
        let mut short = BTreeMap::new();
        let mut narrow = BTreeMap::new();

        populate_unit_map(&units_format_data.long, unit.as_str(), &mut long)?;
        populate_unit_map(&units_format_data.short, unit.as_str(), &mut short)?;
        populate_unit_map(&units_format_data.narrow, unit.as_str(), &mut narrow)?;

        let result = UnitsDisplayNameV1 {
            long: ZeroMap::from_iter(long.iter().map(|(k, v)| (k, v.as_str()))),
            short: ZeroMap::from_iter(short.iter().map(|(k, v)| (k, v.as_str()))),
            narrow: ZeroMap::from_iter(narrow.iter().map(|(k, v)| (k, v.as_str()))),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProvider<UnitsDisplayNameV1Marker> for DatagenProvider {
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
        fn make_request_element(
            langid: &LanguageIdentifier,
            unit: &str,
        ) -> Result<(DataLocale, DataKeyAttributes), DataError> {
            let data_locale = DataLocale::from(langid);
            let attribute = unit.parse().map_err(|_| {
                DataError::custom("Failed to parse the attribute").with_debug_context(unit)
            })?;
            Ok((data_locale, attribute))
        }

        let mut data_locales = HashSet::new();

        let numbers = self.cldr()?.numbers();
        let langids = numbers.list_langs()?;
        for langid in langids {
            let units_format_data: &cldr_serde::units::data::Resource =
                self.cldr()?.units().read_and_parse(&langid, "units.json")?;
            let units_format_data = &units_format_data.main.value.units;
            let quantities: HashSet<_> = units_format_data
                // TODO(younies): shall we filter also on short and narrow, in case there are another units in these.
                .long
                .keys()
                .filter(|&long_key| {
                    !long_key.starts_with(|c: char| c.is_ascii_digit())
                        && !["per", "times", "power"]
                            .iter()
                            .any(|&prefix| long_key.starts_with(prefix))
                })
                // TODO(younies): this filter just as a start, Add the other categories later after finalizing the design.
                .filter(|&long_key| {
                    long_key.starts_with("length") || long_key.starts_with("duration")
                })
                .filter_map(|long_key| long_key.split_once('-').map(|(_, rest)| rest))
                .collect();

            for &truncated_quantity in &quantities {
                data_locales.insert(make_request_element(&langid, truncated_quantity)?);
            }
        }

        Ok(data_locales)
    }
}

#[test]
fn test_basic() {
    use icu_locale_core::langid;
    use icu_provider::prelude::*;

    let provider = DatagenProvider::new_testing();

    let us_locale: DataPayload<UnitsDisplayNameV1Marker> = provider
        .load(DataRequest {
            locale: &langid!("en").into(),
            key_attributes: &"meter".parse().unwrap(),
            ..Default::default()
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let units_us = us_locale.get().to_owned();
    let long = units_us.long.get(&Count::One).unwrap();
    assert_eq!(long, "{0} meter");
    let short = units_us.short.get(&Count::One).unwrap();
    assert_eq!(short, "{0} m");
    let narrow = units_us.narrow.get(&Count::One).unwrap();
    assert_eq!(narrow, "{0}m");

    let ar_eg_locale: DataPayload<UnitsDisplayNameV1Marker> = provider
        .load(DataRequest {
            locale: &langid!("ar-EG").into(),
            key_attributes: &"meter".parse().unwrap(),
            ..Default::default()
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let ar_eg_units = ar_eg_locale.get().to_owned();
    let long = ar_eg_units.long.get(&Count::One).unwrap();
    assert_eq!(long, "متر");
    let short = ar_eg_units.short.get(&Count::One).unwrap();
    assert_eq!(short, "متر");
    let narrow = ar_eg_units.narrow.get(&Count::One).unwrap();
    assert_eq!(narrow, "{0} م");

    let fr_locale: DataPayload<UnitsDisplayNameV1Marker> = provider
        .load(DataRequest {
            locale: &langid!("fr").into(),
            key_attributes: &"meter".parse().unwrap(),
            ..Default::default()
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let fr_units = fr_locale.get().to_owned();
    let long = fr_units.long.get(&Count::One).unwrap();
    assert_eq!(long, "{0} mètre");
    let short = fr_units.short.get(&Count::One).unwrap();
    assert_eq!(short, "{0} m");
    let narrow = fr_units.narrow.get(&Count::One).unwrap();
    assert_eq!(narrow, "{0}m");
}
