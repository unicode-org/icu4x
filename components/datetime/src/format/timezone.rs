// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::TimeZoneInput;
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, FieldLength, FieldSymbol};
use crate::pattern::PatternItem;
use crate::TimeZoneFormat;
use std::fmt;
use writeable::Writeable;

pub struct FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    pub(crate) time_zone_format: &'l TimeZoneFormat<'l>,
    pub(crate) time_zone: &'l T,
}

impl<'l, T> Writeable for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern(&self.time_zone_format, self.time_zone, sink).map_err(|_| std::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(&self.time_zone_format, self.time_zone, f).map_err(|_| std::fmt::Error)
    }
}

pub fn write_pattern<T, W>(
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    let pattern = &time_zone_format.pattern;
    for item in pattern.items() {
        match item {
            PatternItem::Field(field) => write_field(&field, time_zone_format, time_zone, w)?,
            PatternItem::Literal(l) => w.write_str(l)?,
        }
    }
    Ok(())
}

pub(super) fn write_field<T, W>(
    field: &fields::Field,
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    if let FieldSymbol::TimeZone(zone_symbol) = field.symbol {
        match zone_symbol {
            fields::TimeZone::LowerZ => {
                let s = match field.length {
                    FieldLength::One | FieldLength::TwoDigit | FieldLength::Abbreviated => {
                        time_zone_format
                            .short_specific_non_location_format(time_zone)
                            .unwrap_or_else(|| time_zone_format.localized_gmt_format(time_zone))
                    }
                    FieldLength::Wide => time_zone_format
                        .long_specific_non_location_format(time_zone)
                        .unwrap_or_else(|| time_zone_format.localized_gmt_format(time_zone)),
                    _ => unreachable!("Invalid field length for `z`"),
                };
                w.write_str(&s)?;
            }
            fields::TimeZone::UpperZ => todo!(),
            fields::TimeZone::UpperO => todo!(),
            fields::TimeZone::LowerV => todo!(),
            fields::TimeZone::UpperV => todo!(),
            fields::TimeZone::LowerX => todo!(),
            fields::TimeZone::UpperX => todo!(),
        }
    }
    Ok(())
}
