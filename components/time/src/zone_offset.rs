// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::{MinutesSinceEpoch, ZoneOffsetPeriodV1};
use crate::{zone::UtcOffset, Time, TimeZone};
use icu_calendar::Date;
use icu_calendar::Iso;
use icu_provider::prelude::*;

/// [`UtcOffsetCalculator`] uses data from the [data provider] to calculate time zone offsets.
///
/// [data provider]: icu_provider
#[derive(Debug)]
pub struct UtcOffsetCalculator {
    pub(super) offset_period: DataPayload<ZoneOffsetPeriodV1>,
}

#[cfg(feature = "compiled_data")]
impl Default for UtcOffsetCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl UtcOffsetCalculator {
    /// Constructs a `UtcOffsetCalculator` using compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub const fn new() -> Self {
        UtcOffsetCalculator {
            offset_period: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_ZONE_OFFSET_PERIOD_V1,
            ),
        }
    }

    icu_provider::gen_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
                        try_new_with_buffer_provider,
            try_new_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable(
        provider: &(impl DataProvider<ZoneOffsetPeriodV1> + ?Sized),
    ) -> Result<Self, DataError> {
        let metazone_period = provider.load(Default::default())?.payload;
        Ok(Self {
            offset_period: metazone_period,
        })
    }

    /// Calculate zone offsets from timezone and local datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::time::Time;
    /// use icu::time::TimeZone;
    /// use icu::time::zone::UtcOffset;
    /// use icu::time::zone::UtcOffsetCalculator;
    /// use tinystr::tinystr;
    ///
    /// let zoc = UtcOffsetCalculator::new();
    ///
    /// // America/Denver observes DST
    /// let offsets = zoc
    ///     .compute_offsets_from_time_zone(
    ///         TimeZone(tinystr!(8, "usden")),
    ///         (Date::try_new_iso(2024, 1, 1).unwrap(), Time::midnight()),
    ///     )
    ///     .unwrap();
    /// assert_eq!(
    ///     offsets.standard,
    ///     UtcOffset::try_from_seconds(-7 * 3600).unwrap()
    /// );
    /// assert_eq!(
    ///     offsets.daylight,
    ///     Some(UtcOffset::try_from_seconds(-6 * 3600).unwrap())
    /// );
    ///
    /// // America/Phoenix does not
    /// let offsets = zoc
    ///     .compute_offsets_from_time_zone(
    ///         TimeZone(tinystr!(8, "usphx")),
    ///         (Date::try_new_iso(2024, 1, 1).unwrap(), Time::midnight()),
    ///     )
    ///     .unwrap();
    /// assert_eq!(
    ///     offsets.standard,
    ///     UtcOffset::try_from_seconds(-7 * 3600).unwrap()
    /// );
    /// assert_eq!(offsets.daylight, None);
    /// ```
    pub fn compute_offsets_from_time_zone(
        &self,
        time_zone_id: TimeZone,
        dt: (Date<Iso>, Time),
    ) -> Option<UtcOffsets> {
        use zerovec::ule::AsULE;
        match self.offset_period.get().0.get0(&time_zone_id) {
            Some(cursor) => {
                let mut offsets = None;
                let minutes_since_epoch_walltime = MinutesSinceEpoch::from(dt);
                for (minutes, id) in cursor.iter1_copied() {
                    if minutes_since_epoch_walltime >= MinutesSinceEpoch::from_unaligned(*minutes) {
                        offsets = Some(id);
                    } else {
                        break;
                    }
                }
                let offsets = offsets?;
                Some(UtcOffsets {
                    standard: UtcOffset::from_eighths_of_hour(offsets.0),
                    daylight: (offsets.1 != 0)
                        .then_some(UtcOffset::from_eighths_of_hour(offsets.0 + offsets.1)),
                })
            }
            None => None,
        }
    }
}

/// Represents the different offsets in use for a time zone
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct UtcOffsets {
    /// The standard offset.
    pub standard: UtcOffset,
    /// The daylight-saving offset, if used.
    pub daylight: Option<UtcOffset>,
}
