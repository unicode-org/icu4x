// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of code for formatting DateTimes with time zones.

use crate::date::{LocalizedDateTimeInput, ZonedDateTimeInputWithLocale};
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, FieldSymbol};
use crate::pattern::PatternItem;
use crate::{date::ZonedDateTimeInput, zoned_datetime::ZonedDateTimeFormat};
use core::fmt;
use writeable::Writeable;

use super::datetime;
use super::time_zone;

#[allow(missing_docs)] // TODO(#686) - Add missing docs.
pub struct FormattedZonedDateTime<'l, 'data, T>
where
    T: ZonedDateTimeInput,
{
    pub(crate) zoned_datetime_format: &'l ZonedDateTimeFormat<'data>,
    pub(crate) zoned_datetime: &'l T,
}

impl<'l, 'd, T> Writeable for FormattedZonedDateTime<'l, 'd, T>
where
    T: ZonedDateTimeInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern(self.zoned_datetime_format, self.zoned_datetime, sink)
            .map_err(|_| core::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, 'd, T> fmt::Display for FormattedZonedDateTime<'l, 'd, T>
where
    T: ZonedDateTimeInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(self.zoned_datetime_format, self.zoned_datetime, f)
            .map_err(|_| core::fmt::Error)
    }
}

pub fn write_pattern<T, W>(
    zoned_datetime_format: &ZonedDateTimeFormat,
    zoned_datetime: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: ZonedDateTimeInput,
    W: fmt::Write + ?Sized,
{
    let locale = &zoned_datetime_format.datetime_format.locale;
    let pattern = &zoned_datetime_format.datetime_format.pattern;
    let loc_datetime = ZonedDateTimeInputWithLocale::new(zoned_datetime, locale);
    for item in pattern.items() {
        match item {
            PatternItem::Field(field) => {
                write_field(field, zoned_datetime_format, &loc_datetime, w)?
            }
            PatternItem::Literal(ch) => w.write_char(*ch)?,
        }
    }
    Ok(())
}

fn write_field<T, W>(
    field: &fields::Field,
    zoned_datetime_format: &ZonedDateTimeFormat,
    loc_datetime: &impl LocalizedDateTimeInput<T>,
    w: &mut W,
) -> Result<(), Error>
where
    T: ZonedDateTimeInput,
    W: fmt::Write + ?Sized,
{
    let pattern = &zoned_datetime_format.datetime_format.pattern;
    let symbols = zoned_datetime_format
        .datetime_format
        .symbols
        .as_ref()
        .map(|s| s.get());

    match field.symbol {
        FieldSymbol::TimeZone(_time_zone) => time_zone::write_field(
            field,
            &zoned_datetime_format.time_zone_format,
            loc_datetime.datetime(),
            w,
        )?,
        _ => datetime::write_field(pattern, field, symbols, loc_datetime, w)?,
    }
    Ok(())
}
