// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde::{self};
use crate::DatagenProvider;

use icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1Marker;

use icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1;
use icu_provider::prelude::*;
use icu_provider::DataMarkerAttributes;
use zerovec::ZeroMap;

impl DataProvider<UnitsEssentialsV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<UnitsEssentialsV1Marker>, DataError> {
        self.check_req::<UnitsEssentialsV1Marker>(req)?;

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource = self
            .cldr()?
            .units()
            .read_and_parse(&req.id.locale.get_langid(), "units.json")?;
        let units_format_data = &units_format_data.main.value.units;
        let length_data = match req.id.marker_attributes.as_str() {
            "long" => &units_format_data.long,
            "short" => &units_format_data.short,
            "narrow" => &units_format_data.narrow,
            _ => return Err(DataError::custom("Failed to get length data")),
        };
        let per = length_data
            .get("per")
            .and_then(|unit| unit.compound_unit_pattern.as_ref())
            .ok_or_else(|| DataError::custom("Failed to get per"))?
            .clone();

        let times = length_data
            .get("times")
            .and_then(|unit| unit.compound_unit_pattern.as_ref())
            .ok_or_else(|| DataError::custom("Failed to get times"))?
            .clone();

        // TODO: Fill prefixes (power, si prefixes, ... etc.) in the next PR.
        let prefixes = ZeroMap::new();

        let result = UnitsEssentialsV1 {
            per: per.into(),
            times: times.into(),
            prefixes,
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<UnitsEssentialsV1Marker> for DatagenProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let units = self.cldr()?.units();
        let langids = units.list_langs()?;
        Ok(langids
            .into_iter()
            .flat_map(|langid| {
                [
                    DataMarkerAttributes::from_str_or_panic("long"),
                    DataMarkerAttributes::from_str_or_panic("short"),
                    DataMarkerAttributes::from_str_or_panic("narrow"),
                ]
                .into_iter()
                .map(move |length| {
                    DataIdentifierCow::from_borrowed_and_owned(
                        length,
                        DataLocale::from(langid.clone()),
                    )
                })
            })
            .collect())
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;
    use icu_provider::prelude::*;

    let provider = DatagenProvider::new_testing();

    let us_long: DataPayload<UnitsEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long"),
                &langid!("en").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let per = us_long.get().per.to_string();
    assert_eq!(per, "{0} per {1}");

    let times = us_long.get().times.to_string();
    assert_eq!(times, "{0}-{1}");

    let fr_long: DataPayload<UnitsEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("long"),
                &langid!("fr").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let per = fr_long.get().per.to_string();
    assert_eq!(per, "{0} par {1}");

    let times = fr_long.get().times.to_string();
    assert_eq!(times, "{0}-{1}");

    let ar_eg_short: DataPayload<UnitsEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("short"),
                &langid!("ar").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let per = ar_eg_short.get().per.to_string();
    assert_eq!(per, "{0}/{1}");

    let times = ar_eg_short.get().times.to_string();
    assert_eq!(times, "{0}⋅{1}");
}
