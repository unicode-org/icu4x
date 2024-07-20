// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::units::data::DurationUnits;
use crate::SourceDataProvider;
use icu::experimental::duration::provider::digital::{HmPadding, HmsPadding, MsPadding};
use icu_provider::prelude::*;
use itertools::Itertools;
use std::{borrow::Cow, collections::HashSet};

use icu::experimental::duration::provider::digital::{
    DigitalDurationDataV1, DigitalDurationDataV1Marker,
};

/// Strips multiples of the given character from the start of the string.
/// Returns padding size and modifies `s` to point to the stripped string.
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

        /// Strips the passed padded characters separated by a uniform separator.
        /// Returns the padding sizes and the separator.
        /// Modifies `s` to point to the remaining string.
        /// Returns an error if the separator is missing or inconsistent.
        fn strip_separated_padded_characters<'s, const N: usize>(
            s: &mut &'s str,
            chars: [char; N],
        ) -> Result<([u8; N], &'s str), DataError> {
            let mut padding = [0u8; N];
            let mut sep = None;

            for (p, (c, next_c)) in padding.iter_mut().zip(chars.iter().tuple_windows()) {
                *p = strip_padded_character(s, *c);
                let (curr_sep, rest) = s
                    .split_once(*next_c)
                    .ok_or_else(|| DataError::custom("Missing separator in pattern"))?;
                *s = rest;
                match sep {
                    Some(s) => {
                        if s != curr_sep {
                            return Err(DataError::custom("Inconsistent separators in pattern"));
                        }
                    }
                    None => sep = Some(curr_sep),
                }
            }
            // Handle last padded character.
            padding[N - 1] = strip_padded_character(s, chars[N - 1]);

            if !s.is_empty() {
                return Err(DataError::custom(
                    "Unexpected characters in duration patterns",
                ));
            }

            Ok((
                padding,
                sep.ok_or_else(|| DataError::custom("Missing separator in pattern"))?,
            ))
        }

        // Find paddings for hm
        let mut hm = hm.pat.as_str();
        let ([hm_hour_pad, hm_min_pad], hm_sep) =
            strip_separated_padded_characters(&mut hm, ['h', 'm'])?;

        // Find paddings for ms
        let mut ms = ms.pat.as_str();
        let ([ms_min_pad, ms_sec_pad], ms_sep) =
            strip_separated_padded_characters(&mut ms, ['m', 's'])?;

        // Find paddings for hms
        let mut hms = hms.pat.as_str();
        let ([hms_hour_pad, hms_min_pad, hms_sec_pad], hms_sep) =
            strip_separated_padded_characters(&mut hms, ['h', 'm', 's'])?;

        // Check that separators are consistent
        if hm_sep != hms_sep || ms_sep != hms_sep || hm_sep != ms_sep {
            return Err(DataError::custom(
                "Inconsistent separators in duration patterns",
            ));
        }

        let result = DigitalDurationDataV1 {
            separator: Cow::Owned(hm_sep.to_string()),
            hms_padding: HmsPadding {
                h: hms_hour_pad,
                m: hms_min_pad,
                s: hms_sec_pad,
            },
            hm_padding: HmPadding {
                h: hm_hour_pad,
                m: hm_min_pad,
            },
            ms_padding: MsPadding {
                m: ms_min_pad,
                s: ms_sec_pad,
            },
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
