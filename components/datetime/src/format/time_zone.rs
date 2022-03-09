// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::error::DateTimeFormatError as Error;
use crate::pattern::PatternItem;
use crate::provider::calendar::patterns::PatternPluralsFromPatternsV1Marker;
use crate::{
    date::TimeZoneInput,
    time_zone::{FormatTimeZone, TimeZoneFormat, TimeZoneFormatKind, TimeZoneFormatUnit},
    DateTimeFormatError,
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
        self.write_zone(sink).map_err(|_| core::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_zone(f).map_err(|_| core::fmt::Error)
    }
}

impl<'l, T> FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    /// Format time zone with fallbacks.
    pub fn write<W>(&self, w: &mut W)
    where
        W: core::fmt::Write + ?Sized,
    {
        match self.write_no_fallback(w) {
            Ok(_) => {}
            Err(_) => match self.time_zone_format.fallback_unit {
                Some(TimeZoneFormatUnit::LocalizedGmt(fallback)) => {
                    let _ =
                        fallback.format(w, self.time_zone, &self.time_zone_format.data_payloads);
                }
                Some(TimeZoneFormatUnit::Iso8601(fallback)) => {
                    let _ =
                        fallback.format(w, self.time_zone, &self.time_zone_format.data_payloads);
                }
                _ => {}
            },
        };
    }

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

    pub(crate) fn write_zone<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: fmt::Write + ?Sized,
    {
        match &self.time_zone_format.kind {
            TimeZoneFormatKind::Pattern(patterns) => self.write_pattern(patterns, w),
            TimeZoneFormatKind::Config(_) => self.write_config(w),
        }
    }

    fn write_config<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: fmt::Write + ?Sized,
    {
        self.write(w);
        Ok(())
    }

    fn write_pattern<W>(
        &self,
        patterns: &DataPayload<PatternPluralsFromPatternsV1Marker>,
        w: &mut W,
    ) -> Result<(), Error>
    where
        W: fmt::Write + ?Sized,
    {
        let pattern = patterns
            .get()
            .0
            .clone()
            .expect_pattern("Expected a single pattern");
        for item in pattern.items.iter() {
            match item {
                PatternItem::Field(_) => self.write_field(w)?,
                PatternItem::Literal(ch) => w.write_char(ch)?,
            }
        }
        Ok(())
    }

    /// Write fields according to the UTS-35 specification.
    /// https://unicode.org/reports/tr35/tr35-dates.html#dfst-zone
    pub(super) fn write_field<W>(&self, w: &mut W) -> Result<(), Error>
    where
        W: fmt::Write + ?Sized,
    {
        self.write(w);
        Ok(())
    }
}
