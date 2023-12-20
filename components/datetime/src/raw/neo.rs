// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::calendar::CldrCalendar;
use crate::input::ExtractedDateTimeInput;
use crate::pattern::runtime;
use icu_plurals::PluralCategory;
use icu_provider::prelude::*;
use zerovec::ZeroMap;
use crate::provider::neo::*;
use crate::format::neo::*;

pub(crate) enum DatePatternSelectionData {
    SingleDate(DataPayload<ErasedDatePatternV1Marker>),
    OptionalEra {
        with_era: DataPayload<ErasedDatePatternV1Marker>,
        without_era: DataPayload<ErasedDatePatternV1Marker>,
    },
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DatePatternDataBorrowed<'a> {
    Resolved(&'a DatePatternV1<'a>)
}

pub(crate) struct RawNeoDateFormatter {
    names: RawDateTimeNames,
    selection: DatePatternSelectionData,
}

pub(crate) struct FormattedNeoDate<'a> {
    pattern: DatePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

pub(crate) enum TimePatternSelectionData {
    SingleTime(DataPayload<TimePatternV1Marker>),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TimePatternDataBorrowed<'a> {
    Resolved(&'a TimePatternV1<'a>)
}

pub(crate) struct RawNeoTimeFormatter {
    names: RawDateTimeNames,
    selection: TimePatternSelectionData,
}

pub(crate) struct FormattedNeoTime<'a> {
    pattern: TimePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

pub(crate) enum DateTimePatternSelectionData {
    Date(DatePatternSelectionData),
    Time(TimePatternSelectionData),
    DateTime {
        date: DatePatternSelectionData,
        time: TimePatternSelectionData,
        glue: DataPayload<DateTimePatternV1Marker>,
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DateTimePatternDataBorrowed<'a> {
    Date(DatePatternDataBorrowed<'a>),
    Time(TimePatternDataBorrowed<'a>),
    DateTime {
        date: DatePatternDataBorrowed<'a>,
        time: TimePatternDataBorrowed<'a>,
        glue: &'a DateTimePatternV1<'a>,
    }
}

pub(crate) struct RawNeoDateTimeFormatter {
    names: RawDateTimeNames,
    selection: DateTimePatternSelectionData,
}

pub(crate) struct FormattedNeoDateTime<'a> {
    pattern: DateTimePatternDataBorrowed<'a>,
    datetime: ExtractedDateTimeInput,
    names: RawDateTimeNamesBorrowed<'a>,
}

pub(crate) struct TypedNeoDateTimeFormatter<C: CldrCalendar> {
    names: TypedDateTimeNames<C>,
}

pub(crate) enum Foo1<'data> {
    SingleDate(runtime::Pattern<'data>),
    WeekPlurals(ZeroMap<'data, PluralCategory, runtime::PatternULE>),
}

pub(crate) struct Foo2<'data> {
    map: ZeroMap<'data, PluralCategory, runtime::PatternULE>,
}

#[cfg(test)]
mod tests {
    use icu_calendar::week::WeekCalculator;
    use icu_decimal::FixedDecimalFormatter;

    use super::*;

    #[test]
    fn test_sizes() {
        assert_eq!(48, core::mem::size_of::<DataPayload<ErasedDatePatternV1Marker>>());
        assert_eq!(80, core::mem::size_of::<DataLocale>());
        assert_eq!(216, core::mem::size_of::<FixedDecimalFormatter>());
        assert_eq!(2, core::mem::size_of::<WeekCalculator>());
        assert_eq!(72, core::mem::size_of::<DataPayload<ErasedYearNamesV1Marker>>());
        assert_eq!(56, core::mem::size_of::<DataPayload<ErasedMonthNamesV1Marker>>());
        assert_eq!(40, core::mem::size_of::<DataPayload<WeekdayNamesV1Marker>>());
        assert_eq!(40, core::mem::size_of::<DataPayload<DayPeriodNamesV1Marker>>());
        assert_eq!(640, core::mem::size_of::<RawNeoDateTimeFormatter>());

        assert_eq!(32, core::mem::size_of::<DatePatternV1>());
        assert_eq!(56, core::mem::size_of::<Foo1>());
        assert_eq!(48, core::mem::size_of::<Foo2>());
    }
}
