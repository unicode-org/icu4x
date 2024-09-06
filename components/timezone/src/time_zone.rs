// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::{MetazoneId, TimeZoneBcp47Id};

use crate::metazone::MetazoneCalculator;
use crate::{GmtOffset, ZoneVariant};
#[cfg(feature = "compiled_data")]
use crate::{TimeZoneIdMapper, UnknownTimeZoneError};
use icu_calendar::{DateTime, Iso};
use tinystr::TinyAsciiStr;

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

    /// Creates a new [`CustomTimeZone`] with a given BCP47 time zone identifier.
    pub const fn new_with_bcp47_id(time_zone_id: TimeZoneBcp47Id) -> Self {
        Self {
            gmt_offset: None,
            time_zone_id: Some(time_zone_id),
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

    /// Creates a time zone infallibly from raw parts.
    pub const fn from_parts(
        offset_eighths_of_hour: i8,
        time_zone_id: TinyAsciiStr<8>,
        metazone_id: TinyAsciiStr<4>,
        zone_variant: TinyAsciiStr<2>,
    ) -> Self {
        Self {
            gmt_offset: Some(GmtOffset::from_offset_eighths_of_hour(
                offset_eighths_of_hour,
            )),
            time_zone_id: Some(TimeZoneBcp47Id(time_zone_id)),
            metazone_id: Some(MetazoneId(metazone_id)),
            zone_variant: Some(ZoneVariant(zone_variant)),
        }
    }

    /// Creates a new [`CustomTimeZone`] with the GMT offset set to UTC.
    ///
    /// All other fields are left empty.
    pub const fn utc() -> Self {
        Self {
            gmt_offset: Some(GmtOffset::zero()),
            time_zone_id: None,
            metazone_id: None,
            zone_variant: None,
        }
    }

    /// Parse a [`CustomTimeZone`] from a UTF-8 string representing a GMT Offset
    /// or an IANA time zone identifier.
    ///
    /// This is a convenience constructor that uses compiled data. For a custom data provider,
    /// use [`GmtOffset`] or [`TimeZoneIdMapper`] directly.
    ///
    /// To parse from an IXDTF string, use [`CustomZonedDateTime::try_iso_from_str`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
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
    ///
    /// [`CustomZonedDateTime::try_iso_from_str`]: crate::CustomZonedDateTime::try_iso_from_str
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub fn try_from_str(s: &str) -> Result<Self, UnknownTimeZoneError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// See [`Self::try_from_str`]
    #[cfg(feature = "compiled_data")]
    pub fn try_from_utf8(code_units: &[u8]) -> Result<Self, UnknownTimeZoneError> {
        if let Ok(gmt_offset) = GmtOffset::try_from_utf8(code_units) {
            return Ok(Self {
                gmt_offset: Some(gmt_offset),
                time_zone_id: None,
                metazone_id: None,
                zone_variant: None,
            });
        }
        let mapper = TimeZoneIdMapper::new();
        if let Some(bcp47_id) = mapper.as_borrowed().iana_bytes_to_bcp47(code_units) {
            return Ok(Self {
                gmt_offset: None,
                time_zone_id: Some(bcp47_id),
                metazone_id: None,
                zone_variant: None,
            });
        }
        Err(UnknownTimeZoneError)
    }

    /// Overwrite the metazone ID.
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

#[cfg(feature = "compiled_data")]
impl core::str::FromStr for CustomTimeZone {
    type Err = UnknownTimeZoneError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}
