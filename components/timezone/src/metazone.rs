// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::{MetazoneId, TimeZoneBcp47Id};

use crate::error::TimeZoneError;
use crate::provider::MetazonePeriodV1Marker;
use icu_calendar::DateTime;
use icu_calendar::Iso;
use icu_provider::prelude::*;
use zerovec::ule::AsULE;

/// [`MetazoneCalculator`] uses data from the [data provider] to calculate metazone id.
///
/// [data provider]: icu_provider
#[derive(Debug)]
pub struct MetazoneCalculator {
    pub(super) metazone_period: DataPayload<MetazonePeriodV1Marker>,
}

impl MetazoneCalculator {
    /// Constructor that loads data before calculating metazone id.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::MetazoneCalculator;
    /// use icu_locid::locale;
    ///
    /// let mzc = MetazoneCalculator::try_new_unstable(&icu_testdata::unstable());
    ///
    /// assert!(mzc.is_ok());
    /// ```
    pub fn try_new_unstable<P>(zone_provider: &P) -> Result<Self, TimeZoneError>
    where
        P: DataProvider<MetazonePeriodV1Marker> + ?Sized,
    {
        let metazone_period = zone_provider
            .load(DataRequest {
                locale: Default::default(),
                metadata: Default::default(),
            })?
            .take_payload()?;
        Ok(Self { metazone_period })
    }

    icu_provider::gen_any_buffer_constructors!(locale: skip, options: skip, error: TimeZoneError);

    /// Calculate metazone id from timezone id and local datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::provider::{MetazoneId, TimeZoneBcp47Id};
    /// use icu::timezone::MetazoneCalculator;
    /// use icu_calendar::DateTime;
    /// use icu_locid::locale;
    /// use tinystr::tinystr;
    ///
    /// let mzc = MetazoneCalculator::try_new_unstable(&icu_testdata::unstable())
    ///     .expect("data exists");
    ///
    /// assert_eq!(
    ///     mzc.compute_metazone_from_time_zone(
    ///         TimeZoneBcp47Id(tinystr!(8, "gugum")),
    ///         &DateTime::try_new_iso_datetime(1969, 1, 1, 0, 0, 0).unwrap()
    ///     ),
    ///     None
    /// );
    ///
    /// assert_eq!(
    ///     mzc.compute_metazone_from_time_zone(
    ///         TimeZoneBcp47Id(tinystr!(8, "gugum")),
    ///         &DateTime::try_new_iso_datetime(1970, 1, 1, 0, 0, 0).unwrap()
    ///     ),
    ///     Some(MetazoneId(tinystr!(4, "guam")))
    /// );
    ///
    /// assert_eq!(
    ///     mzc.compute_metazone_from_time_zone(
    ///         TimeZoneBcp47Id(tinystr!(8, "gugum")),
    ///         &DateTime::try_new_iso_datetime(1975, 1, 1, 0, 0, 0).unwrap()
    ///     ),
    ///     Some(MetazoneId(tinystr!(4, "guam")))
    /// );
    ///
    /// assert_eq!(
    ///     mzc.compute_metazone_from_time_zone(
    ///         TimeZoneBcp47Id(tinystr!(8, "gugum")),
    ///         &DateTime::try_new_iso_datetime(2000, 12, 22, 15, 0, 0).unwrap()
    ///     ),
    ///     Some(MetazoneId(tinystr!(4, "cham")))
    /// );
    /// ```
    pub fn compute_metazone_from_time_zone(
        &self,
        time_zone_id: TimeZoneBcp47Id,
        local_datetime: &DateTime<Iso>,
    ) -> Option<MetazoneId> {
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
