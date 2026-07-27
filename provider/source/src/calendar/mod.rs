// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{IterableDataProviderCached, SourceDataProvider};
use icu::calendar::preferences::CalendarAlgorithm;
use icu::calendar::provider::{CalendarPreference, CalendarPreferredV1};
use icu::locale::{
    LanguageIdentifier,
    extensions::unicode::Value,
    preferences::extensions::unicode::keywords::HijriCalendarAlgorithm,
    subtags::{Language, Region, region},
};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashSet};

/// Data for calendar arithmetic
pub(crate) mod eras;

#[derive(serde::Deserialize)]
struct Resource {
    supplemental: Supplemental,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Supplemental {
    calendar_preference_data: BTreeMap<Region, Vec<String>>,
}

impl DataProvider<CalendarPreferredV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarPreferredV1>, DataError> {
        self.check_req::<CalendarPreferredV1>(req)?;

        let mut algorithms = self
            .cldr()
            .unwrap()
            .core()
            .read_and_parse::<Resource>("supplemental/calendarPreferenceData.json")
            .unwrap()
            .supplemental
            .calendar_preference_data[&req.id.locale.region.unwrap_or(region!("001"))]
            .iter()
            .map(|a| match a.as_str() {
                "gregorian" => CalendarAlgorithm::Gregory,
                a => CalendarAlgorithm::try_from(&Value::try_from_str(a).unwrap()).unwrap(),
            });

        let default_algorithm = algorithms
            .clone()
            .next()
            .unwrap_or(CalendarAlgorithm::Gregory);

        let default_hijri_algorithm = algorithms
            .find_map(|a| {
                if let CalendarAlgorithm::Hijri(Some(h)) = a {
                    Some(h)
                } else {
                    None
                }
            })
            .unwrap_or(HijriCalendarAlgorithm::Civil);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CalendarPreference {
                default_algorithm,
                default_hijri_algorithm,
            }),
        })
    }
}

impl IterableDataProviderCached<CalendarPreferredV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()
            .unwrap()
            .core()
            .read_and_parse::<Resource>("supplemental/calendarPreferenceData.json")
            .unwrap()
            .supplemental
            .calendar_preference_data
            .keys()
            .map(|&region| {
                DataIdentifierCow::from_locale(
                    LanguageIdentifier::from((Language::UNKNOWN, None, Some(region))).into(),
                )
            })
            .chain([Default::default()])
            .collect())
    }
}
