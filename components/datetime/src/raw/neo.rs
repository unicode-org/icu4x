// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::format::datetime::write_pattern;
use crate::format::neo::*;
use crate::input::{DateTimeInputWithWeekConfig, ExtractedDateTimeInput};
use crate::neo_pattern::DateTimePattern;
use crate::options::length;
use crate::pattern::runtime::PatternMetadata;
use crate::pattern::{runtime, PatternItem};
use crate::provider::neo::*;
use crate::Error;
use icu_provider::prelude::*;
use zerovec::ule::AsULE;

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
pub(crate) enum TimePatternSelectionData {
    SingleTime(DataPayload<TimePatternV1Marker>),
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum TimePatternDataBorrowed<'a> {
    Resolved(&'a TimePatternV1<'a>),
}

#[derive(Debug)]
pub(crate) enum DateTimePatternSelectionData {
    Date(DatePatternSelectionData),
    Time(TimePatternSelectionData),
    DateTimeGlue(DateTimeGluePatternSelectionData),
}

#[derive(Debug)]
pub(crate) struct DateTimeGluePatternSelectionData {
    date: DatePatternSelectionData,
    time: TimePatternSelectionData,
    glue: DataPayload<DateTimePatternV1Marker>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum DateTimePatternDataBorrowed<'a> {
    Date(DatePatternDataBorrowed<'a>),
    Time(TimePatternDataBorrowed<'a>),
    DateTimeGlue {
        date: DatePatternDataBorrowed<'a>,
        time: TimePatternDataBorrowed<'a>,
        glue: &'a DateTimePatternV1<'a>,
    },
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct DateTimeWriter<'a, 'b, I>
where
    I: Iterator<Item = PatternItem> + 'b,
    'a: 'b,
{
    pub(crate) datetime: &'b ExtractedDateTimeInput,
    pub(crate) names: RawDateTimeNamesBorrowed<'a>,
    pub(crate) pattern_items: I,
    pub(crate) pattern_metadata: PatternMetadata,
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
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(&self) -> impl Iterator<Item = PatternItem> + '_ {
        match self {
            DatePatternSelectionData::SingleDate(payload) => payload.get(),
            // Assumption: with_era has all the fields of without_era
            DatePatternSelectionData::OptionalEra { with_era, .. } => with_era.get(),
        }
        .pattern
        .items
        .iter()
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

impl<'a> DatePatternDataBorrowed<'a> {
    #[inline]
    pub(crate) fn metadata(&self) -> PatternMetadata {
        match self {
            Self::Resolved(data) => data.pattern.metadata,
        }
    }

    #[inline]
    pub(crate) fn iter_items(&self) -> impl Iterator<Item = PatternItem> + '_ {
        match self {
            Self::Resolved(data) => data.pattern.items.iter(),
        }
    }

    #[inline]
    pub(crate) fn to_pattern(&self) -> DateTimePattern {
        let pattern = match self {
            Self::Resolved(data) => &data.pattern,
        };
        DateTimePattern::from_runtime_pattern(pattern.clone().into_owned())
    }
}

impl TimePatternSelectionData {
    pub(crate) fn try_new_with_length<P>(
        provider: &P,
        locale: &DataLocale,
        length: length::Time,
    ) -> Result<Self, Error>
    where
        P: DataProvider<TimePatternV1Marker> + ?Sized,
    {
        let mut locale = locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::pattern_subtag_for(
            match length {
                length::Time::Full => aux::PatternLength::Full,
                length::Time::Long => aux::PatternLength::Long,
                length::Time::Medium => aux::PatternLength::Medium,
                length::Time::Short => aux::PatternLength::Short,
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
        Ok(Self::SingleTime(payload))
    }

    /// Borrows a pattern containing all of the fields that need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(&self) -> impl Iterator<Item = PatternItem> + '_ {
        match self {
            TimePatternSelectionData::SingleTime(payload) => payload.get(),
        }
        .pattern
        .items
        .iter()
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

impl<'a> TimePatternDataBorrowed<'a> {
    #[inline]
    pub(crate) fn metadata(&self) -> PatternMetadata {
        match self {
            Self::Resolved(data) => data.pattern.metadata,
        }
    }

    #[inline]
    pub(crate) fn iter_items(&self) -> impl Iterator<Item = PatternItem> + '_ {
        match self {
            Self::Resolved(data) => data.pattern.items.iter(),
        }
    }

    #[inline]
    pub(crate) fn to_pattern(&self) -> DateTimePattern {
        let pattern = match self {
            Self::Resolved(data) => &data.pattern,
        };
        DateTimePattern::from_runtime_pattern(pattern.clone().into_owned())
    }
}

impl DateTimeGluePatternSelectionData {
    pub(crate) fn try_new_with_lengths<M, P>(
        provider: &P,
        locale: &DataLocale,
        date_length: length::Date,
        time_length: length::Time,
    ) -> Result<Self, Error>
    where
        P: DataProvider<M>
            + DataProvider<TimePatternV1Marker>
            + DataProvider<DateTimePatternV1Marker>
            + ?Sized,
        M: KeyedDataMarker<Yokeable = DatePatternV1<'static>>,
    {
        let date =
            DatePatternSelectionData::try_new_with_length::<M, _>(provider, locale, date_length)?;
        let time = TimePatternSelectionData::try_new_with_length(provider, locale, time_length)?;
        let mut locale = locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(aux::pattern_subtag_for(
            // According to UTS 35, use the date length here: use the glue
            // pattern "whose type matches the type of the date pattern"
            match date_length {
                length::Date::Full => aux::PatternLength::Full,
                length::Date::Long => aux::PatternLength::Long,
                length::Date::Medium => aux::PatternLength::Medium,
                length::Date::Short => aux::PatternLength::Short,
            },
            None, // no hour cycle for date patterns
        )));
        let glue = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self { date, time, glue })
    }

    /// Returns an iterator over the pattern items that may need to be loaded.
    #[inline]
    pub(crate) fn pattern_items_for_data_loading(&self) -> impl Iterator<Item = PatternItem> + '_ {
        let date_items = self.date.pattern_items_for_data_loading();
        let time_items = self.time.pattern_items_for_data_loading();
        date_items.chain(time_items)
    }
}

impl DateTimePatternSelectionData {
    /// Borrows a resolved pattern based on the given datetime
    pub(crate) fn select(&self, datetime: &ExtractedDateTimeInput) -> DateTimePatternDataBorrowed {
        match self {
            DateTimePatternSelectionData::Date(date) => {
                DateTimePatternDataBorrowed::Date(date.select(datetime))
            }
            DateTimePatternSelectionData::Time(time) => {
                DateTimePatternDataBorrowed::Time(time.select(datetime))
            }
            DateTimePatternSelectionData::DateTimeGlue(DateTimeGluePatternSelectionData {
                date,
                time,
                glue,
            }) => DateTimePatternDataBorrowed::DateTimeGlue {
                date: date.select(datetime),
                time: time.select(datetime),
                glue: glue.get(),
            },
        }
    }
}

impl<'a> DateTimePatternDataBorrowed<'a> {
    #[inline]
    fn date_pattern(&self) -> Option<DatePatternDataBorrowed<'a>> {
        match self {
            Self::Date(date) => Some(*date),
            Self::Time(_) => None,
            Self::DateTimeGlue { date, .. } => Some(*date),
        }
    }

    #[inline]
    fn time_pattern(&self) -> Option<TimePatternDataBorrowed<'a>> {
        match self {
            Self::Date(_) => None,
            Self::Time(time) => Some(*time),
            Self::DateTimeGlue { time, .. } => Some(*time),
        }
    }

    #[inline]
    fn glue_pattern(&self) -> Option<&'a DateTimePatternV1<'a>> {
        match self {
            Self::Date(_) => None,
            Self::Time(_) => None,
            Self::DateTimeGlue { glue, .. } => Some(glue),
        }
    }

    #[inline]
    pub(crate) fn metadata(&self) -> PatternMetadata {
        match self {
            Self::Date(DatePatternDataBorrowed::Resolved(data)) => data.pattern.metadata,
            Self::Time(TimePatternDataBorrowed::Resolved(data)) => data.pattern.metadata,
            Self::DateTimeGlue {
                date: DatePatternDataBorrowed::Resolved(date),
                time: TimePatternDataBorrowed::Resolved(time),
                ..
            } => PatternMetadata::merge_date_and_time_metadata(
                date.pattern.metadata,
                time.pattern.metadata,
            ),
        }
    }

    pub(crate) fn iter_items(&self) -> impl Iterator<Item = PatternItem> + '_ {
        let glue_pattern_slice = match self.glue_pattern() {
            Some(glue) => &glue.pattern.items.as_ule_slice(),
            None => runtime::ZERO_ONE_SLICE.as_ule_slice(),
        };
        glue_pattern_slice
            .iter()
            .flat_map(
                |generic_item_ule| match generic_item_ule.to_pattern_item_ule() {
                    Ok(pattern_item_ule) => core::slice::from_ref(pattern_item_ule),
                    Err(0) => self
                        .date_pattern()
                        .map(|data| match data {
                            DatePatternDataBorrowed::Resolved(pattern) => {
                                pattern.pattern.items.as_ule_slice()
                            }
                        })
                        .unwrap_or(&[]),
                    Err(1) => self
                        .time_pattern()
                        .map(|data| match data {
                            TimePatternDataBorrowed::Resolved(pattern) => {
                                pattern.pattern.items.as_ule_slice()
                            }
                        })
                        .unwrap_or(&[]),
                    _ => &[],
                },
            )
            .map(|unaligned| PatternItem::from_unaligned(*unaligned))
    }

    pub(crate) fn to_pattern(&self) -> DateTimePattern {
        let pattern = match self {
            Self::Date(DatePatternDataBorrowed::Resolved(data)) => &data.pattern,
            Self::Time(TimePatternDataBorrowed::Resolved(data)) => &data.pattern,
            Self::DateTimeGlue { .. } => todo!(),
        };
        DateTimePattern::from_runtime_pattern(pattern.clone().into_owned())
    }
}

impl<'a, 'b, I> DateTimeWriter<'a, 'b, I>
where
    I: Iterator<Item = PatternItem> + 'b,
    'a: 'b,
{
    pub(crate) fn write_to<W: fmt::Write + ?Sized>(self, sink: &mut W) -> fmt::Result {
        let loc_datetime =
            DateTimeInputWithWeekConfig::new(self.datetime, self.names.week_calculator);
        let Some(fixed_decimal_formatter) = self.names.fixed_decimal_formatter else {
            // TODO(#4340): Make the FixedDecimalFormatter optional
            icu_provider::_internal::log::warn!("FixedDecimalFormatter not loaded");
            return Err(core::fmt::Error);
        };
        write_pattern(
            self.pattern_items,
            self.pattern_metadata,
            Some(&self.names),
            Some(&self.names),
            &loc_datetime,
            fixed_decimal_formatter,
            sink,
        )
        .map_err(|_e| {
            icu_provider::_internal::log::warn!("{_e:?}");
            core::fmt::Error
        })
    }
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
