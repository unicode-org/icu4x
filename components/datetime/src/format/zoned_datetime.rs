// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of code for formatting DateTimes with time zones.

use crate::fields::{self, FieldSymbol};
use crate::input::{
    DateTimeInput, DateTimeInputWithWeekConfig, ExtractedDateTimeInput, ExtractedTimeZoneInput,
    LocalizedDateTimeInput, TimeZoneInput,
};
use crate::pattern::runtime::PatternMetadata;
use crate::pattern::PatternItem;
use crate::Error;
use crate::{raw, FormattedTimeZone};
use core::fmt;
use writeable::Writeable;

use super::datetime;

#[cfg(doc)]
use crate::ZonedDateTimeFormatter;

/// [`FormattedTimeZone`] is a intermediate structure which can be retrieved
/// as an output from [`ZonedDateTimeFormatter`].
#[derive(Debug, Copy, Clone)]
pub struct FormattedZonedDateTime<'l> {
    pub(crate) zoned_datetime_format: &'l raw::ZonedDateTimeFormatter,
    pub(crate) datetime: ExtractedDateTimeInput,
    pub(crate) time_zone: ExtractedTimeZoneInput,
}

impl<'l> Writeable for FormattedZonedDateTime<'l> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let _lossy = try_write_pattern(
            self.zoned_datetime_format,
            &self.datetime,
            &self.time_zone,
            &mut writeable::adapters::CoreWriteAsPartsWrite(sink),
        )?;
        debug_assert!(matches!(_lossy, Ok(())));
        Ok(())
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'l> fmt::Display for FormattedZonedDateTime<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

pub(crate) fn try_write_pattern<D, Z, W>(
    zoned_datetime_format: &raw::ZonedDateTimeFormatter,
    datetime: &D,
    time_zone: &Z,
    w: &mut W,
) -> Result<Result<(), Error>, fmt::Error>
where
    D: DateTimeInput,
    Z: TimeZoneInput,
    W: writeable::PartsWrite + ?Sized,
{
    let mut r = Ok(());

    let patterns = &zoned_datetime_format.datetime_format.patterns;
    let loc_datetime = DateTimeInputWithWeekConfig::new(
        datetime,
        zoned_datetime_format.datetime_format.week_data.as_ref(),
    );

    let pattern = patterns
        .get()
        .0
        .select(
            &loc_datetime,
            zoned_datetime_format.datetime_format.ordinal_rules.as_ref(),
        )
        .unwrap_or_else(|e| {
            r = Err(e);
            todo!("fallback pattern")
        });

    let mut iter = pattern.items.iter().peekable();
    loop {
        match iter.next() {
            Some(PatternItem::Field(field)) => {
                r = r.and(try_write_field(
                    pattern.metadata,
                    field,
                    iter.peek(),
                    zoned_datetime_format,
                    &loc_datetime,
                    time_zone,
                    w,
                )?)
            }
            Some(PatternItem::Literal(ch)) => w.write_char(ch)?,
            None => break,
        }
    }
    Ok(r)
}

fn try_write_field<D, Z, W>(
    pattern_metadata: PatternMetadata,
    field: fields::Field,
    next_item: Option<&PatternItem>,
    zoned_datetime_format: &raw::ZonedDateTimeFormatter,
    loc_datetime: &impl LocalizedDateTimeInput<D>,
    time_zone: &Z,
    w: &mut W,
) -> Result<Result<(), Error>, fmt::Error>
where
    D: DateTimeInput,
    Z: TimeZoneInput,
    W: writeable::PartsWrite + ?Sized,
{
    let date_symbols = zoned_datetime_format
        .datetime_format
        .date_symbols
        .as_ref()
        .map(|s| s.get());

    let time_symbols = zoned_datetime_format
        .datetime_format
        .time_symbols
        .as_ref()
        .map(|s| s.get());

    if let FieldSymbol::TimeZone(..) = field.symbol {
        FormattedTimeZone {
            time_zone_format: &zoned_datetime_format.time_zone_format,
            time_zone,
        }
        .write_to(w)
        .map(Ok)
    } else {
        datetime::try_write_field(
            pattern_metadata,
            field,
            next_item,
            date_symbols,
            time_symbols,
            loc_datetime,
            Some(&zoned_datetime_format.datetime_format.fixed_decimal_format),
            w,
        )
    }
}
