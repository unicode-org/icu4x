// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;

/// An ICU4X mapping to a subset of CLDR weekData.
/// See CLDR-JSON's weekData.json for more context.
#[icu_provider::data_struct(WeekDataV1Marker = "datetime/week_data@1")]
#[derive(Clone, Copy, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct WeekDataV1(pub icu_calendar::arithmetic::week_of::CalendarInfo);
