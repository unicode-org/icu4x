// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::{scaffold::IntoOption, TimeZoneBcp47Id, UtcOffset, ZoneVariant};
use icu_calendar::{Date, Iso, Time};

// TODO: Make the trait sealed
/// Trait encoding a particular data model for time zones.
pub trait TimeZoneModel {
    /// The zone variant, if required for this time zone model.
    type ZoneVariant: IntoOption<ZoneVariant> + fmt::Debug + Copy;
    /// The local time, if required for this time zone model.
    type LocalTime: IntoOption<(Date<Iso>, Time)> + fmt::Debug + Copy;
}

/// Time zone data model choices.
pub mod models {
    use super::*;

    /// A time zone containing a time zone ID and optional offset.
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Base {}

    impl TimeZoneModel for Base {
        type ZoneVariant = ();
        type LocalTime = ();
    }

    /// A time zone containing a time zone ID, optional offset, and local time.
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum AtTime {}

    impl TimeZoneModel for AtTime {
        type ZoneVariant = ();
        type LocalTime = (Date<Iso>, Time);
    }

    /// A time zone containing a time zone ID, optional offset, local time, and zone variant.
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Full {}

    impl TimeZoneModel for Full {
        type ZoneVariant = ZoneVariant;
        type LocalTime = (Date<Iso>, Time);
    }
}

// TODO: Make the fields private

/// A utility type that can hold time zone information.
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // these four fields fully cover the needs of UTS 35
pub struct TimeZoneInfo<Model: TimeZoneModel> {
    /// The BCP47 time-zone identifier.
    pub time_zone_id: TimeZoneBcp47Id,
    /// The UTC offset, if known.
    ///
    /// This field is not enforced to be consistent with the time zone id.
    pub offset: Option<UtcOffset>,
    /// The time variant e.g. daylight or standard, if known.
    pub zone_variant: Model::ZoneVariant,
    /// The time at which to interpret the time zone.
    ///
    /// This can be set in order to get correct historical time zone names.
    /// If it's not set, the most recent data for the time zone will be used.
    pub local_time: Model::LocalTime,
}

impl<Model: TimeZoneModel> Clone for TimeZoneInfo<Model> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Model: TimeZoneModel> Copy for TimeZoneInfo<Model> {}

impl UtcOffset {
    /// Associates this [`UtcOffset`] with a time zone ID, returning a [`TimeZoneInfo`].
    pub const fn with_id(self, time_zone_id: TimeZoneBcp47Id) -> TimeZoneInfo<models::Base> {
        TimeZoneInfo {
            offset: Some(self),
            time_zone_id,
            zone_variant: (),
            local_time: (),
        }
    }
}

impl TimeZoneBcp47Id {
    /// Associates this [`TimeZoneBcp47Id`] with a UTC offset, returning a [`TimeZoneInfo`].
    pub const fn with_offset(self, offset: Option<UtcOffset>) -> TimeZoneInfo<models::Base> {
        TimeZoneInfo {
            offset,
            time_zone_id: self,
            zone_variant: (),
            local_time: (),
        }
    }
}

impl TimeZoneInfo<models::Base> {
    /// Creates a time zone info with no information.
    pub const fn unknown() -> Self {
        Self {
            offset: None,
            time_zone_id: TimeZoneBcp47Id::unknown(),
            zone_variant: (),
            local_time: (),
        }
    }

    /// Creates a new [`TimeZoneInfo`] for the UTC time zone.
    pub const fn utc() -> Self {
        Self {
            offset: Some(UtcOffset::zero()),
            time_zone_id: TimeZoneBcp47Id(tinystr::tinystr!(8, "utc")),
            zone_variant: (),
            local_time: (),
        }
    }

    /// Creates a new [`TimeZoneInfo`] with the given offset and time zone ID.
    pub const fn from_id_and_offset(time_zone_id: TimeZoneBcp47Id, offset: UtcOffset) -> Self {
        Self {
            offset: Some(offset),
            time_zone_id,
            zone_variant: (),
            local_time: (),
        }
    }

    /// Sets a local time on this time zone.
    pub const fn at_time(self, local_time: (Date<Iso>, Time)) -> TimeZoneInfo<models::AtTime> {
        TimeZoneInfo {
            offset: self.offset,
            time_zone_id: self.time_zone_id,
            zone_variant: (),
            local_time,
        }
    }
}

impl TimeZoneInfo<models::AtTime> {
    /// Sets a zone variant on this time zone.
    pub const fn with_zone_variant(self, zone_variant: ZoneVariant) -> TimeZoneInfo<models::Full> {
        TimeZoneInfo {
            offset: self.offset,
            time_zone_id: self.time_zone_id,
            zone_variant,
            local_time: self.local_time,
        }
    }
}

impl TimeZoneInfo<models::Full> {
    /// Creates a new [`TimeZoneInfo`] for the UTC time zone
    /// with a reference point at the UNIX Epoch.
    pub fn utc_full() -> Self {
        Self {
            offset: Some(UtcOffset::zero()),
            time_zone_id: TimeZoneBcp47Id(tinystr::tinystr!(8, "utc")),
            zone_variant: ZoneVariant::standard(),
            local_time: (Date::unix_epoch(), Time::midnight()),
        }
    }
}

impl From<UtcOffset> for TimeZoneInfo<models::Base> {
    fn from(offset: UtcOffset) -> Self {
        Self {
            offset: Some(offset),
            time_zone_id: TimeZoneBcp47Id::unknown(),
            zone_variant: (),
            local_time: (),
        }
    }
}
