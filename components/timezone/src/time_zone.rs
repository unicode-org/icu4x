// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::{MetaZoneId, TimeZoneBcp47Id};

use crate::metazone::MetaZoneCalculator;
use crate::{GmtOffset, TimeZoneError, ZoneVariant};
use core::str::FromStr;
use icu_calendar::{DateTime, Iso};

/// A utility type that can hold time zone information.
///
/// The GMT offset is used as a final fallback for formatting. The other three fields are used
/// for more human-friendly rendering of the time zone.
///
/// This type does not enforce that the four fields are consistent with each other. If they do not
/// represent a real time zone, unexpected results when formatting may occur.
///
/// # Examples
///
/// ```
/// use icu::timezone::{GmtOffset, CustomTimeZone};
///
/// let tz1 = CustomTimeZone {
///     gmt_offset: Some(GmtOffset::default()),
///     time_zone_id: None,
///     meta_zone_id: None,
///     zone_variant: None,
/// };
///
/// let tz2: CustomTimeZone = "+05:00".parse().expect("Failed to parse a time zone.");
/// ```
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // these four fields fully cover the needs of UTS 35
pub struct CustomTimeZone {
    /// The GMT offset in seconds.
    pub gmt_offset: Option<GmtOffset>,
    /// The IANA time-zone identifier
    pub time_zone_id: Option<TimeZoneBcp47Id>,
    /// The CLDR metazone identifier
    pub meta_zone_id: Option<MetaZoneId>,
    /// The time variant e.g. "daylight" or "standard"
    pub zone_variant: Option<ZoneVariant>,
}

impl CustomTimeZone {
    /// Creates a new [`CustomTimeZone`] with the given GMT offset.
    pub const fn new_with_offset(gmt_offset: GmtOffset) -> Self {
        Self {
            gmt_offset: Some(gmt_offset),
            time_zone_id: None,
            meta_zone_id: None,
            zone_variant: None,
        }
    }

    /// Creates a time zone with no information.
    ///
    /// One or more fields must be specified before this time zone is usable.
    pub const fn new_empty() -> Self {
        Self {
            gmt_offset: None,
            time_zone_id: None,
            meta_zone_id: None,
            zone_variant: None,
        }
    }

    /// Creates a new [`CustomTimeZone`] with the GMT offset set to UTC.
    ///
    /// All other fields are left empty.
    pub const fn utc() -> Self {
        Self {
            gmt_offset: Some(GmtOffset::utc()),
            time_zone_id: None,
            meta_zone_id: None,
            zone_variant: None,
        }
    }

    /// Overwrite the metazone id in MockTimeZone.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::GmtOffset;
    /// use icu::timezone::MetaZoneCalculator;
    /// use icu::timezone::CustomTimeZone;
    /// use icu::timezone::provider::{MetaZoneId, TimeZoneBcp47Id};
    /// use icu_calendar::DateTime;
    /// use icu_locid::locale;
    /// use tinystr::tinystr;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let mzc = MetaZoneCalculator::try_new_with_buffer_provider(&provider).expect("data exists");
    /// let mut tz = CustomTimeZone {
    ///     gmt_offset: Some("+11".parse().expect("Failed to parse a GMT offset.")),
    ///     time_zone_id: Some(TimeZoneBcp47Id(tinystr!(8, "gugum"))),
    ///     meta_zone_id: None,
    ///     zone_variant: None,
    /// };
    /// tz.maybe_calculate_meta_zone(
    ///     &DateTime::new_iso_datetime(1971, 10, 31, 2, 0, 0).unwrap(),
    ///     &mzc,
    /// );
    /// assert_eq!(tz.meta_zone_id, Some(MetaZoneId(tinystr!(4, "guam"))));
    /// ```
    pub fn maybe_calculate_meta_zone(
        &mut self,
        local_datetime: &DateTime<Iso>,
        metazone_calculator: &MetaZoneCalculator,
    ) -> &mut Self {
        if let Some(time_zone_id) = self.time_zone_id {
            self.meta_zone_id =
                metazone_calculator.compute_meta_zone_from_time_zone(time_zone_id, local_datetime);
        }
        self
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
        let gmt_offset = input.parse::<GmtOffset>().ok();
        Ok(Self {
            gmt_offset,
            time_zone_id: None,
            meta_zone_id: None,
            zone_variant: None,
        })
    }
}
