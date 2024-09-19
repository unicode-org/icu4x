// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::{MetazoneId, TimeZoneBcp47Id, ZoneOffsetPeriodV1Marker};

use crate::provider::MetazonePeriodV1Marker;
use crate::UtcOffset;
use icu_calendar::DateTime;
use icu_calendar::Iso;
use icu_provider::prelude::*;
use zerovec::ule::AsULE;

/// [`TimeZoneCalculator`] uses data from the [data provider] to compute [`ResolvedTimeZone`](crate::ResolvedTimeZone)s.
///
/// [data provider]: icu_provider
#[derive(Debug)]
pub struct TimeZoneCalculator {
    pub(super) metazone_period: DataPayload<MetazonePeriodV1Marker>,
    pub(super) offset_period: DataPayload<ZoneOffsetPeriodV1Marker>,
}

#[cfg(feature = "compiled_data")]
impl Default for TimeZoneCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeZoneCalculator {
    /// Constructs a `TimeZoneCalculator` using compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub const fn new() -> Self {
        TimeZoneCalculator {
            metazone_period: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_METAZONE_PERIOD_V1_MARKER,
            ),
            offset_period: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_ZONE_OFFSET_PERIOD_V1_MARKER,
            ),
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable(
        provider: &(impl DataProvider<MetazonePeriodV1Marker>
              + DataProvider<ZoneOffsetPeriodV1Marker>
              + ?Sized),
    ) -> Result<Self, DataError> {
        let metazone_period = provider.load(Default::default())?.payload;
        let offset_period = provider.load(Default::default())?.payload;
        Ok(Self {
            metazone_period,
            offset_period,
        })
    }

    /// Calculate metazone id from timezone id and local datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::timezone::provider::{MetazoneId, TimeZoneBcp47Id};
    /// use icu::timezone::TimeZoneCalculator;
    /// use tinystr::tinystr;
    ///
    /// let mzc = TimeZoneCalculator::new();
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

    /// Calculate zone offsets from timezone and local datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::DateTime;
    /// use icu::timezone::TimeZoneBcp47Id;
    /// use icu::timezone::TimeZoneCalculator;
    /// use icu::timezone::UtcOffset;
    /// use tinystr::tinystr;
    ///
    /// let zoc = TimeZoneCalculator::new();
    ///
    /// // America/Denver observes DST
    /// assert_eq!(
    ///     zoc.compute_offsets_from_time_zone(
    ///         TimeZoneBcp47Id(tinystr!(8, "usden")),
    ///         &DateTime::try_new_iso_datetime(2024, 1, 1, 0, 0, 0).unwrap()
    ///     ),
    ///     Some((UtcOffset::try_from_offset_seconds(-7 * 3600).unwrap(), Some(UtcOffset::try_from_offset_seconds(-6 * 3600).unwrap())))
    /// );
    ///
    /// // America/Phoenix does not
    /// assert_eq!(
    ///     zoc.compute_offsets_from_time_zone(
    ///         TimeZoneBcp47Id(tinystr!(8, "usphx")),
    ///         &DateTime::try_new_iso_datetime(2024, 1, 1, 0, 0, 0).unwrap()
    ///     ),
    ///     Some((UtcOffset::try_from_offset_seconds(-7 * 3600).unwrap(), None))
    /// );
    /// ```
    pub fn compute_offsets_from_time_zone(
        &self,
        time_zone_id: TimeZoneBcp47Id,
        local_datetime: &DateTime<Iso>,
    ) -> Option<(UtcOffset, Option<UtcOffset>)> {
        use zerovec::ule::AsULE;
        match self.offset_period.get().0.get0(&time_zone_id) {
            Some(cursor) => {
                let mut offsets = None;
                let minutes_since_local_unix_epoch =
                    local_datetime.minutes_since_local_unix_epoch();
                for (minutes, id) in cursor.iter1_copied().rev() {
                    if minutes_since_local_unix_epoch <= i32::from_unaligned(*minutes) {
                        offsets = Some(id);
                    } else {
                        break;
                    }
                }
                let offsets = offsets?;
                Some((
                    UtcOffset::from_offset_eighths_of_hour(offsets.0),
                    (offsets.1 != 0).then_some(UtcOffset::from_offset_eighths_of_hour(
                        offsets.0 + offsets.1,
                    )),
                ))
            }
            None => None,
        }
    }

    // resolve_at impl in `time_zone.rs`
    // resolve impl in `zoned_datetime.rs`
}
