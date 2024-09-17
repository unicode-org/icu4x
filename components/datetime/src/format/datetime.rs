// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::{self, Field, FieldLength, FieldSymbol, Second, Week, Year};
use crate::input::ExtractedInput;
use crate::pattern::runtime::{PatternBorrowed, PatternMetadata};
use crate::pattern::PatternItem;
use crate::provider::date_time::GetSymbolForDayPeriodError;
use crate::provider::date_time::{
    DateSymbols, GetSymbolForEraError, GetSymbolForMonthError, GetSymbolForWeekdayError,
    MonthPlaceholderValue, TimeSymbols, ZoneSymbols,
};
use crate::time_zone::{
    Bcp47IdFormat, ExemplarCityFormat, FallbackTimeZoneFormatterUnit, FormatTimeZone,
    FormatTimeZoneError, GenericLocationFormat, GenericNonLocationLongFormat,
    GenericNonLocationShortFormat, Iso8601Format, LocalizedOffsetFormat,
    SpecificNonLocationLongFormat, SpecificNonLocationShortFormat, TimeZoneDataPayloadsBorrowed,
    TimeZoneFormatterUnit,
};
use crate::time_zone::{IsoFormat, IsoMinutes, IsoSeconds, ResolvedNeoTimeZoneSkeleton};

use core::fmt::{self, Write};
use fixed_decimal::FixedDecimal;
use icu_calendar::types::{
    Era, {DayOfWeekInMonth, IsoWeekday, MonthCode},
};
use icu_calendar::week::WeekCalculator;
use icu_calendar::AnyCalendarKind;
use icu_decimal::FixedDecimalFormatter;
use icu_timezone::UtcOffset;
use writeable::{Part, Writeable};

/// Apply length to input number and write to result using fixed_decimal_format.
fn try_write_number<W>(
    result: &mut W,
    fixed_decimal_format: Option<&FixedDecimalFormatter>,
    mut num: FixedDecimal,
    length: FieldLength,
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
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
        }

        fdf.format(&num).write_to(result)?;
        Ok(Ok(()))
    } else {
        result.with_part(writeable::Part::ERROR, |r| num.write_to(r))?;
        Ok(Err(DateTimeWriteError::MissingFixedDecimalFormatter))
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn try_write_pattern<'data, W, DS, TS, ZS>(
    pattern: PatternBorrowed<'data>,
    input: &ExtractedInput,
    date_symbols: Option<&DS>,
    time_symbols: Option<&TS>,
    zone_symbols: Option<&ZS>,
    week_data: Option<&'data WeekCalculator>,
    fixed_decimal_format: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
    DS: DateSymbols<'data>,
    TS: TimeSymbols,
    ZS: ZoneSymbols<'data>,
{
    try_write_pattern_items(
        pattern.metadata,
        pattern.items.iter(),
        input,
        date_symbols,
        time_symbols,
        zone_symbols,
        week_data,
        fixed_decimal_format,
        w,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn try_write_pattern_items<'data, W, DS, TS, ZS>(
    pattern_metadata: PatternMetadata,
    pattern_items: impl Iterator<Item = PatternItem>,
    input: &ExtractedInput,
    date_symbols: Option<&DS>,
    time_symbols: Option<&TS>,
    zone_symbols: Option<&ZS>,
    week_data: Option<&'data WeekCalculator>,
    fixed_decimal_format: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
    DS: DateSymbols<'data>,
    TS: TimeSymbols,
    ZS: ZoneSymbols<'data>,
{
    let mut r = Ok(());
    for item in pattern_items {
        match item {
            PatternItem::Literal(ch) => w.write_char(ch)?,
            PatternItem::Field(Field {
                symbol: fields::FieldSymbol::TimeZone(time_zone_field),
                length,
            }) => {
                r = r.and(try_write_zone(
                    time_zone_field,
                    length,
                    input,
                    zone_symbols,
                    fixed_decimal_format,
                    w,
                )?)
            }
            PatternItem::Field(field) => {
                r = r.and(try_write_field(
                    field,
                    pattern_metadata,
                    input,
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

#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone, displaydoc::Display)]
/// Error for `TryWriteable` implementations
pub enum DateTimeWriteError {
    // Data not loaded
    /// Missing FixedDecimalFormatter
    #[displaydoc("FixedDecimalFormatter not loaded")]
    MissingFixedDecimalFormatter,
    // TODO: Remove Missing*Symbols and use exclusively MissingNames
    /// Missing DateSymbols
    #[displaydoc("DateSymbols not loaded")]
    MissingDateSymbols,
    /// Missing ZoneSymbols
    #[displaydoc("ZoneSymbols not loaded")]
    MissingZoneSymbols,
    /// Missing TimeSymbols
    #[displaydoc("TimeSymbols not loaded")]
    MissingTimeSymbols,
    /// Missing OrdinalRules
    #[displaydoc("OrdinalRules not loaded")]
    MissingOrdinalRules,
    /// Missing WeekCalculator
    #[displaydoc("WeekCalculator not loaded")]
    MissingWeekCalculator,
    /// TODO
    #[displaydoc("Names for {0:?} not loaded")]
    MissingNames(Field),

    // Something not found in data
    // TODO: Are these actionable? Can clients even invent their own months and days?
    /// Missing month symbol
    #[displaydoc("Cannot find symbol for month {0:?}")]
    MissingMonthSymbol(MonthCode),
    /// Missing era symbol
    #[displaydoc("Cannot find symbol for era {0:?}")]
    MissingEraSymbol(Era),
    /// Missing weekday symbol
    #[displaydoc("Cannot find symbol for weekday {0:?}")]
    MissingWeekdaySymbol(IsoWeekday),
    /// Missing time zone symbol
    #[displaydoc("Not enough time zone information to format anything.")]
    UnsupportedTimeZone,

    // Invalid input
    /// Incomplete input
    #[displaydoc("Incomplete input, missing value for {0:?}")]
    MissingInputField(&'static str),
    /// Cyclic year overflow
    #[displaydoc("Cyclic year overflow, found {value}, maximum {max}")]
    CyclicYearOverflow {
        /// Value
        value: usize,
        /// Max
        max: usize,
    },
    /// Unsupported field
    #[displaydoc("Unsupported field {0:?}")]
    UnsupportedField(Field),
}

// This function assumes that the correct decision has been
// made regarding availability of symbols in the caller.
//
// When modifying the list of fields using symbols,
// update the matching query in `analyze_pattern` function.
#[allow(clippy::too_many_arguments)]
pub(crate) fn try_write_field<'data, W, DS, TS>(
    field: fields::Field,
    pattern_metadata: PatternMetadata,
    input: &ExtractedInput,
    date_symbols: Option<&DS>,
    time_symbols: Option<&TS>,
    week_data: Option<&WeekCalculator>,
    fdf: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
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
        (FieldSymbol::Era, l) => match input.year {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("year"))
            }
            Some(year) => {
                let era_symbol = date_symbols
                    .ok_or(DateTimeWriteError::MissingDateSymbols)
                    .and_then(|ds| {
                        let Some(era) = year.formatting_era() else {
                            return Err(DateTimeWriteError::MissingInputField("era"));
                        };
                        ds.get_symbol_for_era(l, era).map_err(|e| match e {
                            GetSymbolForEraError::Missing => {
                                DateTimeWriteError::MissingEraSymbol(era)
                            }
                            GetSymbolForEraError::MissingNames(f) => {
                                DateTimeWriteError::MissingNames(f)
                            }
                        })
                    });
                match era_symbol {
                    Err(e) => {
                        w.with_part(Part::ERROR, |w| {
                            if let Some(era) = year.formatting_era() {
                                w.write_str(&era.0)
                            } else {
                                w.write_str("missing era")
                            }
                        })?;
                        Err(e)
                    }
                    Ok(era) => Ok(w.write_str(era)?),
                }
            }
        },
        (FieldSymbol::Year(Year::Calendar), l) => match input.year {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("year"))
            }
            Some(year) => try_write_number(w, fdf, year.era_year_or_extended().into(), l)?,
        },
        (FieldSymbol::Year(Year::WeekOf), l) => match week_data
            .ok_or(DateTimeWriteError::MissingWeekCalculator)
            .and_then(|w| input.week_of_year(w))
        {
            Err(e) => {
                write_value_missing(w, field)?;
                Err(e)
            }
            Ok((year, _)) => try_write_number(w, fdf, year.era_year_or_extended().into(), l)?,
        },
        (FieldSymbol::Year(Year::Cyclic), l) => match input.year {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("year"))
            }
            Some(year) => {
                let r = year
                    .cyclic()
                    .ok_or(DateTimeWriteError::MissingInputField("cyclic"))
                    .and_then(|cyclic| {
                        // TODO(#3761): This is a hack, we should use actual data for cyclic years
                        let cyclics: &[&str; 60] = match input.any_calendar_kind {
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
                        let value: usize = cyclic.get() as usize;
                        cyclics
                            .get(value - 1)
                            .ok_or(DateTimeWriteError::CyclicYearOverflow {
                                value,
                                max: cyclics.len() + 1,
                            })
                    });
                match r {
                    Err(e) => {
                        w.with_part(Part::ERROR, |w| {
                            try_write_number(w, fdf, year.era_year_or_extended().into(), l)
                                .map(|_| ())
                        })?;
                        Err(e)
                    }
                    Ok(cyclic_str) => Ok(w.write_str(cyclic_str)?),
                }
            }
        },
        (FieldSymbol::Year(Year::RelatedIso), l) => {
            match input
                .year
                .ok_or(DateTimeWriteError::MissingInputField("year"))
                .and_then(|year| {
                    year.related_iso()
                        .ok_or(DateTimeWriteError::MissingInputField("related_iso"))
                }) {
                Err(e) => {
                    write_value_missing(w, field)?;
                    Err(e)
                }
                Ok(iso) => try_write_number(w, fdf, iso.into(), l)?,
            }
        }
        (FieldSymbol::Month(_), l @ (FieldLength::One | FieldLength::TwoDigit)) => {
            match input.month {
                None => {
                    write_value_missing(w, field)?;
                    Err(DateTimeWriteError::MissingInputField("month"))
                }
                Some(month) => try_write_number(w, fdf, month.ordinal.into(), l)?,
            }
        }
        (FieldSymbol::Month(month), l) => match input.month {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("month"))
            }
            Some(month_info) => match date_symbols
                .ok_or(DateTimeWriteError::MissingDateSymbols)
                .and_then(|ds| {
                    ds.get_symbol_for_month(month, l, month_info.formatting_code)
                        .map_err(|e| match e {
                            GetSymbolForMonthError::Missing => {
                                DateTimeWriteError::MissingMonthSymbol(month_info.formatting_code)
                            }
                            GetSymbolForMonthError::MissingNames(f) => {
                                DateTimeWriteError::MissingNames(f)
                            }
                        })
                }) {
                Err(e) => {
                    w.with_part(Part::ERROR, |w| w.write_str(&month_info.formatting_code.0))?;
                    Err(e)
                }
                Ok(MonthPlaceholderValue::PlainString(symbol)) => {
                    w.write_str(symbol)?;
                    Ok(())
                }
                Ok(MonthPlaceholderValue::StringNeedingLeapPrefix(symbol)) => {
                    // FIXME (#3766) this should be using actual data for leap months
                    let leap_str = match input.any_calendar_kind {
                        Some(AnyCalendarKind::Chinese) => "閏",
                        Some(AnyCalendarKind::Dangi) => "윤",
                        _ => "(leap)",
                    };
                    w.write_str(leap_str)?;
                    w.write_str(symbol)?;
                    Ok(())
                }
                Ok(MonthPlaceholderValue::Numeric) => {
                    try_write_number(w, fdf, month_info.ordinal.into(), l)?
                }
                Ok(MonthPlaceholderValue::NumericPattern(substitution_pattern)) => {
                    w.write_str(substitution_pattern.get_prefix())?;
                    let r = try_write_number(w, fdf, month_info.ordinal.into(), l)?;
                    w.write_str(substitution_pattern.get_suffix())?;
                    r
                }
            },
        },
        (FieldSymbol::Week(week), l) => match week {
            Week::WeekOfYear => match week_data
                .ok_or(DateTimeWriteError::MissingWeekCalculator)
                .and_then(|w| input.week_of_year(w))
            {
                Err(e) => {
                    write_value_missing(w, field)?;
                    Err(e)
                }
                Ok((_, week_of_year)) => try_write_number(w, fdf, week_of_year.0.into(), l)?,
            },
            Week::WeekOfMonth => match week_data
                .ok_or(DateTimeWriteError::MissingWeekCalculator)
                .and_then(|w| input.week_of_month(w))
            {
                Err(e) => {
                    write_value_missing(w, field)?;
                    Err(e)
                }
                Ok(week_of_month) => try_write_number(w, fdf, week_of_month.0.into(), l)?,
            },
        },
        (FieldSymbol::Weekday(weekday), l) => match input.iso_weekday {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("iso_weekday"))
            }
            Some(wd) => match date_symbols
                .ok_or(DateTimeWriteError::MissingDateSymbols)
                .and_then(|ds| {
                    ds.get_symbol_for_weekday(weekday, l, wd)
                        .map_err(|e| match e {
                            GetSymbolForWeekdayError::Missing => {
                                DateTimeWriteError::MissingWeekdaySymbol(wd)
                            }
                            GetSymbolForWeekdayError::MissingNames(f) => {
                                DateTimeWriteError::MissingNames(f)
                            }
                        })
                }) {
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
        (FieldSymbol::Day(fields::Day::DayOfMonth), l) => match input.day_of_month {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("day_of_month"))
            }
            Some(d) => try_write_number(w, fdf, d.0.into(), l)?,
        },
        (FieldSymbol::Day(fields::Day::DayOfWeekInMonth), l) => {
            match input.day_of_month.map(DayOfWeekInMonth::from) {
                None => {
                    write_value_missing(w, field)?;
                    Err(DateTimeWriteError::MissingInputField("day_of_month"))
                }
                Some(d) => try_write_number(w, fdf, d.0.into(), l)?,
            }
        }
        (FieldSymbol::Hour(hour), l) => match input.hour {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("hour"))
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
        (FieldSymbol::Minute, l) => match input.minute {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("minute"))
            }
            Some(iso_minute) => try_write_number(w, fdf, usize::from(iso_minute).into(), l)?,
        },
        (FieldSymbol::Second(Second::Second), l) => match input.second {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("second"))
            }
            Some(second) => {
                // Normal seconds formatting with no fractional second digits
                try_write_number(w, fdf, usize::from(second).into(), l)?
            }
        },
        (FieldSymbol::DecimalSecond(decimal_second), l) => {
            match (input.second, input.nanosecond) {
                (None, _) | (_, None) => {
                    write_value_missing(w, field)?;
                    Err(DateTimeWriteError::MissingInputField("second"))
                }
                (Some(second), Some(ns)) => {
                    // Formatting with fractional seconds
                    let mut s = FixedDecimal::from(usize::from(second));
                    let _infallible =
                        s.concatenate_end(FixedDecimal::from(usize::from(ns)).multiplied_pow10(-9));
                    debug_assert!(_infallible.is_ok());
                    let position = -(decimal_second as i16);
                    s.trunc(position);
                    s.pad_end(position);
                    try_write_number(w, fdf, s, l)?
                }
            }
        }
        (FieldSymbol::DayPeriod(period), l) => match input.hour {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("hour"))
            }
            Some(hour) => {
                match time_symbols
                    .ok_or(DateTimeWriteError::MissingTimeSymbols)
                    .and_then(|ts| {
                        ts.get_symbol_for_day_period(
                            period,
                            l,
                            hour,
                            pattern_metadata.time_granularity().is_top_of_hour(
                                input.minute.map(u8::from).unwrap_or(0),
                                input.second.map(u8::from).unwrap_or(0),
                                input.nanosecond.map(u32::from).unwrap_or(0),
                            ),
                        )
                        .map_err(|e| match e {
                            GetSymbolForDayPeriodError::MissingNames(f) => {
                                DateTimeWriteError::MissingNames(f)
                            }
                        })
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
        (FieldSymbol::TimeZone(_), _) => {
            debug_assert!(false, "unreachable: time zone formatted in its own fn");
            Err(DateTimeWriteError::UnsupportedField(field))
        }
        (FieldSymbol::Day(_) | FieldSymbol::Second(Second::Millisecond), _) => {
            w.with_part(Part::ERROR, |w| {
                w.write_str("{unsupported:")?;
                w.write_char(char::from(field.symbol))?;
                w.write_str("}")
            })?;
            Err(DateTimeWriteError::UnsupportedField(field))
        }
    })
}

impl ExtractedInput {
    fn week_of_month(
        &self,
        calculator: &WeekCalculator,
    ) -> Result<icu_calendar::types::WeekOfMonth, DateTimeWriteError> {
        let day_of_month = self
            .day_of_month
            .ok_or(DateTimeWriteError::MissingInputField("day_of_month"))?;
        let iso_weekday = self
            .iso_weekday
            .ok_or(DateTimeWriteError::MissingInputField("iso_weekday"))?;
        Ok(calculator.week_of_month(day_of_month, iso_weekday))
    }

    fn week_of_year(
        &self,
        calculator: &WeekCalculator,
    ) -> Result<
        (
            icu_calendar::types::YearInfo,
            icu_calendar::types::WeekOfYear,
        ),
        DateTimeWriteError,
    > {
        let day_of_year_info = self
            .day_of_year_info
            .ok_or(DateTimeWriteError::MissingInputField("day_of_year_info"))?;
        let iso_weekday = self
            .iso_weekday
            .ok_or(DateTimeWriteError::MissingInputField("iso_weekday"))?;
        let week_of = calculator.week_of_year(day_of_year_info, iso_weekday);
        let year = match week_of.unit {
            icu_calendar::week::RelativeUnit::Previous => day_of_year_info.prev_year,
            icu_calendar::week::RelativeUnit::Current => self
                .year
                .ok_or(DateTimeWriteError::MissingInputField("year"))?,
            icu_calendar::week::RelativeUnit::Next => day_of_year_info.next_year,
        };
        Ok((year, icu_calendar::types::WeekOfYear(week_of.week as u32)))
    }
}

// #[allow(clippy::too_many_arguments)]
pub(crate) fn try_write_zone<'data, W, ZS>(
    field_symbol: fields::TimeZone,
    field_length: FieldLength,
    input: &ExtractedInput,
    zone_symbols: Option<&ZS>,
    _fdf: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
    ZS: ZoneSymbols<'data>,
{
    fn write_time_zone_missing(
        offset: Option<UtcOffset>,
        w: &mut (impl writeable::PartsWrite + ?Sized),
    ) -> fmt::Result {
        match offset {
            Some(offset) => w.with_part(Part::ERROR, |w| {
                Iso8601Format::default_for_fallback().format_infallible(w, offset)
            }),
            None => w.with_part(Part::ERROR, |w| "{GMT+?}".write_to(w)),
        }
    }

    // for errors only:
    let field = Field {
        symbol: FieldSymbol::TimeZone(field_symbol),
        length: field_length,
    };

    // TODO: Implement proper formatting logic here
    Ok(match zone_symbols {
        None => {
            write_time_zone_missing(input.offset, w)?;
            Err(DateTimeWriteError::MissingZoneSymbols)
        }
        Some(zs) => match ResolvedNeoTimeZoneSkeleton::from_field(field_symbol, field_length) {
            None => {
                write_time_zone_missing(input.offset, w)?;
                Err(DateTimeWriteError::UnsupportedField(field))
            }
            Some(time_zone) => {
                let payloads = zs.get_payloads();
                let units = select_zone_units(time_zone);
                match do_write_zone(units, input, payloads, w)? {
                    Ok(()) => Ok(()),
                    Err(()) => {
                        write_time_zone_missing(input.offset, w)?;
                        // Return an error since offset data was missing
                        Err(DateTimeWriteError::MissingZoneSymbols)
                    }
                }
            }
        },
    })
}

/// Given a [`ResolvedNeoTimeZoneSkeleton`], select the formatter units
fn select_zone_units(time_zone: ResolvedNeoTimeZoneSkeleton) -> [Option<TimeZoneFormatterUnit>; 3] {
    // Select which formatters to try based on the field.
    let mut formatters = (
        None,
        None,
        // Friendly Localized offset Format (requires "essentials" data)
        Some(TimeZoneFormatterUnit::WithFallback(
            FallbackTimeZoneFormatterUnit::LocalizedOffset(LocalizedOffsetFormat {}),
        )),
    );
    match time_zone {
        // `z..zzz`
        ResolvedNeoTimeZoneSkeleton::SpecificShort => {
            formatters.0 = Some(TimeZoneFormatterUnit::SpecificNonLocationShort(
                SpecificNonLocationShortFormat {},
            ));
        }
        // `zzzz`
        ResolvedNeoTimeZoneSkeleton::SpecificLong => {
            formatters.0 = Some(TimeZoneFormatterUnit::SpecificNonLocationLong(
                SpecificNonLocationLongFormat {},
            ));
        }
        // 'v'
        ResolvedNeoTimeZoneSkeleton::GenericShort => {
            formatters.0 = Some(TimeZoneFormatterUnit::GenericNonLocationShort(
                GenericNonLocationShortFormat {},
            ));
            formatters.1 = Some(TimeZoneFormatterUnit::GenericLocation(
                GenericLocationFormat {},
            ));
        }
        // 'vvvv'
        ResolvedNeoTimeZoneSkeleton::GenericLong => {
            formatters.0 = Some(TimeZoneFormatterUnit::GenericNonLocationLong(
                GenericNonLocationLongFormat {},
            ));
            formatters.1 = Some(TimeZoneFormatterUnit::GenericLocation(
                GenericLocationFormat {},
            ));
        }
        // 'VVV'
        ResolvedNeoTimeZoneSkeleton::City => {
            formatters.0 = Some(TimeZoneFormatterUnit::ExemplarCity(ExemplarCityFormat {}));
        }
        // 'VVVV'
        ResolvedNeoTimeZoneSkeleton::Location => {
            formatters.0 = Some(TimeZoneFormatterUnit::GenericLocation(
                GenericLocationFormat {},
            ));
        }
        // `O`
        ResolvedNeoTimeZoneSkeleton::OffsetShort => {
            // TODO: For now, use the long format. This should be GMT-8
        }
        // `OOOO`, `ZZZZ`
        ResolvedNeoTimeZoneSkeleton::OffsetLong => {
            // no-op
        }
        // 'V'
        ResolvedNeoTimeZoneSkeleton::Bcp47Id => {
            formatters.0 = Some(TimeZoneFormatterUnit::Bcp47Id(Bcp47IdFormat {}));
        }
        // 'X'
        ResolvedNeoTimeZoneSkeleton::IsoX => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcBasic,
                    minutes: IsoMinutes::Optional,
                    seconds: IsoSeconds::Never,
                }),
            ));
        }
        // 'XX'
        ResolvedNeoTimeZoneSkeleton::IsoXX => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcBasic,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Never,
                }),
            ));
        }
        // 'XXX'
        ResolvedNeoTimeZoneSkeleton::IsoXXX => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcExtended,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Never,
                }),
            ));
        }
        // 'XXXX'
        ResolvedNeoTimeZoneSkeleton::IsoXXXX => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcBasic,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Optional,
                }),
            ));
        }
        // 'XXXXX', 'ZZZZZ'
        ResolvedNeoTimeZoneSkeleton::IsoXXXXX => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcExtended,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Optional,
                }),
            ));
        }
        // 'x'
        ResolvedNeoTimeZoneSkeleton::Isox => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Basic,
                    minutes: IsoMinutes::Optional,
                    seconds: IsoSeconds::Never,
                }),
            ));
        }
        // 'xx'
        ResolvedNeoTimeZoneSkeleton::Isoxx => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Basic,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Never,
                }),
            ));
        }
        // 'xxx'
        ResolvedNeoTimeZoneSkeleton::Isoxxx => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Extended,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Never,
                }),
            ));
        }
        // 'xxxx', 'Z', 'ZZ', 'ZZZ'
        ResolvedNeoTimeZoneSkeleton::Isoxxxx => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Basic,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Optional,
                }),
            ));
        }
        // 'xxxxx', 'ZZZZZ'
        ResolvedNeoTimeZoneSkeleton::Isoxxxxx => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Extended,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Optional,
                }),
            ));
        }
    };
    // TODO:
    // `VV` "America/Los_Angeles"
    // Generic Partial Location: "Pacific Time (Los Angeles)"
    // All `x` and `X` formats
    [formatters.0, formatters.1, formatters.2]
}

/// Perform the formatting given all of the resolved parameters
fn do_write_zone<W>(
    units: [Option<TimeZoneFormatterUnit>; 3],
    input: &ExtractedInput,
    payloads: TimeZoneDataPayloadsBorrowed,
    w: &mut W,
) -> Result<Result<(), ()>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
{
    let [mut f0, mut f1, mut f2] = units;
    Ok(loop {
        let Some(formatter) = f0.take().or_else(|| f1.take()).or_else(|| f2.take()) else {
            break Err(());
        };
        match formatter.format(w, input, payloads)? {
            Ok(()) => break Ok(()),
            Err(FormatTimeZoneError::MissingInputField(_)) => {
                // The time zone input doesn't have the fields for this formatter.
                // TODO: What behavior makes the most sense here?
                // We can keep trying other formatters.
                continue;
            }
            Err(FormatTimeZoneError::NameNotFound) => {
                // Expected common case: data is loaded, but this time zone's
                // name was not found in the data.
                continue;
            }
            Err(FormatTimeZoneError::MissingZoneSymbols) => {
                // We don't have the necessary data for this formatter.
                // TODO: What behavior makes the most sense here?
                // We can keep trying other formatters.
                continue;
            }
        }
    })
}

#[cfg(test)]
#[allow(unused_imports)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use crate::{neo_marker::NeoAutoDateMarker, neo_skeleton::NeoSkeletonLength, pattern::runtime};
    use icu_decimal::options::{FixedDecimalFormatterOptions, GroupingStrategy};
    use tinystr::tinystr;

    #[test]
    fn test_mixed_calendar_eras() {
        use crate::neo::NeoFormatter;
        use crate::options::length;
        use icu_calendar::japanese::JapaneseExtended;
        use icu_calendar::Date;

        let locale = icu::locale::locale!("en-u-ca-japanese");
        let dtf = NeoFormatter::<NeoAutoDateMarker>::try_new(
            &locale.into(),
            NeoSkeletonLength::Medium.into(),
        )
        .expect("DateTimeFormat construction succeeds");

        let date = Date::try_new_gregorian_date(1800, 9, 1).expect("Failed to construct Date.");
        let date = date
            .to_calendar(JapaneseExtended::new())
            .into_japanese_date()
            .to_any();

        writeable::assert_try_writeable_eq!(
            dtf.strict_format(&date).unwrap(),
            "Sep 1, 12 kansei-1789",
            Err(DateTimeWriteError::MissingEraSymbol(Era(tinystr!(
                16,
                "kansei-1789"
            ))))
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_basic() {
        use crate::provider::calendar::{GregorianDateSymbolsV1Marker, TimeSymbolsV1Marker};
        use icu_calendar::{Calendar, DateTime};
        use icu_provider::prelude::*;

        let locale = "en".parse().unwrap();
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_locale(&locale),
            ..Default::default()
        };
        let date_data =
            DataProvider::<GregorianDateSymbolsV1Marker>::load(&crate::provider::Baked, req)
                .unwrap();
        let time_data =
            DataProvider::<TimeSymbolsV1Marker>::load(&crate::provider::Baked, req).unwrap();
        let pattern: runtime::Pattern = "MMM".parse().unwrap();
        let datetime = DateTime::try_new_gregorian_datetime(2020, 8, 1, 12, 34, 28).unwrap();
        let fixed_decimal_format =
            FixedDecimalFormatter::try_new(&locale, Default::default()).unwrap();

        let mut sink = String::new();
        try_write_pattern(
            pattern.as_borrowed(),
            &ExtractedInput {
                year: Some(datetime.date.year()),
                month: Some(datetime.date.month()),
                day_of_month: Some(datetime.date.day_of_month()),
                iso_weekday: Some(datetime.date.day_of_week()),
                day_of_year_info: Some(datetime.date.day_of_year_info()),
                any_calendar_kind: datetime.date.calendar().any_calendar_kind(),
                hour: Some(datetime.time.hour),
                minute: Some(datetime.time.minute),
                second: Some(datetime.time.second),
                nanosecond: Some(datetime.time.nanosecond),
                ..Default::default()
            },
            Some(date_data.payload.get()),
            Some(time_data.payload.get()),
            None::<()>.as_ref(),
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
            &icu_locale_core::locale!("en").into(),
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
