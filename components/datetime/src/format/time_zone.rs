// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::error::DateTimeFormatError as Error;
use crate::pattern::PatternItem;
use crate::provider::calendar::patterns::PatternPluralsFromPatternsV1Marker;
use crate::{
    date::TimeZoneInput,
    time_zone::{TimeZoneFormat, TimeZoneFormatKind},
};
use icu_provider::DataPayload;
use writeable::Writeable;

pub struct FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    pub(crate) time_zone_format: &'l TimeZoneFormat,
    pub(crate) time_zone: &'l T,
}

impl<'l, T> Writeable for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_zone(self.time_zone_format, self.time_zone, sink).map_err(|_| core::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_zone(self.time_zone_format, self.time_zone, f).map_err(|_| core::fmt::Error)
    }
}

pub(crate) fn write_zone<T, W>(
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    match &time_zone_format.kind {
        TimeZoneFormatKind::Pattern(patterns) => {
            write_pattern(time_zone_format, time_zone, patterns, w)
        }
        TimeZoneFormatKind::Config(_) => write_config(time_zone_format, time_zone, w),
    }
}

fn write_config<T, W>(
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    time_zone_format.format(w, time_zone);
    Ok(())
}

fn write_pattern<T, W>(
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    patterns: &DataPayload<PatternPluralsFromPatternsV1Marker>,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    let pattern = patterns
        .get()
        .0
        .clone()
        .expect_pattern("Expected a single pattern");
    for item in pattern.items.iter() {
        match item {
            PatternItem::Field(_) => write_field(time_zone_format, time_zone, w)?,
            PatternItem::Literal(ch) => w.write_char(ch)?,
        }
    }
    Ok(())
}

/// Write fields according to the UTS-35 specification.
/// https://unicode.org/reports/tr35/tr35-dates.html#dfst-zone
pub(super) fn write_field<T, W>(
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    time_zone_format.format(w, time_zone);
    Ok(())
}
