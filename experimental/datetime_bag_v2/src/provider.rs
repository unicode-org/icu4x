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

#[icu_provider::data_struct(FormatLengthsV1Marker = "datetime/formatlengths@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FormatLengthsV1<'data> {
    // Isn't this already provided in datetime/timelengths?
    // pub preferred_hour_cycle: pattern::CoarseHourCycle
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub long: FormatPatternsV1<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub medium: FormatPatternsV1<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short: FormatPatternsV1<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub short_lossy: FormatPatternsV1<'data>,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct FormatPatternsV1<'data> {
    pub glue: DateTimeFormatGlueV1<'data>,
    pub date: DateFormatV1<'data>,
    pub date_weekday: DateWeekdayFormatV1<'data>,
    pub time: TimeFormatV1<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub weekday: Pattern<'data>,
    pub time_zone: (), // TODO
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DateTimeFormatGlueV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub datetime: GenericPatternV1<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub time_zone: GenericPatternV1<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub weekday: MixedPatternV1<'data>,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DateFormatV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub glue_era: MixedPatternV1<'data>,
    pub components: DateFormatComponentsV1<'data>,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DateFormatComponentsV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub year_month_day: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub year_month: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub year: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub era: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub month_day: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub month: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub day: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub era_year_month_day: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub era_year_month: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub era_year: Option<Pattern<'data>>,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DateWeekdayFormatV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub year_month_day: Option<MixedPatternV1<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub year_month: Option<MixedPatternV1<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub year: Option<MixedPatternV1<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub era: Option<MixedPatternV1<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub month_day: Option<MixedPatternV1<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub month: Option<MixedPatternV1<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub day: Option<MixedPatternV1<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub era_year_month_day: Option<MixedPatternV1<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub era_year_month: Option<MixedPatternV1<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub era_year: Option<MixedPatternV1<'data>>,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct TimeFormatV1<'data> {
    // TODO: is `glue` used in time formats?
    // pub glue: (),
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub h11_h12: TimeFormatComponentsV1<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub h23_h24: TimeFormatComponentsV1<'data>,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct TimeFormatComponentsV1<'data> {
    // TODO: is `glue` used in time formats?
    // pub glue: (),
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub hour_minute_second_fractional_second: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub hour_minute_second: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub hour_minute: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub hour: Pattern<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub weekday_hour_minute_second_fractional_second: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub weekday_hour_minute_second: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub weekday_hour_minute: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub weekday_hour: Option<Pattern<'data>>,
}
