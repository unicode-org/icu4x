// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "compiled_data")]
use crate::TimeZoneIdMapper;
use crate::{
    MetazoneCalculator, MetazoneId, TimeZoneBcp47Id, UtcOffset, ZoneOffsetCalculator, ZoneVariant,
};
use icu_calendar::{DateTime, Iso};
use tinystr::TinyAsciiStr;

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
/// use icu::timezone::CustomTimeZone;
///
/// let tz1 = CustomTimeZone::utc();
///
/// let tz2: CustomTimeZone =
///     "+05:00".parse().expect("Failed to parse a time zone.");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // these four fields fully cover the needs of UTS 35
pub struct CustomTimeZone {
    /// The UTC offset.
    pub offset: Option<UtcOffset>,
    /// The BCP47 time-zone identifier
    pub time_zone_id: TimeZoneBcp47Id,
    /// The CLDR metazone identifier
    ///
    /// This is `Some(None)` if the metazone has been resolved, but the time zone does not
    /// have a metazone.
    pub metazone_id: Option<Option<MetazoneId>>,
    /// The time variant e.g. daylight or standard
    pub zone_variant: Option<ZoneVariant>,
}

impl CustomTimeZone {
    /// Creates a new [`CustomTimeZone`] with the given UTC offset.
    pub const fn new_with_offset(offset: UtcOffset) -> Self {
        Self {
            offset: Some(offset),
            time_zone_id: TimeZoneBcp47Id::unknown(),
            metazone_id: None,
            zone_variant: None,
        }
    }

    /// Creates a new [`CustomTimeZone`] with a given BCP47 time zone identifier.
    pub const fn new_with_bcp47_id(time_zone_id: TimeZoneBcp47Id) -> Self {
        Self {
            offset: None,
            time_zone_id,
            metazone_id: None,
            zone_variant: None,
        }
    }

    /// Creates a time zone with no information.
    ///
    /// One or more fields must be specified before this time zone is usable.
    pub const fn unknown() -> Self {
        Self {
            offset: None,
            time_zone_id: TimeZoneBcp47Id::unknown(),
            metazone_id: Some(None),
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
            offset: Some(UtcOffset::from_eighths_of_hour(offset_eighths_of_hour)),
            time_zone_id: TimeZoneBcp47Id(time_zone_id),
            metazone_id: Some(Some(MetazoneId(metazone_id))),
            zone_variant: Some(ZoneVariant(zone_variant)),
        }
    }

    /// Creates a new [`CustomTimeZone`] for the UTC time zone.
    pub const fn utc() -> Self {
        Self {
            offset: Some(UtcOffset::zero()),
            time_zone_id: TimeZoneBcp47Id(tinystr::tinystr!(8, "utc")),
            metazone_id: Some(Some(MetazoneId(tinystr::tinystr!(4, "utc")))),
            zone_variant: Some(ZoneVariant::standard()),
        }
    }

    /// Parse a [`CustomTimeZone`] from a UTF-8 string representing a UTC offset
    /// or an IANA time zone identifier.
    ///
    /// This is a convenience constructor that uses compiled data. For a custom data provider,
    /// use [`UtcOffset`] or [`TimeZoneIdMapper`] directly.
    ///
    /// To parse from an IXDTF string, use [`CustomZonedDateTime::try_iso_from_str`].
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::CustomTimeZone;
    /// use icu::timezone::UtcOffset;
    ///
    /// let tz0: CustomTimeZone = CustomTimeZone::from_str("Z");
    /// let tz1: CustomTimeZone = CustomTimeZone::from_str("+02");
    /// let tz2: CustomTimeZone = CustomTimeZone::from_str("-0230");
    /// let tz3: CustomTimeZone = CustomTimeZone::from_str("+02:30");
    ///
    /// assert_eq!(tz0.offset.map(UtcOffset::to_seconds), Some(0));
    /// assert_eq!(tz1.offset.map(UtcOffset::to_seconds), Some(7200));
    /// assert_eq!(tz2.offset.map(UtcOffset::to_seconds), Some(-9000));
    /// assert_eq!(tz3.offset.map(UtcOffset::to_seconds), Some(9000));
    /// ```
    ///
    /// [`CustomZonedDateTime::try_iso_from_str`]: crate::CustomZonedDateTime::try_iso_from_str
    #[cfg(feature = "compiled_data")]
    #[inline]
    #[allow(clippy::should_implement_trait)] // does implement trait
    pub fn from_str(s: &str) -> Self {
        Self::from_utf8(s.as_bytes())
    }

    /// See [`Self::from_str`]
    #[cfg(feature = "compiled_data")]
    pub fn from_utf8(code_units: &[u8]) -> Self {
        if let Ok(offset) = UtcOffset::try_from_utf8(code_units) {
            return Self {
                offset: Some(offset),
                time_zone_id: TimeZoneBcp47Id::unknown(),
                metazone_id: None,
                zone_variant: None,
            };
        }
        let mapper = TimeZoneIdMapper::new();
        Self {
            offset: None,
            time_zone_id: mapper.as_borrowed().iana_bytes_to_bcp47(code_units),
            metazone_id: None,
            zone_variant: None,
        }
    }

    /// Infer the metazone ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::timezone::MetazoneId;
    /// use icu::timezone::TimeZoneBcp47Id;
    /// use icu::timezone::CustomTimeZone;
    /// use icu::timezone::MetazoneCalculator;
    /// use tinystr::tinystr;
    ///
    /// let mzc = MetazoneCalculator::new();
    /// let mut tz = CustomTimeZone::new_with_bcp47_id(TimeZoneBcp47Id(tinystr!(8, "gugum")));
    /// tz.maybe_calculate_metazone(
    ///     &mzc,
    ///     &DateTime::try_new_iso_datetime(1971, 10, 31, 2, 0, 0).unwrap(),
    /// );
    /// assert_eq!(tz.metazone_id.unwrap(), Some(MetazoneId(tinystr!(4, "guam"))));
    /// ```
    pub fn maybe_calculate_metazone(
        &mut self,
        metazone_calculator: &MetazoneCalculator,
        local_datetime: &DateTime<Iso>,
    ) -> &mut Self {
        self.metazone_id = Some(
            metazone_calculator.compute_metazone_from_time_zone(self.time_zone_id, local_datetime),
        );
        self
    }

    /// Infer the zone variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::timezone::ZoneVariant;
    /// use icu::timezone::TimeZoneBcp47Id;
    /// use icu::timezone::CustomTimeZone;
    /// use icu::timezone::ZoneOffsetCalculator;
    /// use tinystr::tinystr;
    ///
    /// let zoc = ZoneOffsetCalculator::new();
    /// let mut tz = CustomTimeZone {
    ///     offset: Some("+10".parse().expect("Failed to parse a UTC offset.")),
    ///     time_zone_id: TimeZoneBcp47Id(tinystr!(8, "gugum")),
    ///     metazone_id: None,
    ///     zone_variant: None,
    /// };
    /// tz.maybe_calculate_zone_variant(
    ///     &zoc,
    ///     &DateTime::try_new_iso_datetime(1971, 10, 31, 2, 0, 0).unwrap(),
    /// );
    /// assert_eq!(tz.zone_variant, Some(ZoneVariant::standard()));
    /// ```
    pub fn maybe_calculate_zone_variant(
        &mut self,
        zone_offset_calculator: &ZoneOffsetCalculator,
        local_datetime: &DateTime<Iso>,
    ) -> &mut Self {
        if let Some(offset) = self.offset {
            if let Some((std, dst)) = zone_offset_calculator
                .compute_offsets_from_time_zone(self.time_zone_id, local_datetime)
            {
                if offset == std {
                    self.zone_variant = Some(ZoneVariant::standard());
                } else if Some(offset) == dst {
                    self.zone_variant = Some(ZoneVariant::daylight());
                }
            }
        }
        self
    }
}

#[cfg(feature = "compiled_data")]
impl core::str::FromStr for CustomTimeZone {
    type Err = core::convert::Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str(s))
    }
}
