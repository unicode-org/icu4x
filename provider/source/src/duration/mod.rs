// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::units::data::DurationUnits;
use crate::SourceDataProvider;
use icu_provider::prelude::*;
use std::{borrow::Cow, collections::HashSet};

use icu::experimental::duration::provider::digital::{
    DigitalDurationDataV1, DigitalDurationDataV1Marker,
};

/// Strips multiples of the given character from the start of the string.
/// Returns padding size and modifies s to point to the stripped string.
fn strip_padded_character(s: &mut &str, c: char) -> u8 {
    let mut padding = 0;
    while s.starts_with(c) {
        padding += 1;
        *s = &s[1..];
    }
    padding
}

impl DataProvider<DigitalDurationDataV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<DigitalDurationDataV1Marker>, DataError> {
        self.check_req::<DigitalDurationDataV1Marker>(req)?;
        let langid = req.id.locale.get_langid();

        // Get units
        let units_format_data: &cldr_serde::units::data::Resource =
            self.cldr()?.units().read_and_parse(&langid, "units.json")?;
        let DurationUnits { hms, hm, ms } = &units_format_data.main.value.units.duration;

        // Find hour padding for hm
        let mut hm = hm.pat.as_str();
        let hour_padding_hm = strip_padded_character(&mut hm, 'h');
        let (hm_separator, _) = hm
            .split_once(|c| c == 'm')
            .ok_or_else(|| DataError::custom("Missing separator in hm pattern"))?;

        // Find minute padding for ms
        let mut ms = ms.pat.as_str();
        let minute_padding = strip_padded_character(&mut ms, 'm');
        let (ms_separator, _) = ms
            .split_once(|c| c == 's')
            .ok_or_else(|| DataError::custom("Missing separator in ms pattern"))?;

        // Find hour padding for hms
        let mut hms = hms.pat.as_str();
        let hour_padding_hms = strip_padded_character(&mut hms, 'h');
        let (hms_separator, _) = hms
            .split_once(|c| c == 'm')
            .ok_or_else(|| DataError::custom("Missing separator in hms pattern"))?;

        // Check that separators are consistent
        if hm_separator != hms_separator
            || ms_separator != hms_separator
            || hm_separator != ms_separator
        {
            return Err(DataError::custom(
                "Inconsistent separators in duration patterns",
            ));
        }

        let result = DigitalDurationDataV1 {
            separator: Cow::Owned(hm_separator.to_string()),
            hms_hour_padding: hour_padding_hm,
            hm_hour_padding: hour_padding_hm,
            minute_padding,
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl crate::IterableDataProviderCached<DigitalDurationDataV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_langs()?
            .filter(|langid| {
                self.cldr()
                    .unwrap()
                    .units()
                    .read_and_parse::<cldr_serde::units::data::Resource>(langid, "units.json")
                    .is_ok()
            })
            .map(|langid| DataIdentifierCow::from_locale(DataLocale::from(&langid)))
            .collect())
    }
}
