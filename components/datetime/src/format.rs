// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::arithmetic;
use crate::date::{DateTimeInput, DateTimeInputWithLocale, LocalizedDateTimeInput};
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, FieldLength, FieldSymbol};
use crate::pattern::{Pattern, PatternItem};
use crate::provider;
use crate::provider::helpers::DateTimeSymbols;
use icu_locid::Locale;
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
/// # use icu_locid::Locale;
/// # use icu_locid_macros::langid;
/// # use icu_datetime::{DateTimeFormat, DateTimeFormatOptions};
/// # use icu_datetime::mock::MockDateTime;
/// # use icu_provider::inv::InvariantDataProvider;
/// # let locale: Locale = langid!("en").into();
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
    T: DateTimeInput,
{
    pub(crate) pattern: &'l Pattern,
    pub(crate) data: &'l provider::gregory::DatesV1,
    pub(crate) date_time: &'l T,
    pub(crate) locale: &'l Locale,
}

impl<'l, T> Writeable for FormattedDateTime<'l, T>
where
    T: DateTimeInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern(self.pattern, self.data, self.date_time, self.locale, sink)
            .map_err(|_| std::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedDateTime<'l, T>
where
    T: DateTimeInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(self.pattern, self.data, self.date_time, self.locale, f)
            .map_err(|_| std::fmt::Error)
    }
}

// Temporary formatting number with length.
fn format_number<W>(result: &mut W, num: isize, length: FieldLength) -> Result<(), std::fmt::Error>
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

pub fn write_pattern<T, W>(
    pattern: &crate::pattern::Pattern,
    data: &provider::gregory::DatesV1,
    date_time: &T,
    locale: &Locale,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    let loc_date_time = DateTimeInputWithLocale::new(date_time, locale);
    for item in pattern.items() {
        match item {
            PatternItem::Field(field) => write_field(pattern, &field, data, &loc_date_time, w)?,
            PatternItem::Literal(l) => w.write_str(l)?,
        }
    }
    Ok(())
}

fn write_field<T, W>(
    pattern: &crate::pattern::Pattern,
    field: &fields::Field,
    data: &crate::provider::gregory::DatesV1,
    date_time: &impl LocalizedDateTimeInput<T>,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    match field.symbol {
        FieldSymbol::Year(..) => format_number(
            w,
            date_time
                .date_time()
                .year()
                .ok_or(Error::MissingInputField)?
                .number as isize,
            field.length,
        )?,
        FieldSymbol::Month(month) => match field.length {
            FieldLength::One | FieldLength::TwoDigit => format_number(
                w,
                date_time
                    .date_time()
                    .month()
                    .ok_or(Error::MissingInputField)?
                    .number as isize,
                field.length,
            )?,
            length => {
                let symbol = data.symbols.get_symbol_for_month(
                    month,
                    length,
                    date_time
                        .date_time()
                        .month()
                        .ok_or(Error::MissingInputField)?
                        .number as usize
                        - 1,
                );
                w.write_str(symbol)?
            }
        },
        FieldSymbol::Weekday(weekday) => {
            let dow = date_time
                .date_time()
                .iso_weekday()
                .ok_or(Error::MissingInputField)?;
            let symbol = data
                .symbols
                .get_symbol_for_weekday(weekday, field.length, dow);
            w.write_str(symbol)?
        }
        FieldSymbol::Day(..) => format_number(
            w,
            date_time
                .date_time()
                .day_of_month()
                .ok_or(Error::MissingInputField)?
                .0 as isize,
            field.length,
        )?,
        FieldSymbol::Hour(hour) => {
            let h = usize::from(
                date_time
                    .date_time()
                    .hour()
                    .ok_or(Error::MissingInputField)?,
            ) as isize;
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
        FieldSymbol::Minute => format_number(
            w,
            usize::from(
                date_time
                    .date_time()
                    .minute()
                    .ok_or(Error::MissingInputField)?,
            ) as isize,
            field.length,
        )?,
        FieldSymbol::Second(..) => format_number(
            w,
            usize::from(
                date_time
                    .date_time()
                    .second()
                    .ok_or(Error::MissingInputField)?,
            ) as isize,
            field.length,
        )?,
        FieldSymbol::DayPeriod(period) => {
            let symbol = data.symbols.get_symbol_for_day_period(
                period,
                field.length,
                date_time
                    .date_time()
                    .hour()
                    .ok_or(Error::MissingInputField)?,
                arithmetic::is_top_of_hour(
                    &pattern,
                    date_time.date_time().minute().map(u8::from).unwrap_or(0),
                    date_time.date_time().second().map(u8::from).unwrap_or(0),
                ),
            );
            w.write_str(symbol)?
        }
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "provider_serde")]
    fn test_basic() {
        use crate::mock::MockDateTime;
        use icu_provider::prelude::*;
        let provider = icu_testdata::get_provider();
        let data = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: provider::key::GREGORY_V1,
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
        let date_time = MockDateTime::try_new(2020, 8, 1, 12, 34, 28).unwrap();
        let mut sink = String::new();
        write_pattern(
            &pattern,
            &data,
            &date_time,
            &"und".parse().unwrap(),
            &mut sink,
        )
        .unwrap();
        println!("{}", sink);
    }

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
