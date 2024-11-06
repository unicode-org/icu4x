// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::neo::RawDateTimeNamesBorrowed;
use super::time_zone::{
    FormatTimeZone, FormatTimeZoneError, Iso8601Format, IsoFormat, IsoMinutes, IsoSeconds,
};
use super::GetNameForDayPeriodError;
use super::{
    GetNameForMonthError, GetNameForWeekdayError, GetSymbolForEraError, MonthPlaceholderValue,
};
use crate::error::DateTimeWriteError;
use crate::fields::{self, Day, FieldLength, FieldSymbol, Second, Week, Year};
use crate::input::ExtractedInput;
use crate::provider::pattern::runtime::PatternMetadata;
use crate::provider::pattern::PatternItem;

use core::fmt::{self, Write};
use fixed_decimal::FixedDecimal;
use icu_calendar::types::{DayOfWeekInMonth, IsoWeekday};
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

    macro_rules! input {
        ($name:ident = $input:expr) => {
            let Some($name) = $input else {
                write_value_missing(w, field)?;
                return Ok(Err(DateTimeWriteError::MissingInputField(stringify!(
                    $name
                ))));
            };
        };
    }

    Ok(match (field.symbol, field.length) {
        (FieldSymbol::Era, l) => {
            input!(year = input.year);
            input!(era = year.formatting_era());
            let era_symbol = datetime_names
                .get_name_for_era(l, era)
                .map_err(|e| match e {
                    GetSymbolForEraError::Missing => DateTimeWriteError::InvalidEra(era),
                    GetSymbolForEraError::MissingNames(f) => DateTimeWriteError::NamesNotLoaded(f),
                });
            match era_symbol {
                Err(e) => {
                    w.with_part(Part::ERROR, |w| w.write_str(&era.fallback_name()))?;
                    Err(e)
                }
                Ok(era) => Ok(w.write_str(era)?),
            }
        }
        (FieldSymbol::Year(Year::Calendar), l) => {
            input!(year = input.year);
            try_write_number(w, fdf, year.era_year_or_extended().into(), l)?
        }
        (FieldSymbol::Year(Year::Cyclic), l) => {
            input!(year = input.year);
            input!(cyclic = year.cyclic());

            // TODO(#3761): This is a hack, we should use actual data for cyclic years
            let cyclics: &[&str; 60] = match input.any_calendar_kind {
                Some(AnyCalendarKind::Dangi) => &[
                    "갑자", "을축", "병인", "정묘", "무진", "기사", "경오", "신미", "임신", "계유",
                    "갑술", "을해", "병자", "정축", "무인", "기묘", "경진", "신사", "임오", "계미",
                    "갑신", "을유", "병술", "정해", "무자", "기축", "경인", "신묘", "임진", "계사",
                    "갑오", "을미", "병신", "정유", "무술", "기해", "경자", "신축", "임인", "계묘",
                    "갑진", "을사", "병오", "정미", "무신", "기유", "경술", "신해", "임자", "계축",
                    "갑인", "을묘", "병진", "정사", "무오", "기미", "경신", "신유", "임술", "계해",
                ],
                // for now assume all other calendars use the stem-branch model
                _ => &[
                    "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉",
                    "甲戌", "乙亥", "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未",
                    "甲申", "乙酉", "丙戌", "丁亥", "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳",
                    "甲午", "乙未", "丙申", "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯",
                    "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥", "壬子", "癸丑",
                    "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未", "庚申", "辛酉", "壬戌", "癸亥",
                ],
            };

            let Some(cyclic_str) = cyclics.get((cyclic.get() as usize) - 1) else {
                w.with_part(Part::ERROR, |w| {
                    try_write_number(w, fdf, year.era_year_or_extended().into(), l).map(|_| ())
                })?;
                return Ok(Err(DateTimeWriteError::InvalidCyclicYear {
                    value: cyclic.get() as usize,
                    max: cyclics.len() + 1,
                }));
            };
            Ok(w.write_str(cyclic_str)?)
        }
        (FieldSymbol::Year(Year::RelatedIso), l) => {
            input!(year = input.year);
            input!(related_iso = year.related_iso());

            try_write_number(w, fdf, related_iso.into(), l)?
        }
        (FieldSymbol::Month(_), l @ (FieldLength::One | FieldLength::TwoDigit)) => {
            input!(month = input.month);
            try_write_number(w, fdf, month.ordinal.into(), l)?
        }
        (FieldSymbol::Month(symbol), l) => {
            input!(month = input.month);
            match datetime_names
                .get_name_for_month(symbol, l, month.formatting_code)
                .map_err(|e| match e {
                    GetNameForMonthError::Missing => {
                        DateTimeWriteError::InvalidMonthCode(month.formatting_code)
                    }
                    GetNameForMonthError::MissingNames(f) => DateTimeWriteError::NamesNotLoaded(f),
                }) {
                Err(e) => {
                    w.with_part(Part::ERROR, |w| w.write_str(&month.formatting_code.0))?;
                    Err(e)
                }
                Ok(MonthPlaceholderValue::PlainString(symbol)) => {
                    w.write_str(symbol)?;
                    Ok(())
                }
                Ok(MonthPlaceholderValue::Numeric) => {
                    try_write_number(w, fdf, month.ordinal.into(), l)?
                }
                Ok(MonthPlaceholderValue::NumericPattern(substitution_pattern)) => {
                    w.write_str(substitution_pattern.get_prefix())?;
                    let r = try_write_number(w, fdf, month.ordinal.into(), l)?;
                    w.write_str(substitution_pattern.get_suffix())?;
                    r
                }
            }
        }
        (FieldSymbol::Weekday(weekday), l) => {
            input!(iso_weekday = input.iso_weekday);
            match datetime_names
                .get_name_for_weekday(weekday, l, iso_weekday)
                .map_err(|e| match e {
                    GetNameForWeekdayError::MissingNames(f) => {
                        DateTimeWriteError::NamesNotLoaded(f)
                    }
                }) {
                Err(e) => {
                    w.with_part(Part::ERROR, |w| {
                        w.write_str(match iso_weekday {
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
            }
        }
        (FieldSymbol::Day(fields::Day::DayOfMonth), l) => {
            input!(day_of_month = input.day_of_month);
            try_write_number(w, fdf, day_of_month.0.into(), l)?
        }
        (FieldSymbol::Day(fields::Day::DayOfWeekInMonth), l) => {
            input!(day_of_month = input.day_of_month);
            try_write_number(w, fdf, DayOfWeekInMonth::from(day_of_month).0.into(), l)?
        }
        (FieldSymbol::Day(fields::Day::DayOfYear), l) => {
            input!(day_of_year = input.day_of_year);
            try_write_number(w, fdf, day_of_year.day_of_year.into(), l)?
        }
        (FieldSymbol::Hour(symbol), l) => {
            input!(hour = input.hour);
            let h = hour.number();
            let h = match symbol {
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
        (FieldSymbol::Minute, l) => {
            input!(minute = input.minute);
            try_write_number(w, fdf, minute.number().into(), l)?
        }
        (FieldSymbol::Second(Second::Second), l) => {
            input!(second = input.second);
            try_write_number(w, fdf, second.number().into(), l)?
        }
        (FieldSymbol::Second(Second::MillisInDay), l) => {
            input!(hour = input.hour);
            input!(minute = input.minute);
            input!(second = input.second);
            input!(nanosecond = input.nanosecond);

            let milliseconds = (((hour.number() as u32 * 60) + minute.number() as u32) * 60
                + second.number() as u32)
                * 1000
                + nanosecond.number() / 1_000_000;
            try_write_number(w, fdf, milliseconds.into(), l)?
        }
        (FieldSymbol::DecimalSecond(decimal_second), l) => {
            input!(second = input.second);
            input!(nanosecond = input.nanosecond);

            // Formatting with fractional seconds
            let mut s = FixedDecimal::from(second.number());
            let _infallible =
                s.concatenate_end(FixedDecimal::from(nanosecond.number()).multiplied_pow10(-9));
            debug_assert!(_infallible.is_ok());
            let position = -(decimal_second as i16);
            s.trunc(position);
            s.pad_end(position);
            try_write_number(w, fdf, s, l)?
        }
        (FieldSymbol::DayPeriod(period), l) => {
            input!(hour = input.hour);

            match datetime_names
                .get_name_for_day_period(
                    period,
                    l,
                    hour,
                    pattern_metadata.time_granularity().is_top_of_hour(
                        input.minute.unwrap_or_default().number(),
                        input.second.unwrap_or_default().number(),
                        input.nanosecond.unwrap_or_default().number(),
                    ),
                )
                .map_err(|e| match e {
                    GetNameForDayPeriodError::MissingNames(f) => {
                        DateTimeWriteError::NamesNotLoaded(f)
                    }
                }) {
                Err(e) => {
                    w.with_part(Part::ERROR, |w| {
                        w.write_str(if hour.number() < 12 { "AM" } else { "PM" })
                    })?;
                    Err(e)
                }
                Ok(s) => Ok(w.write_str(s)?),
            }
        }
        (FieldSymbol::TimeZone(time_zone), length) => {
            let payloads = datetime_names.get_payloads();

            let mut r = Err(FormatTimeZoneError::Fallback);
            for formatter in time_zone.units(length) {
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
        (
            FieldSymbol::Year(Year::WeekOf)
            | FieldSymbol::Week(Week::WeekOfYear)
            | FieldSymbol::Week(Week::WeekOfMonth)
            | FieldSymbol::Day(Day::ModifiedJulianDay),
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
