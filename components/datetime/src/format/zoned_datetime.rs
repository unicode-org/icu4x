// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of code for formatting DateTimes with time zones.

use crate::fields::{Field, FieldSymbol};
use crate::format::datetime::{self, DateTimeWriteError};
use crate::input::{ExtractedDateTimeInput, ExtractedTimeZoneInput};
use crate::pattern::runtime::PatternBorrowed;
use crate::pattern::PatternItem;
use crate::provider::date_time::{DateSymbols, TimeSymbols};
use crate::time_zone::TimeZoneFormatter;
use crate::{FormattedDateTime, FormattedTimeZone};
use core::fmt;
use icu_calendar::week::WeekCalculator;
use icu_decimal::FixedDecimalFormatter;
use writeable::Writeable;

/// [`FormattedTimeZone`] is a intermediate structure which can be retrieved
/// as an output from [`ZonedDateTimeFormatter`](super::super::ZonedDateTimeFormatter).
#[derive(Debug, Copy, Clone)]
pub struct FormattedZonedDateTime<'l> {
    pub(crate) formatted_datetime: FormattedDateTime<'l>,
    pub(crate) time_zone_format: &'l TimeZoneFormatter,
    pub(crate) time_zone: ExtractedTimeZoneInput,
}

impl<'l> Writeable for FormattedZonedDateTime<'l> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let (pattern, mut r) = self.formatted_datetime.select_pattern_lossy();

        r = r.and(try_write_pattern(
            pattern.as_borrowed(),
            &self.formatted_datetime.datetime,
            self.formatted_datetime.date_symbols,
            self.formatted_datetime.time_symbols,
            self.formatted_datetime.week_data,
            Some(self.formatted_datetime.fixed_decimal_format),
            &self.time_zone,
            self.time_zone_format,
            &mut writeable::adapters::CoreWriteAsPartsWrite(sink),
        )?);

        debug_assert!(r.is_ok(), "{r:?}");
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
fn try_write_pattern<'data, W, DS, TS>(
    pattern: PatternBorrowed<'data>,
    datetime: &ExtractedDateTimeInput,
    date_symbols: Option<&DS>,
    time_symbols: Option<&TS>,
    week_data: Option<&'data WeekCalculator>,
    fixed_decimal_format: Option<&FixedDecimalFormatter>,
    time_zone: &ExtractedTimeZoneInput,
    time_zone_format: &TimeZoneFormatter,
    w: &mut W,
) -> Result<Result<(), DateTimeWriteError>, fmt::Error>
where
    W: writeable::PartsWrite + ?Sized,
    DS: DateSymbols<'data>,
    TS: TimeSymbols,
{
    let mut r = Ok(());
    let mut iter = pattern.items.iter().peekable();
    while let Some(item) = iter.next() {
        match item {
            PatternItem::Literal(ch) => w.write_char(ch)?,
            PatternItem::Field(Field {
                symbol: FieldSymbol::TimeZone(..),
                ..
            }) => FormattedTimeZone {
                time_zone_format,
                time_zone: *time_zone,
            }
            .write_to(w)?,
            PatternItem::Field(field) => {
                r = r.and(datetime::try_write_field(
                    field,
                    &mut iter,
                    pattern.metadata,
                    Default::default(),
                    datetime,
                    date_symbols,
                    time_symbols,
                    week_data,
                    fixed_decimal_format,
                    w,
                )?);
            }
        }
    }
    Ok(r)
}
