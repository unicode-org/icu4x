// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use tinystr::TinyStr4;

use crate::date::*;
use std::{borrow::Cow, str::FromStr};

#[derive(Debug, Default)]
pub struct MockTimeZone {
    pub gmt_offset: GmtOffset,
    pub time_zone_id: Option<String>,
    pub metazone_id: Option<String>,
    pub time_variant: Option<String>,
    pub country_code: Option<String>,
}

impl MockTimeZone {
    pub const fn new(
        gmt_offset: GmtOffset,
        time_zone_id: Option<String>,
        metazone_id: Option<String>,
        time_variant: Option<String>,
        country_code: Option<String>,
    ) -> Self {
        Self {
            gmt_offset,
            time_zone_id,
            metazone_id,
            time_variant,
            country_code,
        }
    }

    pub fn try_new(
        gmt_offset: GmtOffset,
        time_zone_id: Option<String>,
        metazone_id: Option<String>,
        time_variant: Option<String>,
        country_code: Option<String>,
    ) -> Result<Self, DateTimeError> {
        Ok(Self {
            gmt_offset,
            time_zone_id,
            metazone_id,
            time_variant,
            country_code,
        })
    }
}
impl FromStr for MockTimeZone {
    type Err = DateTimeError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let gmt_offset = GmtOffset::from_str(input)?;
        Ok(Self {
            gmt_offset,
            time_zone_id: None,
            metazone_id: None,
            time_variant: None,
            country_code: None,
        })
    }
}

impl TimeZoneInput for MockTimeZone {
    fn gmt_offset(&self) -> GmtOffset {
        self.gmt_offset
    }

    fn time_zone_id(&self) -> Option<&str> {
        self.time_zone_id.as_ref().map(AsRef::as_ref)
    }

    fn metazone_id(&self) -> Option<&str> {
        self.metazone_id.as_ref().map(AsRef::as_ref)
    }

    fn time_variant(&self) -> Option<&str> {
        self.time_variant.as_ref().map(AsRef::as_ref)
    }

    fn country_code(&self) -> Option<&str> {
        self.country_code.as_ref().map(AsRef::as_ref)
    }
}

/// Determines which ISO-8601 format should be used to format a `GmtOffset`.
pub(crate) enum IsoFormat {
    /// ISO-8601 Basic Format.
    /// Formats zero-offset numerically.
    /// e.g. +0500, +0000
    Basic,

    /// ISO-8601 Extended Format.
    /// Formats zero-offset numerically.
    /// e.g. +05:00, +00:00
    Extended,

    /// ISO-8601 Basic Format.
    /// Formats zero-offset with the ISO-8601 UTC indicator: "Z"
    /// e.g. +0500, Z
    UtcBasic,

    /// ISO-8601 Extended Format.
    /// Formats zero-offset with the ISO-8601 UTC indicator: "Z"
    /// e.g. +05:00, Z
    UtcExtended,
}

/// Whether the minutes field should be optional or required in ISO-8601 format.
pub(crate) enum IsoMinutes {
    /// Minutes are always displayed.
    Required,

    /// Minutes are displayed only if they are non-zero.
    Optional,
}

/// Whether the seconds field should be optional or excluded in ISO-8601 format.
pub(crate) enum IsoSeconds {
    /// Seconds are displayed only if they are non-zero.
    Optional,

    /// Seconds are not displayed.
    None,
}

/// Whether a field should be zero-padded in ISO-8601 format.
pub(crate) enum ZeroPadding {
    /// Add zero-padding.
    On,

    /// Do not add zero-padding.
    Off,
}

/// The GMT offset in seconds for a `ZonedDateTime`.
#[derive(Copy, Clone, Debug, Default)]
pub struct GmtOffset(i32);

/// Get the ascii byte of a numeric digit.
/// Only called on digits from in range 0..=9.
const fn ascii_digit(digit: u8) -> u8 {
    digit + b'0'
}

/// Formats a time segment with optional zero padding.
const fn format_segment(n: u8, padding: ZeroPadding) -> TinyStr4 {
    // This section is safe because it operates on a finite set of 0..=60
    // and it ensures that all TinyStr4s are constructed from valid
    // little-endian bytes, which is the required internal representation.
    unsafe {
        if n >= 10 {
            TinyStr4::new_unchecked(u32::from_le_bytes([
                ascii_digit(n / 10),
                ascii_digit(n % 10),
                0,
                0,
            ]))
        } else {
            match padding {
                ZeroPadding::On => {
                    TinyStr4::new_unchecked(u32::from_le_bytes([b'0', ascii_digit(n), 0, 0]))
                }
                ZeroPadding::Off => {
                    TinyStr4::new_unchecked(u32::from_le_bytes([ascii_digit(n), 0, 0, 0]))
                }
            }
        }
    }
}

impl GmtOffset {
    /// Formats the hours as a `TinyStr4` with optional zero-padding.
    pub(crate) fn format_hours(&self, padding: ZeroPadding) -> TinyStr4 {
        format_segment((self.0 / 3600).abs() as u8, padding)
    }

    /// Formats the minutes as a `TinyStr4` with zero-padding.
    pub(crate) fn format_minutes(&self) -> TinyStr4 {
        format_segment((self.0 % 3600 / 60).abs() as u8, ZeroPadding::On)
    }

    /// Formats the seconds as a `TinyStr4` with zero-padding.
    pub(crate) fn format_seconds(&self) -> TinyStr4 {
        format_segment((self.0 % 3600 % 60).abs() as u8, ZeroPadding::On)
    }

    /// Whether the GMT offset is positive.
    pub(crate) fn is_positive(&self) -> bool {
        self.0 >= 0
    }

    /// Whether the GMT offset is zero.
    pub(crate) fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Whether the GMT offset has non-zero minutes.
    pub(crate) fn has_minutes(&self) -> bool {
        self.0 % 3600 / 60 > 0
    }

    /// Whether the GMT offset has non-zero seconds.
    pub(crate) fn has_seconds(&self) -> bool {
        self.0 % 3600 % 60 > 0
    }

    /// Formats a GMT offset in ISO-8601 format.
    pub(crate) fn iso8601_format<'a>(
        self,
        format: IsoFormat,
        minutes: IsoMinutes,
        seconds: IsoSeconds,
    ) -> Cow<'a, str> {
        if self.is_zero() && matches!(format, IsoFormat::UtcBasic | IsoFormat::UtcExtended) {
            return Cow::Owned(String::from("Z"));
        }

        let extended_format = matches!(format, IsoFormat::Extended | IsoFormat::UtcExtended);
        let mut s = String::from(if self.is_positive() { '+' } else { '-' });
        s.push_str(&self.format_hours(ZeroPadding::On));

        match minutes {
            IsoMinutes::Required => {
                extended_format.then(|| s.push(':'));
                s.push_str(&self.format_minutes());
            }
            IsoMinutes::Optional => {
                if self.has_minutes() {
                    extended_format.then(|| s.push(':'));
                    s.push_str(&self.format_minutes());
                }
            }
        }

        if let IsoSeconds::Optional = seconds {
            if self.has_seconds() {
                extended_format.then(|| s.push(':'));
                s.push_str(&self.format_seconds());
            }
        }

        Cow::Owned(s)
    }
}

impl FromStr for GmtOffset {
    type Err = DateTimeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let offset_sign;
        match input.chars().next() {
            Some('+') => offset_sign = 1,
            Some('-') => offset_sign = -1,
            Some('Z') => return Ok(GmtOffset(0)),
            _ => return Err(DateTimeError::MissingTimeZoneOffset),
        };

        let seconds = match input.chars().count() {
            /* ±hh */
            3 => {
                let hour: u8 = input[1..3].parse()?;
                offset_sign * (hour as i32 * 60 * 60)
            }
            /* ±hhmm */
            5 => {
                let hour: u8 = input[1..3].parse()?;
                let minute: u8 = input[3..5].parse()?;
                offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
            }
            /* ±hh:mm */
            6 => {
                let hour: u8 = input[1..3].parse()?;
                let minute: u8 = input[4..6].parse()?;
                offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
            }
            _ => panic!("Invalid time-zone designator"),
        };

        // Valid range is from GMT-12 to GMT+14 in seconds.
        if seconds < -43200 {
            Err(DateTimeError::Underflow {
                field: "GmtOffset",
                min: -43200,
            })
        } else if seconds > 50400 {
            Err(DateTimeError::Overflow {
                field: "GmtOffset",
                max: 50400,
            })
        } else {
            Ok(Self(seconds))
        }
    }
}
