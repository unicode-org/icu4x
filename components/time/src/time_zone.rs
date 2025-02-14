// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::{
    scaffold::IntoOption,
    zone::{TimeZoneVariant, UtcOffset, UtcOffsetCalculator},
    Time, TimeZone,
};
use icu_calendar::{Date, Iso};

mod private {
    pub trait Sealed {}
}

/// Trait encoding a particular data model for time zones.
///
/// <div class="stab unstable">
/// 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this
/// trait, please consider using a type from the implementors listed below.
/// </div>
pub trait TimeZoneModel: private::Sealed {
    /// The zone variant, if required for this time zone model.
    type TimeZoneVariant: IntoOption<TimeZoneVariant> + fmt::Debug + Copy;
    /// The local time, if required for this time zone model.
    type LocalTime: IntoOption<(Date<Iso>, Time)> + fmt::Debug + Copy;
}

/// Time zone data model choices.
pub mod models {
    use super::*;

    /// A time zone containing a time zone ID and optional offset.
    #[derive(Debug, PartialEq, Eq)]
    #[non_exhaustive]
    pub enum Base {}

    impl super::private::Sealed for Base {}
    impl TimeZoneModel for Base {
        type TimeZoneVariant = ();
        type LocalTime = ();
    }

    /// A time zone containing a time zone ID, optional offset, and local time.
    #[derive(Debug, PartialEq, Eq)]
    #[non_exhaustive]
    pub enum AtTime {}

    impl super::private::Sealed for AtTime {}
    impl TimeZoneModel for AtTime {
        type TimeZoneVariant = ();
        type LocalTime = (Date<Iso>, Time);
    }

    /// A time zone containing a time zone ID, optional offset, local time, and zone variant.
    #[derive(Debug, PartialEq, Eq)]
    #[non_exhaustive]
    pub enum Full {}

    impl super::private::Sealed for Full {}
    impl TimeZoneModel for Full {
        type TimeZoneVariant = TimeZoneVariant;
        type LocalTime = (Date<Iso>, Time);
    }
}

/// A utility type that can hold time zone information.
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // these four fields fully cover the needs of UTS 35
pub struct TimeZoneInfo<Model: TimeZoneModel> {
    time_zone_id: TimeZone,
    offset: Option<UtcOffset>,
    local_time: Model::LocalTime,
    zone_variant: Model::TimeZoneVariant,
}

impl<Model: TimeZoneModel> Clone for TimeZoneInfo<Model> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Model: TimeZoneModel> Copy for TimeZoneInfo<Model> {}

impl<Model: TimeZoneModel> TimeZoneInfo<Model> {
    /// The BCP47 time-zone identifier.
    pub fn time_zone_id(self) -> TimeZone {
        self.time_zone_id
    }

    /// The UTC offset, if known.
    ///
    /// This field is not enforced to be consistent with the time zone id.
    pub fn offset(self) -> Option<UtcOffset> {
        self.offset
    }
}

impl<Model> TimeZoneInfo<Model>
where
    Model: TimeZoneModel<LocalTime = (Date<Iso>, Time)>,
{
    /// The time at which to interpret the time zone.
    pub fn local_time(self) -> (Date<Iso>, Time) {
        self.local_time
    }
}

impl<Model> TimeZoneInfo<Model>
where
    Model: TimeZoneModel<TimeZoneVariant = TimeZoneVariant>,
{
    /// The time variant e.g. daylight or standard, if known.
    ///
    /// This field is not enforced to be consistent with the time zone id and offset.
    pub fn zone_variant(self) -> TimeZoneVariant {
        self.zone_variant
    }
}

impl TimeZone {
    /// Associates this [`TimeZone`] with a UTC offset, returning a [`TimeZoneInfo`].
    pub const fn with_offset(self, offset: Option<UtcOffset>) -> TimeZoneInfo<models::Base> {
        TimeZoneInfo {
            offset,
            time_zone_id: self,
            local_time: (),
            zone_variant: (),
        }
    }

    /// Converts this [`TimeZone`] into a [`TimeZoneInfo`] without an offset.
    pub const fn without_offset(self) -> TimeZoneInfo<models::Base> {
        TimeZoneInfo {
            offset: None,
            time_zone_id: self,
            local_time: (),
            zone_variant: (),
        }
    }
}

impl TimeZoneInfo<models::Base> {
    /// Creates a time zone info with no information.
    pub const fn unknown() -> Self {
        TimeZone::unknown().with_offset(None)
    }

    /// Creates a new [`TimeZoneInfo`] for the UTC time zone.
    pub const fn utc() -> Self {
        TimeZone(tinystr::tinystr!(8, "utc")).with_offset(Some(UtcOffset::zero()))
    }

    /// Sets a local time on this time zone.
    pub const fn at_time(self, local_time: (Date<Iso>, Time)) -> TimeZoneInfo<models::AtTime> {
        TimeZoneInfo {
            offset: self.offset,
            time_zone_id: self.time_zone_id,
            local_time,
            zone_variant: (),
        }
    }
}

impl TimeZoneInfo<models::AtTime> {
    /// Sets a zone variant on this time zone.
    pub const fn with_zone_variant(
        self,
        zone_variant: TimeZoneVariant,
    ) -> TimeZoneInfo<models::Full> {
        TimeZoneInfo {
            offset: self.offset,
            time_zone_id: self.time_zone_id,
            local_time: self.local_time,
            zone_variant,
        }
    }

    /// Sets the zone variant by calculating it using a [`UtcOffsetCalculator`].
    ///
    /// If `offset()` is `None`, or if it doesn't match either of the
    /// timezone's standard or daylight offset around `local_time()`,
    /// the variant will be set to [`TimeZoneVariant::Standard`] and the time zone
    /// to [`TimeZone::unknown()`].
    pub fn infer_zone_variant(
        self,
        calculator: &UtcOffsetCalculator,
    ) -> TimeZoneInfo<models::Full> {
        let Some(offset) = self.offset else {
            return TimeZone::unknown()
                .with_offset(self.offset)
                .at_time(self.local_time)
                .with_zone_variant(TimeZoneVariant::Standard);
        };
        let Some(zone_variant) = calculator
            .compute_offsets_from_time_zone(self.time_zone_id, self.local_time)
            .and_then(|os| {
                if os.standard == offset {
                    Some(TimeZoneVariant::Standard)
                } else if os.daylight == Some(offset) {
                    Some(TimeZoneVariant::Daylight)
                } else {
                    None
                }
            })
        else {
            return TimeZone::unknown()
                .with_offset(self.offset)
                .at_time(self.local_time)
                .with_zone_variant(TimeZoneVariant::Standard);
        };
        self.with_zone_variant(zone_variant)
    }

    /// Sets a zone variant from a TZDB `isdst` flag, if it is known that the TZDB was built with
    /// `DATAFORM=rearguard`.
    ///
    /// If it is known that the database was *not* built with `rearguard`, a caller can try to adjust
    /// for the differences. This is a moving target, for example the known differences for 2025a are:
    ///
    /// * `Europe/Dublin` since 1968-10-27
    /// * `Africa/Windhoek` between 1994-03-20 and 2017-10-24
    /// * `Africa/Casablanca` and `Africa/El_Aaiun` since 2018-10-28
    ///
    /// If the TZDB build mode is unknown or variable, use [`Self::infer_zone_variant`].
    pub const fn with_rearguard_isdst(self, isdst: bool) -> TimeZoneInfo<models::Full> {
        self.with_zone_variant(if isdst {
            TimeZoneVariant::Daylight
        } else {
            TimeZoneVariant::Standard
        })
    }
}
