// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::{
    DateTimeInput, DateTimeInputWithLocale, ExtractedDateTimeInput, LocalizedDateTimeInput,
};
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, Field, FieldLength, FieldSymbol, Second, Week, Year};
use crate::pattern::{
    runtime::{Pattern, PatternPlurals},
    PatternItem,
};
use crate::provider;
use crate::provider::calendar::patterns::PatternPluralsFromPatternsV1Marker;
use crate::provider::date_time::{DateSymbols, TimeSymbols};
use crate::provider::week_data::WeekDataV1;

use core::fmt;
use fixed_decimal::FixedDecimal;
use icu_decimal::FixedDecimalFormat;
use icu_locid::Locale;
use icu_plurals::PluralRules;
use icu_provider::DataPayload;
use writeable::Writeable;

/// [`FormattedDateTime`] is a intermediate structure which can be retrieved as
/// an output from [`DateTimeFormat`](crate::DateTimeFormat).
///
/// The structure contains all the information needed to display formatted value,
/// and it will also contain additional methods allowing the user to introspect
/// and even manipulate the formatted data.
///
/// # Examples
///
/// ```
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::DateTimeFormat;
/// use icu::locid::locale;
/// # let provider = icu_provider::inv::InvariantDataProvider;
/// # let options = icu::datetime::DateTimeFormatOptions::default();
/// let dtf = DateTimeFormat::<Gregorian>::try_new(locale!("en"), &provider, &options)
///     .expect("Failed to create DateTimeFormat instance.");
///
/// let datetime = DateTime::new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// let formatted_date = dtf.format(&datetime);
///
/// let _ = format!("Date: {}", formatted_date);
/// ```
pub struct FormattedDateTime<'l> {
    pub(crate) patterns: &'l DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub(crate) date_symbols: Option<&'l provider::calendar::DateSymbolsV1<'l>>,
    pub(crate) time_symbols: Option<&'l provider::calendar::TimeSymbolsV1<'l>>,
    pub(crate) datetime: ExtractedDateTimeInput,
    pub(crate) week_data: Option<&'l WeekDataV1>,
    pub(crate) locale: &'l Locale,
    pub(crate) ordinal_rules: Option<&'l PluralRules>,
    pub(crate) fixed_decimal_format: &'l FixedDecimalFormat,
}

impl<'l> Writeable for FormattedDateTime<'l> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern_plurals(
            &self.patterns.get().0,
            self.date_symbols,
            self.time_symbols,
            &self.datetime,
            self.week_data,
            self.ordinal_rules,
            self.fixed_decimal_format,
            self.locale,
            sink,
        )
        .map_err(|_| core::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l> fmt::Display for FormattedDateTime<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

// Apply length to input number and write to result using fixed_decimal_format.
fn format_number<W>(
    result: &mut W,
    fixed_decimal_format: &FixedDecimalFormat,
    mut num: FixedDecimal,
    length: FieldLength,
) -> fmt::Result
where
    W: fmt::Write + ?Sized,
{
    match length {
        FieldLength::One => {}
        FieldLength::TwoDigit => {
            num.pad_left(2);
            num.truncate_left(2);
        }
        FieldLength::Abbreviated => {
            num.pad_left(3);
        }
        FieldLength::Wide => {
            num.pad_left(4);
        }
        FieldLength::Narrow => {
            num.pad_left(5);
        }
        FieldLength::Six => {
            num.pad_left(6);
        }
        FieldLength::Fixed(p) => {
            num.pad_left(p as i16);
            num.truncate_left(p as i16);
        }
    }

    let formatted = fixed_decimal_format.format(&num);
    formatted.write_to(result)
}

fn write_pattern<T, W>(
    pattern: &crate::pattern::runtime::Pattern,
    date_symbols: Option<&provider::calendar::DateSymbolsV1>,
    time_symbols: Option<&provider::calendar::TimeSymbolsV1>,
    loc_datetime: &impl LocalizedDateTimeInput<T>,
    fixed_decimal_format: &FixedDecimalFormat,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    let mut iter = pattern.items.iter().peekable();
    loop {
        match iter.next() {
            Some(PatternItem::Field(field)) => write_field(
                pattern,
                field,
                iter.peek(),
                date_symbols,
                time_symbols,
                loc_datetime,
                fixed_decimal_format,
                w,
            )?,
            Some(PatternItem::Literal(ch)) => w.write_char(ch)?,
            None => break,
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn write_pattern_plurals<T, W>(
    patterns: &PatternPlurals,
    date_symbols: Option<&provider::calendar::DateSymbolsV1>,
    time_symbols: Option<&provider::calendar::TimeSymbolsV1>,
    datetime: &T,
    week_data: Option<&WeekDataV1>,
    ordinal_rules: Option<&PluralRules>,
    fixed_decimal_format: &FixedDecimalFormat,
    locale: &Locale,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    let loc_datetime = DateTimeInputWithLocale::new(datetime, week_data.map(|d| &d.0), locale);
    let pattern = patterns.select(&loc_datetime, ordinal_rules)?;
    write_pattern(
        pattern,
        date_symbols,
        time_symbols,
        &loc_datetime,
        fixed_decimal_format,
        w,
    )
}

// This function assumes that the correct decision has been
// made regarding availability of symbols in the caller.
//
// When modifying the list of fields using symbols,
// update the matching query in `analyze_pattern` function.
#[allow(clippy::too_many_arguments)]
pub(super) fn write_field<T, W>(
    pattern: &crate::pattern::runtime::Pattern,
    field: fields::Field,
    next_item: Option<&PatternItem>,
    date_symbols: Option<&crate::provider::calendar::DateSymbolsV1>,
    time_symbols: Option<&crate::provider::calendar::TimeSymbolsV1>,
    datetime: &impl LocalizedDateTimeInput<T>,
    fixed_decimal_format: &FixedDecimalFormat,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    match field.symbol {
        FieldSymbol::Era => {
            #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
            let symbol = date_symbols
                .expect("Expect date symbols to be present")
                .get_symbol_for_era(
                    field.length,
                    datetime
                        .datetime()
                        .year()
                        .ok_or(Error::MissingInputField)?
                        .era,
                )?;
            w.write_str(symbol)?
        }
        FieldSymbol::Year(year) => match year {
            Year::Calendar => format_number(
                w,
                fixed_decimal_format,
                FixedDecimal::from(
                    datetime
                        .datetime()
                        .year()
                        .ok_or(Error::MissingInputField)?
                        .number,
                ),
                field.length,
            )?,
            Year::WeekOf => format_number(
                w,
                fixed_decimal_format,
                FixedDecimal::from(datetime.year_week()?.number),
                field.length,
            )?,
        },
        FieldSymbol::Month(month) => match field.length {
            FieldLength::One | FieldLength::TwoDigit => format_number(
                w,
                fixed_decimal_format,
                FixedDecimal::from(
                    datetime
                        .datetime()
                        .month()
                        .ok_or(Error::MissingInputField)?
                        .ordinal,
                ),
                field.length,
            )?,
            length => {
                #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
                let symbol = date_symbols
                    .expect("Expect date symbols to be present")
                    .get_symbol_for_month(
                        month,
                        length,
                        datetime
                            .datetime()
                            .month()
                            .ok_or(Error::MissingInputField)?
                            .code,
                    )?;
                w.write_str(symbol)?
            }
        },
        FieldSymbol::Week(week) => match week {
            Week::WeekOfYear => format_number(
                w,
                fixed_decimal_format,
                FixedDecimal::from(datetime.week_of_year()?.0),
                field.length,
            )?,
            Week::WeekOfMonth => format_number(
                w,
                fixed_decimal_format,
                FixedDecimal::from(datetime.week_of_month()?.0),
                field.length,
            )?,
        },
        FieldSymbol::Weekday(weekday) => {
            let dow = datetime
                .datetime()
                .iso_weekday()
                .ok_or(Error::MissingInputField)?;
            #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
            let symbol = date_symbols
                .expect("Expect date symbols to be present")
                .get_symbol_for_weekday(weekday, field.length, dow)?;
            w.write_str(symbol)?
        }
        symbol @ FieldSymbol::Day(day) => format_number(
            w,
            fixed_decimal_format,
            FixedDecimal::from(match day {
                fields::Day::DayOfMonth => {
                    datetime
                        .datetime()
                        .day_of_month()
                        .ok_or(Error::MissingInputField)?
                        .0
                }
                fields::Day::DayOfWeekInMonth => datetime.day_of_week_in_month()?.0,
                _ => return Err(Error::UnsupportedField(symbol)),
            }),
            field.length,
        )?,
        FieldSymbol::Hour(hour) => {
            let h =
                usize::from(datetime.datetime().hour().ok_or(Error::MissingInputField)?) as isize;
            let value = match hour {
                fields::Hour::H11 => h % 12,
                fields::Hour::H12 => {
                    let v = h % 12;
                    if v == 0 {
                        12
                    } else {
                        v
                    }
                }
                fields::Hour::H23 => h,
                fields::Hour::H24 => {
                    if h == 0 {
                        24
                    } else {
                        h
                    }
                }
            };
            format_number(
                w,
                fixed_decimal_format,
                FixedDecimal::from(value),
                field.length,
            )?
        }
        FieldSymbol::Minute => format_number(
            w,
            fixed_decimal_format,
            FixedDecimal::from(usize::from(
                datetime
                    .datetime()
                    .minute()
                    .ok_or(Error::MissingInputField)?,
            )),
            field.length,
        )?,
        FieldSymbol::Second(Second::Second) => {
            let mut seconds = FixedDecimal::from(usize::from(
                datetime
                    .datetime()
                    .second()
                    .ok_or(Error::MissingInputField)?,
            ));
            if let Some(PatternItem::Field(next_field)) = next_item {
                if let FieldSymbol::Second(Second::FractionalSecond) = next_field.symbol {
                    let mut fraction = FixedDecimal::from(usize::from(
                        datetime
                            .datetime()
                            .nanosecond()
                            .ok_or(Error::MissingInputField)?,
                    ));

                    // We only support fixed field length for fractional seconds.
                    let precision = match next_field.length {
                        FieldLength::Fixed(p) => p,
                        _ => {
                            return Err(Error::Pattern(
                                crate::pattern::PatternError::FieldLengthInvalid(
                                    FieldSymbol::Second(Second::FractionalSecond),
                                ),
                            ));
                        }
                    };

                    // We store fractional seconds as nanoseconds, convert to seconds.
                    fraction
                        .multiply_pow10(-9)
                        .map_err(|_| Error::FixedDecimal)?;

                    seconds
                        .concatenate_right(fraction)
                        .map_err(|_| Error::FixedDecimal)?;
                    seconds.pad_right(-(precision as i16));
                }
            }
            format_number(w, fixed_decimal_format, seconds, field.length)?
        }
        FieldSymbol::Second(Second::FractionalSecond) => {
            // Formatting of fractional seconds is handled when formatting seconds.
        }
        field @ FieldSymbol::Second(Second::Millisecond) => {
            return Err(Error::UnsupportedField(field))
        }
        FieldSymbol::DayPeriod(period) => {
            #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
            let symbol = time_symbols
                .expect("Expect time symbols to be present")
                .get_symbol_for_day_period(
                    period,
                    field.length,
                    datetime.datetime().hour().ok_or(Error::MissingInputField)?,
                    pattern.time_granularity.is_top_of_hour(
                        datetime.datetime().minute().map(u8::from).unwrap_or(0),
                        datetime.datetime().second().map(u8::from).unwrap_or(0),
                        datetime.datetime().nanosecond().map(u32::from).unwrap_or(0),
                    ),
                )?;
            w.write_str(symbol)?
        }
        field @ FieldSymbol::TimeZone(_) => return Err(Error::UnsupportedField(field)),
    };
    Ok(())
}

/// What data is required to format a given pattern.
#[derive(Default)]
pub struct RequiredData {
    // DateSymbolsV1 is required.
    pub date_symbols_data: bool,
    // TimeSymbolsV1 is required.
    pub time_symbols_data: bool,
    // WeekDataV1 is required.
    pub week_data: bool,
}

impl RequiredData {
    // Checks if formatting `pattern` would require us to load data & if so adds
    // them to this struct. Returns true if requirements are saturated and would
    // not change by any further calls.
    // Keep it in sync with the `write_field` use of symbols.
    fn add_requirements_from_pattern(
        &mut self,
        pattern: &Pattern,
        supports_time_zones: bool,
    ) -> Result<bool, Field> {
        let fields = pattern.items.iter().filter_map(|p| match p {
            PatternItem::Field(field) => Some(field),
            _ => None,
        });

        for field in fields {
            if !self.date_symbols_data {
                self.date_symbols_data = match field.symbol {
                    FieldSymbol::Era => true,
                    FieldSymbol::Month(_) => {
                        !matches!(field.length, FieldLength::One | FieldLength::TwoDigit)
                    }
                    FieldSymbol::Weekday(_) => true,
                    _ => false,
                }
            }
            if !self.time_symbols_data {
                self.time_symbols_data = matches!(field.symbol, FieldSymbol::DayPeriod(_));
            }

            if !self.week_data {
                self.week_data = matches!(
                    field.symbol,
                    FieldSymbol::Year(Year::WeekOf) | FieldSymbol::Week(_)
                )
            }

            if supports_time_zones {
                if self.date_symbols_data && self.time_symbols_data && self.week_data {
                    // If we support time zones, and require everything else, we
                    // know all we need to return already.
                    return Ok(true);
                }
            } else if matches!(field.symbol, FieldSymbol::TimeZone(_)) {
                // If we don't support time zones, and encountered a time zone
                // field, error out.
                return Err(field);
            }
        }

        Ok(false)
    }
}

// Determines what optional data needs to be loaded to format `patterns`.
pub fn analyze_patterns(
    patterns: &PatternPlurals,
    supports_time_zones: bool,
) -> Result<RequiredData, Field> {
    let mut required = RequiredData::default();
    for pattern in patterns.patterns_iter() {
        if required.add_requirements_from_pattern(pattern, supports_time_zones)? {
            // We can bail early if everything is required & we don't need to
            // validate the absence of TimeZones.
            break;
        }
    }
    Ok(required)
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_decimal::options::{FixedDecimalFormatOptions, GroupingStrategy};

    #[test]
    #[cfg(feature = "serde")]
    fn test_basic() {
        use crate::provider::calendar::{DateSymbolsV1Marker, TimeSymbolsV1Marker};
        use icu_calendar::DateTime;
        use icu_provider::prelude::*;

        let provider = icu_testdata::get_provider();
        let locale: Locale = "en-u-ca-gregory".parse().unwrap();
        let date_data: DataPayload<DateSymbolsV1Marker> = provider
            .load_resource(&DataRequest {
                options: locale.clone().into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let time_data: DataPayload<TimeSymbolsV1Marker> = provider
            .load_resource(&DataRequest {
                options: locale.clone().into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let pattern = "MMM".parse().unwrap();
        let datetime = DateTime::new_gregorian_datetime(2020, 8, 1, 12, 34, 28).unwrap();
        let fixed_decimal_format =
            FixedDecimalFormat::try_new(locale.id.language, &provider, Default::default()).unwrap();

        let mut sink = String::new();
        let loc_datetime = DateTimeInputWithLocale::new(&datetime, None, &"und".parse().unwrap());
        write_pattern(
            &pattern,
            Some(date_data.get()),
            Some(time_data.get()),
            &loc_datetime,
            &fixed_decimal_format,
            &mut sink,
        )
        .unwrap();
        println!("{}", sink);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_format_number() {
        let values = &[2, 20, 201, 2017, 20173];
        let samples = &[
            (FieldLength::One, ["2", "20", "201", "2017", "20173"]),
            (FieldLength::TwoDigit, ["02", "20", "01", "17", "73"]),
            (
                FieldLength::Abbreviated,
                ["002", "020", "201", "2017", "20173"],
            ),
            (FieldLength::Wide, ["0002", "0020", "0201", "2017", "20173"]),
        ];

        let provider = icu_testdata::get_provider();
        let mut fixed_decimal_format_options = FixedDecimalFormatOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;
        let fixed_decimal_format = FixedDecimalFormat::try_new(
            icu_locid::locale!("en"),
            &provider,
            fixed_decimal_format_options,
        )
        .unwrap();

        for (length, expected) in samples {
            for (value, expected) in values.iter().zip(expected) {
                let mut s = String::new();
                format_number(
                    &mut s,
                    &fixed_decimal_format,
                    FixedDecimal::from(*value as i32),
                    *length,
                )
                .unwrap();
                assert_eq!(s, *expected);
            }
        }
    }
}
