// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    MetazoneCalculator, MetazoneId, TimeZoneBcp47Id, UtcOffset, ZoneOffsetCalculator, ZoneVariant,
};
#[cfg(feature = "compiled_data")]
use crate::{TimeZoneIdMapper, UnknownTimeZoneError};
use icu_calendar::{DateTime, Iso};

/// A utility type that can hold time zone information.
///
/// The UTC offset is used as a final fallback for formatting. The other three fields are used
/// for more human-friendly rendering of the time zone.
///
/// This type does not enforce that the four fields are consistent with each other. If they do not
/// represent a real time zone, unexpected results when formatting may occur.
///
/// # Examples
///
/// ```
/// use icu::timezone::{TimeZone, UtcOffset};
///
/// let tz1 = TimeZone::new_with_offset(UtcOffset::default());
///
/// let tz2: TimeZone =
///     "+05:00".parse().expect("Failed to parse a time zone.");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TimeZone {
    offset: Option<UtcOffset>,
    bcp47_id: Option<TimeZoneBcp47Id>,
}

impl TimeZone {
    /// Creates a new [`TimeZone`] with the given UTC offset and BCP47 time zone identifier.
    pub const fn new(offset: UtcOffset, time_zone_id: TimeZoneBcp47Id) -> Self {
        Self {
            offset: Some(offset),
            bcp47_id: Some(time_zone_id),
        }
    }

    /// Creates a new [`TimeZone`] with the given UTC offset.
    pub const fn new_with_offset(offset: UtcOffset) -> Self {
        Self {
            offset: Some(offset),
            bcp47_id: None,
        }
    }

    /// Creates a new [`TimeZone`] with a given BCP47 time zone identifier.
    pub const fn new_with_bcp47_id(time_zone_id: TimeZoneBcp47Id) -> Self {
        Self {
            offset: None,
            bcp47_id: Some(time_zone_id),
        }
    }

    /// Creates a new [`TimeZone`] for the UTC time zone.
    pub const fn utc() -> Self {
        Self {
            offset: Some(UtcOffset::zero()),
            bcp47_id: Some(TimeZoneBcp47Id(tinystr::tinystr!(8, "utc"))),
        }
    }

    /// The UTC offset.
    pub const fn offset(&self) -> Option<UtcOffset> {
        self.offset
    }

    /// The BCP47 time-zone identifier
    pub const fn bcp47_id(&self) -> Option<TimeZoneBcp47Id> {
        self.bcp47_id
    }

    /// Parse a [`TimeZone`] from a UTF-8 string representing a UTC offset
    /// or an IANA time zone identifier.
    ///
    /// This is a convenience constructor that uses compiled data. For a custom data provider,
    /// use [`UtcOffset`] or [`TimeZoneIdMapper`] directly.
    ///
    /// To parse from an IXDTF string, use [`ZonedDateTime::try_iso_from_str`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::TimeZone;
    /// use icu::timezone::UtcOffset;
    ///
    /// let tz0: TimeZone = TimeZone::try_from_str("Z")
    ///     .expect("Failed to parse a time zone");
    /// let tz1: TimeZone = TimeZone::try_from_str("+02")
    ///     .expect("Failed to parse a time zone");
    /// let tz2: TimeZone = TimeZone::try_from_str("-0230")
    ///     .expect("Failed to parse a time zone");
    /// let tz3: TimeZone = TimeZone::try_from_str("+02:30")
    ///     .expect("Failed to parse a time zone");
    ///
    /// assert_eq!(tz0.offset().map(UtcOffset::offset_seconds), Some(0));
    /// assert_eq!(tz1.offset().map(UtcOffset::offset_seconds), Some(7200));
    /// assert_eq!(tz2.offset().map(UtcOffset::offset_seconds), Some(-9000));
    /// assert_eq!(tz3.offset().map(UtcOffset::offset_seconds), Some(9000));
    /// ```
    ///
    /// [`ZonedDateTime::try_iso_from_str`]: crate::ZonedDateTime::try_iso_from_str
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub fn try_from_str(s: &str) -> Result<Self, UnknownTimeZoneError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// See [`Self::try_from_str`]
    #[cfg(feature = "compiled_data")]
    pub fn try_from_utf8(code_units: &[u8]) -> Result<Self, UnknownTimeZoneError> {
        if let Ok(offset) = UtcOffset::try_from_utf8(code_units) {
            return Ok(Self {
                offset: Some(offset),
                bcp47_id: None,
            });
        }
        let mapper = TimeZoneIdMapper::new();
        if let Some(bcp47_id) = mapper.as_borrowed().iana_bytes_to_bcp47(code_units) {
            return Ok(Self {
                offset: None,
                bcp47_id: Some(bcp47_id),
            });
        }
        Err(UnknownTimeZoneError)
    }

    /// Infer the metazone ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::timezone::MetazoneId;
    /// use icu::timezone::TimeZoneBcp47Id;
    /// use icu::timezone::TimeZone;
    /// use icu::timezone::MetazoneCalculator;
    /// use icu::timezone::ZoneVariant;
    /// use icu::timezone::ZoneOffsetCalculator;
    /// use tinystr::tinystr;
    ///
    /// let mut tz = TimeZone::new("+11".parse().expect("Failed to parse a UTC offset."), TimeZoneBcp47Id(tinystr!(8, "gugum")));
    ///
    /// let ftz = tz.to_formattable_at(
    ///     &DateTime::try_new_iso_datetime(1971, 10, 31, 2, 0, 0).unwrap(),
    /// );
    ///
    /// assert_eq!(ftz.metazone_id(), Some(MetazoneId(tinystr!(4, "guam"))));
    /// assert_eq!(ftz.zone_variant(), Some(ZoneVariant::daylight()));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub fn into_formattable_at(self, local_datetime: &DateTime<Iso>) -> FormattableTimeZone {
        MetazoneCalculator::new().compute(self, &ZoneOffsetCalculator::new(), local_datetime)
    }
}

impl MetazoneCalculator {
    /// Converts a [`TimeZone`] to a [`FormattableTimeZone`] at the given datetime.
    pub fn compute(
        &self,
        timezone: TimeZone,
        zoc: &ZoneOffsetCalculator,
        local_datetime: &DateTime<Iso>,
    ) -> FormattableTimeZone {
        FormattableTimeZone {
            timezone,
            metazone_id: timezone
                .bcp47_id
                .and_then(|id| self.compute_metazone_from_time_zone(id, local_datetime)),
            zone_variant: timezone.bcp47_id.and_then(|id| {
                timezone.offset.and_then(|offset| {
                    zoc.compute_offsets_from_time_zone(id, local_datetime)
                        .and_then(|(std, dst)| {
                            if offset == std {
                                Some(ZoneVariant::standard())
                            } else if Some(offset) == dst {
                                Some(ZoneVariant::daylight())
                            } else {
                                None
                            }
                        })
                })
            }),
        }
    }
}

#[cfg(feature = "compiled_data")]
impl core::str::FromStr for TimeZone {
    type Err = UnknownTimeZoneError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

/// [`TimeZone`] annotated with additional formatting information.
///
/// This is obtained from [`TimeZone::to_formattable_at`].
#[derive(Debug, Copy, Clone)]
pub struct FormattableTimeZone {
    timezone: TimeZone,
    metazone_id: Option<MetazoneId>,
    zone_variant: Option<ZoneVariant>,
}

impl FormattableTimeZone {
    /// Creates a new [`FormattableTimeZone`] for the UTC time zone.
    pub fn utc() -> Self {
        Self {
            timezone: TimeZone::utc(),
            metazone_id: Some(MetazoneId(tinystr::tinystr!(4, "utc"))),
            zone_variant: Some(ZoneVariant::standard()),
        }
    }

    /// The UTC offset.
    pub fn offset(&self) -> Option<UtcOffset> {
        self.timezone.offset
    }

    /// The BCP47 time-zone identifier
    pub fn bcp47_id(&self) -> Option<TimeZoneBcp47Id> {
        self.timezone.bcp47_id
    }

    /// The CLDR metazone identifier
    pub fn metazone_id(&self) -> Option<MetazoneId> {
        self.metazone_id
    }

    /// The CLDR metazone time variant e.g. daylight or standard
    pub fn zone_variant(&self) -> Option<ZoneVariant> {
        self.zone_variant
    }
}
