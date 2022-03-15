// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::{DateTimeInput, DateTimeInputWithLocale, LocalizedDateTimeInput};
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, Field, FieldLength, FieldSymbol, Week, Year};
use crate::pattern::{
    runtime::{Pattern, PatternPlurals},
    PatternItem,
};
use crate::provider;
use crate::provider::calendar::patterns::PatternPluralsFromPatternsV1Marker;
use crate::provider::date_time::DateTimeSymbols;
use crate::provider::week_data::WeekDataV1;

use alloc::string::ToString;
use core::fmt;
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
/// use icu::locid::Locale;
/// use icu::locid::langid;
/// use icu::datetime::{DateTimeFormat, DateTimeFormatOptions};
/// use icu::calendar::{DateTime, Gregorian};
/// use icu_provider::inv::InvariantDataProvider;
/// let locale: Locale = langid!("en").into();
/// # let provider = InvariantDataProvider;
/// # let options = DateTimeFormatOptions::default();
/// let dtf = DateTimeFormat::<Gregorian>::try_new(locale, &provider, &options)
///     .expect("Failed to create DateTimeFormat instance.");
///
/// let datetime = DateTime::new_gregorian_datetime_from_integers(2020, 9, 1, 12, 34, 28)
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
    pub(crate) patterns: &'l DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub(crate) symbols: Option<&'l provider::calendar::DateSymbolsV1<'l>>,
    pub(crate) datetime: &'l T,
    pub(crate) week_data: Option<&'l WeekDataV1>,
    pub(crate) locale: &'l Locale,
    pub(crate) ordinal_rules: Option<&'l PluralRules>,
}

impl<'l, T> Writeable for FormattedDateTime<'l, T>
where
    T: DateTimeInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern_plurals(
            &self.patterns.get().0,
            self.symbols,
            self.datetime,
            self.week_data,
            self.ordinal_rules,
            self.locale,
            sink,
        )
        .map_err(|_| core::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedDateTime<'l, T>
where
    T: DateTimeInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

// Temporary formatting number with length.
fn format_number<W>(result: &mut W, num: isize, length: FieldLength) -> Result<(), core::fmt::Error>
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

fn write_pattern<T, W>(
    pattern: &crate::pattern::runtime::Pattern,
    symbols: Option<&provider::calendar::DateSymbolsV1>,
    loc_datetime: &impl LocalizedDateTimeInput<T>,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    for item in pattern.items.iter() {
        match item {
            PatternItem::Field(field) => write_field(pattern, field, symbols, loc_datetime, w)?,
            PatternItem::Literal(ch) => w.write_char(ch)?,
        }
    }
    Ok(())
}

pub fn write_pattern_plurals<T, W>(
    patterns: &PatternPlurals,
    symbols: Option<&provider::calendar::DateSymbolsV1>,
    datetime: &T,
    week_data: Option<&WeekDataV1>,
    ordinal_rules: Option<&PluralRules>,
    locale: &Locale,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    let loc_datetime = DateTimeInputWithLocale::new(datetime, week_data.map(|d| &d.0), locale);
    let pattern = patterns.select(&loc_datetime, ordinal_rules)?;
    write_pattern(pattern, symbols, &loc_datetime, w)
}

// This function assumes that the correct decision has been
// made regarding availability of symbols in the caller.
//
// When modifying the list of fields using symbols,
// update the matching query in `analyze_pattern` function.
pub(super) fn write_field<T, W>(
    pattern: &crate::pattern::runtime::Pattern,
    field: fields::Field,
    symbols: Option<&crate::provider::calendar::DateSymbolsV1>,
    datetime: &impl LocalizedDateTimeInput<T>,
    w: &mut W,
) -> Result<(), Error>
where
    T: DateTimeInput,
    W: fmt::Write + ?Sized,
{
    match field.symbol {
        FieldSymbol::Era => {
            let symbol = symbols
                .expect("Expect symbols to be present")
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
                datetime
                    .datetime()
                    .year()
                    .ok_or(Error::MissingInputField)?
                    .number as isize,
                field.length,
            )?,
            Year::WeekOf => format_number(w, datetime.year_week()?.number as isize, field.length)?,
        },
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
                    )?;
                w.write_str(symbol)?
            }
        },
        FieldSymbol::Week(Week::WeekOfYear) => {
            format_number(w, datetime.week_of_year()?.0 as isize, field.length)?
        }
        field @ FieldSymbol::Week(_) => return Err(Error::UnsupportedField(field)),
        FieldSymbol::Weekday(weekday) => {
            let dow = datetime
                .datetime()
                .iso_weekday()
                .ok_or(Error::MissingInputField)?;
            let symbol = symbols
                .expect("Expect symbols to be present")
                .get_symbol_for_weekday(weekday, field.length, dow)?;
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
                    pattern.time_granularity.is_top_of_hour(
                        datetime.datetime().minute().map(u8::from).unwrap_or(0),
                        datetime.datetime().second().map(u8::from).unwrap_or(0),
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
    pub symbols_data: bool,
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
            if !self.symbols_data {
                self.symbols_data = match field.symbol {
                    FieldSymbol::Era => true,
                    FieldSymbol::Month(_) => {
                        !matches!(field.length, FieldLength::One | FieldLength::TwoDigit)
                    }
                    FieldSymbol::Weekday(_) | FieldSymbol::DayPeriod(_) => true,
                    _ => false,
                }
            }
            if !self.week_data {
                self.week_data = matches!(
                    field.symbol,
                    FieldSymbol::Year(Year::WeekOf) | FieldSymbol::Week(_)
                )
            }

            if supports_time_zones {
                if self.symbols_data && self.week_data {
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
    #[test]
    #[cfg(feature = "provider_serde")]
    fn test_basic() {
        use crate::provider::calendar::DateSymbolsV1Marker;
        use icu_calendar::DateTime;
        use icu_provider::prelude::*;

        let provider = icu_testdata::get_provider();
        let data: DataPayload<DateSymbolsV1Marker> = provider
            .load_resource(&DataRequest {
                options: ResourceOptions {
                    variant: Some("gregory".into()),
                    langid: Some("en".parse().unwrap()),
                },
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        let pattern = "MMM".parse().unwrap();
        let datetime =
            DateTime::new_gregorian_datetime_from_integers(2020, 8, 1, 12, 34, 28).unwrap();
        let mut sink = String::new();
        let loc_datetime = DateTimeInputWithLocale::new(&datetime, None, &"und".parse().unwrap());
        write_pattern(&pattern, Some(data.get()), &loc_datetime, &mut sink).unwrap();
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
