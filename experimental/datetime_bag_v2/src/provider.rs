use icu_datetime::{pattern::runtime::Pattern, provider::calendar::patterns::GenericPatternV1};
use icu_provider::{yoke, zerofrom};

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
    // TODO: Several fields use a single `FieldSymbol` and a `GenericPattern`
    // or a single `FieldSymbol`, so they should be defined with a custom struct
    pub weekday: GenericPatternV1<'data>,
}

pub struct DateFormatV1<'data> {
    // TODO: Same as `DateTimeGlueV1::weekday`
    pub glue_era: GenericPatternV1<'data>,
    pub components: DateFormatComponentsV1<'data>,
}

pub struct DateFormatComponentsV1<'data> {
    // TODO: must definitely wrap all `Pattern`s into a `PatternV1` struct,
    // or should `components` also allow plural alternatives?
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
    // TODO: This uses generic and regular patterns, so it should be
    // its own custom struct
    pub year_month_day: Option<GenericPatternV1<'data>>,
    pub year_month: Option<GenericPatternV1<'data>>,
    pub year: Option<GenericPatternV1<'data>>,
    pub era: Option<GenericPatternV1<'data>>,
    pub month_day: Option<GenericPatternV1<'data>>,
    pub month: Option<GenericPatternV1<'data>>,
    pub day: Option<GenericPatternV1<'data>>,
    pub era_year_month_day: Option<GenericPatternV1<'data>>,
    pub era_year_month: Option<GenericPatternV1<'data>>,
    pub era_year: Option<GenericPatternV1<'data>>,
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
