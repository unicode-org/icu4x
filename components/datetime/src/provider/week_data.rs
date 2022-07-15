// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;

/// An ICU4X mapping to a subset of CLDR weekData.
/// See CLDR-JSON's weekData.json for more context.
#[icu_provider::data_struct(
    marker(WeekDataV1Marker, "datetime/week_data@1", fallback_by = "region")
)]
#[derive(Clone, Copy, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::provider::week_data),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct WeekDataV1(pub icu_calendar::arithmetic::week_of::CalendarInfo);
