// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::date::{self, DateTimeType};
use crate::date_new::DateTimeInput;
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, FieldLength, FieldSymbol};
use crate::pattern::{Pattern, PatternItem};
use crate::provider::DateTimeDates;
use icu_provider::structs;
use std::fmt;

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
/// # let lid = langid!("en");
/// # let provider = InvariantDataProvider;
/// # let options = DateTimeFormatOptions::default();
/// let dtf = DateTimeFormat::try_new(lid, &provider, &options)
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
    pub(crate) data: &'l structs::dates::gregory::DatesV1,
    pub(crate) date_time: &'l T,
}

// TODO(#181): Implement Writeable instead of fmt::Display
impl<'l, T> fmt::Display for FormattedDateTime<'l, T>
where
    T: DateTimeType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(self.pattern, self.data, self.date_time, f).map_err(|_| std::fmt::Error)
    }
}

// Temporary formatting number with length.
fn format_number(
    result: &mut impl fmt::Write,
    num: isize,
    length: &FieldLength,
) -> Result<(), std::fmt::Error> {
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
        length => write!(result, "{:0>width$}", num, width = *length as usize),
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

pub fn write_pattern(
    pattern: &crate::pattern::Pattern,
    data: &structs::dates::gregory::DatesV1,
    date_time: &impl DateTimeType,
    w: &mut impl fmt::Write,
) -> Result<(), Error> {
    for item in &pattern.0 {
        match item {
            PatternItem::Field(field) => write_field_old(field, data, date_time, w)?,
            PatternItem::Literal(l) => w.write_str(l)?,
        }
    }
    Ok(())
}

pub fn write_pattern_new(
    pattern: &crate::pattern::Pattern,
    data: &structs::dates::gregory::DatesV1,
    date_time: &impl DateTimeInput,
    w: &mut impl fmt::Write,
) -> Result<(), Error> {
    for item in &pattern.0 {
        match item {
            PatternItem::Field(field) => write_field_new(field, data, date_time, w)?,
            PatternItem::Literal(l) => w.write_str(l)?,
        }
    }
    Ok(())
}

fn write_field_old(
    field: &fields::Field,
    data: &structs::dates::gregory::DatesV1,
    date_time: &impl DateTimeType,
    w: &mut impl fmt::Write,
) -> Result<(), Error> {
    match field.symbol {
        FieldSymbol::Year(..) => format_number(w, date_time.year() as isize, &field.length)?,
        FieldSymbol::Month(month) => match field.length {
            FieldLength::One | FieldLength::TwoDigit => {
                format_number(w, usize::from(date_time.month()) as isize + 1, &field.length)?
            }
            length => {
                let symbol = data.get_symbol_for_month(month, length, date_time.month().into());
                w.write_str(symbol)?
            }
        },
        FieldSymbol::Weekday(weekday) => {
            // FIXME: Get rid of unwrap
            use std::convert::TryFrom;
            let dow = get_day_of_week(usize::try_from(date_time.year()).unwrap(), date_time.month(), date_time.day());
            let symbol = data.get_symbol_for_weekday(weekday, field.length, dow);
            w.write_str(symbol)?
        }
        FieldSymbol::Day(..) => format_number(w, usize::from(date_time.day()) as isize + 1, &field.length)?,
        FieldSymbol::Hour(hour) => {
            let h = usize::from(date_time.hour()) as isize;
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
            format_number(w, value, &field.length)?
        }
        FieldSymbol::Minute => format_number(w, usize::from(date_time.minute()) as isize, &field.length)?,
        FieldSymbol::Second(..) => format_number(w, usize::from(date_time.second()) as isize, &field.length)?,
        FieldSymbol::DayPeriod(period) => match period {
            fields::DayPeriod::AmPm => {
                let symbol = data.get_symbol_for_day_period(period, field.length, date_time.hour());
                w.write_str(symbol)?
            }
        },
    };
    Ok(())
}

fn write_field_new(
    field: &fields::Field,
    data: &structs::dates::gregory::DatesV1,
    date_time: &impl DateTimeInput,
    w: &mut impl fmt::Write,
) -> Result<(), Error> {
    match field {
        fields::Field {
            symbol: fields::FieldSymbol::Year(fields::Year::Calendar),
            length,
        } => format_number(w, date_time.year().ok_or(Error::MissingInputField)?.number as isize, length)?,

        // fields::Field {
        //     symbol: fields::FieldSymbol::Year(fields::Year::WeekOf),
        //     length,
        // } => format_number(w, date_time.year_week().number, length)?,

        fields::Field {
            symbol: fields::FieldSymbol::Month(month_type),
            length,
        } => match length {
            FieldLength::One | FieldLength::TwoDigit => {
                format_number(w, date_time.month().ok_or(Error::MissingInputField)?.number as isize + 1, length)?
            }
            _ => {
                let symbol =
                    data.get_symbol_for_month(*month_type, *length, date_time.month().ok_or(Error::MissingInputField)?.number as usize);
                w.write_str(symbol)?;
            }
        },

        _ => unimplemented!(),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_provider::prelude::*;

    #[test]
    fn test_new() {
        let provider = icu_testdata::get_provider();
        let data = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: structs::dates::key::GREGORY_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some("en".parse().unwrap()),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let pattern = crate::pattern::Pattern::from_bytes("MMM").unwrap();
        let date_time = date::MockDateTime::try_new(2020, 8, 1, 12, 34, 28).unwrap();
        let mut sink = String::new();
        write_pattern_new(&pattern, &data, &date_time, &mut sink).unwrap();
        println!("{}", sink);
    }

    #[test]
    fn test_format_numer() {
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
                format_number(&mut s, *value, length).unwrap();
                assert_eq!(s, *expected);
            }
        }
    }
}
