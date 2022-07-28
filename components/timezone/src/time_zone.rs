// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::{MetaZoneId, TimeZoneBcp47Id};
use tinystr::TinyStr8;

use crate::{GmtOffset, TimeZoneError};
use core::str::FromStr;

/// A utility type that can hold time zone information
///
/// # Examples
///
/// ```
/// use icu::timezone::{GmtOffset, CustomTimeZone};
///
/// let tz1 = CustomTimeZone::new(
///     GmtOffset::default(),
///     /* time_zone_id */ None,
///     /* metazone_id */ None,
///     /* time_variaint */ None,
/// );
///
/// let tz2: CustomTimeZone = "+05:00".parse().expect("Failed to parse a time zone.");
/// ```
#[derive(Debug, Default)]
#[allow(clippy::exhaustive_structs)] // this type will not add fields (it is largely an example type)
pub struct CustomTimeZone {
    /// The GMT offset in seconds.
    pub gmt_offset: GmtOffset,
    /// The IANA time-zone identifier
    pub time_zone_id: Option<TimeZoneBcp47Id>,
    /// The CLDR metazone identifier
    pub metazone_id: Option<MetaZoneId>,
    /// The time variant e.g. "daylight" or "standard"
    pub time_variant: Option<TinyStr8>,
}

impl CustomTimeZone {
    /// Creates a new [`CustomTimeZone`].
    /// A GMT offset is required, as it is used as a final fallback for formatting.
    /// The other arguments optionally allow access to more robust formats.
    pub const fn new(
        gmt_offset: GmtOffset,
        time_zone_id: Option<TimeZoneBcp47Id>,
        metazone_id: Option<MetaZoneId>,
        time_variant: Option<TinyStr8>,
    ) -> Self {
        Self {
            gmt_offset,
            time_zone_id,
            metazone_id,
            time_variant,
        }
    }
}

impl FromStr for CustomTimeZone {
    type Err = TimeZoneError;

    /// Parse a [`CustomTimeZone`] from a string.
    ///
    /// This utility is for easily creating time zones, not a complete robust solution.
    ///
    /// The offset must range from GMT-12 to GMT+14.
    /// The string must be an ISO-8601 time zone designator:
    /// e.g. Z
    /// e.g. +05
    /// e.g. +0500
    /// e.g. +05:00
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::CustomTimeZone;
    ///
    /// let tz0: CustomTimeZone = "Z".parse().expect("Failed to parse a time zone.");
    /// let tz1: CustomTimeZone = "+02".parse().expect("Failed to parse a time zone.");
    /// let tz2: CustomTimeZone = "-0230".parse().expect("Failed to parse a time zone.");
    /// let tz3: CustomTimeZone = "+02:30".parse().expect("Failed to parse a time zone.");
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let gmt_offset = GmtOffset::from_str(input)?;
        Ok(Self {
            gmt_offset,
            time_zone_id: None,
            metazone_id: None,
            time_variant: None,
        })
    }
}
