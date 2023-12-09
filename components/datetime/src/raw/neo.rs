// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::calendar::CldrCalendar;
use icu_provider::prelude::*;
use crate::provider::neo::*;
use crate::format::neo::*;

pub(crate) enum DatePatternSelectionData {
    SingleDate(DataPayload<ErasedDatePatternV1Marker>),
    OptionalEra {
        with_era: DataPayload<ErasedDatePatternV1Marker>,
        without_era: DataPayload<ErasedDatePatternV1Marker>,
    },
}

pub(crate) enum TimePatternData {
    SingleTime(DataPayload<TimePatternV1Marker>),
}

pub(crate) enum DateTimePatternSelectionData {
    Date(DatePatternSelectionData),
    Time(TimePatternData),
    Glue(DataPayload<DateTimePatternV1Marker>),
}

pub(crate) struct TypedNeoDateTimeFormatter<C: CldrCalendar> {
    names: TypedDateTimeNames<C>,
}

pub(crate) struct RawNeoDateTimeFormatter {
    names: RawDateTimeNames,
    selection: DateTimePatternSelectionData,
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
    }
}
