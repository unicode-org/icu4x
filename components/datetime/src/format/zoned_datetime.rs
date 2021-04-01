// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::ZonedDateTimeInput;
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, FieldSymbol};
use crate::pattern::PatternItem;
use crate::{
    date::{LocalizedDateTimeInput, ZonedDateTimeInputWithLocale},
    ZonedDateTimeFormat,
};
use std::fmt;
use writeable::Writeable;

use super::datetime;
use super::timezone;

pub struct FormattedZonedDateTime<'l, T>
where
    T: ZonedDateTimeInput,
{
    pub(crate) zoned_date_time_format: &'l ZonedDateTimeFormat<'l>,
    pub(crate) zoned_datetime: &'l T,
}

impl<'l, T> Writeable for FormattedZonedDateTime<'l, T>
where
    T: ZonedDateTimeInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern(self.zoned_date_time_format, self.zoned_datetime, sink)
            .map_err(|_| std::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedZonedDateTime<'l, T>
where
    T: ZonedDateTimeInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(self.zoned_date_time_format, self.zoned_datetime, f)
            .map_err(|_| std::fmt::Error)
    }
}

pub fn write_pattern<T, W>(
    zoned_date_time_format: &ZonedDateTimeFormat,
    zoned_datetime: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: ZonedDateTimeInput,
    W: fmt::Write + ?Sized,
{
    let locale = &zoned_date_time_format.date_time_format.locale;
    let pattern = &zoned_date_time_format.date_time_format.pattern;
    let loc_date_time = ZonedDateTimeInputWithLocale::new(zoned_datetime, locale);
    for item in pattern.items() {
        match item {
            PatternItem::Field(field) => {
                write_field(&field, zoned_date_time_format, &loc_date_time, w)?
            }
            PatternItem::Literal(l) => w.write_str(l)?,
        }
    }
    Ok(())
}

fn write_field<T, W>(
    field: &fields::Field,
    zoned_date_time_format: &ZonedDateTimeFormat,
    loc_date_time: &impl LocalizedDateTimeInput<T>,
    w: &mut W,
) -> Result<(), Error>
where
    T: ZonedDateTimeInput,
    W: fmt::Write + ?Sized,
{
    let pattern = &zoned_date_time_format.date_time_format.pattern;
    let symbols = &zoned_date_time_format.date_time_format.symbols;

    match field.symbol {
        FieldSymbol::TimeZone(_timezone) => timezone::write_field(
            field,
            &zoned_date_time_format.time_zone_format,
            loc_date_time.date_time(),
            w,
        )?,
        _ => datetime::write_field(pattern, field, symbols, loc_date_time, w)?,
    }
    Ok(())
}
