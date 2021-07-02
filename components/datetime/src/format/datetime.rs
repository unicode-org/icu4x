// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::arithmetic;
use crate::date::{DateTimeInput, DateTimeInputWithLocale, LocalizedDateTimeInput};
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, Field, FieldLength, FieldSymbol};
use crate::pattern::{Pattern, PatternItem};
use crate::provider;
use crate::provider::helpers::DateTimeSymbols;
use icu_locid::Locale;
use std::fmt;
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
/// use icu::locid::Locale;
/// use icu::locid::macros::langid;
/// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
/// use icu::datetime::mock::datetime::MockDateTime;
/// use icu_provider::inv::InvariantDataProvider;
/// let locale: Locale = langid!("en").into();
/// # let provider = InvariantDataProvider;
/// # let options = DateTimeFormatOptions::default();
/// let dtf = DateTimeFormat::try_new(locale, &provider, &options)
///     .expect("Failed to create DateTimeFormat instance.");
///
/// let datetime = MockDateTime::try_new(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// let formatted_date = dtf.format(&datetime);
///
/// let _ = format!("Date: {}", formatted_date);
/// ```
pub struct FormattedDateTime<'l, T>
where
    T: DateTimeInput,
{
    pub(crate) pattern: &'l Pattern,
    pub(crate) symbols: Option<&'l provider::gregory::DateSymbolsV1>,
    pub(crate) datetime: &'l T,
    pub(crate) locale: &'l Locale,
}

impl<'l, T> Writeable for FormattedDateTime<'l, T>
where
    T: DateTimeInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern(self.pattern, self.symbols, self.datetime, self.locale, sink)
            .map_err(|_| std::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedDateTime<'l, T>
where
    T: DateTimeInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(self.pattern, self.symbols, self.datetime, self.locale, f)
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
    symbols: Option<&provider::gregory::DateSymbolsV1>,
    datetime: &T,
    locale: &Locale,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    let loc_datetime = DateTimeInputWithLocale::new(datetime, locale);
    for item in pattern.items() {
        match item {
            PatternItem::Field(field) => write_field(pattern, field, symbols, &loc_datetime, w)?,
            PatternItem::Literal(l) => w.write_str(l)?,
        }
    }
    Ok(())
}

// This function assumes that the correct decision has been
// made regarding availability of symbols in the caller.
//
// When modifying the list of fields using symbols,
// update the matching query in `analyze_pattern` function.
pub(super) fn write_field<T, W>(
    pattern: &crate::pattern::Pattern,
    field: &fields::Field,
    symbols: Option<&crate::provider::gregory::DateSymbolsV1>,
    datetime: &impl LocalizedDateTimeInput<T>,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    match field.symbol {
        FieldSymbol::Year(..) => format_number(
            w,
            datetime
                .datetime()
                .year()
                .ok_or(Error::MissingInputField)?
                .number as isize,
            field.length,
        )?,
        FieldSymbol::Month(month) => match field.length {
            FieldLength::One | FieldLength::TwoDigit => format_number(
                w,
                datetime
                    .datetime()
                    .month()
                    .ok_or(Error::MissingInputField)?
                    .number as isize,
                field.length,
            )?,
            length => {
                let symbol = symbols
                    .expect("Expect symbols to be present")
                    .get_symbol_for_month(
                        month,
                        length,
                        datetime
                            .datetime()
                            .month()
                            .ok_or(Error::MissingInputField)?
                            .number as usize
                            - 1,
                    );
                w.write_str(symbol)?
            }
        },
        FieldSymbol::Weekday(weekday) => {
            let dow = datetime
                .datetime()
                .iso_weekday()
                .ok_or(Error::MissingInputField)?;
            let symbol = symbols
                .expect("Expect symbols to be present")
                .get_symbol_for_weekday(weekday, field.length, dow);
            w.write_str(symbol)?
        }
        FieldSymbol::Day(..) => format_number(
            w,
            datetime
                .datetime()
                .day_of_month()
                .ok_or(Error::MissingInputField)?
                .0 as isize,
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
            format_number(w, value, field.length)?
        }
        FieldSymbol::Minute => format_number(
            w,
            usize::from(
                datetime
                    .datetime()
                    .minute()
                    .ok_or(Error::MissingInputField)?,
            ) as isize,
            field.length,
        )?,
        FieldSymbol::Second(..) => format_number(
            w,
            usize::from(
                datetime
                    .datetime()
                    .second()
                    .ok_or(Error::MissingInputField)?,
            ) as isize,
            field.length,
        )?,
        FieldSymbol::DayPeriod(period) => {
            let symbol = symbols
                .expect("Expect symbols to be present")
                .get_symbol_for_day_period(
                    period,
                    field.length,
                    datetime.datetime().hour().ok_or(Error::MissingInputField)?,
                    arithmetic::is_top_of_hour(
                        pattern,
                        datetime.datetime().minute().map(u8::from).unwrap_or(0),
                        datetime.datetime().second().map(u8::from).unwrap_or(0),
                    ),
                );
            w.write_str(symbol)?
        }
        field @ FieldSymbol::TimeZone(_) => return Err(Error::UnsupportedField(field)),
    };
    Ok(())
}

// This function determins whether the struct will load symbols data.
// Keep it in sync with the `write_field` use of symbols.
pub fn analyze_pattern(pattern: &Pattern, supports_time_zones: bool) -> Result<bool, &Field> {
    let fields = pattern.items().iter().filter_map(|p| match p {
        PatternItem::Field(field) => Some(field),
        _ => None,
    });

    let mut requires_symbols = false;

    for field in fields {
        if !requires_symbols {
            requires_symbols = match field.symbol {
                FieldSymbol::Month(_) => {
                    !matches!(field.length, FieldLength::One | FieldLength::TwoDigit)
                }
                FieldSymbol::Weekday(_) | FieldSymbol::DayPeriod(_) => true,
                _ => false,
            }
        }

        if supports_time_zones {
            if requires_symbols {
                // If we require time zones, and symbols, we know all
                // we need to return already.
                break;
            }
        } else if matches!(field.symbol, FieldSymbol::TimeZone(_)) {
            // If we don't support time zones, and encountered a time zone
            // field, error out.
            return Err(field);
        }
    }

    Ok(requires_symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "provider_serde")]
    fn test_basic() {
        use crate::mock::datetime::MockDateTime;
        use crate::provider::gregory::DateSymbolsV1Marker;
        use icu_provider::prelude::*;

        let provider = icu_testdata::get_provider();
        let data: DataPayload<DateSymbolsV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: provider::key::GREGORY_DATE_SYMBOLS_V1,
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
        let datetime = MockDateTime::try_new(2020, 8, 1, 12, 34, 28).unwrap();
        let mut sink = String::new();
        write_pattern(
            &pattern,
            Some(&data.get()),
            &datetime,
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
