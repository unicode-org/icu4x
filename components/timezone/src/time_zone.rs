// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::InvalidOffsetError;
use crate::provider::{MetazoneId, TimeZoneBcp47Id};

use crate::metazone::MetazoneCalculator;
use crate::{GmtOffset, ZoneVariant};
use core::str::FromStr;
use icu_calendar::{DateTime, Iso};
use tinystr::tinystr;

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
/// use icu::timezone::{CustomTimeZone, GmtOffset};
///
/// let tz1 = CustomTimeZone {
///     gmt_offset: Some(GmtOffset::default()),
///     time_zone_id: None,
///     metazone_id: None,
///     zone_variant: None,
/// };
///
/// let tz2: CustomTimeZone =
///     "+05:00".parse().expect("Failed to parse a time zone.");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // these four fields fully cover the needs of UTS 35
pub struct CustomTimeZone {
    /// The GMT offset in seconds.
    pub gmt_offset: Option<GmtOffset>,
    /// The BCP47 time-zone identifier
    pub time_zone_id: Option<TimeZoneBcp47Id>,
    /// The CLDR metazone identifier
    pub metazone_id: Option<MetazoneId>,
    /// The time variant e.g. daylight or standard
    pub zone_variant: Option<ZoneVariant>,
}

impl CustomTimeZone {
    /// Creates a new [`CustomTimeZone`] with the given GMT offset.
    pub const fn new_with_offset(gmt_offset: GmtOffset) -> Self {
        Self {
            gmt_offset: Some(gmt_offset),
            time_zone_id: None,
            metazone_id: None,
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
            metazone_id: None,
            zone_variant: None,
        }
    }

    /// Creates a new [`CustomTimeZone`] with the GMT offset set to 0.
    ///
    /// All other fields are left empty.
    pub const fn utc() -> Self {
        Self {
            gmt_offset: Some(GmtOffset::utc()),
            time_zone_id: None,
            metazone_id: None,
            zone_variant: None,
        }
    }

    /// Creates a new [`CustomTimeZone`] representing Greenwich Mean Time
    /// (London Time as observed in the winter).
    ///
    /// This is the same local time as UTC, but there are more
    /// localizations available.
    pub const fn gmt() -> Self {
        Self {
            gmt_offset: Some(GmtOffset::utc()),
            time_zone_id: Some(TimeZoneBcp47Id(tinystr!(8, "gblon"))),
            metazone_id: Some(MetazoneId(tinystr!(4, "mgmt"))),
            zone_variant: Some(ZoneVariant::standard()),
        }
    }

    /// Parse a [`CustomTimeZone`] from a UTF-8 string representing a GMT Offset. See also [`GmtOffset`].
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::CustomTimeZone;
    /// use icu::timezone::GmtOffset;
    ///
    /// let tz0: CustomTimeZone = CustomTimeZone::try_from_str("Z")
    ///     .expect("Failed to parse a time zone");
    /// let tz1: CustomTimeZone = CustomTimeZone::try_from_str("+02")
    ///     .expect("Failed to parse a time zone");
    /// let tz2: CustomTimeZone = CustomTimeZone::try_from_str("-0230")
    ///     .expect("Failed to parse a time zone");
    /// let tz3: CustomTimeZone = CustomTimeZone::try_from_str("+02:30")
    ///     .expect("Failed to parse a time zone");
    ///
    /// assert_eq!(tz0.gmt_offset.map(GmtOffset::offset_seconds), Some(0));
    /// assert_eq!(tz1.gmt_offset.map(GmtOffset::offset_seconds), Some(7200));
    /// assert_eq!(tz2.gmt_offset.map(GmtOffset::offset_seconds), Some(-9000));
    /// assert_eq!(tz3.gmt_offset.map(GmtOffset::offset_seconds), Some(9000));
    /// ```
    #[inline]
    pub fn try_from_str(s: &str) -> Result<Self, InvalidOffsetError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// See [`Self::try_from_str`]
    pub fn try_from_utf8(code_units: &[u8]) -> Result<Self, InvalidOffsetError> {
        let gmt_offset = GmtOffset::try_from_utf8(code_units)?;
        Ok(Self {
            gmt_offset: Some(gmt_offset),
            time_zone_id: None,
            metazone_id: None,
            zone_variant: None,
        })
    }

    /// Overwrite the metazone id in MockTimeZone.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::timezone::provider::{MetazoneId, TimeZoneBcp47Id};
    /// use icu::timezone::CustomTimeZone;
    /// use icu::timezone::MetazoneCalculator;
    /// use tinystr::tinystr;
    ///
    /// let mzc = MetazoneCalculator::new();
    /// let mut tz = CustomTimeZone {
    ///     gmt_offset: Some("+11".parse().expect("Failed to parse a GMT offset.")),
    ///     time_zone_id: Some(TimeZoneBcp47Id(tinystr!(8, "gugum"))),
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// };
    /// tz.maybe_calculate_metazone(
    ///     &mzc,
    ///     &DateTime::try_new_iso_datetime(1971, 10, 31, 2, 0, 0).unwrap(),
    /// );
    /// assert_eq!(tz.metazone_id, Some(MetazoneId(tinystr!(4, "guam"))));
    /// ```
    pub fn maybe_calculate_metazone(
        &mut self,
        metazone_calculator: &MetazoneCalculator,
        local_datetime: &DateTime<Iso>,
    ) -> &mut Self {
        if let Some(time_zone_id) = self.time_zone_id {
            self.metazone_id =
                metazone_calculator.compute_metazone_from_time_zone(time_zone_id, local_datetime);
        }
        self
    }
}

impl FromStr for CustomTimeZone {
    type Err = InvalidOffsetError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}
