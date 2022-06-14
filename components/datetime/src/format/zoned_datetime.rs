// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of code for formatting DateTimes with time zones.

use crate::date::ZonedDateTimeInput;
use crate::date::{LocalizedDateTimeInput, ZonedDateTimeInputWithLocale};
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, FieldSymbol};
use crate::pattern::{runtime, PatternItem};
use crate::{raw, FormattedTimeZone};
use core::fmt;
use writeable::Writeable;

use super::datetime;

#[allow(missing_docs)] // TODO(#686) - Add missing docs.
pub struct FormattedZonedDateTime<'l, T>
where
    T: ZonedDateTimeInput,
{
    pub(crate) zoned_datetime_format: &'l raw::ZonedDateTimeFormat,
    pub(crate) zoned_datetime: &'l T,
}

impl<'l, T> Writeable for FormattedZonedDateTime<'l, T>
where
    T: ZonedDateTimeInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern(self.zoned_datetime_format, self.zoned_datetime, sink)
            .map_err(|_| core::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedZonedDateTime<'l, T>
where
    T: ZonedDateTimeInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(self.zoned_datetime_format, self.zoned_datetime, f)
            .map_err(|_| core::fmt::Error)
    }
}

pub(crate) fn write_pattern<T, W>(
    zoned_datetime_format: &raw::ZonedDateTimeFormat,
    zoned_datetime: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: ZonedDateTimeInput,
    W: fmt::Write + ?Sized,
{
    let locale = &zoned_datetime_format.datetime_format.locale;
    let patterns = &zoned_datetime_format.datetime_format.patterns;
    let loc_datetime = ZonedDateTimeInputWithLocale::new(
        zoned_datetime,
        zoned_datetime_format
            .datetime_format
            .week_data
            .as_ref()
            .map(|d| &d.get().0),
        locale,
    );

    let pattern = patterns.get().0.select(
        &loc_datetime,
        zoned_datetime_format.datetime_format.ordinal_rules.as_ref(),
    )?;

    let mut iter = pattern.items.iter().peekable();
    loop {
        match iter.next() {
            Some(PatternItem::Field(field)) => write_field(
                pattern,
                field,
                iter.peek(),
                zoned_datetime_format,
                &loc_datetime,
                w,
            )?,
            Some(PatternItem::Literal(ch)) => w.write_char(ch)?,
            None => break,
        }
    }
    Ok(())
}

fn write_field<T, W>(
    pattern: &runtime::Pattern,
    field: fields::Field,
    next_item: Option<&PatternItem>,
    zoned_datetime_format: &raw::ZonedDateTimeFormat,
    loc_datetime: &impl LocalizedDateTimeInput<T>,
    w: &mut W,
) -> Result<(), Error>
where
    T: ZonedDateTimeInput,
    W: fmt::Write + ?Sized,
{
    let symbols = zoned_datetime_format
        .datetime_format
        .symbols
        .as_ref()
        .map(|s| s.get());

    match field.symbol {
        FieldSymbol::TimeZone(_time_zone) => FormattedTimeZone {
            time_zone_format: &zoned_datetime_format.time_zone_format,
            time_zone: loc_datetime.datetime(),
        }
        .write_to(w)?,
        _ => datetime::write_field(
            pattern,
            field,
            next_item,
            symbols,
            loc_datetime,
            &zoned_datetime_format.datetime_format.fixed_decimal_format,
            w,
        )?,
    }
    Ok(())
}
