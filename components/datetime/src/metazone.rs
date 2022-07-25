// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::time_zones::{MetaZoneId, TimeZoneBcp47Id};

use crate::error::DateTimeFormatterError;
use crate::provider::time_zones::MetaZonePeriodV1Marker;
use icu_calendar::DateTime;
use icu_calendar::Iso;
use icu_locid::Locale;
use icu_provider::prelude::*;
use zerovec::ule::AsULE;

/// [`MetaZoneCalculator`] uses data from the [data provider], the selected [`Locale`] to calculate
/// metazone id.
pub struct MetaZoneCalculator {
    pub(super) metazone_period: DataPayload<MetaZonePeriodV1Marker>,
}

impl MetaZoneCalculator {
    /// Constructor that loads data before calculating metazone id.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu::datetime::metazone::MetaZoneCalculator;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let mzc = MetaZoneCalculator::new(
    ///     locale!("en"),
    ///     &provider,
    /// );
    ///
    /// assert!(mzc.is_ok());
    /// ```
    pub fn new<L, ZP>(locale: L, zone_provider: &ZP) -> Result<Self, DateTimeFormatterError>
    where
        L: Into<Locale>,
        ZP: DataProvider<MetaZonePeriodV1Marker> + ?Sized,
    {
        let locale = locale.into();
        let metazone_period = zone_provider
            .load(DataRequest {
                locale: &DataLocale::from(locale),
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self { metazone_period })
    }

    /// Calculate metazone id from timezone id and local datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::metazone::MetaZoneCalculator;
    /// use icu_calendar::DateTime;
    /// use icu_datetime::provider::time_zones::{MetaZoneId, TimeZoneBcp47Id};
    /// use icu_locid::locale;
    /// use tinystr::tinystr;
    ///
    /// let provider = icu_testdata::get_provider();
    /// let mzc = MetaZoneCalculator::new(locale!("en"), &provider).expect("data exists");
    ///
    /// assert_eq!(
    ///     mzc.compute_metazone_from_timezone(
    ///         TimeZoneBcp47Id(tinystr!(8, "gugum")),
    ///         DateTime::new_iso_datetime(1969, 1, 1, 0, 0, 0).unwrap()
    ///     ),
    ///     None
    /// );
    ///
    /// assert_eq!(
    ///     mzc.compute_metazone_from_timezone(
    ///         TimeZoneBcp47Id(tinystr!(8, "gugum")),
    ///         DateTime::new_iso_datetime(1970, 1, 1, 0, 0, 0).unwrap()
    ///     ),
    ///     Some(MetaZoneId(tinystr!(4, "guam")))
    /// );
    ///
    /// assert_eq!(
    ///     mzc.compute_metazone_from_timezone(
    ///         TimeZoneBcp47Id(tinystr!(8, "gugum")),
    ///         DateTime::new_iso_datetime(1975, 1, 1, 0, 0, 0).unwrap()
    ///     ),
    ///     Some(MetaZoneId(tinystr!(4, "guam")))
    /// );
    ///
    /// assert_eq!(
    ///     mzc.compute_metazone_from_timezone(
    ///         TimeZoneBcp47Id(tinystr!(8, "gugum")),
    ///         DateTime::new_iso_datetime(2000, 12, 22, 15, 0, 0).unwrap()
    ///     ),
    ///     Some(MetaZoneId(tinystr!(4, "cham")))
    /// );
    /// ```
    pub fn compute_metazone_from_timezone(
        &self,
        time_zone_id: TimeZoneBcp47Id,
        local_datetime: &DateTime<Iso>,
    ) -> Option<MetaZoneId> {
        match self.metazone_period.get().0.get0(&time_zone_id) {
            Some(cursor) => {
                let mut metazone_id = None;
                let minutes_since_local_unix_epoch =
                    local_datetime.minutes_since_local_unix_epoch();
                for (minutes, id) in cursor.iter1() {
                    if minutes_since_local_unix_epoch >= i32::from_unaligned(*minutes) {
                        metazone_id = id.get()
                    } else {
                        break;
                    }
                }
                metazone_id
            }
            None => None,
        }
    }
}
