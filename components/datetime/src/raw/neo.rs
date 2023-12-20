// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::format::neo::*;
use crate::input::ExtractedDateTimeInput;
use crate::options::length;
use crate::pattern::runtime;
use crate::provider::neo::*;
use crate::Error;
use icu_plurals::PluralCategory;
use icu_provider::prelude::*;
use zerovec::ZeroMap;

#[derive(Debug)]
pub(crate) enum DatePatternSelectionData {
    SingleDate(DataPayload<ErasedDatePatternV1Marker>),
    #[allow(dead_code)] // TODO(#4478)
    OptionalEra {
        with_era: DataPayload<ErasedDatePatternV1Marker>,
        without_era: DataPayload<ErasedDatePatternV1Marker>,
    },
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DatePatternDataBorrowed<'a> {
    Resolved(&'a DatePatternV1<'a>),
}

#[derive(Debug)]
pub(crate) struct RawNeoDateFormatter {
    names: RawDateTimeNames,
    selection: DatePatternSelectionData,
}

#[derive(Debug)]
pub(crate) enum TimePatternSelectionData {
    SingleTime(DataPayload<TimePatternV1Marker>),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TimePatternDataBorrowed<'a> {
    Resolved(&'a TimePatternV1<'a>),
}

#[derive(Debug)]
pub(crate) struct RawNeoTimeFormatter {
    names: RawDateTimeNames,
    selection: TimePatternSelectionData,
}

#[derive(Debug)]
pub(crate) enum DateTimePatternSelectionData {
    Date(DatePatternSelectionData),
    Time(TimePatternSelectionData),
    DateTime {
        date: DatePatternSelectionData,
        time: TimePatternSelectionData,
        glue: DataPayload<DateTimePatternV1Marker>,
    },
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DateTimePatternDataBorrowed<'a> {
    Date(DatePatternDataBorrowed<'a>),
    Time(TimePatternDataBorrowed<'a>),
    DateTime {
        date: DatePatternDataBorrowed<'a>,
        time: TimePatternDataBorrowed<'a>,
        glue: &'a DateTimePatternV1<'a>,
    },
}

#[derive(Debug)]
pub(crate) struct RawNeoDateTimeFormatter {
    names: RawDateTimeNames,
    selection: DateTimePatternSelectionData,
}

impl DatePatternSelectionData {
    pub(crate) fn try_new_with_length<M, P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, Error>
    where
        P: DataProvider<M> + ?Sized,
        M: KeyedDataMarker<Yokeable = DatePatternV1<'static>>,
    {
        let mut locale = locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::pattern_subtag_for(
            match length {
                length::Date::Full => aux::PatternLength::Full,
                length::Date::Long => aux::PatternLength::Long,
                length::Date::Medium => aux::PatternLength::Medium,
                length::Date::Short => aux::PatternLength::Short,
            },
            None, // no hour cycle for date patterns
        )));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?
            .cast();
        Ok(Self::SingleDate(payload))
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    pub(crate) fn pattern_for_data_loading(&self) -> &runtime::Pattern {
        match self {
            DatePatternSelectionData::SingleDate(payload) => &payload.get().pattern,
            // Assumption: with_era has all the fields of without_era
            DatePatternSelectionData::OptionalEra { with_era, .. } => &with_era.get().pattern,
        }
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, _datetime: &ExtractedDateTimeInput) -> DatePatternDataBorrowed {
        match self {
            DatePatternSelectionData::SingleDate(payload) => {
                DatePatternDataBorrowed::Resolved(payload.get())
            }
            DatePatternSelectionData::OptionalEra { .. } => unimplemented!("#4478"),
        }
    }
}

impl TimePatternSelectionData {
    /// Borrows a pattern containing all of the fields that need to be loaded.
    pub(crate) fn pattern_for_data_loading(&self) -> &runtime::Pattern {
        match self {
            TimePatternSelectionData::SingleTime(payload) => &payload.get().pattern,
        }
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, _datetime: &ExtractedDateTimeInput) -> TimePatternDataBorrowed {
        match self {
            TimePatternSelectionData::SingleTime(payload) => {
                TimePatternDataBorrowed::Resolved(payload.get())
            }
        }
    }
}

impl DateTimePatternSelectionData {
    /// Borrows a pattern containing all of the fields that need to be loaded.
    pub(crate) fn pattern_for_data_loading(&self) -> &runtime::Pattern {
        match self {
            DateTimePatternSelectionData::Date(date) => date.pattern_for_data_loading(),
            DateTimePatternSelectionData::Time(time) => time.pattern_for_data_loading(),
            DateTimePatternSelectionData::DateTime { date, time, glue } => todo!(),
        }
    }

    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, datetime: &ExtractedDateTimeInput) -> DateTimePatternDataBorrowed {
        match self {
            DateTimePatternSelectionData::Date(date) => {
                DateTimePatternDataBorrowed::Date(date.select(datetime))
            }
            DateTimePatternSelectionData::Time(time) => {
                DateTimePatternDataBorrowed::Time(time.select(datetime))
            }
            DateTimePatternSelectionData::DateTime { date, time, glue } => todo!(),
        }
    }
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
        assert_eq!(
            48,
            core::mem::size_of::<DataPayload<ErasedDatePatternV1Marker>>()
        );
        assert_eq!(80, core::mem::size_of::<DataLocale>());
        assert_eq!(216, core::mem::size_of::<FixedDecimalFormatter>());
        assert_eq!(2, core::mem::size_of::<WeekCalculator>());
        assert_eq!(
            72,
            core::mem::size_of::<DataPayload<ErasedYearNamesV1Marker>>()
        );
        assert_eq!(
            56,
            core::mem::size_of::<DataPayload<ErasedMonthNamesV1Marker>>()
        );
        assert_eq!(
            40,
            core::mem::size_of::<DataPayload<WeekdayNamesV1Marker>>()
        );
        assert_eq!(
            40,
            core::mem::size_of::<DataPayload<DayPeriodNamesV1Marker>>()
        );
        assert_eq!(640, core::mem::size_of::<RawNeoDateTimeFormatter>());

        assert_eq!(32, core::mem::size_of::<DatePatternV1>());
        assert_eq!(56, core::mem::size_of::<Foo1>());
        assert_eq!(48, core::mem::size_of::<Foo2>());
    }
}
