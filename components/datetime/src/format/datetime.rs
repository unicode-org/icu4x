// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::{self, Field, FieldLength, FieldSymbol, Second, Week, Year};
use crate::input::{
    DateTimeInput, DateTimeInputWithWeekConfig, ExtractedDateTimeInput, LocalizedDateTimeInput,
};
use crate::pattern::runtime::PatternMetadata;
use crate::pattern::{
    runtime::{Pattern, PatternPlurals},
    PatternItem,
};
use crate::provider;
use crate::provider::calendar::patterns::PatternPluralsFromPatternsV1Marker;
use crate::provider::date_time::MonthPlaceholderValue;
use crate::provider::date_time::{DateSymbols, TimeSymbols};
use crate::Error;

use core::fmt::{self, Write};
use fixed_decimal::FixedDecimal;
use icu_calendar::types::{IsoHour, IsoWeekday};
use icu_calendar::week::WeekCalculator;
use icu_calendar::AnyCalendarKind;
use icu_decimal::FixedDecimalFormatter;
use icu_plurals::PluralRules;
use icu_provider::DataPayload;
use writeable::{Part, Writeable};

/// [`FormattedDateTime`] is a intermediate structure which can be retrieved as
/// an output from [`TypedDateTimeFormatter`](crate::TypedDateTimeFormatter).
///
/// The structure contains all the information needed to display formatted value,
/// and it will also contain additional methods allowing the user to introspect
/// and even manipulate the formatted data.
///
/// # Examples
///
/// ```no_run
/// use icu::calendar::{DateTime, Gregorian};
/// use icu::datetime::TypedDateTimeFormatter;
/// use icu::locid::locale;
/// let dtf = TypedDateTimeFormatter::<Gregorian>::try_new(
///     &locale!("en").into(),
///     Default::default(),
/// )
/// .expect("Failed to create TypedDateTimeFormatter instance.");
///
/// let datetime = DateTime::try_new_gregorian_datetime(2020, 9, 1, 12, 34, 28)
///     .expect("Failed to construct DateTime.");
///
/// let formatted_date = dtf.format(&datetime);
///
/// let _ = format!("Date: {}", formatted_date);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct FormattedDateTime<'l> {
    pub(crate) patterns: &'l DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub(crate) date_symbols: Option<&'l provider::calendar::DateSymbolsV1<'l>>,
    pub(crate) time_symbols: Option<&'l provider::calendar::TimeSymbolsV1<'l>>,
    pub(crate) datetime: ExtractedDateTimeInput,
    pub(crate) week_data: Option<&'l WeekCalculator>,
    pub(crate) ordinal_rules: Option<&'l PluralRules>,
    pub(crate) fixed_decimal_format: &'l FixedDecimalFormatter,
}

impl<'l> Writeable for FormattedDateTime<'l> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut r = Ok(());

        let loc_datetime = DateTimeInputWithWeekConfig::new(&self.datetime, self.week_data);
        let (pattern, pattern_err) = self
            .patterns
            .get()
            .0
            .select_lossy(&loc_datetime, self.ordinal_rules);

        if let Some(e) = pattern_err {
            r = Err(e);
        }

        r = r.and(try_write_pattern(
            pattern.items.iter(),
            pattern.metadata,
            self.date_symbols,
            self.time_symbols,
            &loc_datetime,
            Some(self.fixed_decimal_format),
            &mut writeable::adapters::CoreWriteAsPartsWrite(sink),
        )?);

        debug_assert!(r.is_ok());
        Ok(())
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'l> fmt::Display for FormattedDateTime<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

// Apply length to input number and write to result using fixed_decimal_format.
fn try_write_number<W>(
    result: &mut W,
    fixed_decimal_format: Option<&FixedDecimalFormatter>,
    mut num: FixedDecimal,
    length: FieldLength,
) -> Result<Result<(), Error>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
{
    if let Some(fdf) = fixed_decimal_format {
        match length {
            FieldLength::One | FieldLength::NumericOverride(_) => {}
            FieldLength::TwoDigit => {
                num.pad_start(2);
                num.set_max_position(2);
            }
            FieldLength::Abbreviated => {
                num.pad_start(3);
            }
            FieldLength::Wide => {
                num.pad_start(4);
            }
            FieldLength::Narrow => {
                num.pad_start(5);
            }
            FieldLength::Six => {
                num.pad_start(6);
            }
            FieldLength::Fixed(p) => {
                num.pad_start(p as i16);
                num.set_max_position(p as i16);
            }
        }

        fdf.format(&num).write_to(result)?;
        Ok(Ok(()))
    } else {
        result.with_part(writeable::Part::ERROR, |r| num.write_to(r))?;
        Ok(Err(Error::FixedDecimal))
    }
}

pub(crate) fn try_write_pattern<'data, T, W, DS, TS>(
    pattern_items: impl Iterator<Item = PatternItem>,
    pattern_metadata: PatternMetadata,
    date_symbols: Option<&DS>,
    time_symbols: Option<&TS>,
    loc_datetime: &impl LocalizedDateTimeInput<T>,
    fixed_decimal_format: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), Error>, fmt::Error>
where
    T: DateTimeInput,
    W: writeable::PartsWrite + ?Sized,
    DS: DateSymbols<'data>,
    TS: TimeSymbols,
{
    let mut r = Ok(());
    let mut iter = pattern_items.peekable();
    loop {
        match iter.next() {
            Some(PatternItem::Field(field)) => {
                r = r.and(try_write_field(
                    pattern_metadata,
                    field,
                    iter.peek(),
                    date_symbols,
                    time_symbols,
                    loc_datetime,
                    fixed_decimal_format,
                    w,
                )?)
            }
            Some(PatternItem::Literal(ch)) => w.write_char(ch)?,
            None => break,
        }
    }
    Ok(r)
}

const CHINESE_CYCLIC_YEARS: [&str; 60] = [
    "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉", "甲戌", "乙亥",
    "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未", "甲申", "乙酉", "丙戌", "丁亥",
    "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳", "甲午", "乙未", "丙申", "丁酉", "戊戌", "己亥",
    "庚子", "辛丑", "壬寅", "癸卯", "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥",
    "壬子", "癸丑", "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未", "庚申", "辛酉", "壬戌", "癸亥",
];
const DANGI_CYCLIC_YEARS: [&str; 60] = [
    "갑자", "을축", "병인", "정묘", "무진", "기사", "경오", "신미", "임신", "계유", "갑술", "을해",
    "병자", "정축", "무인", "기묘", "경진", "신사", "임오", "계미", "갑신", "을유", "병술", "정해",
    "무자", "기축", "경인", "신묘", "임진", "계사", "갑오", "을미", "병신", "정유", "무술", "기해",
    "경자", "신축", "임인", "계묘", "갑진", "을사", "병오", "정미", "무신", "기유", "경술", "신해",
    "임자", "계축", "갑인", "을묘", "병진", "정사", "무오", "기미", "경신", "신유", "임술", "계해",
];

const CHINESE_LEAP_PREFIX: &str = "閏";
const DANGI_LEAP_PREFIX: &str = "윤";
const PLACEHOLDER_LEAP_PREFIX: &str = "(leap)";
// This function assumes that the correct decision has been
// made regarding availability of symbols in the caller.
//
// When modifying the list of fields using symbols,
// update the matching query in `analyze_pattern` function.
#[allow(clippy::too_many_arguments)]
pub(super) fn try_write_field<'data, T, W, DS, TS>(
    pattern_metadata: PatternMetadata,
    field: fields::Field,
    next_item: Option<&PatternItem>,
    date_symbols: Option<&DS>,
    time_symbols: Option<&TS>,
    datetime: &impl LocalizedDateTimeInput<T>,
    fdf: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), Error>, fmt::Error>
where
    T: DateTimeInput,
    W: writeable::PartsWrite + ?Sized,
    DS: DateSymbols<'data>,
    TS: TimeSymbols,
{
    // Writes an error string for the current symbol
    let write_value_missing = |w: &mut W| {
        w.with_part(Part::ERROR, |w| {
            "{".write_to(w)?;
            char::from(field.symbol).write_to(w)?;
            "}".write_to(w)
        })
    };

    Ok(match (field.symbol, field.length) {
        (FieldSymbol::Era, l) => {
            if let Some(year) = datetime.datetime().year() {
                match date_symbols
                    .ok_or(Error::MissingDateSymbols)
                    .and_then(|ds| {
                        ds.get_symbol_for_era(l, &year.era)
                            .map(|e| e.unwrap_or(&year.era.0))
                    }) {
                    Ok(era) => Ok(w.write_str(era)?),
                    Err(e) => {
                        w.with_part(Part::ERROR, |w| w.write_str(&year.era.0))?;
                        Err(e)
                    }
                }
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("year")))
            }
        }
        (FieldSymbol::Year(Year::Calendar), l) => {
            if let Some(year) = datetime.datetime().year() {
                try_write_number(w, fdf, year.number.into(), l)?
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("year")))
            }
        }
        (FieldSymbol::Year(Year::WeekOf), l) => match datetime.week_of_year() {
            Ok((year, _)) => try_write_number(w, fdf, year.number.into(), l)?,
            Err(e) => {
                write_value_missing(w)?;
                Err(Error::DateTimeInput(e))
            }
        },
        (FieldSymbol::Year(Year::Cyclic), l) => {
            let datetime = datetime.datetime();

            // TODO(#3761): This is a hack, we should use actual data for cyclic years
            let cyclics = match datetime.any_calendar_kind() {
                Some(AnyCalendarKind::Dangi) => &DANGI_CYCLIC_YEARS,
                _ => &CHINESE_CYCLIC_YEARS, /* for now assume all other calendars use the stem-branch model */
            };

            if let Some(year) = datetime.year() {
                match year
                    .cyclic
                    .ok_or(Error::MissingInputField(Some("cyclic")))
                    .and_then(|cyclic| {
                        cyclics
                            .get((cyclic.get() - 1) as usize)
                            .ok_or(Error::DateTimeInput(
                                icu_calendar::CalendarError::Overflow {
                                    field: "cyclic",
                                    max: cyclics.len(),
                                },
                            ))
                    }) {
                    Ok(cyclic_str) => Ok(w.write_str(cyclic_str)?),
                    Err(e) => {
                        w.with_part(Part::ERROR, |w| {
                            try_write_number(w, fdf, year.number.into(), l).map(|_| ())
                        })?;
                        Err(e)
                    }
                }
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("year")))
            }
        }
        (FieldSymbol::Year(Year::RelatedIso), l) => {
            match datetime
                .datetime()
                .year()
                .ok_or(Error::MissingInputField(Some("year")))
                .and_then(|year| {
                    year.related_iso
                        .ok_or(Error::MissingInputField(Some("related_iso")))
                }) {
                Ok(iso) => try_write_number(w, fdf, iso.into(), l)?,
                Err(e) => {
                    write_value_missing(w)?;
                    Err(e)
                }
            }
        }
        (FieldSymbol::Month(_), l @ (FieldLength::One | FieldLength::TwoDigit)) => {
            if let Some(month) = datetime.datetime().month() {
                try_write_number(w, fdf, month.ordinal.into(), l)?
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("month")))
            }
        }
        (FieldSymbol::Month(month), l) => {
            let datetime = datetime.datetime();
            if let Some(formattable_month) = datetime.month() {
                match date_symbols
                    .ok_or(Error::MissingDateSymbols)
                    .and_then(|ds| ds.get_symbol_for_month(month, l, formattable_month.code))
                {
                    Ok(MonthPlaceholderValue::PlainString(symbol)) => {
                        w.write_str(symbol)?;
                        Ok(())
                    }
                    Ok(MonthPlaceholderValue::StringNeedingLeapPrefix(symbol)) => {
                        // FIXME (#3766) this should be using actual data for leap months
                        let leap_str = match datetime.any_calendar_kind() {
                            Some(AnyCalendarKind::Chinese) => CHINESE_LEAP_PREFIX,
                            Some(AnyCalendarKind::Dangi) => DANGI_LEAP_PREFIX,
                            _ => PLACEHOLDER_LEAP_PREFIX,
                        };
                        w.write_str(leap_str)?;
                        w.write_str(symbol)?;
                        Ok(())
                    }
                    #[cfg(feature = "experimental")]
                    Ok(MonthPlaceholderValue::Numeric) => {
                        try_write_number(w, fdf, formattable_month.ordinal.into(), l)?
                    }
                    #[cfg(feature = "experimental")]
                    Ok(MonthPlaceholderValue::NumericPattern(substitution_pattern)) => {
                        w.write_str(substitution_pattern.get_prefix())?;
                        let r = try_write_number(w, fdf, formattable_month.ordinal.into(), l)?;
                        w.write_str(substitution_pattern.get_suffix())?;
                        r
                    }
                    Err(e) => {
                        w.with_part(Part::ERROR, |w| w.write_str(&formattable_month.code.0))?;
                        Err(e)
                    }
                }
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("month")))
            }
        }
        (FieldSymbol::Week(week), l) => match week {
            Week::WeekOfYear => match datetime.week_of_year() {
                Ok((_, week_of_year)) => try_write_number(w, fdf, week_of_year.0.into(), l)?,
                Err(e) => {
                    write_value_missing(w)?;
                    Err(Error::DateTimeInput(e))
                }
            },
            Week::WeekOfMonth => match datetime.week_of_month() {
                Ok(week_of_month) => try_write_number(w, fdf, week_of_month.0.into(), l)?,
                Err(e) => {
                    write_value_missing(w)?;
                    Err(Error::DateTimeInput(e))
                }
            },
        },
        (FieldSymbol::Weekday(weekday), l) => {
            if let Some(wd) = datetime.datetime().iso_weekday() {
                match date_symbols
                    .ok_or(Error::MissingDateSymbols)
                    .and_then(|ds| ds.get_symbol_for_weekday(weekday, l, wd))
                {
                    Ok(s) => Ok(w.write_str(s)?),
                    Err(e) => {
                        w.with_part(Part::ERROR, |w| {
                            w.write_str(match wd {
                                IsoWeekday::Monday => "mon",
                                IsoWeekday::Tuesday => "tue",
                                IsoWeekday::Wednesday => "wed",
                                IsoWeekday::Thursday => "thu",
                                IsoWeekday::Friday => "fri",
                                IsoWeekday::Saturday => "sat",
                                IsoWeekday::Sunday => "sun",
                            })
                        })?;
                        Err(e)
                    }
                }
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("iso_weekday")))
            }
        }
        (FieldSymbol::Day(fields::Day::DayOfMonth), l) => {
            if let Some(d) = datetime.datetime().day_of_month() {
                try_write_number(w, fdf, d.0.into(), l)?
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("day_of_month")))
            }
        }
        (FieldSymbol::Day(fields::Day::DayOfWeekInMonth), l) => {
            match datetime.day_of_week_in_month() {
                Ok(d) => try_write_number(w, fdf, d.0.into(), l)?,
                Err(e) => {
                    write_value_missing(w)?;
                    Err(Error::DateTimeInput(e))
                }
            }
        }
        (FieldSymbol::Hour(hour), l) => {
            if let Some(h) = datetime.datetime().hour() {
                let h = usize::from(h) as isize;
                let h = match hour {
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
                try_write_number(w, fdf, h.into(), l)?
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("hour")))
            }
        }
        (FieldSymbol::Minute, l) => {
            if let Some(iso_minute) = datetime.datetime().minute() {
                try_write_number(w, fdf, usize::from(iso_minute).into(), l)?
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("minute")))
            }
        }
        (FieldSymbol::Second(Second::Second), l) => {
            if let Some(seconds) = datetime.datetime().second() {
                if let Some(PatternItem::Field(Field {
                    symbol: FieldSymbol::Second(Second::FractionalSecond),
                    length,
                })) = next_item
                {
                    // We only support fixed field length for fractional seconds.
                    if let &FieldLength::Fixed(p) = length {
                        if let Some(nanoseconds) = datetime.datetime().nanosecond() {
                            // Assemble the seconds and nanoseconds into a FixedDecimal
                            let mut fd = FixedDecimal::from(usize::from(seconds));
                            // n * 10^-9 is fractional
                            let _infallible = fd.concatenate_end(
                                FixedDecimal::from(usize::from(nanoseconds)).multiplied_pow10(-9),
                            );
                            debug_assert!(_infallible.is_ok());
                            fd.pad_end(-(p as i16));
                            try_write_number(w, fdf, fd, l)?
                        } else {
                            write_value_missing(w)?;
                            Err(Error::MissingInputField(Some("nanosecond")))
                        }
                    } else {
                        Err(Error::Pattern(
                            crate::pattern::PatternError::FieldLengthInvalid(FieldSymbol::Second(
                                Second::FractionalSecond,
                            )),
                        ))
                    }
                } else {
                    // No fractional seconds
                    try_write_number(w, fdf, usize::from(seconds).into(), l)?
                }
            } else {
                write_value_missing(w)?;
                Err(Error::MissingInputField(Some("second")))
            }
        }
        // Formatting of fractional seconds is handled when formatting seconds.
        (FieldSymbol::Second(Second::FractionalSecond), _) => Ok(()),
        (FieldSymbol::DayPeriod(period), l) => {
            let is_top_of_hour = pattern_metadata.time_granularity().is_top_of_hour(
                datetime.datetime().minute().map(u8::from).unwrap_or(0),
                datetime.datetime().second().map(u8::from).unwrap_or(0),
                datetime.datetime().nanosecond().map(u32::from).unwrap_or(0),
            );

            let hour = datetime.datetime().hour();

            match (
                time_symbols
                    .ok_or(Error::MissingTimeSymbols)
                    .and_then(|ts| {
                        ts.get_symbol_for_day_period(
                            period,
                            l,
                            // Hour falls back to 0, so the day period should be calculated for that
                            #[allow(clippy::unwrap_used)]
                            hour.unwrap_or(IsoHour::try_from(0u8).unwrap()),
                            is_top_of_hour,
                        )
                    }),
                hour,
            ) {
                (Ok(symbol), Some(_)) => Ok(w.write_str(symbol)?),
                (Ok(symbol), None) => {
                    w.write_str(symbol)?;
                    Err(Error::MissingInputField(Some("hour")))
                }
                (Err(e), h) => {
                    w.with_part(Part::ERROR, |w| {
                        w.write_str(if h.map(usize::from).unwrap_or_default() < 12 {
                            "AM"
                        } else {
                            "PM"
                        })
                    })?;
                    Err(e)
                }
            }
        }
        (
            FieldSymbol::TimeZone(_)
            | FieldSymbol::Day(_)
            | FieldSymbol::Second(Second::Millisecond),
            _,
        ) => {
            w.with_part(Part::ERROR, |w| {
                w.write_str("{unsupported:")?;
                w.write_char(char::from(field.symbol))?;
                w.write_str("}")
            })?;
            Err(Error::UnsupportedField(field.symbol))
        }
    })
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
#[allow(unused_imports)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use crate::pattern::runtime;
    use icu_decimal::options::{FixedDecimalFormatterOptions, GroupingStrategy};
    use icu_locid::Locale;

    #[test]
    fn test_mixed_calendar_eras() {
        use icu::calendar::japanese::JapaneseExtended;
        use icu::calendar::Date;
        use icu::datetime::options::length;
        use icu::datetime::DateFormatter;

        let locale: Locale = "en-u-ca-japanese".parse().unwrap();
        let dtf = DateFormatter::try_new_with_length(&locale.into(), length::Date::Medium)
            .expect("DateTimeFormat construction succeeds");

        let date = Date::try_new_gregorian_date(1800, 9, 1).expect("Failed to construct Date.");
        let date = date
            .to_calendar(JapaneseExtended::new())
            .into_japanese_date()
            .to_any();

        writeable::assert_writeable_eq!(dtf.format(&date).unwrap(), "Sep 1, 12 kansei-1789")
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_basic() {
        use crate::provider::calendar::{GregorianDateSymbolsV1Marker, TimeSymbolsV1Marker};
        use icu_calendar::DateTime;
        use icu_provider::prelude::*;

        let locale = "en-u-ca-gregory".parse::<Locale>().unwrap().into();
        let req = DataRequest {
            locale: &locale,
            metadata: Default::default(),
        };
        let date_data: DataPayload<GregorianDateSymbolsV1Marker> = crate::provider::Baked
            .load(req)
            .unwrap()
            .take_payload()
            .unwrap();
        let time_data: DataPayload<TimeSymbolsV1Marker> = crate::provider::Baked
            .load(req)
            .unwrap()
            .take_payload()
            .unwrap();
        let pattern: runtime::Pattern = "MMM".parse().unwrap();
        let datetime = DateTime::try_new_gregorian_datetime(2020, 8, 1, 12, 34, 28).unwrap();
        let fixed_decimal_format =
            FixedDecimalFormatter::try_new(&locale, Default::default()).unwrap();

        let mut sink = String::new();
        let loc_datetime = DateTimeInputWithWeekConfig::new(&datetime, None);
        try_write_pattern(
            pattern.items.iter(),
            pattern.metadata,
            Some(date_data.get()),
            Some(time_data.get()),
            &loc_datetime,
            Some(&fixed_decimal_format),
            &mut writeable::adapters::CoreWriteAsPartsWrite(&mut sink),
        )
        .unwrap()
        .unwrap();
        println!("{sink}");
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

        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;
        let fixed_decimal_format = FixedDecimalFormatter::try_new(
            &icu_locid::locale!("en").into(),
            fixed_decimal_format_options,
        )
        .unwrap();

        for (length, expected) in samples {
            for (value, expected) in values.iter().zip(expected) {
                let mut s = String::new();
                try_write_number(
                    &mut writeable::adapters::CoreWriteAsPartsWrite(&mut s),
                    Some(&fixed_decimal_format),
                    FixedDecimal::from(*value),
                    *length,
                )
                .unwrap()
                .unwrap();
                assert_eq!(s, *expected);
            }
        }
    }
}
