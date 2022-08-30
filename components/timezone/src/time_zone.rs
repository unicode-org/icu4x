// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::{MetaZoneId, TimeZoneBcp47Id};

use crate::metazone::MetaZoneCalculator;
use crate::{GmtOffset, TimeZoneError, ZoneVariant};
use core::str::FromStr;
use icu_calendar::{DateTime, Iso};

/// A utility type that can hold time zone information
///
/// # Examples
///
/// ```
/// use icu::timezone::{GmtOffset, CustomTimeZone};
///
/// let tz1 = CustomTimeZone::new(
///     Some(GmtOffset::default()),
///     /* time_zone_id */ None,
///     /* meta_zone_id */ None,
///     /* time_variaint */ None,
/// );
///
/// let tz2: CustomTimeZone = "+05:00".parse().expect("Failed to parse a time zone.");
/// ```
#[derive(Debug, Default)]
#[allow(clippy::exhaustive_structs)] // this type will not add fields (it is largely an example type)
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
    /// Creates a new [`CustomTimeZone`].
    /// A GMT offset is required, as it is used as a final fallback for formatting.
    /// The other arguments optionally allow access to more robust formats.
    pub fn new(
        gmt_offset: Option<GmtOffset>,
        time_zone_id: Option<TimeZoneBcp47Id>,
        meta_zone_id: Option<MetaZoneId>,
        zone_variant: Option<ZoneVariant>,
    ) -> Self {
        Self {
            gmt_offset,
            time_zone_id,
            meta_zone_id,
            zone_variant,
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
    /// let mut tz = CustomTimeZone::new(
    ///     /* gmt_offset */ Some("+11".parse().expect("Failed to parse a GMT offset.")),
    ///     /* time_zone_id */ Some(TimeZoneBcp47Id(tinystr!(8, "gugum"))),
    ///     /* meta_zone_id */ None,
    ///     /* time_variaint */ None,
    /// );
    /// tz.maybe_set_meta_zone(
    ///     &DateTime::new_iso_datetime(1971, 10, 31, 2, 0, 0).unwrap(),
    ///     &mzc,
    /// );
    /// assert_eq!(tz.meta_zone_id, Some(MetaZoneId(tinystr!(4, "guam"))));
    /// ```
    pub fn maybe_set_meta_zone(
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
