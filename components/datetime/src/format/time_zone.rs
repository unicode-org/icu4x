// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, FieldSymbol};
use crate::pattern::{PatternError, PatternItem};
use crate::provider::calendar::patterns::PatternPluralsFromPatternsV1Marker;
use crate::{
    date::TimeZoneInput,
    time_zone::{
        IsoFormat, IsoMinutes, IsoSeconds, TimeZoneFormat, TimeZoneFormatConfig, TimeZoneFormatKind,
    },
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
    time_zone_format.format(w, time_zone);
    Ok(())
}
