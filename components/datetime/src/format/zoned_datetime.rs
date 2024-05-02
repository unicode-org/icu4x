// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of code for formatting DateTimes with time zones.

use crate::fields::{Field, FieldSymbol};
use crate::input::{
    DateTimeInput, DateTimeInputWithWeekConfig, ExtractedTimeZoneInput, LocalizedDateTimeInput,
};
use crate::pattern::runtime::PatternMetadata;
use crate::pattern::PatternItem;
use crate::provider::date_time::{DateSymbols, TimeSymbols};
use crate::time_zone::TimeZoneFormatter;
use crate::FormattedTimeZone;
use crate::{Error, FormattedDateTime};
use core::fmt;
use icu_decimal::FixedDecimalFormatter;
use writeable::Writeable;

use super::datetime;

#[cfg(doc)]
use crate::ZonedDateTimeFormatter;

/// [`FormattedTimeZone`] is a intermediate structure which can be retrieved
/// as an output from [`ZonedDateTimeFormatter`].
#[derive(Debug, Copy, Clone)]
pub struct FormattedZonedDateTime<'l> {
    pub(crate) formatted_datetime: FormattedDateTime<'l>,
    pub(crate) time_zone_format: &'l TimeZoneFormatter,
    pub(crate) time_zone: ExtractedTimeZoneInput,
}

impl<'l> Writeable for FormattedZonedDateTime<'l> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut r = Ok(());

        let loc_datetime = DateTimeInputWithWeekConfig::new(
            &self.formatted_datetime.datetime,
            self.formatted_datetime.week_data,
        );

        let (pattern, pattern_err) = self
            .formatted_datetime
            .patterns
            .get()
            .0
            .select_lossy(&loc_datetime, self.formatted_datetime.ordinal_rules);

        if let Some(e) = pattern_err {
            r = Err(e);
        }

        r = r.and(try_write_pattern(
            pattern.items.iter(),
            pattern.metadata,
            self.formatted_datetime.date_symbols,
            self.formatted_datetime.time_symbols,
            &loc_datetime,
            &self.time_zone,
            self.time_zone_format,
            Some(self.formatted_datetime.fixed_decimal_format),
            &mut writeable::adapters::CoreWriteAsPartsWrite(sink),
        )?);

        debug_assert!(r.is_ok());
        Ok(())
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'l> fmt::Display for FormattedZonedDateTime<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

#[allow(clippy::too_many_arguments)]
fn try_write_pattern<'data, T, W, DS, TS>(
    pattern_items: impl Iterator<Item = PatternItem>,
    pattern_metadata: PatternMetadata,
    date_symbols: Option<&DS>,
    time_symbols: Option<&TS>,
    loc_datetime: &impl LocalizedDateTimeInput<T>,
    time_zone: &ExtractedTimeZoneInput,
    time_zone_format: &TimeZoneFormatter,
    fixed_decimal_format: Option<&FixedDecimalFormatter>,
    w: &mut W,
) -> Result<Result<(), Error>, fmt::Error>
where
    T: DateTimeInput,
    W: writeable::PartsWrite + ?Sized,
    DS: DateSymbols<'data>,
    TS: TimeSymbols,
{
    let mut r = Ok(());
    let mut iter = pattern_items.peekable();
    loop {
        match iter.next() {
            Some(PatternItem::Field(Field {
                symbol: FieldSymbol::TimeZone(..),
                ..
            })) => FormattedTimeZone {
                time_zone_format,
                time_zone,
            }
            .write_to(w)?,
            Some(PatternItem::Field(field)) => {
                r = r.and(datetime::try_write_field(
                    pattern_metadata,
                    field,
                    iter.peek(),
                    date_symbols,
                    time_symbols,
                    loc_datetime,
                    fixed_decimal_format,
                    w,
                )?);
            }
            Some(PatternItem::Literal(ch)) => w.write_char(ch)?,
            None => break,
        }
    }
    Ok(r)
}
