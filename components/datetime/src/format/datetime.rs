// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::{self, Field, FieldLength, FieldSymbol, Second, Week, Year};
use crate::input::{DateInput, ExtractedDateTimeInput, IsoTimeInput};
use crate::pattern::runtime::{PatternBorrowed, PatternMetadata};
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
use core::iter::Peekable;
use fixed_decimal::FixedDecimal;
use icu_calendar::types::{DayOfWeekInMonth, IsoWeekday};
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
    pub(crate) datetime: ExtractedDateTimeInput,
    pub(crate) patterns: &'l DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub(crate) date_symbols: Option<&'l provider::calendar::DateSymbolsV1<'l>>,
    pub(crate) time_symbols: Option<&'l provider::calendar::TimeSymbolsV1<'l>>,
    pub(crate) week_data: Option<&'l WeekCalculator>,
    pub(crate) ordinal_rules: Option<&'l PluralRules>,
    pub(crate) fixed_decimal_format: &'l FixedDecimalFormatter,
}

impl<'l> FormattedDateTime<'l> {
    pub(crate) fn select_pattern_lossy<'a>(&'a self) -> (&'l Pattern<'l>, Result<(), Error>) {
        let mut r = Ok(());
        let pattern = match self.patterns.get().0 {
            PatternPlurals::SinglePattern(ref pattern) => pattern,
            PatternPlurals::MultipleVariants(ref plural_pattern) => {
                let week_number = match plural_pattern.pivot_field() {
                    Week::WeekOfMonth => self
                        .week_data
                        .ok_or(Error::MissingWeekCalculator)
                        .and_then(|w| self.datetime.week_of_month(w))
                        .map(|w| w.0)
                        .unwrap_or_else(|e| {
                            r = r.and(Err(e));
                            0
                        }),
                    Week::WeekOfYear => self
                        .week_data
                        .ok_or(Error::MissingWeekCalculator)
                        .and_then(|w| self.datetime.week_of_year(w))
                        .map(|w| w.1 .0)
                        .unwrap_or_else(|e| {
                            r = r.and(Err(e));
                            0
                        }),
                };
                let category = self
                    .ordinal_rules
                    .map(|p| p.category_for(week_number))
                    .unwrap_or_else(|| {
                        r = r.and(Err(Error::MissingOrdinalRules));
                        icu_plurals::PluralCategory::One
                    });
                plural_pattern.variant(category)
            }
        };
        (pattern, r)
    }
}

impl<'l> Writeable for FormattedDateTime<'l> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let (pattern, mut r) = self.select_pattern_lossy();

        r = r.and(try_write_pattern(
            pattern.as_borrowed(),
            &self.datetime,
            self.date_symbols,
            self.time_symbols,
            self.week_data,
            Some(self.fixed_decimal_format),
            &mut writeable::adapters::CoreWriteAsPartsWrite(sink),
        )?);

        debug_assert!(r.is_ok(), "{r:?}");
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

#[allow(clippy::too_many_arguments)]
pub(crate) fn try_write_pattern<'data, W, DS, TS>(
    pattern: PatternBorrowed<'data>,
    datetime: &ExtractedDateTimeInput,
    date_symbols: Option<&DS>,
    time_symbols: Option<&TS>,
    week_data: Option<&'data WeekCalculator>,
    fixed_decimal_format: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), Error>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
    DS: DateSymbols<'data>,
    TS: TimeSymbols,
{
    let mut r = Ok(());
    let mut iter = pattern.items.iter().peekable();
    while let Some(item) = iter.next() {
        match item {
            PatternItem::Literal(ch) => w.write_char(ch)?,
            PatternItem::Field(field) => {
                r = r.and(try_write_field(
                    field,
                    &mut iter,
                    pattern.metadata,
                    datetime,
                    date_symbols,
                    time_symbols,
                    week_data,
                    fixed_decimal_format,
                    w,
                )?)
            }
        }
    }
    Ok(r)
}

// This function assumes that the correct decision has been
// made regarding availability of symbols in the caller.
//
// When modifying the list of fields using symbols,
// update the matching query in `analyze_pattern` function.
#[allow(clippy::too_many_arguments)]
pub(crate) fn try_write_field<'data, W, DS, TS>(
    field: fields::Field,
    iter: &mut Peekable<impl Iterator<Item = PatternItem>>,
    pattern_metadata: PatternMetadata,
    datetime: &ExtractedDateTimeInput,
    date_symbols: Option<&DS>,
    time_symbols: Option<&TS>,
    week_data: Option<&WeekCalculator>,
    fdf: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), Error>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
    DS: DateSymbols<'data>,
    TS: TimeSymbols,
{
    // Writes an error string for the given symbol
    fn write_value_missing(
        w: &mut (impl writeable::PartsWrite + ?Sized),
        field: fields::Field,
    ) -> Result<(), fmt::Error> {
        w.with_part(Part::ERROR, |w| {
            "{".write_to(w)?;
            char::from(field.symbol).write_to(w)?;
            "}".write_to(w)
        })
    }

    Ok(match (field.symbol, field.length) {
        (FieldSymbol::Era, l) => match datetime.year() {
            None => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("year")))
            }
            Some(year) => match date_symbols
                .ok_or(Error::MissingDateSymbols)
                .and_then(|ds| {
                    ds.get_symbol_for_era(l, &year.era)?
                        .ok_or(Error::MissingDateSymbols)
                }) {
                Ok(era) => Ok(w.write_str(era)?),
                Err(e) => {
                    w.with_part(Part::ERROR, |w| w.write_str(&year.era.0))?;
                    Err(e)
                }
            },
        },
        (FieldSymbol::Year(Year::Calendar), l) => match datetime.year() {
            None => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("year")))
            }
            Some(year) => try_write_number(w, fdf, year.number.into(), l)?,
        },
        (FieldSymbol::Year(Year::WeekOf), l) => match week_data
            .ok_or(Error::MissingWeekCalculator)
            .and_then(|w| datetime.week_of_year(w))
        {
            Err(e) => {
                write_value_missing(w, field)?;
                Err(e)
            }
            Ok((year, _)) => try_write_number(w, fdf, year.number.into(), l)?,
        },
        (FieldSymbol::Year(Year::Cyclic), l) => match datetime.year() {
            None => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("year")))
            }
            Some(year) => {
                let r = year
                    .cyclic
                    .ok_or(Error::MissingInputField(Some("cyclic")))
                    .and_then(|cyclic| {
                        // TODO(#3761): This is a hack, we should use actual data for cyclic years
                        let cyclics: &[&str; 60] = match datetime.any_calendar_kind() {
                            Some(AnyCalendarKind::Dangi) => &[
                                "갑자", "을축", "병인", "정묘", "무진", "기사", "경오", "신미",
                                "임신", "계유", "갑술", "을해", "병자", "정축", "무인", "기묘",
                                "경진", "신사", "임오", "계미", "갑신", "을유", "병술", "정해",
                                "무자", "기축", "경인", "신묘", "임진", "계사", "갑오", "을미",
                                "병신", "정유", "무술", "기해", "경자", "신축", "임인", "계묘",
                                "갑진", "을사", "병오", "정미", "무신", "기유", "경술", "신해",
                                "임자", "계축", "갑인", "을묘", "병진", "정사", "무오", "기미",
                                "경신", "신유", "임술", "계해",
                            ],
                            // for now assume all other calendars use the stem-branch model
                            _ => &[
                                "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未",
                                "壬申", "癸酉", "甲戌", "乙亥", "丙子", "丁丑", "戊寅", "己卯",
                                "庚辰", "辛巳", "壬午", "癸未", "甲申", "乙酉", "丙戌", "丁亥",
                                "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳", "甲午", "乙未",
                                "丙申", "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯",
                                "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥",
                                "壬子", "癸丑", "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未",
                                "庚申", "辛酉", "壬戌", "癸亥",
                            ],
                        };
                        cyclics
                            .get((cyclic.get() - 1) as usize)
                            .ok_or(Error::DateTimeInput(
                                icu_calendar::CalendarError::Overflow {
                                    field: "cyclic",
                                    max: cyclics.len(),
                                },
                            ))
                    });
                match r {
                    Err(e) => {
                        w.with_part(Part::ERROR, |w| {
                            try_write_number(w, fdf, year.number.into(), l).map(|_| ())
                        })?;
                        Err(e)
                    }
                    Ok(cyclic_str) => Ok(w.write_str(cyclic_str)?),
                }
            }
        },
        (FieldSymbol::Year(Year::RelatedIso), l) => {
            match datetime
                .year()
                .ok_or(Error::MissingInputField(Some("year")))
                .and_then(|year| {
                    year.related_iso
                        .ok_or(Error::MissingInputField(Some("related_iso")))
                }) {
                Err(e) => {
                    write_value_missing(w, field)?;
                    Err(e)
                }
                Ok(iso) => try_write_number(w, fdf, iso.into(), l)?,
            }
        }
        (FieldSymbol::Month(_), l @ (FieldLength::One | FieldLength::TwoDigit)) => {
            match datetime.month() {
                None => {
                    write_value_missing(w, field)?;
                    Err(Error::MissingInputField(Some("month")))
                }
                Some(month) => try_write_number(w, fdf, month.ordinal.into(), l)?,
            }
        }
        (FieldSymbol::Month(month), l) => match datetime.month() {
            None => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("month")))
            }
            Some(formattable_month) => match date_symbols
                .ok_or(Error::MissingDateSymbols)
                .and_then(|ds| ds.get_symbol_for_month(month, l, formattable_month.code))
            {
                Err(e) => {
                    w.with_part(Part::ERROR, |w| w.write_str(&formattable_month.code.0))?;
                    Err(e)
                }
                Ok(MonthPlaceholderValue::PlainString(symbol)) => {
                    w.write_str(symbol)?;
                    Ok(())
                }
                Ok(MonthPlaceholderValue::StringNeedingLeapPrefix(symbol)) => {
                    // FIXME (#3766) this should be using actual data for leap months
                    let leap_str = match datetime.any_calendar_kind() {
                        Some(AnyCalendarKind::Chinese) => "閏",
                        Some(AnyCalendarKind::Dangi) => "윤",
                        _ => "(leap)",
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
            },
        },
        (FieldSymbol::Week(week), l) => match week {
            Week::WeekOfYear => match week_data
                .ok_or(Error::MissingWeekCalculator)
                .and_then(|w| datetime.week_of_year(w))
            {
                Err(e) => {
                    write_value_missing(w, field)?;
                    Err(e)
                }
                Ok((_, week_of_year)) => try_write_number(w, fdf, week_of_year.0.into(), l)?,
            },
            Week::WeekOfMonth => match week_data
                .ok_or(Error::MissingWeekCalculator)
                .and_then(|w| datetime.week_of_month(w))
            {
                Err(e) => {
                    write_value_missing(w, field)?;
                    Err(e)
                }
                Ok(week_of_month) => try_write_number(w, fdf, week_of_month.0.into(), l)?,
            },
        },
        (FieldSymbol::Weekday(weekday), l) => match datetime.iso_weekday() {
            None => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("iso_weekday")))
            }
            Some(wd) => match date_symbols
                .ok_or(Error::MissingDateSymbols)
                .and_then(|ds| ds.get_symbol_for_weekday(weekday, l, wd))
            {
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
                Ok(s) => Ok(w.write_str(s)?),
            },
        },
        (FieldSymbol::Day(fields::Day::DayOfMonth), l) => match datetime.day_of_month() {
            None => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("day_of_month")))
            }
            Some(d) => try_write_number(w, fdf, d.0.into(), l)?,
        },
        (FieldSymbol::Day(fields::Day::DayOfWeekInMonth), l) => {
            match datetime.day_of_month().map(DayOfWeekInMonth::from) {
                None => {
                    write_value_missing(w, field)?;
                    Err(Error::MissingInputField(Some("day_of_month")))
                }
                Some(d) => try_write_number(w, fdf, d.0.into(), l)?,
            }
        }
        (FieldSymbol::Hour(hour), l) => match datetime.hour() {
            None => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("hour")))
            }
            Some(h) => {
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
            }
        },
        (FieldSymbol::Minute, l) => match datetime.minute() {
            None => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("minute")))
            }
            Some(iso_minute) => try_write_number(w, fdf, usize::from(iso_minute).into(), l)?,
        },
        (FieldSymbol::Second(Second::Second), l) => match (datetime.second(), iter.peek()) {
            (
                None,
                Some(&PatternItem::Field(
                    next_field @ Field {
                        symbol: FieldSymbol::Second(Second::FractionalSecond),
                        ..
                    },
                )),
            ) => {
                iter.next(); // Advance over nanosecond symbol
                write_value_missing(w, field)?;
                // Write error value for nanos even if we have them
                write_value_missing(w, next_field)?;
                Err(Error::MissingInputField(Some("second")))
            }
            (None, _) => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("second")))
            }
            (
                Some(second),
                Some(&PatternItem::Field(
                    next_field @ Field {
                        symbol: FieldSymbol::Second(Second::FractionalSecond),
                        length,
                    },
                )),
            ) => {
                iter.next(); // Advance over nanosecond symbol
                let r = datetime
                    .nanosecond()
                    .ok_or(Error::MissingInputField(Some("nanosecond")))
                    .and_then(|ns| {
                        // We only support fixed field length for fractional seconds.
                        let FieldLength::Fixed(p) = length else {
                            return Err(Error::Pattern(
                                crate::pattern::PatternError::FieldLengthInvalid(
                                    FieldSymbol::Second(Second::FractionalSecond),
                                ),
                            ));
                        };
                        Ok((ns, p))
                    });
                match r {
                    Err(e) => {
                        let seconds_result =
                            try_write_number(w, fdf, usize::from(second).into(), l)?;
                        write_value_missing(w, next_field)?;
                        // Return the earlier error
                        seconds_result.and(Err(e))
                    }
                    Ok((ns, p)) => {
                        let mut s = FixedDecimal::from(usize::from(second));
                        let _infallible = s.concatenate_end(
                            FixedDecimal::from(usize::from(ns)).multiplied_pow10(-9),
                        );
                        debug_assert!(_infallible.is_ok());
                        s.pad_end(-(p as i16));
                        try_write_number(w, fdf, s, l)?
                    }
                }
            }
            (Some(second), _) => try_write_number(w, fdf, usize::from(second).into(), l)?,
        },
        (FieldSymbol::Second(Second::FractionalSecond), _) => {
            // Fractional second not following second
            write_value_missing(w, field)?;
            Err(Error::UnsupportedField(field.symbol))
        }
        (FieldSymbol::DayPeriod(period), l) => match datetime.hour() {
            None => {
                write_value_missing(w, field)?;
                Err(Error::MissingInputField(Some("hour")))
            }
            Some(hour) => {
                match time_symbols
                    .ok_or(Error::MissingTimeSymbols)
                    .and_then(|ts| {
                        ts.get_symbol_for_day_period(
                            period,
                            l,
                            hour,
                            pattern_metadata.time_granularity().is_top_of_hour(
                                datetime.minute().map(u8::from).unwrap_or(0),
                                datetime.second().map(u8::from).unwrap_or(0),
                                datetime.nanosecond().map(u32::from).unwrap_or(0),
                            ),
                        )
                    }) {
                    Err(e) => {
                        w.with_part(Part::ERROR, |w| {
                            w.write_str(if usize::from(hour) < 12 { "AM" } else { "PM" })
                        })?;
                        Err(e)
                    }
                    Ok(s) => Ok(w.write_str(s)?),
                }
            }
        },
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
        use icu::datetime::neo::NeoDateFormatter;
        use icu::datetime::options::length;
        use icu::datetime::DateTimeError;

        let locale: Locale = "en-u-ca-japanese".parse().unwrap();
        let dtf = NeoDateFormatter::try_new_with_length(&locale.into(), length::Date::Medium)
            .expect("DateTimeFormat construction succeeds");

        let date = Date::try_new_gregorian_date(1800, 9, 1).expect("Failed to construct Date.");
        let date = date
            .to_calendar(JapaneseExtended::new())
            .into_japanese_date()
            .to_any();

        writeable::assert_try_writeable_eq!(
            dtf.format(&date).unwrap(),
            "Sep 1, 12 kansei-1789",
            Err(DateTimeError::MissingDateSymbols)
        );
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
        try_write_pattern(
            pattern.as_borrowed(),
            &ExtractedDateTimeInput::extract_from(&datetime),
            Some(date_data.get()),
            Some(time_data.get()),
            None,
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
