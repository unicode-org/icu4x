// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields::{self, Field, FieldLength, FieldSymbol, Second, Week, Year};
use crate::input::{DateInput, ExtractedDateTimeInput, ExtractedTimeZoneInput, IsoTimeInput};
use crate::pattern::runtime::{PatternBorrowed, PatternMetadata};
use crate::pattern::{
    runtime::{Pattern, PatternPlurals},
    PatternItem,
};
use crate::provider;
use crate::provider::calendar::patterns::PatternPluralsFromPatternsV1Marker;
#[cfg(feature = "experimental")]
use crate::provider::date_time::GetSymbolForDayPeriodError;
use crate::provider::date_time::{
    DateSymbols, GetSymbolForEraError, GetSymbolForMonthError, GetSymbolForWeekdayError,
    MonthPlaceholderValue, TimeSymbols, ZoneSymbols,
};
use crate::time_zone::ResolvedNeoTimeZoneSkeleton;
use crate::time_zone::{
    Bcp47IdFormat, ExemplarCityFormat, FallbackTimeZoneFormatterUnit, FormatTimeZone,
    FormatTimeZoneError, GenericLocationFormat, GenericNonLocationLongFormat,
    GenericNonLocationShortFormat, Iso8601Format, LocalizedGmtFormat,
    SpecificNonLocationLongFormat, SpecificNonLocationShortFormat, TimeZoneDataPayloadsBorrowed,
    TimeZoneFormatterUnit,
};

use super::FormattingOptions;
use core::fmt::{self, Write};
use core::iter::Peekable;
use fixed_decimal::FixedDecimal;
use icu_calendar::types::{
    Era, {DayOfWeekInMonth, IsoWeekday, MonthCode},
};
use icu_calendar::week::WeekCalculator;
use icu_calendar::AnyCalendarKind;
use icu_decimal::FixedDecimalFormatter;
use icu_plurals::PluralRules;
use icu_provider::DataPayload;
use icu_timezone::{CustomTimeZone, GmtOffset};
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
/// use icu::locale::locale;
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
    pub(crate) fn select_pattern_lossy<'a>(
        &'a self,
    ) -> (&'l Pattern<'l>, Result<(), DateTimeWriteError>) {
        let mut r = Ok(());
        let pattern = match self.patterns.get().0 {
            PatternPlurals::SinglePattern(ref pattern) => pattern,
            PatternPlurals::MultipleVariants(ref plural_pattern) => {
                let week_number = match plural_pattern.pivot_field() {
                    Week::WeekOfMonth => self
                        .week_data
                        .ok_or(DateTimeWriteError::MissingWeekCalculator)
                        .and_then(|w| {
                            self.datetime
                                .week_of_month(w)
                                .map_err(DateTimeWriteError::MissingInputField)
                        })
                        .map(|w| w.0)
                        .unwrap_or_else(|e| {
                            r = r.and(Err(e));
                            0
                        }),
                    Week::WeekOfYear => self
                        .week_data
                        .ok_or(DateTimeWriteError::MissingWeekCalculator)
                        .and_then(|w| {
                            self.datetime
                                .week_of_year(w)
                                .map_err(DateTimeWriteError::MissingInputField)
                        })
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
                        r = r.and(Err(DateTimeWriteError::MissingOrdinalRules));
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
            Default::default(),
            &self.datetime,
            self.date_symbols,
            self.time_symbols,
            None::<()>.as_ref(),
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
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
{
    if let Some(fdf) = fixed_decimal_format {
        match length {
            FieldLength::One
            | FieldLength::NumericOverride(_)
            | FieldLength::TimeZoneFallbackOverride(_) => {}
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
        Ok(Err(DateTimeWriteError::MissingFixedDecimalFormatter))
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn try_write_pattern<'data, W, DS, TS, ZS>(
    pattern: PatternBorrowed<'data>,
    formatting_options: FormattingOptions,
    datetime: &ExtractedDateTimeInput,
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
        formatting_options,
        datetime,
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
    formatting_options: FormattingOptions,
    datetime: &ExtractedDateTimeInput,
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
    let mut iter = pattern_items.peekable();
    while let Some(item) = iter.next() {
        match item {
            PatternItem::Literal(ch) => w.write_char(ch)?,
            PatternItem::Field(Field {
                symbol: fields::FieldSymbol::TimeZone(time_zone_field),
                length,
            }) => {
                r = r.and(try_write_zone(
                    time_zone_field,
                    length,
                    datetime,
                    zone_symbols,
                    fixed_decimal_format,
                    w,
                )?)
            }
            PatternItem::Field(field) => {
                r = r.and(try_write_field(
                    field,
                    &mut iter,
                    pattern_metadata,
                    formatting_options,
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
    #[displaydoc("Cannot find symbol for time zone {0:?}")]
    MissingTimeZoneSymbol(CustomTimeZone),

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
    iter: &mut Peekable<impl Iterator<Item = PatternItem>>,
    pattern_metadata: PatternMetadata,
    formatting_options: FormattingOptions,
    datetime: &ExtractedDateTimeInput,
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
        (FieldSymbol::Era, l) => match datetime.year() {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("year"))
            }
            Some(year) => match date_symbols
                .ok_or(DateTimeWriteError::MissingDateSymbols)
                .and_then(|ds| {
                    ds.get_symbol_for_era(l, &year.era).map_err(|e| match e {
                        GetSymbolForEraError::Missing => {
                            DateTimeWriteError::MissingEraSymbol(year.era)
                        }
                        #[cfg(feature = "experimental")]
                        GetSymbolForEraError::MissingNames(f) => {
                            DateTimeWriteError::MissingNames(f)
                        }
                    })
                }) {
                Err(e) => {
                    w.with_part(Part::ERROR, |w| w.write_str(&year.era.0))?;
                    Err(e)
                }
                Ok(era) => Ok(w.write_str(era)?),
            },
        },
        (FieldSymbol::Year(Year::Calendar), l) => match datetime.year() {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("year"))
            }
            Some(year) => try_write_number(w, fdf, year.number.into(), l)?,
        },
        (FieldSymbol::Year(Year::WeekOf), l) => match week_data
            .ok_or(DateTimeWriteError::MissingWeekCalculator)
            .and_then(|w| {
                datetime
                    .week_of_year(w)
                    .map_err(DateTimeWriteError::MissingInputField)
            }) {
            Err(e) => {
                write_value_missing(w, field)?;
                Err(e)
            }
            Ok((year, _)) => try_write_number(w, fdf, year.number.into(), l)?,
        },
        (FieldSymbol::Year(Year::Cyclic), l) => match datetime.year() {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("year"))
            }
            Some(year) => {
                let r = year
                    .cyclic
                    .ok_or(DateTimeWriteError::MissingInputField("cyclic"))
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
                .ok_or(DateTimeWriteError::MissingInputField("year"))
                .and_then(|year| {
                    year.related_iso
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
            match datetime.month() {
                None => {
                    write_value_missing(w, field)?;
                    Err(DateTimeWriteError::MissingInputField("month"))
                }
                Some(month) => try_write_number(w, fdf, month.ordinal.into(), l)?,
            }
        }
        (FieldSymbol::Month(month), l) => match datetime.month() {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("month"))
            }
            Some(formattable_month) => match date_symbols
                .ok_or(DateTimeWriteError::MissingDateSymbols)
                .and_then(|ds| {
                    ds.get_symbol_for_month(month, l, formattable_month.code)
                        .map_err(|e| match e {
                            GetSymbolForMonthError::Missing => {
                                DateTimeWriteError::MissingMonthSymbol(formattable_month.code)
                            }
                            #[cfg(feature = "experimental")]
                            GetSymbolForMonthError::MissingNames(f) => {
                                DateTimeWriteError::MissingNames(f)
                            }
                        })
                }) {
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
                .ok_or(DateTimeWriteError::MissingWeekCalculator)
                .and_then(|w| {
                    datetime
                        .week_of_year(w)
                        .map_err(DateTimeWriteError::MissingInputField)
                }) {
                Err(e) => {
                    write_value_missing(w, field)?;
                    Err(e)
                }
                Ok((_, week_of_year)) => try_write_number(w, fdf, week_of_year.0.into(), l)?,
            },
            Week::WeekOfMonth => match week_data
                .ok_or(DateTimeWriteError::MissingWeekCalculator)
                .and_then(|w| {
                    datetime
                        .week_of_month(w)
                        .map_err(DateTimeWriteError::MissingInputField)
                }) {
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
                            #[cfg(feature = "experimental")]
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
        (FieldSymbol::Day(fields::Day::DayOfMonth), l) => match datetime.day_of_month() {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("day_of_month"))
            }
            Some(d) => try_write_number(w, fdf, d.0.into(), l)?,
        },
        (FieldSymbol::Day(fields::Day::DayOfWeekInMonth), l) => {
            match datetime.day_of_month().map(DayOfWeekInMonth::from) {
                None => {
                    write_value_missing(w, field)?;
                    Err(DateTimeWriteError::MissingInputField("day_of_month"))
                }
                Some(d) => try_write_number(w, fdf, d.0.into(), l)?,
            }
        }
        (FieldSymbol::Hour(hour), l) => match datetime.hour() {
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
        (FieldSymbol::Minute, l) => match datetime.minute() {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("minute"))
            }
            Some(iso_minute) => try_write_number(w, fdf, usize::from(iso_minute).into(), l)?,
        },
        (FieldSymbol::Second(Second::Second), l) => match datetime.second() {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("second"))
            }
            Some(second) => {
                match match (iter.peek(), formatting_options.fractional_second_digits) {
                    (
                        Some(&PatternItem::Field(
                            next_field @ Field {
                                symbol: FieldSymbol::Second(Second::FractionalSecond),
                                length: FieldLength::Fixed(p),
                            },
                        )),
                        _,
                    ) => {
                        // Fractional second digits via field symbol
                        iter.next(); // Advance over nanosecond symbol
                        Some((-(p as i16), Some(next_field), datetime.nanosecond()))
                    }
                    (_, Some(p)) => {
                        // Fractional second digits via semantic option
                        Some((-(p as i16), None, datetime.nanosecond()))
                    }
                    _ => None,
                } {
                    Some((_, maybe_next_field, None)) => {
                        // Request to format nanoseconds but we don't have nanoseconds
                        let seconds_result =
                            try_write_number(w, fdf, usize::from(second).into(), l)?;
                        if let Some(next_field) = maybe_next_field {
                            write_value_missing(w, next_field)?;
                        }
                        // Return the earlier error
                        seconds_result.and(Err(DateTimeWriteError::MissingInputField("nanosecond")))
                    }
                    Some((position, maybe_next_field, Some(ns))) => {
                        // Formatting with fractional seconds
                        let mut s = FixedDecimal::from(usize::from(second));
                        let _infallible = s.concatenate_end(
                            FixedDecimal::from(usize::from(ns)).multiplied_pow10(-9),
                        );
                        debug_assert!(_infallible.is_ok());
                        s.pad_end(position);
                        if maybe_next_field.is_none() {
                            // Truncate on semantic option but not "S" field
                            // TODO: Does this make sense?
                            s.trunc(position);
                        }
                        try_write_number(w, fdf, s, l)?
                    }
                    None => {
                        // Normal seconds formatting with no fractional second digits
                        try_write_number(w, fdf, usize::from(second).into(), l)?
                    }
                }
            }
        },
        (FieldSymbol::Second(Second::FractionalSecond), _) => {
            // Fractional second not following second or with invalid length
            write_value_missing(w, field)?;
            Err(DateTimeWriteError::UnsupportedField(field))
        }
        (FieldSymbol::DayPeriod(period), l) => match datetime.hour() {
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
                                datetime.minute().map(u8::from).unwrap_or(0),
                                datetime.second().map(u8::from).unwrap_or(0),
                                datetime.nanosecond().map(u32::from).unwrap_or(0),
                            ),
                        )
                        .map_err(|e| match e {
                            #[cfg(feature = "experimental")]
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

// #[allow(clippy::too_many_arguments)]
pub(crate) fn try_write_zone<'data, W, ZS>(
    field_symbol: fields::TimeZone,
    field_length: FieldLength,
    datetime: &ExtractedDateTimeInput,
    zone_symbols: Option<&ZS>,
    _fdf: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
    ZS: ZoneSymbols<'data>,
{
    fn write_time_zone_missing(
        gmt_offset: Option<GmtOffset>,
        w: &mut (impl writeable::PartsWrite + ?Sized),
    ) -> fmt::Result {
        match gmt_offset {
            Some(gmt_offset) => w.with_part(Part::ERROR, |w| {
                Iso8601Format::default_for_fallback().format_infallible(w, gmt_offset)
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
    Ok(match datetime.time_zone() {
        None => {
            write_time_zone_missing(None, w)?;
            Err(DateTimeWriteError::MissingInputField("time_zone"))
        }
        Some(custom_time_zone) => match zone_symbols {
            None => {
                write_time_zone_missing(custom_time_zone.gmt_offset, w)?;
                Err(DateTimeWriteError::MissingZoneSymbols)
            }
            Some(zs) => match ResolvedNeoTimeZoneSkeleton::from_field(field_symbol, field_length) {
                None => {
                    write_time_zone_missing(custom_time_zone.gmt_offset, w)?;
                    Err(DateTimeWriteError::UnsupportedField(field))
                }
                Some(time_zone) => {
                    let payloads = zs.get_payloads();
                    let zone_input = custom_time_zone.into();
                    let units = select_zone_units(time_zone);
                    match do_write_zone(units, &zone_input, payloads, w)? {
                        Ok(()) => Ok(()),
                        Err(()) => {
                            write_time_zone_missing(custom_time_zone.gmt_offset, w)?;
                            // Return an error since GMT data was missing
                            Err(DateTimeWriteError::MissingZoneSymbols)
                        }
                    }
                }
            },
        },
    })
}

/// Given a [`ResolvedNeoTimeZoneSkeleton`], select the formatter units
fn select_zone_units(time_zone: ResolvedNeoTimeZoneSkeleton) -> [Option<TimeZoneFormatterUnit>; 3] {
    // Select which formatters to try based on the field.
    let mut formatters = (
        None,
        None,
        // Friendly Localized GMT Format (requires "essentials" data)
        Some(TimeZoneFormatterUnit::WithFallback(
            FallbackTimeZoneFormatterUnit::LocalizedGmt(LocalizedGmtFormat {}),
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
        ResolvedNeoTimeZoneSkeleton::GmtShort => {
            // TODO: For now, use the long format. This should be GMT-8
        }
        // `OOOO`, `ZZZZ`
        ResolvedNeoTimeZoneSkeleton::GmtLong => {
            // no-op
        }
        ResolvedNeoTimeZoneSkeleton::Bcp47Id => {
            formatters.0 = Some(TimeZoneFormatterUnit::Bcp47Id(Bcp47IdFormat {}))
        }
        ResolvedNeoTimeZoneSkeleton::IsoBasic => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format::basic()),
            ))
        }
        ResolvedNeoTimeZoneSkeleton::IsoExtended => {
            formatters.2 = Some(TimeZoneFormatterUnit::WithFallback(
                FallbackTimeZoneFormatterUnit::Iso8601(Iso8601Format::extended()),
            ))
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
    zone_input: &ExtractedTimeZoneInput,
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
        match formatter.format(w, zone_input, payloads)? {
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
        use icu_calendar::DateTime;
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
            Default::default(),
            &ExtractedDateTimeInput::extract_from(&datetime),
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
