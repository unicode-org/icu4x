// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::error::DateTimeFormatterError as Error;
use crate::{
    date::TimeZoneInput,
    time_zone::{FormatTimeZone, TimeZoneFormatter, TimeZoneFormatterUnit},
    DateTimeFormatterError,
};
use writeable::Writeable;

/// [`FormattedTimeZone`] is a intermediate structure which can be retrieved as an output from [`TimeZoneFormatter`].
pub struct FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    pub(crate) time_zone_format: &'l TimeZoneFormatter,
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
                TimeZoneFormatterUnit::LocalizedGmt(fallback) => {
                    match fallback.format(
                        sink,
                        self.time_zone,
                        &self.time_zone_format.data_payloads,
                    ) {
                        Ok(Ok(r)) => Ok(r),
                        Ok(Err(e)) => Err(e),
                        Err(e) => match e {
                            DateTimeFormatterError::MissingInputField(Some("gmt_offset")) => {
                                debug_assert!(
                                    false,
                                    "{:?}",
                                    &self
                                        .time_zone_format
                                        .data_payloads
                                        .zone_formats
                                        .get()
                                        .gmt_offset_fallback
                                );
                                sink.write_str(
                                    &self
                                        .time_zone_format
                                        .data_payloads
                                        .zone_formats
                                        .get()
                                        .gmt_offset_fallback,
                                )
                            }
                            _ => Err(core::fmt::Error),
                        },
                    }
                }
                TimeZoneFormatterUnit::Iso8601(fallback) => {
                    match fallback.format(
                        sink,
                        self.time_zone,
                        &self.time_zone_format.data_payloads,
                    ) {
                        Ok(Ok(r)) => Ok(r),
                        Ok(Err(e)) => Err(e),
                        Err(e) => match e {
                            DateTimeFormatterError::MissingInputField(Some("gmt_offset")) => {
                                debug_assert!(
                                    false,
                                    "{:?}",
                                    &self
                                        .time_zone_format
                                        .data_payloads
                                        .zone_formats
                                        .get()
                                        .gmt_offset_fallback
                                );
                                sink.write_str(
                                    &self
                                        .time_zone_format
                                        .data_payloads
                                        .zone_formats
                                        .get()
                                        .gmt_offset_fallback,
                                )
                            }
                            _ => Err(core::fmt::Error),
                        },
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
                Err(DateTimeFormatterError::UnsupportedOptions) => continue,
                Err(e) => return Err(e),
            }
        }
        Err(DateTimeFormatterError::UnsupportedOptions)
    }
}
