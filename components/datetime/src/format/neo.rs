// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::datetime::write_pattern;
use crate::calendar::CldrCalendar;
use crate::error::DateTimeError as Error;
use crate::fields;
use crate::input;
use crate::input::DateTimeInput;
use crate::input::DateTimeInputWithWeekConfig;
use crate::input::ExtractedDateTimeInput;
use crate::input::LocalizedDateTimeInput;
use crate::pattern::runtime::Pattern;
use crate::provider::calendar::months;
use crate::provider::date_time::{DateSymbols, MonthPlaceholderValue, TimeSymbols};
use crate::provider::neo::*;
use core::fmt;
use icu_calendar::types::Era;
use icu_calendar::types::MonthCode;
use icu_calendar::week::WeekCalculator;
use icu_decimal::options::FixedDecimalFormatterOptions;
use icu_decimal::options::GroupingStrategy;
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_decimal::FixedDecimalFormatter;
use icu_locid::extensions::private::subtag;
use icu_provider::prelude::*;
use icu_provider::prelude::*;
use writeable::Writeable;

/// This can be extended in the future to support multiple lengths.
/// For now, this type wraps a symbols object tagged with a single length.
#[derive(Debug, Copy, Clone)]
enum OptionalSymbolsWithLength<T> {
    None,
    SingleLength(fields::FieldLength, T),
}

impl<T> OptionalSymbolsWithLength<T> {
    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }
}

impl<T> OptionalSymbolsWithLength<T>
where
    T: Copy,
{
    pub(crate) fn get_with_length(&self, length: fields::FieldLength) -> Option<T> {
        match self {
            Self::None => None,
            Self::SingleLength(actual_length, t) if length == *actual_length => Some(*t),
            _ => None,
        }
    }
}

impl<M> OptionalSymbolsWithLength<DataPayload<M>>
where
    M: KeyedDataMarker,
{
    pub(crate) fn as_borrowed(
        &self,
    ) -> OptionalSymbolsWithLength<&<M::Yokeable as icu_provider::yoke::Yokeable>::Output> {
        match self {
            Self::None => OptionalSymbolsWithLength::None,
            Self::SingleLength(length, payload) => {
                OptionalSymbolsWithLength::SingleLength(*length, payload.get())
            }
        }
    }
}

/// A low-level type that formats datetime patterns with localized symbols.
pub struct DateTimePatternInterpolator<C: CldrCalendar> {
    locale: DataLocale,
    year_symbols: OptionalSymbolsWithLength<DataPayload<C::YearSymbolsV1Marker>>,
    month_symbols: OptionalSymbolsWithLength<DataPayload<C::MonthSymbolsV1Marker>>,
    weekday_symbols: OptionalSymbolsWithLength<DataPayload<WeekdaySymbolsV1Marker>>,
    dayperiod_symbols: OptionalSymbolsWithLength<DataPayload<DayPeriodSymbolsV1Marker>>,
    // TODO: Make the FixedDecimalFormatter optional?
    fixed_decimal_formatter: FixedDecimalFormatter,
    week_calculator: Option<WeekCalculator>,
    // To add later:
    // ordinal_rules: Option<PluralRules>,
}

impl<C: CldrCalendar> DateTimePatternInterpolator<C> {
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: &DataLocale) -> Result<Self, Error> {
        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;
        let fixed_decimal_formatter =
            FixedDecimalFormatter::try_new(locale, fixed_decimal_format_options)?;
        Ok(Self::new_internal(locale.clone(), fixed_decimal_formatter))
    }

    pub fn try_new_unstable<P>(provider: &P, locale: &DataLocale) -> Result<Self, Error>
    where
        P: DataProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;
        let fixed_decimal_formatter = FixedDecimalFormatter::try_new_unstable(
            provider,
            locale,
            fixed_decimal_format_options,
        )?;
        Ok(Self::new_internal(locale.clone(), fixed_decimal_formatter))
    }

    fn new_internal(locale: DataLocale, fixed_decimal_formatter: FixedDecimalFormatter) -> Self {
        DateTimePatternInterpolator {
            locale,
            year_symbols: OptionalSymbolsWithLength::None,
            month_symbols: OptionalSymbolsWithLength::None,
            weekday_symbols: OptionalSymbolsWithLength::None,
            dayperiod_symbols: OptionalSymbolsWithLength::None,
            fixed_decimal_formatter,
            week_calculator: None,
        }
    }

    pub fn load_month_symbols_abbreviated<P>(&mut self, provider: &P) -> Result<&mut Self, Error>
    where
        P: DataProvider<C::MonthSymbolsV1Marker> + ?Sized,
    {
        if self.month_symbols.is_some() {
            // TODO: Discuss what do do here
            return Err(Error::Data(DataError::custom(
                "month symbols already loaded",
            )));
        }
        let mut locale = self.locale.clone();
        locale.set_aux(AuxiliaryKeys::from_subtag(subtag!("3")));
        let payload = provider
            .load(DataRequest {
                locale: &locale,
                metadata: Default::default(),
            })?
            .take_payload()?;
        self.month_symbols =
            OptionalSymbolsWithLength::SingleLength(fields::FieldLength::Abbreviated, payload);
        Ok(self)
    }
}

#[derive(Debug, Copy, Clone)]
struct ErasedDateTimePatternInterpolatorBorrowed<'l> {
    year_symbols: OptionalSymbolsWithLength<&'l YearSymbolsV1<'l>>,
    month_symbols: OptionalSymbolsWithLength<&'l MonthSymbolsV1<'l>>,
    weekday_symbols: OptionalSymbolsWithLength<&'l LinearSymbolsV1<'l>>,
    dayperiod_symbols: OptionalSymbolsWithLength<&'l LinearSymbolsV1<'l>>,
    fixed_decimal_formatter: &'l FixedDecimalFormatter,
    week_calculator: Option<&'l WeekCalculator>,
}

impl<C: CldrCalendar> DateTimePatternInterpolator<C> {
    fn as_borrowed(&self) -> ErasedDateTimePatternInterpolatorBorrowed {
        ErasedDateTimePatternInterpolatorBorrowed {
            year_symbols: self.year_symbols.as_borrowed(),
            month_symbols: self.month_symbols.as_borrowed(),
            weekday_symbols: self.weekday_symbols.as_borrowed(),
            dayperiod_symbols: self.dayperiod_symbols.as_borrowed(),
            fixed_decimal_formatter: &self.fixed_decimal_formatter,
            week_calculator: self.week_calculator.as_ref(),
        }
    }

    // TODO: Discuss the design of this API
    pub fn format<'l, T>(
        &'l self,
        pattern: &'l Pattern<'l>,
        datetime: &T,
    ) -> FormattedDateTimePattern<'l>
    where
        T: DateTimeInput<Calendar = C>,
    {
        FormattedDateTimePattern {
            pattern,
            datetime: ExtractedDateTimeInput::extract_from(datetime),
            interpolator: self.as_borrowed(),
        }
    }
}

pub struct FormattedDateTimePattern<'l> {
    pattern: &'l Pattern<'l>,
    datetime: ExtractedDateTimeInput,
    interpolator: ErasedDateTimePatternInterpolatorBorrowed<'l>,
}

impl<'l> Writeable for FormattedDateTimePattern<'l> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let loc_datetime =
            DateTimeInputWithWeekConfig::new(&self.datetime, self.interpolator.week_calculator);
        write_pattern(
            self.pattern,
            Some(&self.interpolator),
            Some(&self.interpolator),
            &loc_datetime,
            self.interpolator.fixed_decimal_formatter,
            sink,
        )
        .map_err(|_e| {
            icu_provider::_internal::log::warn!("{_e:?}");
            core::fmt::Error
        })
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'l> fmt::Display for FormattedDateTimePattern<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

impl<'data> DateSymbols<'data> for ErasedDateTimePatternInterpolatorBorrowed<'data> {
    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        code: MonthCode,
    ) -> Result<MonthPlaceholderValue, Error> {
        // TODO: This should be MissingMonthSymbols in neo
        let month_symbols = self
            .month_symbols
            .get_with_length(length)
            .ok_or(Error::MissingDateSymbols)?;
        let Some((month_number, is_leap)) = code.parsed() else {
            return Err(Error::MissingMonthSymbol(code));
        };
        let Some(month_index) = month_number.checked_sub(1) else {
            return Err(Error::MissingMonthSymbol(code));
        };
        let month_index = usize::from(month_index);
        let symbol = match month_symbols {
            MonthSymbolsV1::Linear(linear) => {
                if is_leap {
                    None
                } else {
                    linear.get(month_index)
                }
            }
            MonthSymbolsV1::LeapLinear(leap_linear) => {
                let num_months = leap_linear.len() / 2;
                if is_leap {
                    leap_linear.get(month_index + num_months)
                } else if month_index < num_months {
                    leap_linear.get(month_index)
                } else {
                    None
                }
            }
            MonthSymbolsV1::LeapNumeric(leap_numeric) => {
                return Ok(MonthPlaceholderValue::NumericPattern(leap_numeric))
            }
        };
        // Note: Always return `false` for the second argument since neo MonthSymbols
        // knows how to handle leap months and we don't need the fallback logic
        symbol
            .map(|s| MonthPlaceholderValue::PlainString(s))
            .ok_or_else(|| Error::MissingMonthSymbol(code))
    }

    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: input::IsoWeekday,
    ) -> Result<&str, Error> {
        // TODO: This should be MissingWeekdaySymbols in neo
        let weekday_symbols = self
            .weekday_symbols
            .get_with_length(length)
            .ok_or(Error::MissingDateSymbols)?;
        todo!()
    }

    fn get_symbol_for_era<'a>(&'a self, length: fields::FieldLength, era_code: &'a Era) -> &str {
        todo!()
    }
}

impl<'data> TimeSymbols for ErasedDateTimePatternInterpolatorBorrowed<'data> {
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: input::IsoHour,
        is_top_of_hour: bool,
    ) -> Result<&str, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::reference;
    use icu_calendar::{DateTime, Gregorian};
    use icu_locid::locale;
    use writeable::assert_writeable_eq;

    #[test]
    fn test_basic_pattern_interpolator() {
        let locale = locale!("en").into();
        let mut interpolator: DateTimePatternInterpolator<Gregorian> =
            DateTimePatternInterpolator::try_new(&locale).unwrap();
        interpolator
            .load_month_symbols_abbreviated(&crate::provider::Baked)
            .unwrap();
        let reference_pattern: reference::Pattern =
            "'It is' MMM d, y 'at' HH:mm'!'".parse().unwrap();
        let pattern: Pattern = (&reference_pattern).into();
        let datetime = DateTime::try_new_gregorian_datetime(2023, 10, 25, 15, 0, 55).unwrap();
        let formatted_pattern = interpolator.format(&pattern, &datetime);

        assert_writeable_eq!(formatted_pattern, "It is Oct 25, 2023 at 15:00!");
    }
}
