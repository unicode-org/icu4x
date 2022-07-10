// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::error::DateTimeFormatError as Error;
use crate::{
    date::TimeZoneInput,
    time_zone::{FormatTimeZone, TimeZoneFormat, TimeZoneFormatUnit},
    DateTimeFormatError,
};
use writeable::Writeable;

/// [`FormattedTimeZone`] is a intermediate structure which can be retrieved as an output from [`TimeZoneFormat`].
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
    /// Format time zone with fallbacks.
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        match self.write_no_fallback(sink) {
            Ok(Ok(r)) => Ok(r),
            _ => match self.time_zone_format.fallback_unit {
                TimeZoneFormatUnit::LocalizedGmt(fallback) => {
                    match fallback.format(
                        sink,
                        self.time_zone,
                        &self.time_zone_format.data_payloads,
                    ) {
                        Ok(Ok(r)) => Ok(r),
                        Ok(Err(e)) => Err(e),
                        Err(_e) => {
                            debug_assert!(false, "{:?}", _e);
                            Err(core::fmt::Error)
                        }
                    }
                }
                TimeZoneFormatUnit::Iso8601(fallback) => {
                    match fallback.format(
                        sink,
                        self.time_zone,
                        &self.time_zone_format.data_payloads,
                    ) {
                        Ok(Ok(r)) => Ok(r),
                        Ok(Err(e)) => Err(e),
                        Err(_e) => {
                            debug_assert!(false, "{:?}", _e);
                            Err(core::fmt::Error)
                        }
                    }
                }
                _ => Err(core::fmt::Error),
            },
        }
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

impl<'l, T> FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    /// Write time zone with no fallback.
    pub fn write_no_fallback<W>(&self, w: &mut W) -> Result<fmt::Result, Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        for unit in self.time_zone_format.format_units.iter() {
            match unit.format(w, self.time_zone, &self.time_zone_format.data_payloads) {
                Ok(r) => return Ok(r),
                Err(DateTimeFormatError::UnsupportedOptions) => continue,
                Err(e) => return Err(e),
            }
        }
        Err(DateTimeFormatError::UnsupportedOptions)
    }
}
