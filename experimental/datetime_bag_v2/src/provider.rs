// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::{
    pattern::runtime::Pattern,
    provider::calendar::patterns::{GenericPatternV1, MixedPatternV1},
};
use icu_provider::{yoke, zerofrom};

// TODO: must definitely wrap all `Pattern`s into a `PatternV1` struct,
// or should those allow plural alternatives?

// won't implement this for now
// #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
// #[cfg_attr(
//         feature = "datagen",
//         derive(serde::Serialize, databake::Bake),
//         // databake(path = icu_datetime::provider::calendar::patterns),
//     )]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DateTimeFormatLengthsV1<'data> {
    // Isn't this already provided in datetime/timelengths?
    // pub preferred_hour_cycle: pattern::CoarseHourCycle
    pub long: DateTimeFormatV1<'data>,
    pub medium: DateTimeFormatV1<'data>,
    pub short: DateTimeFormatV1<'data>,
    pub short_lossy: DateTimeFormatV1<'data>,
}

pub struct DateTimeFormatV1<'data> {
    pub glue: DateTimeFormatGlueV1<'data>,
    pub date: DateFormatV1<'data>,
    pub date_weekday: DateWeekdayFormatV1<'data>,
    pub time: TimeFormatV1<'data>,
    pub weekday: Pattern<'data>,
    pub time_zone: (), // TODO
}
pub struct DateTimeFormatGlueV1<'data> {
    pub datetime: GenericPatternV1<'data>,
    pub time_zone: GenericPatternV1<'data>,
    pub weekday: MixedPatternV1<'data>,
}

pub struct DateFormatV1<'data> {
    pub glue_era: MixedPatternV1<'data>,
    pub components: DateFormatComponentsV1<'data>,
}

pub struct DateFormatComponentsV1<'data> {
    pub year_month_day: Pattern<'data>,
    pub year_month: Pattern<'data>,
    pub year: Pattern<'data>,
    pub era: Pattern<'data>,
    pub month_day: Pattern<'data>,
    pub month: Pattern<'data>,
    pub day: Pattern<'data>,
    pub era_year_month_day: Option<Pattern<'data>>,
    pub era_year_month: Option<Pattern<'data>>,
    pub era_year: Option<Pattern<'data>>,
}

pub struct DateWeekdayFormatV1<'data> {
    pub year_month_day: Option<MixedPatternV1<'data>>,
    pub year_month: Option<MixedPatternV1<'data>>,
    pub year: Option<MixedPatternV1<'data>>,
    pub era: Option<MixedPatternV1<'data>>,
    pub month_day: Option<MixedPatternV1<'data>>,
    pub month: Option<MixedPatternV1<'data>>,
    pub day: Option<MixedPatternV1<'data>>,
    pub era_year_month_day: Option<MixedPatternV1<'data>>,
    pub era_year_month: Option<MixedPatternV1<'data>>,
    pub era_year: Option<MixedPatternV1<'data>>,
}

pub struct TimeFormatV1<'data> {
    // TODO: is `glue` used in time formats?
    // pub glue: (),
    pub h11_h12: TimeFormatComponentsV1<'data>,
    pub h23_h24: TimeFormatComponentsV1<'data>,
}

pub struct TimeFormatComponentsV1<'data> {
    // TODO: is `glue` used in time formats?
    // pub glue: (),
    pub hour_minute_second_fractional_second: Option<Pattern<'data>>,
    pub hour_minute_second: Pattern<'data>,
    pub hour_minute: Pattern<'data>,
    pub hour: Pattern<'data>,
    pub weekday_hour_minute_second_fractional_second: Option<Pattern<'data>>,
    pub weekday_hour_minute_second: Option<Pattern<'data>>,
    pub weekday_hour_minute: Option<Pattern<'data>>,
    pub weekday_hour: Option<Pattern<'data>>,
}
