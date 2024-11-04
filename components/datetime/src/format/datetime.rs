// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::neo::RawDateTimeNamesBorrowed;
use super::GetNameForDayPeriodError;
use super::{
    GetNameForMonthError, GetNameForWeekdayError, GetSymbolForEraError, MonthPlaceholderValue,
};
use crate::fields::{self, Day, Field, FieldLength, FieldSymbol, Second, Week, Year};
use crate::input::ExtractedInput;
use crate::provider::pattern::runtime::PatternMetadata;
use crate::provider::pattern::PatternItem;
use crate::time_zone::{
    FormatTimeZone, FormatTimeZoneError, Iso8601Format, IsoFormat, IsoMinutes, IsoSeconds,
    ResolvedNeoTimeZoneSkeleton,
};
#[cfg(doc)]
use crate::TypedDateTimeNames;
#[cfg(doc)]
use icu_calendar::types::YearInfo;

use core::fmt::{self, Write};
use fixed_decimal::FixedDecimal;
use icu_calendar::types::{
    FormattingEra, {DayOfWeekInMonth, IsoWeekday, MonthCode},
};
use icu_calendar::AnyCalendarKind;
use icu_decimal::FixedDecimalFormatter;
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
        Ok(Err(DateTimeWriteError::FixedDecimalFormatterNotLoaded))
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn try_write_pattern_items<W>(
    pattern_metadata: PatternMetadata,
    pattern_items: impl Iterator<Item = PatternItem>,
    input: &ExtractedInput,
    datetime_names: &RawDateTimeNamesBorrowed,
    fixed_decimal_format: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
{
    let mut r = Ok(());
    for item in pattern_items {
        match item {
            PatternItem::Literal(ch) => w.write_char(ch)?,
            PatternItem::Field(field) => {
                r = r.and(try_write_field(
                    field,
                    pattern_metadata,
                    input,
                    datetime_names,
                    fixed_decimal_format,
                    w,
                )?);
            }
        }
    }
    Ok(r)
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone, displaydoc::Display)]
/// Error for `TryWriteable` implementations
pub enum DateTimeWriteError {
    /// An input field (such as "hour" or "month" is missing).
    ///
    /// This is guaranteed not to happen for `icu::calendar` inputs, but may happen for custom inputs.
    ///
    /// The output will contain the string `{X}` instead, where `X` is the symbol for which the input is missing.
    #[displaydoc("Incomplete input, missing value for {0:?}")]
    MissingInputField(&'static str),
    /// The [`MonthCode`] of the input is not valid for this calendar.
    ///
    /// This is guaranteed not to happen for `icu::calendar` inputs, but may happen for custom inputs.
    ///
    /// The output will contain the raw [`MonthCode`] as a fallback value.
    #[displaydoc("Invalid month {0:?}")]
    InvalidMonthCode(MonthCode),
    /// The [`FormattingEra`] of the input is not valid for this calendar.
    ///
    /// This is guaranteed not to happen for `icu::calendar` inputs, but may happen for custom inputs.
    ///
    /// The output will contain [`FormattingEra::fallback_name`] as the fallback.
    #[displaydoc("Invalid era {0:?}")]
    InvalidEra(FormattingEra),
    /// The [`YearInfo::cyclic`] of the input is not valid for this calendar.
    ///
    /// This is guaranteed not to happen for `icu::calendar` inputs, but may happen for custom inputs.
    ///
    /// The output will contain [`YearInfo::extended_year`] as a fallback value.
    #[displaydoc("Invalid cyclic year {value} (maximum {max})")]
    InvalidCyclicYear {
        /// Value
        value: usize,
        /// Max
        max: usize,
    },

    /// The [`FixedDecimalFormatter`] has not been loaded.
    ///
    /// This *only* happens if the formatter has been created using
    /// [`TypedDateTimeNames::with_pattern`], the pattern requires decimal
    /// formatting, and the decimal formatter was not loaded.
    ///
    /// The output will contain fallback values using Latin numerals.
    #[displaydoc("FixedDecimalFormatter not loaded")]
    FixedDecimalFormatterNotLoaded,
    /// The localized names for a field have not been loaded.
    ///
    /// This *only* happens if the formatter has been created using
    /// [`TypedDateTimeNames::with_pattern`], and the pattern requires names
    /// that were not loaded.
    ///
    /// The output will contain fallback values using field identifiers (such as `tue` for `IsoWeekday::Tuesday`,
    /// `M02` for month 2, etc.).
    #[displaydoc("Names for {0:?} not loaded")]
    NamesNotLoaded(Field),
    /// Unsupported field
    ///
    /// This *only* happens if the formatter has been created using
    /// [`TypedDateTimeNames::with_pattern`], and the pattern contains unsupported fields.
    ///
    /// The output will contain the string `{unsupported:X}`, where `X` is the symbol of the unsupported field.
    #[displaydoc("Unsupported field {0:?}")]
    UnsupportedField(Field),
}

// This function assumes that the correct decision has been
// made regarding availability of symbols in the caller.
//
// When modifying the list of fields using symbols,
// update the matching query in `analyze_pattern` function.
fn try_write_field<W>(
    field: fields::Field,
    pattern_metadata: PatternMetadata,
    input: &ExtractedInput,
    datetime_names: &RawDateTimeNamesBorrowed,
    fdf: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
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
            Some(year) => match year.formatting_era() {
                None => {
                    write_value_missing(w, field)?;
                    Err(DateTimeWriteError::MissingInputField("era"))
                }
                Some(era) => {
                    let era_symbol = datetime_names
                        .get_name_for_era(l, era)
                        .map_err(|e| match e {
                            GetSymbolForEraError::Missing => DateTimeWriteError::InvalidEra(era),
                            GetSymbolForEraError::MissingNames(f) => {
                                DateTimeWriteError::NamesNotLoaded(f)
                            }
                        });
                    match era_symbol {
                        Err(e) => {
                            w.with_part(Part::ERROR, |w| w.write_str(&era.fallback_name()))?;
                            Err(e)
                        }
                        Ok(era) => Ok(w.write_str(era)?),
                    }
                }
            },
        },
        (FieldSymbol::Year(Year::Calendar), l) => match input.year {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("year"))
            }
            Some(year) => try_write_number(w, fdf, year.era_year_or_extended().into(), l)?,
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
                            .ok_or(DateTimeWriteError::InvalidCyclicYear {
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
            Some(month_info) => match datetime_names
                .get_name_for_month(month, l, month_info.formatting_code)
                .map_err(|e| match e {
                    GetNameForMonthError::Missing => {
                        DateTimeWriteError::InvalidMonthCode(month_info.formatting_code)
                    }
                    GetNameForMonthError::MissingNames(f) => DateTimeWriteError::NamesNotLoaded(f),
                }) {
                Err(e) => {
                    w.with_part(Part::ERROR, |w| w.write_str(&month_info.formatting_code.0))?;
                    Err(e)
                }
                Ok(MonthPlaceholderValue::PlainString(symbol)) => {
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
        (FieldSymbol::Weekday(weekday), l) => match input.iso_weekday {
            None => {
                write_value_missing(w, field)?;
                Err(DateTimeWriteError::MissingInputField("iso_weekday"))
            }
            Some(wd) => match datetime_names
                .get_name_for_weekday(weekday, l, wd)
                .map_err(|e| match e {
                    GetNameForWeekdayError::MissingNames(f) => {
                        DateTimeWriteError::NamesNotLoaded(f)
                    }
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
                match datetime_names
                    .get_name_for_day_period(
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
                        GetNameForDayPeriodError::MissingNames(f) => {
                            DateTimeWriteError::NamesNotLoaded(f)
                        }
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
        (FieldSymbol::TimeZone(time_zone), length) => {
            // TODO: Implement proper formatting logic here
            match ResolvedNeoTimeZoneSkeleton::from_field(time_zone, length) {
                None => {
                    w.with_part(Part::ERROR, |w| {
                        w.write_str("{unsupported:")?;
                        w.write_char(char::from(field.symbol))?;
                        w.write_str("}")
                    })?;
                    Err(DateTimeWriteError::UnsupportedField(field))
                }
                Some(time_zone) => {
                    let payloads = datetime_names.get_payloads();
                    let mut r = Err(FormatTimeZoneError::Fallback);
                    for formatter in time_zone.units() {
                        match formatter.format(w, input, payloads, fdf)? {
                            Err(FormatTimeZoneError::Fallback) => {
                                // Expected common case: the unit needs fall back to the next one
                                continue;
                            }
                            r2 => {
                                r = r2;
                                break;
                            }
                        }
                    }

                    match r {
                        Ok(()) => Ok(()),
                        Err(FormatTimeZoneError::MissingInputField(f)) => {
                            write_value_missing(w, field)?;
                            Err(DateTimeWriteError::MissingInputField(f))
                        }
                        Err(
                            e @ (FormatTimeZoneError::FixedDecimalFormatterNotLoaded
                            | FormatTimeZoneError::NamesNotLoaded),
                        ) => {
                            if let Some(offset) = input.offset {
                                w.with_part(Part::ERROR, |w| {
                                    Iso8601Format {
                                        format: IsoFormat::Basic,
                                        minutes: IsoMinutes::Required,
                                        seconds: IsoSeconds::Optional,
                                    }
                                    .format_infallible(w, offset)
                                })?;
                            } else {
                                write_value_missing(w, field)?;
                            }
                            Err(match e {
                                FormatTimeZoneError::FixedDecimalFormatterNotLoaded => {
                                    DateTimeWriteError::FixedDecimalFormatterNotLoaded
                                }
                                FormatTimeZoneError::NamesNotLoaded => {
                                    DateTimeWriteError::NamesNotLoaded(field)
                                }
                                _ => unreachable!(),
                            })
                        }
                        Err(FormatTimeZoneError::Fallback) => {
                            // unreachable because our current fallback chains don't fall through
                            w.with_part(Part::ERROR, |w| {
                                w.write_str("{unsupported:")?;
                                w.write_char(char::from(field.symbol))?;
                                w.write_str("}")
                            })?;
                            Err(DateTimeWriteError::UnsupportedField(field))
                        }
                    }
                }
            }
        }
        (
            FieldSymbol::Year(Year::WeekOf)
            | FieldSymbol::Week(Week::WeekOfYear)
            | FieldSymbol::Week(Week::WeekOfMonth)
            | FieldSymbol::Day(Day::DayOfYear)
            | FieldSymbol::Day(Day::ModifiedJulianDay)
            | FieldSymbol::Second(Second::Millisecond),
            _,
        ) => {
            w.with_part(Part::ERROR, |w| {
                w.write_str("{unsupported:")?;
                w.write_char(char::from(field.symbol))?;
                w.write_str("}")
            })?;
            Err(DateTimeWriteError::UnsupportedField(field))
        }
    })
}

#[cfg(test)]
#[allow(unused_imports)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use icu_decimal::options::{FixedDecimalFormatterOptions, GroupingStrategy};

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
