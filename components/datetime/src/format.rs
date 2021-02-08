// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::date::{self, DateTimeType};
use crate::fields::{self, FieldLength, FieldSymbol};
use crate::pattern::{Pattern, PatternItem};
use crate::provider;
use crate::provider::helpers::DateTimeDates;
use crate::{error::DateTimeFormatError, pattern::TimeGranularity};
use std::fmt;
use writeable::Writeable;

/// `FormattedDateTime` is a intermediate structure which can be retrieved as
/// an output from `DateTimeFormat`.
///
/// The structure contains all the information needed to display formatted value,
/// and it will also contain additional methods allowing the user to introspect
/// and even manipulate the formatted data.
///
/// # Examples
///
/// ```
/// # use icu_locid_macros::langid;
/// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
/// # use icu_datetime::date::MockDateTime;
/// # use icu_provider::inv::InvariantDataProvider;
/// # let locale = langid!("en").into();
/// # let provider = InvariantDataProvider;
/// # let options = DateTimeFormatOptions::default();
/// let dtf = DateTimeFormat::try_new(locale, &provider, &options)
///     .expect("Failed to create DateTimeFormat instance.");
///
/// let date_time = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// let formatted_date = dtf.format(&date_time);
///
/// let _ = format!("Date: {}", formatted_date);
/// ```
pub struct FormattedDateTime<'l, T>
where
    T: DateTimeType,
{
    pub(crate) pattern: &'l Pattern,
    pub(crate) data: &'l provider::gregory::DatesV1,
    pub(crate) date_time: &'l T,
}

impl<'l, T> Writeable for FormattedDateTime<'l, T>
where
    T: DateTimeType,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern(self.pattern, self.data, self.date_time, sink).map_err(|_| std::fmt::Error)
    }

    // TODO: Implement write_len
}

impl<'l, T> fmt::Display for FormattedDateTime<'l, T>
where
    T: DateTimeType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(self.pattern, self.data, self.date_time, f).map_err(|_| std::fmt::Error)
    }
}

// Temporary formatting number with length.
fn format_number<W>(result: &mut W, num: usize, length: FieldLength) -> Result<(), std::fmt::Error>
where
    W: fmt::Write + ?Sized,
{
    match length {
        FieldLength::One => write!(result, "{}", num),
        FieldLength::TwoDigit => {
            if num < 100 {
                write!(result, "{:0>width$}", num, width = 2)
            } else {
                let buffer = num.to_string();
                let len = buffer.len();
                result.write_str(&buffer[len - 2..])
            }
        }
        length => write!(result, "{:0>width$}", num, width = length as usize),
    }
}

// Temporary simplified function to get the day of the week
fn get_day_of_week(year: usize, month: date::Month, day: date::Day) -> date::WeekDay {
    let month: usize = month.into();
    let day: usize = day.into();
    let t = &[0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = if month < 2 { year - 1 } else { year };
    let result = (year + year / 4 - year / 100 + year / 400 + t[month] + day + 1) % 7;
    date::WeekDay::new_unchecked(result as u8)
}

/// Returns `true` if the most granular time being displayed will align with
/// the top of the hour, otherwise returns `false`.
/// e.g. `12:00:00` is at the top of the hour for hours, minutes, and seconds.
/// e.g. `12:00:05` is only at the top of the hour if the seconds are not displayed.
fn is_top_of_hour<T: DateTimeType>(pattern: &Pattern, date_time: &T) -> bool {
    match pattern.most_granular_time() {
        None | Some(TimeGranularity::Hours) => true,
        Some(TimeGranularity::Minutes) => u8::from(date_time.minute()) == 0,
        Some(TimeGranularity::Seconds) => {
            u8::from(date_time.minute()) + u8::from(date_time.second()) == 0
        }
    }
}

pub fn write_pattern<T, W>(
    pattern: &crate::pattern::Pattern,
    data: &provider::gregory::DatesV1,
    date_time: &T,
    w: &mut W,
) -> Result<(), DateTimeFormatError>
where
    T: DateTimeType,
    W: fmt::Write + ?Sized,
{
    for item in pattern.items() {
        match item {
            PatternItem::Field(field) => match field.symbol {
                FieldSymbol::Year(..) => format_number(w, date_time.year(), field.length)?,
                FieldSymbol::Month(month) => match field.length {
                    FieldLength::One | FieldLength::TwoDigit => {
                        format_number(w, usize::from(date_time.month()) + 1, field.length)?
                    }
                    length => {
                        let symbol = data.get_symbol_for_month(month, length, date_time.month());
                        w.write_str(symbol)?
                    }
                },
                FieldSymbol::Weekday(weekday) => {
                    let dow = get_day_of_week(date_time.year(), date_time.month(), date_time.day());
                    let symbol = data.get_symbol_for_weekday(weekday, field.length, dow);
                    w.write_str(symbol)?
                }
                FieldSymbol::Day(..) => {
                    format_number(w, usize::from(date_time.day()) + 1, field.length)?
                }
                FieldSymbol::Hour(hour) => {
                    let h = date_time.hour().into();
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
                    format_number(w, value, field.length)?
                }
                FieldSymbol::Minute => format_number(w, date_time.minute().into(), field.length)?,
                FieldSymbol::Second(..) => {
                    format_number(w, date_time.second().into(), field.length)?
                }
                FieldSymbol::DayPeriod(period) => {
                    let symbol = data.get_symbol_for_day_period(
                        period,
                        field.length,
                        date_time.hour(),
                        is_top_of_hour(&pattern, date_time),
                    );
                    w.write_str(symbol)?
                }
            },
            PatternItem::Literal(l) => w.write_str(&l)?,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
        for (length, expected) in samples {
            for (value, expected) in values.iter().zip(expected) {
                let mut s = String::new();
                format_number(&mut s, *value, *length).unwrap();
                assert_eq!(s, *expected);
            }
        }
    }
}
